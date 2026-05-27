# Rejected Systems Registry (for Current Phase)

These directions have been evaluated and are considered incompatible with or premature for the current substrate proof target.

## Rejected for v0 (and likely longer)

| System                        | Reason for Rejection                                      | Status     |
|-------------------------------|-----------------------------------------------------------|------------|
| LBM / Full fluid dynamics     | Overkill for ant-scale capillary + gravity gameplay       | Rejected   |
| Generic engine abstraction    | Violates substrate-first principle                        | Rejected   |
| Full ECS before gameplay      | Inverts priority (infrastructure before validation)       | Rejected   |
| Infinite / procedural world   | Violates single-chunk proof scope                         | Rejected   |
| Flat tile / grid RTS framing  | Directly contradicts material continuity goal             | Rejected   |
| Menu-first colony management  | Moves focus away from world-click material interaction    | Rejected   |
| "Pretty renderer over tiles"  | Produces the wrong visual identity                        | Rejected   |
| Temperature / complex chemistry | Adds complexity without proving core loop first         | Deferred   |
| Networked / multiplayer sync  | Irrelevant until single-player substrate is proven        | Rejected   |

These rejections are scoped to the proof phase. Some may be revisited after validation.