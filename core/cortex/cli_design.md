# Chyren CLI High-Fidelity TUI Transformation Plan

## Goal:
Transform the basic print-based CLI into a modern, neon-backlit TUI using `Rich` and integrating a `Textual` dashboard experience.

## Phase 1: Aesthetics & Branding
- **Custom Banner:** Implement a color-shifting ASCII/Unicode banner that pulses with neon aesthetics using `Rich` gradient styles.
- **Color Theme:** Define a global "Chyren Neon" palette (Cyan, Magenta, Electric Purple, and Lime Green accents).
- **Syntax Highlighting:** Implement dynamic, color-shifting syntax highlighting for code blocks.

## Phase 2: Interactive UX
- **Live Status Pane:** Implement a persistent status dashboard showing:
    - Current Provider & Latency
    - Ledger Signing Status
    - ADCCL Verification Score
- **Streaming UI:** Transition from block-printing to smooth-streaming responses with a typing effect.
- **Command Palette:** Introduce a command menu (Ctrl+K style) for quick task switching.

## Phase 3: Integration
- Integrate with existing `Chyren` Hub via a wrapper that maintains the ledger and ADCCL state while providing the rich UI layer.
