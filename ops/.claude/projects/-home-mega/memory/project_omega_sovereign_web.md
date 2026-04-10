---
name: OmegA Sovereign Web — Stack & Known Issues
description: omega-sovereign Vercel project details, tech stack, and bugs fixed
type: project
---

## Vercel Project

- **Project:** `omega-sovereign` — `prj_JIYEkVJ0QfTbd0HshlLvUJLVgOYP`
- **Team:** OmegA — `team_XV2729DxcXdAeyqg1ZagLxEY` (slug: `megas-projects-1fcf4ba6`)
- **Repo:** `github.com/Mega-Therion/OmegA-Architecture` (branch: `main`)
- **Web root:** `/home/mega/OMEGA_WORKSPACE/CANON/OmegA-Architecture/web`
- **Framework:** Next.js 16.2.1 with Turbopack, React 19

## Pages

- `/` — full-stack OmegA chat UI (hooks + omega/ components)
- `/tranquility` — voice-first chat interface (canvas orb + TTS)
- `/dashboard`, `/research`, `/about` — supporting pages

## Key Components

- `src/components/omega/` — OmegaHeader, ChatView, ChatComposer, ConversationSidebar, TelemetryPanel, VoiceMode, SystemStateDrawer, SettingsModal, ActionSheet
- `src/hooks/use-omega-chat.ts` — streaming /api/chat, conversation management, telemetry
- `src/hooks/use-voice.ts` — wrapper around useVoiceEngine
- `src/hooks/use-mobile.ts` — responsive breakpoint

## Bug Fixed 2026-03-29

**Symptom:** App rendered unstyled / "looked like shit"

**Root causes:**
1. `tailwindcss` not installed — all `omega/` components use Tailwind classes that were silently no-ops. Fix: `npm install tailwindcss@^4` + `@import "tailwindcss"` at top of globals.css
2. CSP blocked Google Fonts — `font-src 'self'` and `style-src 'self' 'unsafe-inline'` in `next.config.ts` blocked Orbitron/Space Grotesk. Fix: added `https://fonts.googleapis.com` to style-src and `https://fonts.gstatic.com` to font-src.

**Commit:** `3aa3dbc` — "fix(web): install Tailwind v4 and unblock Google Fonts in CSP"

**Why:** Tailwind v4 + Next.js 16/Turbopack requires no config file — just install the package and add `@import "tailwindcss"` to the global CSS. No postcss.config.js needed.
