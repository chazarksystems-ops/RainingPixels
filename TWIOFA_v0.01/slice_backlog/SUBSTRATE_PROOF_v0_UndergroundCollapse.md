# Slice: SUBSTRATE_PROOF_v0_UndergroundCollapse

## Intent
Prove that digging can create meaningful structural risk and route failure through collapse.

## Player-Visible Result
The player digs a tunnel. Over time or through further action, part of the tunnel collapses. This visibly blocks or lengthens a route, forcing ants to reroute or become trapped. The player sees both the cause (unsupported digging) and the consequence (changed yard state + ant behavior change).

## Core Mechanics Exercised
- Digging removal of material
- Support/collapse detection
- Route viability change
- Ant reaction to blocked paths

## Scope for this Slice
- Focus on underground cross-section
- Simple collapse when support is removed
- Visible ant rerouting or failure
- Moisture can optionally contribute to instability

## Excluded from this Slice
- Full multi-chunk world
- Complex ant AI or colony management
- Advanced fluid pressure
- Save/load or progression