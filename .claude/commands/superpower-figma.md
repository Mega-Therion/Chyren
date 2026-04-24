# /superpower-figma — Figma Design-to-Code Superpower

You are the Figma design-to-code superpower for Chyren OS web and gateway frontends.

## Action / Figma URL
$ARGUMENTS

## Workflow

**1. Get design context:**
Use `mcp__claude_ai_Figma__get_design_context` with the fileKey and nodeId from the Figma URL.
URL parsing: `figma.com/design/:fileKey/:name?node-id=:nodeId` — convert `-` to `:` in nodeId.

**2. Get screenshot for visual reference:**
Use `mcp__claude_ai_Figma__get_screenshot`

**3. Adapt to Chyren OS stack:**
- **Web** (`web/`): Next.js 15, TypeScript, Tailwind CSS
- **Gateway** (`gateway/`): Vite + React 19, TypeScript

Rules:
- Reuse existing components before generating new ones: `ls web/components/ web/app/ 2>/dev/null`
- Match existing Tailwind token patterns: `grep -r "className" web/app/ | head -20 2>/dev/null`
- No absolute positioning unless the design absolutely requires it
- Code Connect mappings: use `mcp__claude_ai_Figma__get_code_connect_map` to find existing component mappings

**4. Implement:**
Write the component in the appropriate directory. Follow existing naming conventions.

**5. Verify:**
```bash
cd web && npm run typecheck && npm run lint 2>&1
# or
cd gateway && pnpm build 2>&1
```

## Design System
Use `mcp__claude_ai_Figma__get_libraries` to find the Chyren design system library and `mcp__claude_ai_Figma__search_design_system` for specific tokens.

## Output
Component file created, typecheck result, screenshot reference used.
