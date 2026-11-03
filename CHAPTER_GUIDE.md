# Chapter planning and section development

This guide governs the instructional shape of chapters. [PROJECT_OUTLINE.md](PROJECT_OUTLINE.md) owns the overall milestones and topic destinations; [ARCHITECTURE.md](ARCHITECTURE.md) owns the technical destination; [LESSON_BRIEF.md](LESSON_BRIEF.md) owns the one current assignment. This is not a numbered section plan or an execution checklist.

## Plan chapters, discover sections

At chapter level, agree on the reader's central question, the relevant starting capability, and the outcome worth reaching. State the prerequisites, concepts and architectural changes the chapter needs, useful examples/visuals, and what belongs later. Explain how the chapter prepares the next capability.

Do not allocate a fixed number of sections or prewrite an exhaustive breakdown. As a chapter is authored, propose a manageable section using the current brief, then review the actual rendered writing and code with the user. Its scope may need to expand for explanation, split at a meaningful intermediate observation, or combine with adjacent material. Update chapter expectations when review reveals a pacing problem; do not treat the first outline as a contract to compress the prose.

A chapter goal can require many sections. A section can contain several closely related edits or supporting Rust ideas. Neither “one feature” nor “one file” establishes a reasonable amount of learning. Human review judges whether the reader has enough context and time to understand the mechanism.

## What a chapter proposal should make clear

Use a concise description, not a rigid reader-facing template:

- **Question and payoff:** what concrete problem drives the chapter, and what can the reader do and explain afterward?
- **Starting point:** what code and understanding already exist, and what is genuinely new?
- **Scope and boundaries:** which language behavior, Rust ideas, architectural transitions, and dependency choices belong here; what is deferred?
- **Teaching material:** a small motivating example, meaningful contrasting/error cases, and the visual questions that matter.
- **Endpoint and bridge:** the observable outcome, how it is checked, and why the next capability becomes useful.
- **Pacing risks:** difficult representation changes, unfamiliar Rust concepts, long stretches before useful behavior, or tempting infrastructure detours.

The proposal does not need section titles or counts. Record a brief local teaching sequence when it helps review the current section, without turning it into a whole-chapter work queue.

## Size by the reasoning the reader must perform

An implementation task can hide several independent learning problems. The old “Keep an Incomplete Tree” section changed the parse-result contract, introduced recovery, added typed wrappers and missing-child semantics, migrated tests, and changed an example in one batch. A coherent feature label did not make that amount of new reasoning manageable.

When approaching comparable material, identify the separate questions: what survives an incomplete expression, who owns an unconsumed delimiter, and how a typed view distinguishes missing operands. Develop and review the explanations before deciding which questions fit together. This is a diagnostic example, not a required three-section replacement.

Likewise, do not give every setup action its own page. The first attempt had 26 numbered pages before scanning began. The issue is the distance to a useful result, not a prohibited page count. Introduce routine setup in proportion to its difficulty; move reference/troubleshooting detail out of the main route where appropriate while keeping required actions discoverable.

## Let examples organize the explanation

For multiplication/division, “add a factor rule” describes an implementation task. The reader's question is why `8 / 4 / 2` groups as `(8 / 4) / 2` rather than `8 / (4 / 2)`.

Use that example to establish the desired tree, show how a growing left operand produces it, and explain the Rust ownership move alongside the relevant edit. Let the reader predict the next tree change and then inspect it. Keep complete files and exhaustive regression cases available without forcing them into that explanatory sequence.

For lookahead, use both `!=` and a failed match such as `!λ` to explain consumption and byte ranges. Place the visual with that explanation. A later scanner showcase cannot replace the explanation when lookahead is first introduced.

These illustrate an approach, not mandatory prose or a repeated set of headings. Write connected explanations specific to the current mechanism. Avoid chapter after chapter of “add one layer,” “apply all edits,” and verification reports.

## Review the section before increasing output

Use [REVIEW_GUIDE.md](REVIEW_GUIDE.md) to assess the actual reading experience: writing quality, reasonable scope, code presentation, visual placement, and the reader's procedural route. Passing tests does not establish acceptable prose or pacing.

If understanding requires holding too many unfamiliar ideas at once, revise the current material. Do not merely add a warning that the section is complex or move the explanation after a patch batch. Human review decides whether a split, an earlier example, more explanation, or a prerequisite adjustment solves the problem.

Establish accepted examples of a code introduction, a focused modification, and an algorithm explanation early. A good example is evidence of the desired standard, not a template to reproduce mechanically. Review consecutive sections at chapter boundaries to catch repetition and cumulative overload.

## Opening and runtime sequencing decisions

The next planning proposal should stay at chapter level: identify a short path to inspecting actual tokens, with the timing of source ranges, Clap, and workspace extraction justified by current use. Keep minimal publication setup separate from the reader's journey. Do not recreate the previous 26-page opening as a default.

Before the syntax/runtime chapters are fixed, review when the reader first executes an expression. Completing full syntax and LSP first is one route; introducing a small evaluator after lossless expression syntax is another. Compare prerequisites, reader payoff, source origins/lowering, and later evolution. Recommend a route for user review. The broad architecture is retained, and no route or implementation is authorized solely by its mention here.

## Using the previous attempt

The earlier implementation can supply candidate examples, malformed-input cases, invariants, and native observations. Check each against the current lesson's source state before reuse. Its prose, headings, section sizes, frozen checkpoints, and claimed completion do not define the new book.

Keep this guidance self-contained. Do not copy the old chapters or instruction archive into the repository to serve as implicit assignments. Record only the current decisions and accepted examples as work proceeds.
