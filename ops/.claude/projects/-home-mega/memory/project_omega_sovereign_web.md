---
name: Chyren Sovereign Web — Stack & Known Issues
description: chyren-sovereign Vercel project details, tech stack, and bugs fixed
type: project
---

## Vercel Project

- **Project:** `chyren-sovereign` — `prj_JIYEkVJ0QfTbd0HshlLvUJLVgOYP`
- **Team:** Chyren — `team_XV2729DxcXdAeyqg1ZagLxEY` (slug: `megas-projects-1fcf4ba6`)
- **Repo:** `github.com/Mega-Therion/Chyren-Architecture` (branch: `main`)
- **Web root:** `/home/mega/CHYREN_WORKSPACE/CANON/Chyren-Architecture/web`
- **Framework:** Next.js 16.2.1 with Turbopack, React 19

## Pages

- `/` — full-stack Chyren chat UI (hooks + chyren/ components)
- `/tranquility` — voice-first chat interface (canvas orb + TTS)
- `/dashboard`, `/research`, `/about` — supporting pages

## Key Components

- `src/components/chyren/` — ChyrenHeader, ChatView, ChatComposer, ConversationSidebar, TelemetryPanel, VoiceMode, SystemStateDrawer, SettingsModal, ActionSheet
- `src/hooks/use-chyren-chat.ts` — streaming /api/chat, conversation management, telemetry
- `src/hooks/use-voice.ts` — wrapper around useVoiceEngine
- `src/hooks/use-mobile.ts` — responsive breakpoint

## Bug Fixed 2026-03-29

**Symptom:** App rendered unstyled / "looked like shit"

**Root causes:**
1. `tailwindcss` not installed — all `chyren/` components use Tailwind classes that were silently no-ops. Fix: `npm install tailwindcss@^4` + `@import "tailwindcss"` at top of globals.css
2. CSP blocked Google Fonts — `font-src 'self'` and `style-src 'self' 'unsafe-inline'` in `next.config.ts` blocked Orbitron/Space Grotesk. Fix: added `https://fonts.googleapis.com` to style-src and `https://fonts.gstatic.com` to font-src.

**Commit:** `3aa3dbc` — "fix(web): install Tailwind v4 and unblock Google Fonts in CSP"

**Why:** Tailwind v4 + Next.js 16/Turbopack requires no config file — just install the package and add `@import "tailwindcss"` to the global CSS. No postcss.config.js needed.
