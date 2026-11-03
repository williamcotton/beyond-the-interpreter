# Curriculum and learning milestones

## Promise and audience

**Beyond the Interpreter: Building Modern Language Tools in Rust** teaches a programmer who knows another language to build, understand, debug, and distribute a complete native Lox toolchain. It is an original, interactive companion to Robert Nystrom's *Crafting Interpreters*, with precise references to that book and the Rust Book.

Teach ownership, borrowing, enums, errors, traits, recursive types, shared ownership, and other Rust concepts when the code gives them a purpose. Do not assume advanced Rust knowledge or reproduce substantial passages from other books. [ARCHITECTURE.md](ARCHITECTURE.md) owns the final technical contract.

## How the journey works

The reader encounters a concrete problem, builds a small mechanism, inspects its behavior, and evolves it when a real limitation appears. A lesson pairs each consequential edit with its explanation. Useful experiments and exercises reinforce the required path; they never conceal required implementation.

Three kinds of transition need different explanations: replace a small mechanism with infrastructure (Clap/Logos), evolve a representation (Rowan/lowering), or expose an existing service (LSP/DAP). Avoid manufacturing a lengthy throwaway implementation just to introduce a library.

The milestones below describe observable capability and understanding. They are not implementation authorization, fixed chapter sizes, or a queue to execute automatically. Plan chapters through their questions, prerequisites, boundaries, and outcomes using [CHAPTER_GUIDE.md](CHAPTER_GUIDE.md). Do not prescribe a complete section breakdown. Propose the current section as needed and revise its scope through human review of the actual writing. Preserve a visible path to the final architecture without scaffolding it early.

## M0 — Read source with a small Rust program

**Reader can do:** build/run the executable, read a Lox file, and understand actionable input errors and honest command help.

**Reader can explain:** Cargo/package basics, owned and borrowed strings, recoverable errors, and why structured command parsing becomes useful.

Start with one small binary. Grow argument handling and file reading before introducing a workspace. Explain source identity and half-open byte ranges through a concrete highlighting/tokenization need. Introduce the first crate extraction and CI when the corresponding code/checks exist.

Pacing question for the opening plan: whether Clap and the workspace belong before or alongside the first token inspector. Do not keep a fake inspection command alive to preserve old chapter ordering. Prefer an early observable lexical result.

## M1 — Inspect real tokens and edit a Lox file

**Prerequisite:** source input and an understood byte-range model.

**Reader can do:** run `rlox tokens` on a file, inspect kinds/ranges/source slices, and install a basic VSIX providing Lox file association, comments, brackets, and lexical highlighting.

**Reader can explain:** scanning, successful/failed lookahead, conditional consumption, variable-length matching, trivia, and the boundary between lexical highlighting and semantic services.

Build the small handwritten scanner, then migrate to Logos and complete lexical syntax. Compare shared behavior and test that tokens partition/reconstruct the original source. Introduce `syntax` when scanning first needs a home. Integrate the token command as soon as a useful subset exists; expand its coverage with the lexer. Keep extension work thin and distinct from native language algorithms.

**Visuals:** cursor and lookahead; retained trivia and source ranges. Include a failed match and multibyte source, not just the happy path.

## M2 — Preserve, inspect, and diagnose syntax in an editor

**Prerequisite:** the lexical toolchain and source-coordinate model.

**Reader can do:** inspect syntax, check malformed programs, and see native diagnostics in VS Code through LSP.

**Reader can explain:** precedence/associativity, recursive descent, owned AST versus lossless CST, typed views, recovery, diagnostic data versus presentation, document revisions, and editor position conversion.

This is a large milestone. The following are areas of learning, not proposed sections or a fixed section count:

1. Represent and parse a small arithmetic-expression subset with owned Rust types.
2. Migrate that subset to Rowan; explain green storage/red context, source retention, infix wrapping, and typed node views.
3. Introduce diagnostic data and its terminal adapter, then teach reviewed snapshots alongside structural assertions.
4. Grow expression grammar, statement/declaration grammar, and recovery through bounded examples. Teach recovery as the relevant malformed cases appear, not only in an end-of-chapter dump.
5. Connect useful syntax inspection/checking commands and protocol-independent document services.
6. Explain one protocol exchange, adopt LSP infrastructure, and expose synchronization, diagnostics, coordinate conversion, and suitable initial semantic tokens.

Do not turn the former Chapter 13 topics into one assignment or assume a grammar feature fits in one section. Establish prerequisite relationships at chapter level; let human review determine section boundaries as writing develops. Recovery, representation changes, and typed access may need more room even when the grammar itself is unchanged. Keep basic parser guarantees distinct from later name/context validation.

**Visuals:** active grammar rule and consumed tokens; precedence/association; growing tree; CST/typed view comparison; missing operands and retained delimiters; recovery that reaches later valid code.

## M3 — Execute complete Lox and navigate its names

**Prerequisite:** the syntax needed by the behavior being evaluated and the syntax-to-execution transition decision. Completing all of M2, including full grammar and LSP, is not inherently required for an initial expression evaluator; its placement remains a planning decision.

**Reader can do:** execute expressions, statements, variables, control flow, functions, closures, classes, and inheritance; use definition, references, rename, hover, symbols, completion, and semantic highlighting where static information supports them.

**Reader can explain:** lowered executable data, values, environments, binding identity, resolution, call frames, captured values, and dynamic dispatch.

Grow the runtime through those behaviors in small steps. Introduce the minimum lowered representation before evaluation; extend HIR/analysis when lexical resolution needs them. Plan the closure/resolution relationship before teaching captured environments. Expose semantic operations first as reusable services, then through the existing editor adapter. Explain limits on dynamic property analysis rather than promising unsupported precision.

