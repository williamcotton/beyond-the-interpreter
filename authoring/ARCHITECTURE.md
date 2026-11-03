# Destination architecture

This document owns the destination architecture agreed in broad terms on 2026-11-02. The [roadmap](PROJECT_OUTLINE.md) owns chapter coverage and [status](AUTHORING_STATUS.md) records what exists. These responsibilities are constraints, not permission to scaffold future systems. Exact APIs, versions, and section-sized introductions require a current need and the text-first review process in [REVIEW_GUIDE.md](REVIEW_GUIDE.md).

## Product contract

The final system implements canonical Lox through both a tree-walk interpreter and a bytecode compiler/VM. Both support functions, closures, classes, inheritance, and the same observable language semantics, checked by shared conformance cases. Explain and obtain agreement for any semantic deviation.

Expose one native executable, `rlox`. Eventual commands include `run`, `check`, `tokens`, `ast`, `disasm`, `debug`, `lsp`, and `dap`. Select exact flags and engine-selection behavior when needed. A command's name and help must describe its current capability: do not ship a `tokens` command that merely echoes a file or advertise execution before it exists.

The thin VS Code extension registers Lox, supplies basic lexical highlighting, and launches the native LSP/DAP commands. Semantic language logic stays in Rust. Protocol stdout carries only protocol traffic; logging and user diagnostics use appropriate separate channels.

## Repository and crate destination

Use `site/` for the publication, `rlox/` for reader code, and `authoring/` for author-only support. The eventual Cargo workspace lives under `rlox/`; crates live under `rlox/crates/`, with the editor extension under `rlox/editors/vscode/`. The accepted starting package is named `rlox` and lives directly under `rlox/`. At the justified workspace extraction, internal packages use `rlox-*` and the app continues to expose the `rlox` executable. Explain and show that migration rather than rewriting the accepted starting point.

Create each crate only when its responsibility becomes useful and explain the extraction. Clear modules may precede a crate. This is the intended destination map; changes require an explicit rationale rather than silently discarding the plan.

- **app**: executable entry point, command dispatch, filesystem/process interaction, and terminal presentation. It assembles services; it does not own language algorithms.
- **text**: file identity, source ranges, validated slicing, and coordinate conversions. Begin with simple types and explain migration to Rowan-compatible coordinates when introduced.
- **diagnostics**: structured diagnostic data, severity, and source labels/ranges. It can depend on text, but its core must not pull in terminal presentation or protocols.
- **syntax**: token kinds, Logos scanning, our recursive-descent grammar, Rowan tree construction, recovery, and typed syntax views. Separate grammar, mechanics, kinds, views, and inspection as they grow.
- **hir**: a compact lowered program representation with stable identities and source origins. HIR means a representation suited to semantic work and execution rather than the full concrete source tree. Lowering may read syntax; executable data must not contain Rowan node handles.
- **analysis**: lexical scopes, declarations, reference resolution, context diagnostics, and semantic queries. Begin with straightforward collections and typed IDs.
- **editor**: document/revision coordination and editor-independent operations such as definition, references, rename, hover, symbols, completion, and semantic tokens. Keep their limitations consistent with dynamic Lox semantics.
- **runtime**: tree-walk execution, values, environments, calls, closures, classes, and the execution state needed for observation and later suspension.
- **debug**: breakpoint and step control plus inspection over real execution state. Reuse a coherent debugger interface across engines where their behavior permits; do not force identical internal representations.
- **bytecode**: instructions, chunks, constants, source maps, disassembly, and compilation from the existing front end/lowered representation. A compiler module can precede any separately justified crate.
- **vm**: operand stack, instruction dispatch, frames, closures/upvalues, objects, tracing garbage collection, and VM observation/control.
- **lsp**: adapt editor services to LSP, including synchronization, coordinate translation, and protocol lifecycle. Name resolution belongs in analysis.
- **dap**: adapt debugger operations to DAP, including launch, breakpoints, steps, stack/scopes/variables, and termination. Suspension belongs in the execution/debugging machinery.

## Dependency direction

Core language services are independently usable from tests, native commands, and adapters. CLI, editor host, and protocol requirements must not leak into lower-level language data.

- Text is foundational; diagnostics use text; syntax uses text/diagnostic data and its selected lexer/tree infrastructure.
- Lowering converts syntax into semantic/executable data while retaining source origins. Analysis and editor queries use the relevant syntax and lowered information without coupling execution to concrete trees.
- Runtime and compilation consume appropriate lowered/resolved data. The VM executes compiled chunks. Do not introduce a permanent second production lexer/parser for compilation.
- Debug control consumes engine observation and suspension interfaces. Runtime and VM must not depend on DAP types; analysis must not depend on LSP types.
- LSP consumes editor services, DAP consumes debugger services, and app coordinates commands. Prevent cyclic crate dependencies.
- Keep Ariadne in an app/presentation adapter or behind an explicit optional rendering feature. A normal syntax consumer must be able to use diagnostics without inheriting terminal rendering.

Some responsibilities are shared without requiring identical storage: tree-walk and VM values, closures, and frames may differ. Share language semantics, source-origin conventions, conformance cases, and suitable service interfaces; justify representation reuse separately.

## Planned dependency roles

These library intentions come from the earlier outline. Verify current official APIs, maintenance, compatibility, and exact versions when adoption is authorized; do not copy historical pins.

