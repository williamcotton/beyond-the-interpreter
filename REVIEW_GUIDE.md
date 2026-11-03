# Proposal, review, and acceptance

This document replaces the repetitive project checklist. It is a review cycle, not authorization to execute future work. Apply the relevant checks to planning and author infrastructure as well as lessons.

## Before work

- Confirm the user's authorization and the exact accepted starting state. Reconcile actual files with status.
- State one objective, a concrete example, prerequisites, the small behavior being added, and explicit exclusions.
- Explain new Rust concepts, crate/dependency/API changes, and any architecture transition. A roadmap entry alone is insufficient justification for adding code now.
- Pair each consequential edit with its teaching explanation. Identify useful references and the purpose/placement of any visual.
- Define the reader observation, meaningful checks, deliverables, and stopping point. Obtain approval only where it is not already present.

## Four human review questions

1. Do I understand why this code is needed?
2. Can I explain its mechanism without repeating the prose?
3. Does the code belong in a clear, understandable place?
4. Does the lesson prepare me for and motivate the next step?

If an answer is no, identify the example, explanation, boundary, or prerequisite that needs revision. Passing implementation tests does not answer these questions.

## Human judgment of section scope

Read the actual rendered section and its code together; an agent summary or passing checks are insufficient. Decide whether an intended reader can follow the reasoning at a reasonable pace. The review should consider:

- Is there a recognizable question and a concrete example before the mechanism is introduced?
- Which ideas are genuinely new, and which prerequisites has the reader already learned? Does a single feature label hide several difficult changes of representation or behavior?
- Does the explanation arrive before or beside the edit it explains? Can the reader predict a useful result before running the code or advancing a visual?
- Does the section reach a satisfying observation without requiring a long batch of unexplained edits?
- Is a routine action padded into its own page, or is a difficult transition compressed to fit an old heading?

The user may request a split, more explanation, a better example, or consolidation with adjacent material. There is no fixed number of sections per chapter, concept quota, word count, or code-line limit. Scope and writing quality require explicit human acceptance. Revise the current section before expanding the manuscript; do not mechanically subdivide every file or helper.

## Explanation and reconstruction

- Open with an observable problem. Explain consequential choices before asking the reader to apply them.
- Interleave focused edits and explanation. Inspect large blocks, whole-file replacements, and repeated unchanged code. A diff must not label unchanged lines as removed/added; inspect the actual rendered display rather than trusting the size of the intended change. Keep complete reference files separately accessible.
- Give paths and stable function/module anchors for edits. Show required imports, helpers, declarations, dependency/configuration changes, and lockfile handling where relevant.
- Provide complete initial files and accessible resulting files for troubleshooting. A disclosure may hold a complete reference view, but must not conceal a required step as an optional exercise.
- Keep lesson endpoints coherent and runnable; label grouped edits and intentional failure experiments, including how to restore a passing state.
- Teach representative tests as evidence of behavior. Keep exhaustive regression listings, test totals, temporary-directory details, fixture generators, and author replay/CI reports out of the narrative. Replace repetitive “apply all edits / all tests pass” reporting with the causal explanation specific to this mechanism. Retain limitations only where they affect the reader’s decisions or understanding.
- Follow the rendered instructions from the stated starting point in order: displayed edits, commands, expected observations, intentional failures, and recovery. Replaying canonical patch data is a separate check. In particular, do not install accepted snapshots before a step that claims to generate initial proposals; verify the reader’s actual baseline and observed result.
- Check source references and external reading. Use original prose; external books supplement the explanation.

## Visual explanation

- Does each promised visual answer the brief's question at the first explanation that needs it, rather than arriving as a recap after the hard work? A delayed explorer does not satisfy a promised visual for an earlier mechanism.
- Can the reader connect code, source positions, operations, and state/structure changes?
- Are contrasting cases included where they reveal the mechanism: failed lookahead, missing operands, shadowing, captured locals, or stepping across a call?
- Does the data agree with the Rust implementation at this exact lesson state, without depending on rules or APIs introduced later? Prefer native observations; small checked authored fixtures are acceptable early. The publication must not contain a competing JavaScript language engine.
- Is useful content present initially and in static/text form? Are controls keyboard-accessible, narrow-screen usable, and compatible with reduced motion? Do not rely only on color or animation.
- Does a static diagram teach the idea sufficiently? Add interaction only when it helps answer a question; arbitrary editable input and a generic trace player are not prerequisites.

Inspect rendered pages and changed interactions. Successful component tests or builds alone do not establish readable layout or adequate teaching coverage.

## Implementation and architecture

- Are the responsibilities and dependency directions in ARCHITECTURE.md respected? Are any deviations explicitly agreed?
- Are new crates/modules/dependencies serving current behavior, with unused future scaffolding absent?
- Are grammar rules coherent, parser mechanics understandable, typed accessors specific, syntax kinds consistent, and malformed children represented accurately?
- Are source/trivia/ranges preserved and Unicode/editor conversions tested where relevant? Does recovery make progress?
- Are diagnostics independent of terminal rendering, runtime data independent of Rowan handles, and protocol adapters thin?
- Is each temporary implementation bounded and retired from production at its intended transition?
- Do meaningful tests check behavior, not just mirror implementation? Have the relevant formatting/lint/build/runtime checks actually run?
- For runtime/debug work, does suspension genuinely control execution? For VM work, do shared language cases still agree?

Do not reorganize code merely to meet a file-length target. Explain the concrete comprehension or maintenance benefit.

## Four kinds of evidence

- **Implementation:** observed behavior and applicable checks.
- **Reader route:** documented edits reconstruct the intended state without hidden work.
- **Presentation:** rendered page and controls inspected, with actual coverage stated.
- **Learning:** the user reviewed the explanation, increments, and visual purpose.

For documentation-only planning tasks, check source coverage, internal consistency, authority, links, and actionable stopping points. Do not run unrelated build suites or imply runtime validation occurred.

## Delivery and acceptance

Submit the actual page/code or planning documents, a short account of behavior/changes, checks actually performed, unresolved issues, and a focused review request. Update status and the brief to **Ready for review**. User acceptance is a separate event.

If revisions are requested, keep the same assignment active and rerun only affected checks plus any necessary regression checks. Do not mark the work accepted or begin the next task to compensate for an unresolved review.

At milestone/chapter boundaries, reread the sequence with the user. Check prerequisites, pacing, terminology, architectural growth, repetition, and promised visual coverage. Replay the cumulative reader route and preserve the accepted endpoint. Agree on checkpoint storage when the first endpoint needs it; Git history alone is insufficient, and an elaborate archive framework is not required up front.
