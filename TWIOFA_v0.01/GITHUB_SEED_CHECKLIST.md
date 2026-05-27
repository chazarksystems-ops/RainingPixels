# GITHUB_SEED_CHECKLIST.md

Use this checklist before pushing the repository to GitHub.

## Required Root Files

- [ ] `README.md`
- [ ] `AGENTS.md`
- [ ] `CURRENT_PRIORITY.md`
- [ ] `DECISION_LOG.md`
- [ ] `GLOSSARY.md`
- [ ] `FINAL_PATCH_REPORT.md`
- [ ] `00_CONSOLIDATION_README.md`
- [ ] `01_CURRENT_TARGET.md`
- [ ] `02_RUNTIME_SUBSTRATE_TRUTH.md`
- [ ] `03_ANTI_DRIFT_RULES.md`
- [ ] `04_IMPLEMENTATION_BOUNDARIES.md`
- [ ] `05_DEFERRED_SYSTEMS_REGISTRY.md`
- [ ] `06_REJECTED_SYSTEMS_REGISTRY.md`
- [ ] `07_BUILD_SEQUENCE_v0.md`

## Required Folders

- [ ] `contracts/`
- [ ] `runtime/`
- [ ] `docs/`
- [ ] `registries/`
- [ ] `playability/`
- [ ] `visual/`
- [ ] `design/`
- [ ] `slice_backlog/`
- [ ] `slice_contracts/`
- [ ] `repo_foundation/`

## Required Validation

Run:

```bash
find . -type f | sort
find . -type f -name "*.md" -exec wc -l {} \; | sort -n
grep -R "FILL_ME_MARKER" .
grep -R "empty stub" .
grep -R "GENERIC_FILLER_TEXT_MARKER" .
```

Expected:

- no empty required files
- no empty stub files
- no gameplay code
- no unrelated archive dump in root

## Commit

```bash
git add .
git commit -m "Seed TWIOFA substrate-proof foundation"
git push -u origin main
```