- **Clap**: command parsing, validation, and generated help after a small manual argument example reveals the need.
- **Logos**: production lexing after a deliberately limited handwritten scanner. Preserve trivia and error spans; compare shared behavior before migration, then extend to complete lexical syntax.
- **Rowan**: lossless tree storage/traversal, positioned nodes, and typed views. We continue to write the grammar algorithm; Rowan is not a parser generator.
- **Ariadne**: terminal diagnostic presentation from independent diagnostic data. A brief handwritten example may clarify rendering, but is not mandatory busywork or a second permanent renderer.
- **Insta**: reviewed syntax/diagnostic snapshots alongside semantic assertions. Snapshot approval is deliberate; CI and reader verification must not silently bless changed output.
- **Tower-based LSP infrastructure**: protocol scaffolding after explaining a small JSON-RPC exchange. Select the actual maintained package and supporting runtime/serialization dependencies at that stage; no exact LSP or DAP library is committed yet.
- **Criterion**: reproducible benchmarks after explaining the limitations of simple timing.
- **Astro, Starlight, MDX, React, TypeScript**: the existing publication stack; exact installed versions live in `../site/package.json` and its lockfile. Starlight currently provides code listings and focused diffs. Add custom viewers or interaction primitives only when the current lesson requires capabilities those features do not supply.

Conditional choices include Proptest, Salsa, Ropey, arena/slot-map libraries, additional protocol libraries, and release automation such as dist. Introduce only for a demonstrated requirement. The old testing chapter prescribed Proptest while its scope section called it optional; this consolidation treats generated-property tooling as conditional and preserves the requirement for meaningful invariant testing.

## Transitions to design before teaching dependent code

Record the decision when its prerequisite becomes relevant. Thinking ahead is required; building the future subsystem early is not.

1. **Handwritten scanner to Logos.** Keep the prototype to punctuation/operators, identifiers, whitespace/comments, and errors. Reserve numbers, strings, and keywords for Logos. Preserve a small regression corpus and retire the superseded production scanner.
2. **Owned expression AST to Rowan.** Limit the owned model/parser to arithmetic, grouping, and unary operations. Choose and explain tree-builder checkpoints or another justified construction mechanism before migration. Preserve grammar behavior, source, trivia, malformed text, and useful errors; then grow the grammar.
3. **Source coordinates.** Preserve UTF-8 byte ranges. Explain capacity limits and distinguish bytes, Unicode scalars, UTF-16 units, and display columns. Coordinate migration and editor conversion need explicit tests and discoverable caller edits.
4. **Syntax to execution.** Before evaluation, establish the smallest lowered representation that avoids runtime dependence on Rowan. Explain lowering and source mapping. Introduce further HIR machinery only when resolution/execution requires it. If an earlier expression evaluator is chosen, preserve this boundary and grow that runtime toward the final engine rather than adding another throwaway execution path.
5. **Resolution and closures.** Before teaching captured environments, account for lexical binding identity and later static resolution. Demonstrate the behavior that naive mutable environments can get wrong; plan the transition without silently changing Lox semantics.
6. **Recursive execution to suspension.** Before committing to runtime control flow, document a credible route to explicit frames, continuations, or another resumable model. When debugging arrives, teach the transformation in bounded lessons. Trace replay is not stepping.
7. **Interpreter to VM.** Reuse the front end and source origins, introduce bytecode and VM state incrementally, and carry shared semantic cases across both engines. Extend the debugger rather than building an unrelated second one.
8. **Guest heap and Rust ownership.** Before VM objects/closures harden, plan identity, stable references, and roots so tracing collection can be introduced coherently. Explain why Rust's memory safety does not collect unreachable guest-language cycles by itself.

## Implementation constraints

Keep recursive-descent grammar rules readable and in coherent order. Avoid a generic parser framework without a demonstrated need. Introduce typed node views/grouping enums as syntax grows; node-specific methods belong on appropriate types. Missing source children and an accessor applied to the wrong node kind are different situations.

Keep syntax kinds and Rowan conversion together, with no manually synchronized parallel kind lists. Preserve source reconstruction and parser progress on malformed input. Decide and teach resource/depth limits when relevant; do not inherit arbitrary historical numbers.

Temporary implementations must have a stated learning purpose, a bounded lifetime, and a clear migration. Keep historical lesson states separately; retain test-only oracles only where they establish useful independent behavior.

## Teaching displays and reproducible source

Keep the representation used to reconstruct exact source changes separate from the presentation used to teach them. Full-file replacement data may support reconstruction, but must not force a whole-file removed/added display. A teaching diff should distinguish genuinely changed lines from unchanged context; complete before/after files remain available separately. Verify the displayed edit as well as the resulting source.

Text-first authoring does not make provisional examples verified. After text review, follow the actual displayed route to establish the implementation and its evidence.

This is a presentation contract, not a request to build a custom diff framework. Use the smallest publishing mechanism that satisfies it. Reuse a historical fixture only after checking its behavior and code against the current lesson state; a final-chapter trace may depend on grammar not yet taught.

## Scope boundary and delivery

Mandatory scope includes both engines, complete Lox semantics, source/syntax/bytecode inspection, semantic editor services, thin VS Code integration, LSP, real source-level debugging and DAP, VM garbage collection, tests, benchmarks, CI, and distribution.

Plan versioned native archives/checksums, GitHub Releases, a Homebrew tap, VSIX distribution, and installation guidance for verified supported targets. Cover extension/executable compatibility and discovery. crates.io and Marketplace publication depend on actual packaging/accounts and authorization. Prepare and test release artifacts before requesting the final external publication step; never claim hosted CI, availability, or cross-platform support without evidence.

Optional scope remains WASM/Monaco, incremental parsing/computation, a full formatter, modules/imports, advanced inference/refactoring, custom editor webviews, and extra release infrastructure. Discussion in the final outlook does not authorize implementation.
