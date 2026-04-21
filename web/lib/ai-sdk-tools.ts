/**
 * AI SDK Tool Adapters
 * 
 * Maps Chyren's MCP tool registry to the AI SDK tool format for native streamText support.
 */

import { listAllTools, dispatchTool } from './mcp/registry';
import { tool, type Tool } from 'ai';
import { z } from 'zod';

/**
 * Returns a map of tools compatible with the AI SDK's `tools` parameter.
 */
export async function getSovereignTools() {
  const registered = await listAllTools();
  const tools: Record<string, Tool> = {};

  for (const t of registered) {
    tools[t.qualifiedName] = tool({
      description: t.description,
      // We use a flexible schema since MCP schemas are dynamic.
      // Ideally, we'd map t.inputSchema to Zod, but for now, we allow any object.
      parameters: z.record(z.string(), z.unknown()),
      execute: async (args) => {
        const result = await dispatchTool(t.qualifiedName, args as Record<string, unknown>);
        if (result.isError) {
          throw new Error(result.content.map(c => ('text' in c ? c.text : '')).join(' '));
        }
        return result.content;
      },
    });
  }

  return tools;
}
