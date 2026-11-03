# Agent operating agreement

Build an understandable interactive book and its real Rust implementation. Reader understanding, correct behavior, reproducible instructions, and useful presentation are separate requirements.

## Orientation and authority

Read [status](authoring/AUTHORING_STATUS.md), the [current brief](authoring/LESSON_BRIEF.md), the [chapter roadmap](authoring/PROJECT_OUTLINE.md), [architecture](authoring/ARCHITECTURE.md), and [review guidance](authoring/REVIEW_GUIDE.md) before substantive work. Inspect actual files before trusting status claims. [README.md](README.md) provides local commands and the document map.

The user’s current instructions and existing authorization govern the task. `authoring/` contains the current project guidance, with one owner per concern. Earlier attempts do not supply assignments, accepted endpoints, version pins, or completion claims. Consult historical material only for a specific current question.

## Text first, then implementation

1. Bound the current section in the brief: reader question/example, accepted starting state, behavior, exclusions, new Rust ideas, references, visual purpose, checks, and stopping point. Obtain approval for scope only when it has not already been authorized.
2. Draft the section in MDX, explaining consequential edits before or beside them. Include complete instructions, code, and expected observations. Inspect the rendered page and submit the text and its scope for human review. Do not create or update the reader’s Rust files during this phase, even in a temporary implementation.
3. Once the user is happy with the text and authorizes following it, start from the accepted endpoint and execute those exact instructions in order to create or update Rust. Verify generated files, displayed edits, commands, observations, expected failures, and recovery. The manuscript is the route to follow, not a summary to retrofit afterward.
4. If that pass exposes an error or missing step, correct the manuscript and implementation together and recheck the affected route. Material changes to the teaching scope return to text review before dependent work.
5. Deliver the actual rendered section and matching code as Ready for review, with observed checks and limits. Only the user’s acceptance makes it Accepted. Do not advance to the next section while corrections remain unresolved.

Text approval is not evidence of Rust correctness or final lesson acceptance. Keep this loop local to one section; do not draft many hypothetical chapters or implement ahead. Author-only infrastructure tasks can have a different, explicitly authorized scope.

## Scope and stopping

- Work on the current assignment only. Chapter descriptions, checkpoint goals, passing tests, and future crate names do not authorize more work.
- Preserve the chapter roadmap’s titles and order unless an explained prerequisite or pacing problem leads to an agreed change. Plan by purpose and boundaries; discover section boundaries through actual writing and human review. Do not prescribe section counts, size quotas, or a full section backlog.
- If a missing prerequisite, destructive transition, or material expansion emerges, explain the evidence and propose a bounded adjustment before dependent work. Continue unaffected authorized work where useful.
- Commit only when authorized. Never push, publish, or deploy without explicit authorization. Preserve unrelated changes and accepted lesson states.
- Match the established commit message style: a lowercase type followed by a colon and a concise, lowercase imperative summary, such as `feat: pair section 1.1 with the first rlox executable` or `docs: consolidate authoring guidance`. Use `feat:` for new capabilities, `fix:` for corrections, and `docs:` for documentation-only changes. Check recent commit subjects before composing a message.

## Teaching and architecture

- Use [ARCHITECTURE.md](authoring/ARCHITECTURE.md) as the destination contract. Introduce crates, dependencies, module boundaries, and abstractions only for a current requirement with a teachable explanation. Record agreed consequential deviations in status.
- Begin with an observable problem. Interleave connected explanation and focused code rather than an overview followed by a patch batch. Teach Rust at the point of need, in original prose, with precise Rust Book and Crafting Interpreters links.
- Show every required file, import, manifest change, module declaration, and helper. Give complete initial files and accessible complete resulting views. Optional exercises must not hide required work.
- Show meaningful edits with context. Never mark unchanged lines as removed/added. Keep reconstruction data separate from teaching display; use existing publishing features until a lesson requires more.
- Keep coherent runnable endpoints. Label pseudocode, experiments, intentional failures, and edit groups that compile only when completed together.
- Use accepted pages as style references; consult their actual paths in status. Keep author logs, fixture machinery, test totals, and repetitive defensive qualifications out of the manuscript.
- For each section, state whether a visual helps. Put promised visuals at the explanation that needs them; check them against that section’s actual Rust state and the accessibility criteria in the review guide. Never implement a second Lox engine in JavaScript for animation.

## Evidence and handoff

Check official documentation and compatibility before adopting changing APIs or exact dependency versions. Check behavior and the reader’s ordered route separately; a final build or patch replay alone does not verify the route. Add validation infrastructure only for a concrete need. Inspect rendered changes; a build alone does not establish visual quality.

Keep accepted endpoints reconstructible beyond Git history alone. Complete code listings and ordered commands suffice for the first tiny endpoint; agree on stronger checkpoint storage before later edits would make earlier states inaccessible.

Update status and the brief with phase, actual checks, limitations, acceptance evidence, and the exact next permitted action. End with the deliverable and a focused review request. Do not leave a generic instruction to continue through the outline.
