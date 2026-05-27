# TWIOFA Playability Feel Locks

## Core Identity

TWIOFA is a **living material yard** game, not a tile-board RTS or menu-driven colony simulator.

The player does not manage a colony through abstract panels. The player shapes a reactive physical yard through world-click orders. Ants then visibly execute those orders inside that yard. The material state of the yard (soil, moisture, tunnels, collapse) is the primary gameplay surface.

## Foundational Interaction Lock

- The player clicks or issues orders **in the world**.
- Ants **physically move through the world** to execute those orders.
- The **material world visibly reacts** to what the ants do (digging removes material, unsupported soil collapses, water wets and weakens dirt, routes become blocked or risky).

Consequence must be **board/world-first**, not text-first or menu-first. The player should understand what happened primarily by watching the yard change.

## Material Consequence as Gameplay

- Bad digging decisions can create real liability (collapse, flooding, blocked return routes, trapped ants).
- Tunnel geometry is gameplay. A poorly shaped tunnel is not just inefficient — it can become dangerous.
- Water and moisture must affect route viability and structural stability in legible ways.
- Collapse is not visual noise. It is a meaningful event that changes the state of the yard and forces the player (and ants) to adapt.

## Visual and Prototype Philosophy

The internal simulation may use a cell grid, but the **intended visual target** is continuous material readability, not a debug grid or clean square tile board.

- Prototype scaffolding (simple visuals) is acceptable during early implementation.
- The direction the prototype must move toward is **Noita-floor material consequence** as a minimum: materials must feel substantial, wetness must be readable, collapse must be legible, and recent disturbance must be visible.
- The proof fails if the result primarily reads as colored squares or a debug tile map.

## Relationship to Historical Work

Early chunk0 / macroquad / tile-grid experiments are **historical reference only**. They helped explore ideas but do not represent the current implementation target or visual direction. The current direction uses GPU packed-cell cellular automata with an emphasis on material continuity and consequence.