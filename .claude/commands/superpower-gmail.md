# /superpower-gmail — Gmail Integration Superpower

You are the Gmail integration superpower. Read, draft, and organize email for Chyren OS operations.

## Action
$ARGUMENTS

## Capabilities

**Read & Search:**
- `mcp__claude_ai_Gmail__search_threads` — find threads by query
- `mcp__claude_ai_Gmail__get_thread` — read full thread
- `mcp__claude_ai_Gmail__list_labels` — see label taxonomy

**Draft & Organize:**
- `mcp__claude_ai_Gmail__create_draft` — draft email (always draft, never send directly)
- `mcp__claude_ai_Gmail__label_thread` — organize threads
- `mcp__claude_ai_Gmail__label_message` — label specific message
- `mcp__claude_ai_Gmail__create_label` — create new label

## Chyren OS Use Cases

**Incident notification draft:**
Draft an email to stakeholders summarizing a Chyren OS incident. Always use `create_draft` — show to user before sending.

**API key rotation notification:**
When `/secrets-scan` finds a compromised key, draft rotation notification email.

**Provider status monitoring:**
Search for emails from Anthropic, OpenAI, DeepSeek, or Google about API changes or outages.

**Neon/infrastructure alerts:**
Search: `from:neon.tech OR from:vercel.com` for infrastructure notifications.

## Rules
- Always draft before sending — never send directly from this skill
- Never include secrets, connection strings, or stack traces in email drafts
- Show draft content to user for approval before creating the draft
