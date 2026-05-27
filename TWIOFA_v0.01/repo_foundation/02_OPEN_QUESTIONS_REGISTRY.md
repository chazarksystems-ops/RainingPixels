# Open Questions Registry

## Purpose

Open questions are preserved so they can be answered in the right slice instead of derailing the current proof.

## Active v0 questions

| ID | Question | Current slice | Decision status |
|---|---|---|---|
| OQ-001 | Should v0 grid be 512x512 or 1024x512? | S03 | Needs implementation measurement |
| OQ-002 | Should falling movement use block CA or reservation/commit first? | S03/S06/S07 | Needs dev choice |
| OQ-003 | How visually continuous must cell rendering be before playtest? | S04 | Needs visual acceptance |
| OQ-004 | Should strict moisture conservation be implemented in v0, or capped absorption approximation? | S06 | Needs dev/time tradeoff |
| OQ-005 | How many ants are enough to prove material consequence? | S08 | Needs playtest |
| OQ-006 | Is scent stored only in `aux_0` for v0 or mirrored into a debug buffer? | S08 | Needs implementation decision |
| OQ-007 | Should collapse be physical falling material in v0 or a two-step instability-to-loose-soil rule? | S07 | Needs slice decision |
| OQ-008 | What exact player input proves world-click visible execution? | S05 | Needs control prototype |
| OQ-009 | What is the first pass/fail playtest script? | S10 | Needs Chaz playtest gate |
| OQ-010 | What old Grok design questions map to future slices? | Repo foundation | Needs Grok harvest |

## Rule

Do not answer future-system questions inside the substrate proof unless they are required to make the proof playable.
