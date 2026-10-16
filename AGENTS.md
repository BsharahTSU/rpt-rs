# STC local fork instructions

- This is the local STC compatibility fork of MrSrsen/rpt-rs. Keep upstream attribution and LICENSE.
- Work on the existing branch. Do not create branches or worktrees or push without explicit user authorization.
- Make focused local commits with the problem, behavior change, and validation documented. Never commit unrelated changes.
- Use Docker Linux for Rust builds, tests, and candidate PDF rendering. Native Crystal baselines run on Windows.
- Start with docs/stc/FORK.md, docs/stc/README.md and docs/stc/WORKLOG.md. Record findings, fixes, evidence, limitations and next steps in Markdown in the same checkpoint as the work.
- School RPTs, database credentials, student selectors, data snapshots, report exports and raw logs belong only in ignored research/. Use synthetic fixtures for committed tests and opaque case IDs in shared documentation.
- Inspect SQL procedure definitions before executing them. Report discovery must not modify student, publication, access, or school configuration data.
- Eligibility, native export success, Rust export success and visual parity are separate gates. Never infer authenticated Parent API success from an offline renderer check.
- Use subagents only with explicit user authorization. Assign separate ownership and have one coordinator make Git commits.
- Checkpoint broad database work: inventory and estimate first, run bounded cases sequentially, record failures without masking them, and commit resumable state.
