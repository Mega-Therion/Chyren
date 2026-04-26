---
name: chyren-sovereign-ui
description: Sovereign UI/UX standards and implementation for Chyren. Use for minimalist, high-impact interface design, CSS/TSX component styling, and guiding operator flow within the Chyren HUD.
---

# Sovereign UI/UX Skill

This skill enforces Chyren's aesthetic and functional standards.

## Design Philosophy

- **Minimalist Clarity**: Eliminate all PII and unnecessary visual clutter. Surface only critical state transitions.
- **Impactful Statements**: Replace descriptive status logs with bold, high-contrast, state-driven UI cues.
- **Guided Flow**: Utilize the HUD to walk operators through complex ADCCL gate approvals.

## Implementation Guidelines

- **Typography**: Use high-contrast, clean sans-serif typography.
- **Color Palette**: Dark-mode, high-impact black/gray/accent.
- **Glassmorphism**: Use subtle transparency and blurring for layering.
- **Components**: Refer to `references/component-library.md` for standardized React/Tailwind/CSS primitives.
- **Architecture Integration**: All UI components must hook into Chyren's four-layer stack (AEGIS/AEON/ADCCL/MYELIN) and the HUD surface.

## References

- [Component Library](references/component-library.md)
- [Interaction Standards](references/interaction-standards.md)
