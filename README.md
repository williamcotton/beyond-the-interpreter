# Beyond the Interpreter

**Building Modern Language Tools in Rust**

An interactive companion to *Crafting Interpreters*. Readers build `rlox`: a native Rust implementation of Lox with a tree-walk interpreter, bytecode VM, reusable language services, editor integration, real debugging, and distribution tooling.

This is a fresh restart. The local book site now contains a draft introduction; the Rust implementation has not started. No implementation, lesson, dependency version, test result, or acceptance status carries over from the previous attempt.

## Preview the book locally

From the repository root:

```sh
cd site
npm ci
npm run dev
```

Open the Local URL Astro prints, normally <http://localhost:4321/>. See
[site/README.md](site/README.md) for prerequisites, checks, and build instructions.
The introduction is a draft for review; later chapters are not scaffolded.

## Start here

Read these documents in order when joining the project:

1. [AGENTS.md](AGENTS.md): authority, scope, working rules, and stopping conditions.
2. [AUTHORING_STATUS.md](AUTHORING_STATUS.md): actual state, accepted decisions, and unresolved questions.
3. [LESSON_BRIEF.md](LESSON_BRIEF.md): the one current assignment and its authorization status.
4. [PROJECT_OUTLINE.md](PROJECT_OUTLINE.md): learning milestones and progression.
5. [ARCHITECTURE.md](ARCHITECTURE.md): destination responsibilities, dependencies, and transition constraints.
6. [CHAPTER_GUIDE.md](CHAPTER_GUIDE.md): chapter purpose and boundaries, flexible section development, and pacing decisions.
7. [REVIEW_GUIDE.md](REVIEW_GUIDE.md): proposal, delivery, and human acceptance checks.

The user-supplied [detailed roadmap](PROJECT_OUTLINE_DETAILED.md) supplements these documents with provisional chapter topics; it does not fix lesson sizes or resolve the open first-execution sequencing decision.

Each document owns one concern. The outline owns the overall progression; the chapter guide governs chapter planning and section development; the brief owns the current assignment. Change the owning document and link to it. Chapter sections are not prescribed in advance: actual writing and reasonable scope are assessed through human review.

## Two products, developed together

- `site/` will contain the publication: explanations, focused code changes, diagrams, exercises, and checkpoints.
- `rlox/` will contain the native toolchain readers build, including its thin VS Code integration.
- `authoring/` will contain author-only fixtures and verification when concrete lessons need them.

`site/` now exists; the other directories remain destinations to create when needed. Readers do not build the website as part of the required curriculum.

## Starting an agent

> Read AGENTS.md, AUTHORING_STATUS.md, LESSON_BRIEF.md, PROJECT_OUTLINE.md, ARCHITECTURE.md, CHAPTER_GUIDE.md, and REVIEW_GUIDE.md. Inspect the repository and reconcile its actual state with the records. State the current authorized task and stopping point. If no implementation is authorized, present only the next bounded proposal. Do not begin website setup, Rust implementation, or future lessons from the roadmap.

Authorization already given in the current conversation remains valid; do not ask for it again merely because a brief has not yet caught up.

## Reference material

The consolidated documents are self-contained; superseded instructions are not kept in this repository. The previous implementation at https://github.com/williamcotton/beyond-the-interpreter is historical reference. Review specific material before reusing it; do not import its architecture implementation, prose conventions, scaffolding, or completion claims wholesale.

The former root `PROJECT_CHECKLIST.md` is superseded by the review cycle in `REVIEW_GUIDE.md`. There is no automatic chapter-by-chapter execution queue.