**Visuals:** scopes and use-to-declaration links, shadowing, call frames, and closures that outlive their defining call.

## M4 — Control execution through a real debugger

**Prerequisite:** executable Lox, source origins, and a credible suspension design.

**Reader can do:** inspect traces, pause/resume live execution, set breakpoints, step into/over/out, inspect stack/scopes/variables, and debug through VS Code using DAP.

**Reader can explain:** events versus suspended state, how execution resumes correctly, and how a protocol adapter exposes a controller.

Introduce observation, then teach the bounded transformation to resumable execution. Establish the native debugger API/command before DAP. Test actual control over execution, including calls/returns and termination; a trace player is insufficient.

**Visuals:** live versus recorded state, suspended frames, and contrasting step operations.

## M5 — Compile and run bytecode with functions and closures

**Prerequisite:** language semantics, lowered data, and source origins.

**Reader can do:** disassemble chunks, run arithmetic bytecode, compile source expressions, then execute variables, branches, loops, calls, and closures on the VM.

**Reader can explain:** instructions/constants, operand stack, instruction pointer, source maps, jump patching, frame/slot layout, heap identity, and upvalues.

Start with manually constructed chunks and a small dispatch loop. Reuse the existing front end when compiling source. Introduce values/strings/tables, variables/control flow, and calls/closures as distinct groups. Plan stable object references and roots before the heap design hardens. Compare with the original book's streaming compiler without maintaining a second production lexer/parser.

**Visuals:** source-to-instruction mapping, stack changes, jumps, frames, and an upvalue before/after its local leaves the stack.

## M6 — Complete, inspect, debug, and measure the VM

**Prerequisite:** working VM calls/closures and the common debugging concepts.

**Reader can do:** inspect instructions/stack/frames, debug VM source through the existing adapter, execute classes/inheritance, exercise a tracing collector, and compare both engines with reproducible benchmarks.

**Reader can explain:** source versus instruction stepping, roots/reachability/mark/sweep, guest memory management, and the evidence behind an optimization.

Extend the debugger where engine behavior permits shared interfaces. Teach garbage collection and VM classes in bounded groups. Run shared semantic cases throughout VM development. Introduce Criterion after demonstrating measurement limitations; make no performance claim without measurements.

**Visuals:** source stepping over several instructions, root/object graphs, reachable cycles versus unreachable cycles, marking and sweeping.

## M7 — Test and distribute a usable toolchain

**Prerequisite:** the completed core capabilities and incrementally accumulated tests.

**Reader can do:** run the conformance/protocol suites, produce versioned native artifacts and extension packages, and follow verified installation instructions.

**Reader can explain:** implementation versus protocol/integration testing, debug/release builds, target compatibility, checksums, release workflows, and extension/binary discovery/version compatibility.

Consolidate tests already introduced along the way; do not postpone language or integration testing until this milestone. Cover release builds, CI/release separation, supported target checks, GitHub Releases, Homebrew, conditional crates.io publication, VSIX/Marketplace pathways, and installation documentation as separate lessons where needed. Publishing requires separate authorization; local artifacts are valid intermediate outcomes, not evidence of public availability.

Close by explaining how the architecture extends to larger languages. Modules, incremental computation, formatting, and browser integration remain outlook/optional topics unless explicitly brought into scope.

## Relationship to the former 38-chapter plan

Retain these topic destinations as a cross-reference, not fixed new numbering:

- Foundations, former 1–4: executable, source input, Clap, source locations/workspace/CI.
- Text to syntax, former 5–15: small scanner, Logos, token inspection, basic editor support, expression representation/parsing, Rowan, diagnostics/full syntax, syntax inspection, LSP.
- Execution, former 16–25: evaluation, state, control flow, functions/closures, resolution, semantic services, classes, observation, debugger, DAP.
- VM, former 26–35: chunks, dispatch, compilation, values/strings/tables, variables/branches, calls/closures, debugging, collection, classes, benchmarking.
- Shipping, former 36–38: comprehensive language testing, packaging/distribution, and outlook.

The milestone plan deliberately allows early token inspection, smaller full-syntax groups, and evidence-based renumbering. Do not rebuild a detailed 38-chapter task checklist before validating the opening lessons.

## First execution: an unresolved sequencing decision

The old sequence completed extensive syntax/editor work before evaluation. Do not silently preserve that delay just because M2 precedes M3 on this page. Before committing to chapters around the parser/runtime boundary, compare a small expression-evaluation milestone after lossless expression syntax with completing the syntax/LSP milestone first. Explain the reader payoff, prerequisite burden, source-mapping/lowering work, and any rework in each route. Recommend a route for user review before dependent implementation. No early evaluator or reordered curriculum is authorized here.

## Publication and pacing

Build the smallest usable publication surface before the first rendered lesson review. It is an author task, not Chapter 1; keep sample content labeled and replace it with the first real lesson. Do not build a full navigation tree, generic diff system, trace framework, or replay framework in advance.

Establish user-accepted examples early: a simple code introduction, a focused modification, and an algorithm lesson with a useful visual. They may emerge naturally from the opening sequence. Record their actual paths in status only after acceptance.

At every milestone, read the lessons consecutively and review architecture, prerequisites, terminology, repetition, diagram coverage, and cumulative complexity. Resolve recurring defects before proceeding. Review the reader’s experience, not section counts: avoid both prolonged setup before useful behavior and dense transitions that require several unfamiliar mechanisms at once. More sections are appropriate when they improve comprehension; routine actions need not become separate pages. Human review decides whether to split, explain further, or consolidate.
