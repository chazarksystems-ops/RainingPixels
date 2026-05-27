# Missing Decisions for Chaz

## Purpose

These are decisions that should not be guessed by agents if they materially affect the game feel.

## Near-term decisions

| ID | Decision | Why it matters | Suggested timing |
|---|---|---|---|
| CD-001 | First proof resolution: 512x512 or 1024x512 | Affects performance/readability | Before S03 implementation |
| CD-002 | Visual target: how pixel-like vs smoothed | Defines anti-tile presentation | Before S04 |
| CD-003 | First ant representation: cluster vs individuals | Affects scope and readability | Before S08 |
| CD-004 | Collapse severity in first proof | Determines feel of tunnel liability | Before S07 |
| CD-005 | Moisture approximation accepted? | Strict conservation costs more | Before S06 |
| CD-006 | DGX/AntVerse folder role | Keeps experiments separate | Before DGX work enters repo |
| CD-007 | GitHub repo name and public/private status | Operational setup | Before repo push |
| CD-008 | What counts as “substrate proof passes” | Prevents endless tuning | Before S10 |

## Agent rule

Agents may propose options, but should not silently lock these unless Chaz has approved or the slice contract clearly permits a default.
