// OpenAI-compatible tool-use loop (works with OpenRouter, OpenAI, and any compat endpoint).
// Loops model ↔ tool calls until a final text answer is produced (or iteration cap hit).

import { listAllTools, dispatchTool, type RegisteredTool } from './registry'

interface ChatMsg {
  role: 'user' | 'assistant'
  content: string
}

interface OAIToolCall {
  id: string
  type: 'function'
  function: { name: string; arguments: string }
}

interface OAIMessage {
  role: 'system' | 'user' | 'assistant' | 'tool'
  content: string | null
  tool_calls?: OAIToolCall[]
  tool_call_id?: string
}

interface OAIResponse {
  choices: Array<{
    finish_reason: string
    message: {
      role: string
      content: string | null
      tool_calls?: OAIToolCall[]
    }
  }>
}

const MAX_TOOL_ITERATIONS = 5

function toolsForOAI(tools: RegisteredTool[]) {
  return tools.map((t) => ({
    type: 'function' as const,
    function: {
      name: t.qualifiedName,
      description: t.description,
      parameters: t.inputSchema,
    },
  }))
}

export async function runAnthropicWithTools(
  _apiKey: string,
  _model: string,
  systemPrompt: string,
  history: ChatMsg[],
  temperature: number,
): Promise<{ text: string; toolCalls: Array<{ name: string; ok: boolean }> }> {
  const openrouterKey = process.env.OPENAI_API_KEY
  const geminiKey = process.env.GEMINI_API_KEY

  if (!openrouterKey && !geminiKey) {
    throw new Error('No OpenRouter or Gemini key — skipping tool-use')
  }

  const tools = await listAllTools()
  if (tools.length === 0) {
    throw new Error('No MCP tools registered')
  }

  // Prefer OpenRouter (can route to Claude or any model); Gemini OpenAI-compat as fallback
  let endpoint: string
  let authHeader: string
  let toolModel: string

  if (openrouterKey) {
    const isOpenRouter = openrouterKey.startsWith('sk-or-')
    endpoint = isOpenRouter
      ? 'https://openrouter.ai/api/v1/chat/completions'
      : 'https://api.openai.com/v1/chat/completions'
    authHeader = `Bearer ${openrouterKey}`
    toolModel = process.env.TOOL_USE_MODEL ?? (isOpenRouter ? 'anthropic/claude-3-5-haiku' : 'gpt-4o-mini')
  } else {
    // Gemini OpenAI-compatible endpoint
    endpoint = `https://generativelanguage.googleapis.com/v1beta/openai/chat/completions`
    authHeader = `Bearer ${geminiKey}`
    toolModel = process.env.TOOL_USE_MODEL ?? 'gemini-2.0-flash'
  }

  const messages: OAIMessage[] = [
    { role: 'system', content: systemPrompt },
    ...history.map((m) => ({ role: m.role, content: m.content })),
  ]
  const toolCalls: Array<{ name: string; ok: boolean }> = []

  for (let iter = 0; iter < MAX_TOOL_ITERATIONS; iter++) {
    const resp = await fetch(endpoint, {
      method: 'POST',
      headers: {
        Authorization: authHeader,
        'Content-Type': 'application/json',
        'HTTP-Referer': 'https://chyren.org',
        'X-Title': 'Chyren',
      },
      body: JSON.stringify({
        model: toolModel,
        messages,
        tools: toolsForOAI(tools),
        tool_choice: 'auto',
        max_tokens: 1024,
        temperature,
      }),
    })

    if (!resp.ok) {
      const body = await resp.text().catch(() => `${resp.status} ${resp.statusText}`)
      throw new Error(`Tool-use request failed (${resp.status}): ${body}`)
    }

    const payload = (await resp.json()) as OAIResponse
    const choice = payload.choices?.[0]
    if (!choice) throw new Error('Tool-use: empty choices array')

    const assistantMsg = choice.message
    messages.push({
      role: 'assistant',
      content: assistantMsg.content ?? null,
      tool_calls: assistantMsg.tool_calls,
    })

    if (choice.finish_reason !== 'tool_calls' || !assistantMsg.tool_calls?.length) {
      const text = (assistantMsg.content ?? '').trim()
      return { text, toolCalls }
    }

    for (const call of assistantMsg.tool_calls) {
      let args: Record<string, unknown> = {}
      try { args = JSON.parse(call.function.arguments) } catch { /* bad json — pass empty */ }

      const result = await dispatchTool(call.function.name, args)
      const text = result.content.map((c) => ('text' in c ? c.text : '')).join('\n')
      messages.push({ role: 'tool', tool_call_id: call.id, content: text })
      toolCalls.push({ name: call.function.name, ok: !result.isError })
    }
  }

  throw new Error(`Tool-use exceeded ${MAX_TOOL_ITERATIONS} iterations`)
}
