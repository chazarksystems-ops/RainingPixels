# Minimal Ant Interaction Contract

## Purpose
Ants exist in this prototype only to make material consequence legible and consequential.

## Minimum Required Behavior
- Visible movement through open cells
- Ability to receive and act on a dig target from player input
- Removal of adjacent diggable material
- Simple scent deposit along path
- Reaction to blocked or collapsed routes (stop, reroute, or fail visibly)

## Allowed Simplifications
- Ants may be represented as a small cluster or particle group rather than fully individual agents
- Pathing may be simple (greedy, local steering, or temporary CPU assistance)
- No requirement for complex individual state machines or long-term planning

## Success Signal
When terrain changes (digging, collapse, water), the player can see ants being affected by those changes without needing to read logs or UI.