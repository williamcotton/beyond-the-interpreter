# Project outline

## Promise and audience

**Beyond the Interpreter: Building Modern Language Tools in Rust** is a web-native interactive companion to Robert Nystrom's *Crafting Interpreters*. Readers build `rlox`, then the infrastructure that makes a language practical to inspect, edit, execute, debug, and distribute.

Readers know another programming language and may be new to Rust. Explain ownership, borrowing, enums, errors, traits, and other Rust concepts when the current code gives them a purpose. Link to precise Rust Book and original-book sections without outsourcing the explanation necessary to understand this lesson.

Preserve Lox semantics. Explain and approve any deviation. Use original prose and examples; do not reproduce substantial passages from other books.

## Teaching approach

The useful progression is: encounter a concrete problem, build a small mechanism, inspect its behavior, then evolve it when a real limitation appears. A library is justified by the requirement it solves. Do not manufacture a lengthy temporary implementation simply to replace it.

The lesson is the unit of authoring and review. Code, prose, and visual explanation develop together. The implementation must not advance chapters ahead of the manuscript. Readers should understand decisions as they make edits, rather than read a summary followed by a batch of patches.

A lesson normally contains a motivating example, relevant starting state, focused explained edits, an observation or interaction, meaningful verification, and a short outcome. Use headings when useful; do not force every lesson into identical boilerplate.

The voice is a knowledgeable programmer walking another programmer through a discovery. Explain mechanisms and tradeoffs with concrete examples. Avoid repetitive qualifications, status language, unnecessary syntax explanations, and unexplained compressed terminology.

## Provisional curriculum

The original 38-chapter map remains a planning reference. These are topic destinations, not approved task sizes, fixed page boundaries, or permission to implement ahead. Refine sequencing with the user as accepted lessons expose prerequisites and pacing problems.

| Part | Provisional topics |
| --- | --- |
| I: Foundations, 1–4 | A Third Lox; Reading Source; Commands and Clap; Source Locations, Workspaces, and CI |
| II: Text to syntax, 5–15 | Handwritten Scanner; Growing Tokens; Logos; Inspecting Tokens; Basic Editor Support; Representing Expressions; Parsing Expressions; Lossless Rowan Trees; Diagnostics and Complete Syntax; Inspecting Syntax; Language Server |
| III: Syntax to execution, 16–25 | Expression Evaluation; Statements and State; Control Flow; Functions and Closures; Name Resolution; Semantic Editor Services; Classes and Inheritance; Execution Observation; Resumable Debugger; DAP |
| IV: Virtual machine, 26–35 | Bytecode; Stack VM; Expression Compilation; Values, Strings, and Tables; Variables and Control Flow; Calls and Closures; VM Inspection and Debugging; Garbage Collection; VM Classes; Optimization and Benchmarking |
| V: Shipping, 36–38 | Language Testing; Packaging and Distribution; Beyond Lox |

Before reaching the former Chapter 13 material, agree on smaller conceptual groups: diagnostic data and presentation; tree assertions and snapshots; expression grammar; program/declaration grammar; recovery. Do not compress all of these into one large implementation assignment.

Show useful observable results along the way. Keep command names and help consistent with their current behavior; avoid long-lived placeholders whose names promise unimplemented capabilities. Consider a visible early milestone such as inspecting real tokens in a Lox file.

## Architecture principles

- Publish under `site/`; reader Rust code lives under `rlox/`; author-only fixtures and validation live under `authoring/`. Create each only when needed.
- Astro, Starlight, MDX, React, and TypeScript are the preferred publishing stack, subject to current compatibility checks. Build the smallest usable lesson page before expanding publishing infrastructure.
- Expose one native executable, `rlox`, with inspection, execution, and later protocol commands. Internal crates use `rlox-*` names.
- Keep CLI, editor, and protocol adapters thin. Core language services must be usable independently of these transports and terminal presentation.
- Use modules to separate grammar, parser mechanics, syntax representation, and printing as they grow. Keep grammar rules in a coherent reading order. Avoid broad whole-file replacements.
- Start with straightforward types. As syntax grows, introduce specific typed node views and grouping enums so node-specific methods belong to the appropriate type. Distinguish missing source children from asking an accessor of the wrong node kind.
- Keep syntax kinds and their Rowan conversion together. Avoid manually synchronized parallel kind lists. Any abstraction introduced to solve this must itself be teachable.
- Diagnostic data should not require terminal-rendering dependencies. Place adapters at an appropriate consumer boundary or behind an explicit feature.
- Preserve source, trivia, malformed text, and UTF-8 byte ranges. Distinguish byte offsets, Unicode scalars, UTF-16 units, and display columns. Introduce coordinate limits and conversions when relevant.
- Handwrite only a small scanner and expression parser before adopting infrastructure. Rowan supplies tree storage and traversal; our grammar algorithm remains ours. Retire superseded production prototypes; retain historical teaching states separately and test oracles only where justified.
- Preserve direct, understandable recursive-descent rules unless demonstrated requirements justify a different design. Do not build a generic parser framework preemptively.
- Plan runtime boundaries so later execution need not depend directly on Rowan nodes. Think ahead about debugger suspension without scaffolding a debugger early. DAP exposes a real resumable debugger; trace playback is not stepping.

## Interactive explanations

Diagrams are a central teaching feature. Identify their purpose in lesson proposals and review their placement with the prose and code.

| Concept | What a reader should be able to inspect or predict |
| --- | --- |
| Source locations | Why the same character can span multiple bytes; how a range selects original source |
| Scanner | Cursor, successful/failed lookahead, conditional consumption, token boundaries, retained trivia |
| Expression parser | Active grammar rule, consumed tokens, precedence, associativity, growing tree |
| Lossless syntax | Original text versus typed roles, green storage/red context, wrapping an existing left operand |
| Recovery | Missing children, retained delimiters, diagnostics, and progress after malformed input |
| Later execution | Scopes, bindings, frames, captured values, suspension/resumption, VM stack, and reachability |

Prefer a few well-chosen examples with meaningful controls over decorative animation or overwhelming dashboards. Fixtures may be precomputed; editable arbitrary input is not required. A static diagram is appropriate when interaction adds little. Keep diagrams faithful to the implementation and accessible without relying solely on motion or color.

## Evidence and scope

Check implementation behavior, reader reproducibility, rendered quality, and instructional clarity separately. A successful final build cannot establish all four. Develop validation incrementally and keep complete troubleshooting code accessible without flooding the narrative.

Introduce testing and delivery progressively. Public release, hosted CI, and platform coverage are separate claims requiring evidence. Record limitations in author notes and relevant setup guidance without repeating them throughout the book.

Mandatory scope includes the native toolchain, thin VS Code integration, LSP, a real debugger and DAP, and both Lox execution engines. WASM/Monaco, incremental parsing, a full formatter, modules, advanced inference/refactoring, and additional infrastructure remain optional and require user approval to enter scope.

Git tracks development. Curated commit history is not the sole reader reconstruction mechanism. Preserve accepted lesson endpoints without freezing an elaborate archive scheme before it is needed.
