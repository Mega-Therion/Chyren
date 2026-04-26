# Chyren Gateway

The gateway is a lightweight React + TypeScript + Vite frontend surface used for fast interaction routing and UI experiments alongside the main Next.js app in `../web`.

## Tech Stack
- React 19
- TypeScript
- Vite 8
- ESLint 9

## Scripts
```bash
# install deps
pnpm install

# local development
pnpm run dev

# production build
pnpm run build

# lint checks
pnpm run lint

# preview built app
pnpm run preview
```

## Project Layout
- `src/`: application source code.
- `public/`: static assets (including icons and favicon).
- `dist/`: build output (generated).
- `bundle.html`: bundled artifact for deployment/testing workflows.

## Engineering Notes
- Keep gateway-specific logic here; shared business logic should live in common libraries only when actively reused.
- Maintain strict TypeScript ergonomics for component props and external API shapes.
- Keep lint warnings at zero before merge.

## Relationship to Chyren
- `web/` remains the primary Next.js production shell.
- `gateway/` is the rapid UI/routing companion, useful for lightweight surfaces and isolated experiments.
