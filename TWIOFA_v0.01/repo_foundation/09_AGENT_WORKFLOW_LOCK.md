# Agent Workflow Lock

## Core rule

Agents do not work from the whole repo.

Agents work from one slice contract at a time.

## Required preflight statement

Every agent must state:

```text
assigned slice:
files expected to change:
files forbidden:
source docs read:
implementation/non-implementation boundary:
validation/audit method:
success criteria:
rollback/supersession note:
```

## Work modes

### Mode 1 — Slice authoring

Allowed:
- expand slice contract,
- add acceptance gates,
- clarify player-visible result,
- map source intent.

Forbidden:
- start implementation,
- create broad engine plan,
- change global locks.

### Mode 2 — Slice audit

Allowed:
- identify contradictions,
- check against locks,
- recommend patch.

Forbidden:
- implement code,
- redesign slice without scope.

### Mode 3 — Implementation

Allowed only after slice contract is accepted.

Must operate on one slice only.

## Anti-drift reminders

Do not drift into:
- flat tile sim,
- menu colony sim,
- generic engine,
- full ECS,
- infinite world,
- LBM/full CFD,
- CUDA-only mainline,
- broad lore expansion before substrate proof.
