# Agent operating agreement

## Authority and orientation

Build an understandable interactive book and its tested Rust implementation together. Correct behavior, reproducible instructions, useful presentation, and reader understanding are separate requirements.

The user's current instructions and existing authorization govern the task. Read [status](AUTHORING_STATUS.md), [current brief](LESSON_BRIEF.md), [curriculum](PROJECT_OUTLINE.md), [chapter guidance](CHAPTER_GUIDE.md), and [architecture](ARCHITECTURE.md) before substantive work; use [review guidance](REVIEW_GUIDE.md) for proposals and delivery. Inspect the actual files before trusting status claims.

The root documents are the single source of project guidance. Earlier attempts do not supply current assignments, completion claims, or authorization. Consult external historical material only for a specific current question.

## Scope and stopping

- Work on the authorized assignment only. Roadmap entries, unchecked items, future crate names, and passing tests do not authorize more work.
- Before implementation, present a bounded proposal using the brief fields and obtain approval, unless the user has already authorized that scope. Planning and implementation may be approved together. Do not repeatedly seek permission for routine choices within approved scope.
- A task normally teaches one mechanism or delivers one small authoring capability. Plan chapters by their purpose and boundaries; let section boundaries emerge through authoring and human review. Do not prescribe section counts, fixed size limits, or a detailed section backlog. Judge scope by the new reasoning required from the reader, not files changed or a feature label.
- Complete code, explanation, promised visuals, and relevant checks together. Do not advance the reference implementation beyond the lesson being authored.
- If a missing prerequisite, destructive transition, or material scope expansion emerges, explain the evidence and propose a bounded adjustment before dependent work. Continue unaffected authorized work where useful.
- Submit the actual rendered section and code as **Ready for review** and stop (documents for planning tasks). Human review must judge both writing and reasonable scope; split, expand, or consolidate in response to that review. Only explicit user acceptance permits **Accepted**. Requested corrections remain the active task; do not advance while they are unresolved.
- Commit only when authorized. Never push, publish, or deploy without explicit authorization. Preserve unrelated changes.

## Architecture and teaching

- Treat ARCHITECTURE.md as the destination contract. Introduce a crate, dependency, module boundary, or abstraction only with a current requirement and a teachable explanation. Propose consequential deviations before implementing them; record agreed decisions in status.
- Begin each lesson with an observable problem. Explain why each consequential edit is needed before presenting it. Interleave explanation and focused code; avoid a dense overview followed by a long patch sequence.
- Show every required file, import, manifest change, module declaration, and helper. Provide complete initial files and accessible complete resulting views. Required reconstruction steps must not be hidden in optional exercises.
- Show the meaningful edit with enough context to understand it. Do not mark unchanged lines as removed/added or display entire reconstruction patches as teaching diffs. Keep complete resulting files separately accessible. Code length is a review signal, never a substitute for judging explanation and scope.
- Keep coherent runnable lesson endpoints. Clearly label experiments, pseudocode, intentional failures, and edit groups that compile only when completed together.
- Teach Rust at the point of need. Link to precise Rust Book and Crafting Interpreters sections while explaining what this lesson requires in original prose.
- Use user-accepted lessons as style references once they exist. Establish and review actual teaching examples before expanding output. Keep author logs, replay/fixture machinery, test totals, and repeated defensive qualifications out of the manuscript. Explain necessary limitations where they affect the reader’s reasoning or action.
- For each lesson, state whether a visual helps. A promised visual must answer a specific question at the point of explanation. Follow the visual and accessibility checks in REVIEW_GUIDE.md; never implement a second Lox engine in JavaScript for animation.

## Evidence and handoff

- Check current official documentation and compatibility before selecting dependency versions or using changing APIs. Historical versions are not approved pins.
- Verify actual behavior and the reader's documented route separately, following displayed edits and commands in their stated order, including expected failures and recovery. A patch replay or passing endpoint alone does not validate that route. Add validation infrastructure only when a concrete lesson needs it. Inspect rendered changes; a build alone does not establish visual quality.
- Keep accepted endpoints reconstructible. Git tracks development but is not the sole reconstruction mechanism. Do not silently alter accepted states.
- End with the actual lesson/code or planning deliverables, observed behavior, checks run, unresolved issues, and a concise review focus. Update status and the brief without asserting user acceptance.
- At handoff, identify the accepted starting point, current scope, unfinished work, actual checks, and exact next permitted action. Never leave a generic instruction to continue through the outline.
