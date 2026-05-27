# Excluded Scope

Excluded:

- ant behavior
- digging gameplay
- collapse gameplay
- water simulation rules beyond field definitions
- renderer implementation
- sparse chunking
- ECS
- save/load
- editor
- production UI

## Why These Are Excluded

These systems depend on a stable cell contract but should not be mixed into the contract itself. Keeping this slice small protects the foundation from becoming a general engine pass.

A correct cell-state slice should produce confidence in data interpretation, not a playable demo. The playable demo belongs to the parent substrate proof after this contract is accepted and implemented in a bounded way.
