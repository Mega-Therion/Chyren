// Anthropic native tool-use loop, MCP-aware.
// Loops Claude ↔ tool calls until the model returns a final text answer
// (or a safety stop is hit), then emits the result as an SSE response.

import { listAllTools, dispatchTool, type RegisteredTool } from './registry'

interface ChatMsg {
  role: 'user' | 'assistant'
  content: string
}

interface AnthropicToolUseBlock {
  type: 'tool_use'
  id: string
  name: string
  input: Record<string, unknown>
}

interface AnthropicTextBlock {
  type: 'text'
  text: string
}

type AnthropicContentBlock = AnthropicToolUseBlock | AnthropicTextBlock | { type: string }

interface AnthropicMessage {
  role: 'user' | 'assistant'
  content:
    | string
    | Array<
        | { type: 'text'; text: string }
        | AnthropicToolUseBlock
        | { type: 'tool_result'; tool_use_id: string; content: string; is_error?: boolean }
      >
}

interface AnthropicResponse {
  id: string
  stop_reason: 'end_turn' | 'tool_use' | 'max_tokens' | 'stop_sequence' | string
  content: AnthropicContentBlock[]
}

const MAX_TOOL_ITERATIONS = 5

function toolsForAnthropic(tools: RegisteredTool[]) {
  return tools.map((t) => ({
    name: t.qualifiedName,
    description: t.description,
    input_schema: t.inputSchema,
  }))
}

export async function runAnthropicWithTools(
  apiKey: string,
  model: string,
  systemPrompt: string,
  history: ChatMsg[],
  temperature: number,
): Promise<{ text: string; toolCalls: Array<{ name: string; ok: boolean }> }> {
  const tools = await listAllTools()
  if (tools.length === 0) {
    throw new Error('No MCP tools available — falling back to plain Anthropic')
  }

  const messages: AnthropicMessage[] = history.map((m) => ({
    role: m.role,
    content: m.content,
  }))
  const toolCalls: Array<{ name: string; ok: boolean }> = []

  for (let iter = 0; iter < MAX_TOOL_ITERATIONS; iter++) {
    const resp = await fetch('https://api.anthropic.com/v1/messages', {
      method: 'POST',
      headers: {
        'x-api-key': apiKey,
        'anthropic-version': '2023-06-01',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        model,
        system: systemPrompt,
        max_tokens: 1024,
        temperature,
        tools: toolsForAnthropic(tools),
        messages,
      }),
    })

    if (!resp.ok) {
      const errorBody = await resp.text().catch(() => `${resp.status} ${resp.statusText}`)
      throw new Error(`Anthropic tool-use failed: ${errorBody}`)
    }

    const payload = (await resp.json()) as AnthropicResponse

    // Push assistant turn (whole content array, including tool_use blocks).
    messages.push({
      role: 'assistant',
      content: payload.content as AnthropicMessage['content'],
    })

    if (payload.stop_reason !== 'tool_use') {
      const text = payload.content
        .filter((b): b is AnthropicTextBlock => b.type === 'text')
        .map((b) => b.text)
        .join('')
        .trim()
      return { text, toolCalls }
    }

    const toolUses = payload.content.filter(
      (b): b is AnthropicToolUseBlock => b.type === 'tool_use',
    )
    const toolResults: AnthropicMessage['content'] = []
    for (const use of toolUses) {
      const result = await dispatchTool(use.name, use.input ?? {})
      const text = result.content.map((c) => ('text' in c ? c.text : '')).join('\n')
      toolResults.push({
        type: 'tool_result',
        tool_use_id: use.id,
        content: text,
        is_error: result.isError ?? false,
      })
      toolCalls.push({ name: use.name, ok: !result.isError })
    }

    messages.push({ role: 'user', content: toolResults })
  }

  throw new Error(`Anthropic tool-use exceeded ${MAX_TOOL_ITERATIONS} iterations`)
}
