# Rendering & Readability Contract

## Primary Goal
The visual presentation must make the player feel they are interacting with a living material substrate, not a grid of colored squares.

## Requirements
- Material masses should read as continuous where appropriate
- Wet vs dry dirt must be visually distinct
- Recent disturbance and collapse must be readable
- Ants must remain clearly visible over terrain
- Grid structure must be de-emphasized by default (soft edges, variation, no hard borders)

## Anti-Requirements
- Do not optimize first for clean tile aesthetics
- Debug grid must be optional and off by default
- Do not treat the internal cell grid as the final visual identity

Success criteria: A new player should be able to infer material state and recent changes primarily through visuals rather than text or overlays.