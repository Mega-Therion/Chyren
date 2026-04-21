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
      parameters: z.object({}).passthrough(),
      execute: async (args: any) => {
        const result = await dispatchTool(t.qualifiedName, args as Record<string, unknown>);
        if (result.isError) {
          throw new Error(result.content.map(c => ('text' in c ? c.text : '')).join(' '));
        }
        return result.content;
      },
    } as any);
  }

  return tools;
}
