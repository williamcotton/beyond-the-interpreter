# Actual status and decisions

Last updated: 2026-11-02.

## Current state

- Fresh restart with nine root guidance/roadmap documents and a local publication under `site/`.
- Draft introduction: `site/src/content/docs/index.mdx`, served at <http://localhost:4321/>. Ready for review; no accepted prose/style reference yet.
- Rust implementation: absent. No lesson implementation endpoint has been accepted.
- Publication stack installed and checked locally: Astro 7.4.5, Starlight 0.42.8 (including MDX), React integration 7.1.2, React 19.3.1, TypeScript 6.0.3; exact resolutions in `site/package-lock.json`. Local Node 26.0.0 and npm 11.12.1 meet the selected package requirements.
- Current authorized task: SITE-01, local publication and introduction; delivered for review.
- Hosted CI, distribution, deployment, and cross-platform coverage: none established.
- Current brief: [LESSON_BRIEF.md](LESSON_BRIEF.md).

Historical implementation, completion, and verification claims do not transfer to this restart. The old implementation was inspected read-only as evidence, not as active instructions.

## Accepted direction

On 2026-11-02 the user agreed to the broad destination: an interactive Rust companion to Crafting Interpreters; complete tree-walk and bytecode Lox engines; reusable language services, lossless syntax, diagnostics, lowering and resolution; one executable, thin VS Code integration, LSP and real debugging/DAP; VM collection, inspection, benchmarks, testing, CI, and distribution. The earlier architectural/dependency intentions remain, introduced gradually through understandable problems.

During DOCS-02, the user clarified the teaching workflow: avoid prescribing chapter sections explicitly and use human review to decide whether the sections are well written and reasonably scoped. Additional sections are appropriate when review shows they help; a fixed count or exhaustive breakdown is not required.

These decisions do not establish acceptance of the delivered wording, chapter order, lesson prose, exact APIs, or dependency versions.

## DOCS-01 — Consolidation

Consolidated the new and old guidance into seven root documents. Restored the crate/dependency destination, replaced the execution checklist with a review cycle, removed automatic-commit instructions and the stale website-setup assignment, and distinguished current state from historical claims. At the user's concern about confusion, the source archive was moved outside the repository. No implementation or lesson was accepted.

The consolidation resolved placeholder command behavior and diagnostic-rendering coupling, kept Proptest conditional, and retained the old 38 chapters only as topic references. Initial links and source preservation were checked; DOCS-02 supersedes the document layout and verification count.

## DOCS-02 — Findings applied to guidance

**Authorization:** user requested Markdown changes based on the old implementation review, then steered section planning toward flexible human review.

**Status:** Ready for review; explicit user acceptance pending.

**Evidence examined:** sampled MDX across foundations, scanning, parsing, Rowan, and full syntax; relevant authored change files, the diff renderer, and the existing audit. Fresh source checks counted 94 numbered pages, including 26 before scanning. The reviewed 12.5 combined recovery and typed views with a 170-line AST file and 141-line test file. A four-line syntax-kind addition had 706 unchanged lines displayed as removed and re-added. These are source/editorial observations, not a new browser audit or runtime verification of the old implementation.

**Changes:** CHAPTER_GUIDE.md defines chapter-level purpose and boundaries while leaving section shape to authoring and human review. AGENTS and REVIEW_GUIDE now explicitly require review of actual writing and reasonable reasoning load, meaningful diffs, first-use visuals, and chronological reader-route checks. PROJECT_OUTLINE distinguishes milestone coverage from a section plan and exposes the first-execution sequencing decision. ARCHITECTURE separates reconstruction data from teaching display. README and LESSON_BRIEF identify the current guidance and bounded task.

**Editorial lessons carried forward:**

- Neither a feature label nor file count determines a reasonable section. Avoid both excessive setup fragmentation and compressed difficult transitions.
- Let a concrete question and example organize the explanation; explain the mechanism before or alongside its edit.
- Never present unchanged replacement lines as actual changes. Full source remains accessible separately.
- Put useful visuals at the explanation that needs them, and verify their data against that lesson's exact state.
- Keep author fixture/replay reports and test totals out of the teaching narrative; avoid formulaic edit-batch summaries.
- Verify the displayed procedural sequence, not only canonical patch replay or the final build. Initial snapshot generation and installation of accepted snapshots must be ordered consistently.

