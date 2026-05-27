# Recommended Repo Insertion Plan

## Phase A — Seed repo

Create the repo as a documentation-first project.

Initial root files:

```text
README.md
PROJECT_LOCK.md
CURRENT_PRIORITY.md
AGENTS.md
DECISION_LOG.md
GLOSSARY.md
```

## Phase B — Insert current substrate proof

Copy corrected substrate-proof docs into:

```text
03_runtime_specs/
04_playability_specs/
02_slice_contracts/S01_substrate_proof_v0/
```

## Phase C — Insert this foundation registry

Copy this `repo_foundation/` folder into:

```text
00_repo_foundation/
```

## Phase D — Archive source packets

Place old full folders under:

```text
99_archive/source_packets/
```

Do not make archive docs part of the active reading path.

## Phase E — Create slice folders

Create empty-but-structured folders for S01-S10 with the slice template.

## Phase F — First agent task after repo push

Task:
Expand `S01_substrate_proof_v0` and `S02_cell_state_and_gpu_buffers` into complete implementation-ready slice contracts.

Do not write game code yet.