**Verification:** Checked all eight live root documents and all 20 local Markdown links. Confirmed one AGENTS.md, no in-project instruction archive, and no implementation scaffolding. Compared every existing crate/dependency-role bullet with the preceding architecture document; all remain unchanged. Reviewed the edits for flexible section boundaries, explicit human acceptance, reader-order verification, and consistent unresolved first-execution sequencing. All live files have final newlines and no trailing whitespace.

**Limits:** no code, site, old MDX, dependencies, or accepted endpoints changed. No runtime/browser/hosted CI/platform checks, commit, push, or publication performed.

## SITE-01 — Local publication and introduction

**Authorization:** on 2026-11-02 the user requested the site and an introduction visible on localhost with the documented publication stack. This directly authorizes this task without asserting acceptance of DOCS-02.

**Status:** Ready for review; user acceptance pending.

**Starting point and direction:** no publication or Rust code existed. The user identified Crafting Interpreters as the teaching reference and added `PROJECT_OUTLINE_DETAILED.md` as a provisional roadmap. It supplements the milestone outline; first-execution sequencing remains unresolved.

**Delivered:** a single MDX introduction at `/`, Astro/Starlight configuration, React integration, strict TypeScript, serif headings and warm light/dark reading palettes, minimal navigation, favicon, exact dependency lockfile, local setup/check/build instructions, and ignore rules. The introduction motivates the project through a short Lox example and explains audience, destination, companion relationship, and reading approach. No algorithm visual is promised for this orientation. No Rust implementation was added.

**Changed paths:** `site/`, `.gitignore`, `README.md`, `LESSON_BRIEF.md`, and this status document. The user-supplied detailed outline and other guidance are preserved.

**Dependency evidence:** checked current official [Starlight setup](https://starlight.astro.build/manual-setup/), [Astro setup](https://docs.astro.build/en/install-and-setup/), and [React integration](https://docs.astro.build/en/guides/integrations-guide/react/) documentation and npm registry engine/peer requirements before installation. Starlight supplies MDX; no duplicate integration is configured. React is ready for later MDX components; this introduction requires no React island.

**Checks observed:** followed `cd site`, `npm ci`, `npm run dev`; clean install succeeded. `npm run check` reported zero errors, warnings, or hints. `npm run build` produced static introduction and 404 pages. Local HTTP returned 200; verified a single introduction heading, expected prose, internal anchor targets, referenced scripts/styles/favicon, and all three external reading URLs. Safari inspection covered desktop opening and ending, contents navigation, light and dark themes, a 390 × 844 responsive viewport, and mobile menu open/close. Restored Auto theme and normal desktop view; left the introduction open at `/`. This is visual/interaction sampling, not a full accessibility or browser matrix audit.

**Build notices and limits:** Starlight emitted missing optional i18n/default-404 content notices; its fallback 404 still built. Sitemap generation was skipped because this local-only site has no canonical deployment URL. npm also reports a pre-existing user-config `always-auth` warning. None prevented checks, build, or serving. No Rust/runtime, hosted CI, real-device, or cross-platform tests; no commit, push, or deployment.

**Review focus:** read the introduction on localhost and judge voice, amount of orientation, relationship to Crafting Interpreters, typography, and navigation. Accepted starting point remains the broad project direction; neither this introduction nor any code endpoint is yet accepted. Requested corrections remain SITE-01; no further manuscript or implementation is authorized.

## Decisions still needed

- User review of the revised guidance and the first actual teaching examples.
- Opening chapter goals, boundaries, and path to useful token inspection; section shape will evolve through review.
- Review of the delivered local publication and introduction (SITE-01).
- When first expression execution belongs: after lossless expression syntax or after the larger syntax/LSP milestone. No route is selected yet.
- Exact versions and concrete lowering, resolution, suspension, and VM heap designs before their dependent work.
- Reader endpoint preservation when the first accepted endpoint exists, and actual release targets/accounts at shipping time.

## Status and handoff conventions

**Proposed** means scope described but not authorized. **In progress** means authorized work is underway. **Ready for review** means the agent delivered evidence and awaits acceptance. **Revision requested** keeps the current work active. **Accepted** requires explicit user acceptance with date/reference.

Keep later task records concise: authorization, outcome, changed paths, actual checks, unresolved issues, and acceptance. Record consequential architecture decisions with their reason and effect on earlier lessons. Do not duplicate the curriculum or maintain a future section checklist here.

## Next permitted action

Review SITE-01 at <http://localhost:4321/> and resolve requested revisions to its introduction or local site. Do not author another chapter or start Rust implementation without user authorization. DOCS-02 remains delivered with acceptance pending; the user’s explicit SITE-01 authorization established the current task.
