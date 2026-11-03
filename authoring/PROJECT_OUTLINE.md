# Chapter roadmap

**Beyond the Interpreter: Building Modern Language Tools in Rust** is a web-native, interactive Rust companion to Robert Nystrom’s *Crafting Interpreters*. Readers know another programming language and may be new to Rust. They build `rlox` and the tools that make Lox practical to inspect, edit, execute, debug, test, and distribute.

## How to use this roadmap

This is the single chapter-by-chapter synopsis. It preserves the supplied five-part, 38-chapter plan: titles, sequence, objectives, principal mechanisms, examples, and checkpoints. It describes the destination, not current capabilities or a standing assignment. [Status](AUTHORING_STATUS.md) records what exists; the [current brief](LESSON_BRIEF.md) records authorized work.

Use the chapter order as the working roadmap. Explain a concrete prerequisite or pacing problem before proposing a change; do not silently reorder or expand it. Earlier discussion of executing expressions sooner remains a possible future proposal, not an approved reorder. Chapter 16 remains the roadmap’s expression-evaluation destination.

Chapter descriptions are coverage goals, not section sizes. Numbered shipping subtopics are planning references, not required final page boundaries. Do not prescribe section counts or a full section backlog. Draft the current section, review its actual writing and scope, then follow its approved instructions to implement and verify it. [REVIEW_GUIDE.md](REVIEW_GUIDE.md) owns that workflow and chapter planning; [ARCHITECTURE.md](ARCHITECTURE.md) owns technical responsibilities and dependency choices.

Code/API sketches here illustrate intent. They do not replace complete, verified lesson instructions or select exact dependency versions. Introduce each library, crate, and site component when the current section needs it. Readers build Rust; they do not build this website.

The former all-in-one operating instructions and initial prototype assignments have been consolidated into those owning documents. The local site and first Rust executable already exist. There is no assignment here to scaffold them again or build generic diff/trace components before they are needed.

## Learning progression

The foundation is a small executable and source input. Milestone 1 supplies real token inspection and basic editor support; milestone 2 supplies lossless syntax, diagnostics, and LSP; milestones 3–4 supply full tree-walk Lox, semantic services, and real debugging; milestones 5–6 supply the bytecode engine, inspection, collection, and measurement; milestone 7 supplies tested distribution. These milestones are outcomes to review, not permission to continue automatically.

Three transitions need distinct explanations: a library can replace a small mechanism (Clap/Logos), a representation can evolve (owned AST to Rowan), or an adapter can expose an existing service (LSP/DAP). Do not manufacture a lengthy throwaway implementation just to justify a dependency. Tests and delivery grow along the way rather than waiting for the final chapters.

# Part I — Foundations

## Chapter 1 — A Third Lox

**Objective:** Introduce the project, Rust tooling, and the executable.

**Original reference:** *Crafting Interpreters*, introductory chapters and Chapter 3.

**Implementation:**

Initialize a minimal Cargo binary package named `rlox`.

```bash
cargo new rlox --bin --edition 2024 --vcs none
cd rlox
cargo run
```

Start with a tiny program:

```rust
fn main() {
    println!("Hello from rlox!");
}
```

Teach:

- Rust toolchain installation.
- `rustc`.
- Cargo.
- Packages versus executables.
- `main`.
- `println!`.
- Basic compilation.

**Accepted chapter boundary:** Chapter 1 is complete with Section 1.1, “A program we can run.” The user chose to stop at the greeting executable after reviewing the proposed testing section. No dedicated Git, README, testing, or formatting section is required here. Introduce formatting briefly when useful and introduce tests in Chapter 2 alongside behavior worth preserving.

Do not introduce multiple crates or complex dependencies.

**Checkpoint:** A working Rust binary whose files and greeting output match the section. No automated test cases are required at this endpoint.

## Chapter 2 — Reading Source

**Objective:** Read Lox source from a file.

Begin with:

```rust
let filename = std::env::args()
    .nth(1)
    .expect("expected a file");

let source = std::fs::read_to_string(filename)?;
```

Teach:

- Command-line arguments.
- `String`.
- `&str`.
- Borrowing.
- `Option`.
- `Result`.
- `?`.
- `std::fs`.
- Recoverable errors.

Improve initial error handling incrementally.

Introduce the first tests alongside source-reading behavior and useful input errors. Choose test boundaries to fit the mechanism being taught; a greeting-only process test is not a prerequisite. Do not add a test framework or new dependency before a concrete need appears.

**Checkpoint:** The executable reads `.lox` files and handles missing files, with meaningful tests for the behavior introduced in this chapter.

**Working section frame:** accepted for drafting after the user requested small code increments. These boundaries remain adjustable through review of the actual text:

- **2.1 — Give the program a filename:** select/display one positional argument in main; explain Cargo’s separator, iteration, Option, and temporary missing-input handling. No file I/O yet.
- **2.2 — Read the source:** add one file-read operation and source output, plus a small Lox example. Teach String/borrowing and Result at their use.
- **2.3 — Explain input failures:** replace temporary panics with useful stderr and failure status; put the one-filename/no-extra-arguments rule in a small function in the same binary.
- **2.4 — Check the input rules:** add unit tests to that argument-policy function and demonstrate a regression/restoration. Keep manual filesystem/CLI checks distinct from what these tests prove. Formatting is supporting material.

No process-testing framework, dependency, library extraction, or fixed later-section schedule is required by this frame.


## Chapter 3 — Commands and Clap

**Objective:** Evolve the CLI from a single positional argument to commands.

Build a small manual dispatcher.

Then introduce Clap.

Example:

```rust
#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Tokens { file: PathBuf },
}
```

Explain derive macros, argument validation, help generation, and structured subcommands.

The `Tokens` variant illustrates the later command shape; expose it only when it produces real tokens. Until scanning exists, command names and help must describe the source-reading behavior actually available.

**Checkpoint:** Command parsing, validation, and honest generated help work; `rlox tokens FILE` becomes available with scanning.

**Working section frame:** accepted when the user said “great, move on to 3.1” after reviewing the proposed breakdown. These boundaries remain adjustable through actual text review:

- **3.1 — Give the operation a name:** introduce `rlox source FILE` with a small manual dispatcher. Select the command, apply the existing one-filename rule to its arguments, and route to the existing source read/output. Explain the intentional transition from `rlox FILE`; reject missing/unknown commands and invalid filename counts. Keep tests alongside the changed contract. No help parser, aliases, flags, dependencies, or language behavior.
- **3.2 — Describe the command with Clap:** replace the manual parser with Clap’s derive API, keeping the same source operation. Teach the dependency/derive feature, CLI struct and subcommand enum, generated help, and parsing/validation behavior. Migrate relevant tests to the declared CLI using supplied arguments, and retire superseded manual parsing code/tests. Show help success versus usage failures separately from file-read failures. Verify exact dependency compatibility and lockfile changes when authoring this section; no version is selected here.

**Chapter boundary:** source reading/output remains the only operation. `source` describes that capability honestly; `run` and `tokens` wait for actual execution and scanning. The command form replaces the bare-filename form rather than adding compatibility aliases. No second command is invented to justify dispatch, and no separate section is reserved for repeating tests already needed by these changes. Review actual 3.1 scope before drafting 3.2; split the migration only if its explanation warrants it.

## Chapter 4 — Source Locations, Workspaces, and CI

**Objective:** Establish source-coordinate types and a small workspace.

Start with handwritten types:

```rust
pub struct TextSize(pub usize);

pub struct TextRange {
    pub start: TextSize,
    pub end: TextSize,
}
```

Add:

- Half-open ranges.
- Range validation.
- `FileId`.
- `SyntaxOrigin`.
- Tests for ranges.
- UTF-8 byte offset explanations.

Introduce the Cargo workspace and separate `app` and `text` crates.

Do not add Rowan yet.

Introduce the project's first GitHub Actions CI workflow.

CI should run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Use currently supported actions and security-conscious permissions.

**Checkpoint:** Modular Rust project with passing CI.

**Working section frame:** accepted when the user requested starting 4.1 after reviewing this breakdown. These boundaries remain adjustable through actual writing and review:

- **4.1 — Locate text with byte ranges:** begin with selecting a fragment from an actual UTF-8 source string. Introduce handwritten `TextSize` and half-open `TextRange`, reject reversed ranges, and safely obtain text only when bounds and UTF-8 boundaries permit it. Use a small library target in the existing package (`rlox/src/lib.rs`) so the coordinate API has its own tested public boundary without moving the CLI yet. Cover ASCII, multibyte text, empty ranges, and end-of-file. Distinguish a numerically ordered range from a valid slice of a particular source. A small byte-boundary illustration should answer why a character can occupy more than one byte; no custom interactive component is needed.
- **4.2 — Identify the source file:** show two files with the same numeric range but different text. Introduce `FileId` and `SyntaxOrigin` to associate coordinates with a source identity, and test that distinction. Keep these in the existing text library; do not build path interning, a virtual filesystem, a document database, or editor revisions. Assignment of IDs and source lookup remain explicit in the example; an ID is not proof that a file exists.
- **4.3 — Extract the workspace:** move the working CLI into `rlox/crates/app` (`rlox-app`) and the tested coordinate library into `rlox/crates/text` (`rlox-text`). Preserve the executable name `rlox`, `source FILE` behavior, fixture location, and commands from the workspace root. Teach manifests, member selection, the shared lockfile/target directory, library visibility, and test commands as part of this concrete move. Preserve the accepted listings and lockfile transition. Do not add unused app dependencies or future crates to complete a diagram.
- **4.4 — Run the checks automatically:** introduce the first root `.github/workflows/ci.yml` for the Rust workspace. Run formatting, Clippy with warnings denied, and workspace tests from `rlox/`; teach the working directory and trigger behavior. Check currently supported actions/toolchain setup and minimal permissions when authoring the section. Verify the exact commands locally; claim passing hosted CI only after an authorized GitHub push and an observed successful run.

**Chapter boundary:** keep the source-display CLI unchanged. Test source-coordinate behavior where introduced; workspace moves and CI add no language behavior. No Rowan, scanner, LSP coordinates, generic offset-arithmetic framework, repository creation, push, or publication is implied. Exact APIs, action pins, and reconstruction steps are to be authored and verified per section. No permanent section count is established.

---

# Part II — From Text to Syntax

## Chapter 5 — Building a Scanner by Hand

**Original reference:** *Crafting Interpreters*, Chapter 4.

**Objective:** Understand lexical scanning mechanics.

Create `rlox-syntax`.

Introduce:

```rust
enum SyntaxKind {
    LeftParen,
    RightParen,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Error,
}
```

And:

```rust
struct Token {
    kind: SyntaxKind,
    range: TextRange,
}
```

Implement:

- Single-character punctuation.
- Single-character operators.
- Two-character operators.
- Character iteration.
- Lookahead.
- Token boundaries.

Example:

```rust
'!' => {
    if chars.next_if(|&(_, ch)| ch == '=').is_some() {
        SyntaxKind::BangEqual
    } else {
        SyntaxKind::Bang
    }
}
```

Build the first interactive character-cursor diagram.

**Checkpoint:** A small, tested handwritten lexer.

**Chapter 5 authoring frame (accepted for drafting):** start from the accepted Chapter 4 workspace and ask how source characters become tokens that still point to their original bytes. Keep the handwritten scanner deliberately limited; Chapter 6 owns variable-length tokens/trivia and Chapter 7 replaces the mechanics with Logos.

The user approved beginning with 5.1 and 5.2 along these boundaries; adjust them through review of the actual writing:

- **Read a character, emit a token.** Open with `()` and the need to distinguish source text from token kinds. Introduce the useful `rlox-syntax` library and its dependency on `rlox-text` together with a tiny scanner, rather than a separate scaffolding lesson. Explain SyntaxKind, Token, character iteration with byte offsets, and exclusive token ends. Start with parentheses and single-character `!`/`=`. Tests observe kinds and selected source slices, including empty input. A temporary unmatched-character fallback consumes one complete character as Error; Chapter 6 develops that behavior and its coverage.
- **Look ahead before choosing.** Contrast `!` with `!=`, and `=` with `==`. Add non-consuming lookahead and conditional consumption, explaining why a failed comparison must leave the next character for the next token. Check adjacent operators, a trailing operator, and byte-range boundaries. Extend the character-cursor visual where lookahead first needs explanation.

The first interactive character-cursor diagram should connect source characters, UTF-8 byte positions, consumed input, and emitted token ranges. Use small authored cases checked against this lesson’s Rust implementation; no JavaScript scanner or generic tracing framework. Include a static/text explanation and accessible controls. Detailed placement and fixtures belong to the text brief before implementation.

Each boundary should reach a working library endpoint. The app and its source command stay unchanged; token inspection belongs to Chapter 8. No file database, parser, EOF-token convention, new registry dependency, or complete Lox token inventory is needed here. Exact APIs and further splits/consolidation remain subject to text review.


## Chapter 6 — Tokens That Grow

**Objective:** Extend the handwritten scanner just enough to learn variable-length matching.

Add:

- Identifiers.
- Whitespace.
- Line comments.
- Unknown-character errors.
- Lossless token ranges.

A conceptual identifier continuation rule:

```rust
while chars
    .next_if(|&(_, ch)| {
        ch.is_ascii_alphanumeric() || ch == '_'
    })
    .is_some()
{}
```

Implement the identifier-start condition separately.

Preserve all source bytes, including trivia.

Test the invariant:

**Token ranges partition the entire source without gaps or overlaps.**

Do not implement numbers, strings, and keywords manually.

Those are reserved for the Logos chapter.

**Checkpoint:** A limited but lossless handwritten scanner and regression corpus.

## Chapter 7 — From Handwritten Scanning to Logos

**Objective:** Replace scanner mechanics with declarative lexical rules.

Introduce Logos.

Explain:

- `#[derive(Logos)]`.
- `#[token]`.
- `#[regex]`.
- Token priority.
- Source spans.
- Error handling.
- Derived code.

Example:

```rust
#[derive(logos::Logos, Debug, PartialEq)]
enum LexKind {
    #[token("+")]
    Plus,

    #[token("!=")]
    BangEqual,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
}
```

Implementation sequence:

1. Preserve baseline handwritten tests.
2. Add Logos.
3. Translate existing rules.
4. Compare outputs.
5. Switch the production scanner to Logos.
6. Add keywords.
7. Add numbers.
8. Add strings.
9. Test malformed input.
10. Remove superseded handwritten scanning code.

Preserve the public scanner interface where practical:

```rust
pub fn scan(source: &str) -> Vec<Token>;
```

Do not use trivia-skipping behavior that prevents lossless source reconstruction.

**Checkpoint:** Complete Lox lexer implemented through Logos.

## Chapter 8 — Inspecting Tokens

**Objective:** Build the first useful language tool.

Connect scanning to:

```bash
rlox tokens examples/hello.lox
```

Output should display:

- Token kind.
- Start offset.
- End offset.
- Source slice.

Teach:

- Library/API boundaries.
- Borrowed slices.
- Display formatting.
- CLI integration tests.

Build a website interactive explaining token ranges.

**Checkpoint:** A functional token inspector.

## Chapter 9 — Giving Lox an Editor

**Objective:** Create a minimal VS Code extension.

Use TypeScript for the thin extension.

Introduce:

- Extension manifest.
- Language registration.
- `.lox` file association.
- TextMate grammar.
- Bracket definitions.
- Comment definitions.
- VS Code extension development.
- VSIX packaging.

Do not implement Lox semantic logic in TypeScript.

Teach why TextMate lexical highlighting differs from LSP semantic tokens.

Add a build/package check to CI.

**Checkpoint:** A locally installable VSIX with Lox syntax highlighting.

**Milestone 1:** Lexical toolchain, CLI token inspector, VS Code syntax highlighting, CI.

## Chapter 10 — Representing Expressions

**Original reference:** *Crafting Interpreters*, Chapter 5.

**Objective:** Teach recursive syntax representations.

Implement only a small AST:

```rust
enum Expr {
    Number(f64),
    Grouping(Box<Expr>),
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}
```

Teach:

- Recursive enums.
- `Box<T>`.
- Ownership.
- Tree traversal.
- Pattern matching.

Build AST diagrams.

Do not implement the complete Lox AST by hand.

**Checkpoint:** Arithmetic expressions can be represented and printed.

## Chapter 11 — Parsing Expressions by Hand

**Original reference:** *Crafting Interpreters*, Chapter 6.

**Objective:** Teach recursive descent and operator precedence.

Implement a deliberately small expression parser.

Support:

- Numbers.
- Parentheses.
- Unary operators.
- Multiplication/division.
- Addition/subtraction.

Explain precedence and associativity.

A representative pattern:

```rust
let mut expr = self.factor()?;

while self.at_additive_operator() {
    let op = self.bump_operator();
    let right = self.factor()?;

    expr = Expr::Binary {
        left: Box::new(expr),
        op,
        right: Box::new(right),
    };
}
```

Implement all referenced helpers.

Demonstrate malformed input and the limitations of the owned AST representation for editor use.

Do not expand the handwritten parser to statements, declarations, or classes.

**Checkpoint:** A small working recursive-descent expression parser.

## Chapter 12 — Building Lossless Trees with Rowan

**Objective:** Evolve the syntax representation for editor tooling.

Introduce Rowan.

Explain:

- Concrete syntax trees.
- Abstract syntax trees.
- Lossless syntax.
- Green nodes.
- Red nodes.
- Typed AST wrappers.
- Syntax-node kinds.
- Source ranges.
- Error-tolerant syntax representation.

Migrate appropriate source-coordinate types to Rowan's representations.

Introduce:

```rust
pub use rowan::{TextRange, TextSize};
```

Show every affected API adjustment explicitly.

Implement `rowan::Language` and a simple tree with `GreenNodeBuilder`.

Migrate the small expression parser.

Use builder checkpoints or an event-based approach for infix expressions, selecting one approach before writing the chapter.

Do not build an elaborate generic parser framework.

Explain that Rowan does not generate parsing algorithms.

Add tests for:

- Precedence.
- Associativity.
- Trivia preservation.
- Source reconstruction.
- Incomplete expressions.
- Unexpected tokens.

Build interactive AST/CST comparisons.

**Checkpoint:** A working lossless Rowan expression parser.

Develop and review this transition in manageable sections. Let the actual explanations determine their boundaries rather than prescribing a lesson count.

## Chapter 13 — Diagnostics, Complete Syntax, and Snapshot Tests

**Objective:** Make the syntax layer robust enough for a real editor and upcoming interpreter.

Implement structured diagnostics:

```rust
pub struct Diagnostic {
    pub message: String,
    pub range: TextRange,
    pub severity: Severity,
}
```

Use a minimal handwritten rendering example if it helps explain the presentation problem.

Introduce Ariadne for terminal presentation without coupling diagnostic data to it.

Explain why errors should be data rather than printed directly by the parser.

Introduce Insta for syntax-tree snapshots.

Extend the Rowan parser to recognize the remaining Lox syntax:

- Statements.
- Declarations.
- Blocks.
- Variables.
- Calls.
- Functions.
- Classes.
- Property access.
- Control flow.
- Return statements.

This grammar work must be taught in manageable lessons.

Do not compress a full grammar implementation into an unexplained code dump.

Parsing need not perform complete semantic validation yet.

**Checkpoint:** Complete, lossless, error-tolerant Lox syntax with readable diagnostics and snapshots.

## Chapter 14 — Inspecting Syntax

**Objective:** Build syntax inspection tools without introducing a browser application.

This chapter replaces the previously considered mandatory WASM/Monaco chapter.

Implement:

```bash
rlox ast examples/hello.lox
rlox check examples/hello.lox
```

Teach:

- Rowan traversal.
- Typed AST access.
- Debug-tree formatting.
- Source ranges.
- Diagnostic presentation.
- Structural inspection.

Optionally add a VS Code command that invokes the native tool and displays a syntax tree in an output view or similar simple interface.

Avoid making a custom editor webview mandatory.

Use the book website's React syntax-tree visualizations to explain the underlying tree.

Readers do not have to implement those visualizations.

**Checkpoint:** CLI syntax inspection and validation.

## Chapter 15 — Building a Language Server

**Objective:** Turn existing syntax services into live editor capabilities.

First demonstrate a minimal JSON-RPC request and response.

Explain:

- JSON-RPC.
- LSP framing.
- Request IDs.
- Notifications.
- Server initialization.
- Open/change/close events.
- Document revisions.
- Diagnostic publishing.

Then introduce the selected Tower-based LSP implementation.

Create a protocol-independent editor service and a thin LSP adapter.

Expose:

```bash
rlox lsp
```

The VS Code extension launches the executable.

Implement initial capabilities:

- Initialization.
- Document synchronization.
- Parser diagnostics.
- Basic semantic tokens if supported.
- Correct coordinate conversion.

Teach the distinction between UTF-8 byte offsets and LSP UTF-16 positions.

Ensure stdout carries only protocol traffic.

**Checkpoint:** Live diagnostics and syntax-aware highlighting in VS Code.

**Milestone 2:** Rowan parser, diagnostics, syntax inspector, and native LSP.

---

# Part III — From Syntax to Execution

## Chapter 16 — Evaluating Expressions

**Original reference:** *Crafting Interpreters*, Chapter 7.

Create the tree-walk runtime.

Implement Lox values:

```rust
enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}
```

Teach:

- Dynamic value representations.
- Recursive evaluation.
- Numeric operations.
- Truthiness.
- Equality.
- Runtime errors.

Do not couple the runtime directly to Rowan syntax nodes.

Introduce a compact lowered representation as needed.

**Checkpoint:** Expressions execute through `rlox run`.

## Chapter 17 — Statements and State

**Original reference:** Chapter 8.

Implement:

- Print statements.
- Variable declarations.
- Assignment.
- Block execution.
- Lexical environments.

Start with standard Rust collections.

Explain ownership and scope lifetimes.

**Checkpoint:** Variables and blocks work.

## Chapter 18 — Control Flow

**Original reference:** Chapter 9.

Implement:

- `if`.
- `while`.
- `for`.
- Logical `and`.
- Logical `or`.
- Short-circuit evaluation.

Discuss nonterminating programs and execution limits.

**Checkpoint:** Control-flow semantics match Lox.

## Chapter 19 — Functions and Closures

**Original reference:** Chapter 10.

Implement:

- Function declarations.
- Calls.
- Parameters.
- Returns.
- Recursion.
- Closures.
- Captured environments.

Explain shared ownership and runtime lifetimes.

Begin considering debugger-friendly call-frame representation without prematurely implementing the debugger.

**Checkpoint:** First-class functions and closures execute correctly.

## Chapter 20 — Resolving Names

**Original reference:** Chapter 11.

Implement:

- Lexical scope stacks.
- Declaration identities.
- Reference resolution.
- Local binding rules.
- Static resolution diagnostics.

Introduce HIR and analysis infrastructure when needed.

Begin with simple typed IDs:

```rust
struct BindingId(u32);
```

Consider an arena library only if its advantages are demonstrated.

Build website diagrams showing uses connected to declarations.

**Checkpoint:** Static lexical resolution works.

## Chapter 21 — Semantic Language Services

**Objective:** Make the editor understand names.

Implement editor-independent operations for:

- Go-to-definition.
- References.
- Rename.
- Hover.
- Document symbols.
- Completion.
- Semantic tokens.

Example conceptual API:

```rust
fn definition(
    &self,
    file: FileId,
    offset: TextSize,
) -> Option<SyntaxOrigin>;
```

Adapt those operations to LSP.

Explain the limits of static analysis in a dynamically typed language.

**Checkpoint:** Semantic navigation and refactoring in VS Code.

## Chapter 22 — Classes and Inheritance

**Original reference:** Chapters 12–13.

Implement:

- Classes.
- Instances.
- Fields.
- Methods.
- `this`.
- `super`.
- Initializers.
- Inheritance.
- Method binding.

Extend semantic services as justified.

**Checkpoint:** Complete tree-walk Lox.

**Milestone 3:** Fully executable Lox with semantic editor features.

## Chapter 23 — Observing Execution

**Objective:** Introduce the information a debugger needs.

Start with execution events:

```rust
enum ExecutionEvent {
    EnterStatement(SyntaxOrigin),
    CallFunction {
        name: String,
        location: SyntaxOrigin,
    },
    ReturnFunction,
}
```

Build a trace inspector.

Explain:

- Events.
- Source locations.
- Call stacks.
- Variable inspection.
- Trace recording.
- Execution history.

Distinguish tracing from suspension.

**Checkpoint:** Structured execution tracing.

## Chapter 24 — Building a Step Debugger

**Objective:** Implement a real resumable execution controller.

Implement:

- Breakpoints.
- Pausing.
- Resuming.
- Step into.
- Step over.
- Step out.
- Call-stack inspection.
- Scope inspection.
- Variable inspection.

Explain why a recursive interpreter does not automatically support suspension across arbitrary calls.

Choose an explicit continuation, frame-based evaluator, or other concrete resumable model.

The selection must be technically sound and explained clearly.

Do not fake stepping by merely replaying execution traces.

Expose an internal debugger API and CLI command:

```bash
rlox debug example.lox
```

**Checkpoint:** Native source-level debugger.

## Chapter 25 — From Debugger to DAP

**Objective:** Connect debugger capabilities to VS Code.

Introduce the Debug Adapter Protocol.

Implement adapter support for:

- Initialize.
- Launch.
- Breakpoints.
- Continue.
- Step into.
- Step over.
- Step out.
- Stack trace.
- Scopes.
- Variables.
- Termination events.

Expose:

```bash
rlox dap
```

Keep the debugger controller separate from the transport.

Extend the VS Code extension to launch the debugger adapter.

Add protocol integration tests.

**Checkpoint:** VS Code debugging works.

**Milestone 4:** Complete tree-walk language with LSP and DAP tooling.

---

# Part IV — The Virtual Machine

## Chapter 26 — Chunks of Bytecode

**Original reference:** Chapter 14.

Implement:

- Bytecode chunks.
- Opcodes.
- Constant pools.
- Source mappings.
- Disassembly.

Start with a simple enum representation if useful, then discuss compact encoding.

**Checkpoint:** Manually constructed bytecode can be disassembled.

## Chapter 27 — A Stack-Based Virtual Machine

**Original reference:** Chapter 15.

Implement:

- Instruction pointer.
- Operand stack.
- Dispatch loop.
- Arithmetic instructions.
- Return behavior.
- Runtime errors.

Example:

```rust
match instruction {
    OpCode::Add => {
        let rhs = stack.pop().ok_or(VmError::StackUnderflow)?;
        let lhs = stack.pop().ok_or(VmError::StackUnderflow)?;
        stack.push(add(lhs, rhs)?);
    }
    // ...
}
```

Build an interactive VM execution diagram for the book.

**Checkpoint:** Arithmetic bytecode executes.

## Chapter 28 — Compiling Expressions

**Original reference:** Chapters 16–17.

Compile source expressions to bytecode.

Teach:

- Code generation.
- Constant indices.
- Instruction emission.
- Source mapping.
- Expression lowering.
- Compiler state.

Compare Nystrom's single-pass scanner/Pratt compiler with our existing syntax infrastructure.

Do not create a permanent second production lexer.

**Checkpoint:** Source expressions compile and execute.

## Chapter 29 — Values, Strings, and Tables

**Original reference:** Chapters 18–20.

Implement VM runtime infrastructure for:

- Dynamic values.
- Strings.
- Heap objects.
- String interning.
- Hash-table lookup.
- Object identity.

Compare Rust's collections with the original book's handwritten tables.

Implement custom structures only where pedagogically justified.

**Checkpoint:** VM value infrastructure supports the next compilation stages.

## Chapter 30 — Variables and Control Flow

**Original reference:** Chapters 21–23.

Implement:

- Globals.
- Locals.
- Stack slots.
- Assignments.
- Conditional jumps.
- Loop instructions.
- Jump patching.

Add branch and jump visualization.

**Checkpoint:** Statements and control flow execute in the VM.

## Chapter 31 — Calls and Closures

**Original reference:** Chapters 24–25.

Implement:

- Function objects.
- Call frames.
- Recursive calls.
- Closures.
- Upvalues.
- Captured local variables.

Build interactive upvalue diagrams.

**Checkpoint:** Functions and closures work in the VM.

**Milestone 5:** Substantial bytecode backend with functions and closures.

## Chapter 32 — Inspecting and Debugging the VM

**Objective:** Extend existing debugging architecture to the VM.

Implement:

- Instruction stepping.
- Source stepping.
- Bytecode inspection.
- Operand-stack inspection.
- Call-frame inspection.
- Source mapping.
- Breakpoints.
- Integration with existing DAP adapter.

Use one shared debugger abstraction where appropriate.

Do not write an unrelated second debugger.

**Checkpoint:** VM execution is inspectable and debuggable.

## Chapter 33 — Garbage Collection

**Original reference:** Chapter 26.

Implement a tracing collector.

Teach:

- Roots.
- Reachability.
- Marking.
- Sweeping.
- Heap allocation.
- Stable references.
- Object graphs.
- Collector thresholds.

Explain why Rust's memory safety does not eliminate the need to manage the guest language's heap.

Build an interactive garbage-collection visualizer for the book.

**Checkpoint:** Functional collector with reachability tests and visualizations.

## Chapter 34 — Classes and Inheritance in the VM

**Original reference:** Chapters 27–29.

Implement:

- Class objects.
- Instances.
- Fields.
- Bound methods.
- Initializers.
- `this`.
- `super`.
- Inheritance.

Run shared language conformance tests against both backends.

**Checkpoint:** Feature-complete bytecode Lox.

## Chapter 35 — Optimization and Benchmarking

**Original reference:** Chapter 30.

Start with basic timing:

```rust
let started = std::time::Instant::now();

for _ in 0..1000 {
    execute_program()?;
}

println!("{:?}", started.elapsed());
```

Discuss the limitations.

Introduce Criterion.

Teach:

- Warmup.
- Measurement variance.
- Benchmark fixtures.
- Profiling.
- Allocation measurement.
- Dispatch overhead.
- Evidence-based optimization.

Compare interpreter and VM performance carefully.

Do not claim speedups without measured evidence.

**Checkpoint:** Reproducible benchmarks and documented optimizations.

**Milestone 6:** Complete, debuggable, observable bytecode VM.

---

# Part V — Shipping and Beyond

## Chapter 36 — Testing a Programming Language

**Objective:** Validate the entire language and toolchain.

Implement a conformance harness shared by the tree-walk interpreter and VM.

Compare:

- Standard output.
- Return values.
- Errors.
- Side effects.
- Relevant observable behavior.

Consider Proptest for generated properties when it supplies useful coverage beyond the handwritten cases. The invariants are required; this dependency is conditional.

Example objective:

**Lexical token ranges must reconstruct the complete original source.**

Add:

- Parser snapshots.
- CLI integration tests.
- LSP protocol tests.
- DAP protocol tests.
- VS Code extension tests where appropriate.

**Checkpoint:** Comprehensive, repeatable toolchain test suite.

## Chapter 37 — Shipping Rlox

**Objective:** Teach real software delivery.

The following topic groups describe shipping coverage. Their numbering is a planning reference; actual lesson boundaries and count emerge through authoring and human review.

### 37.1 — Release Builds

Teach:

```bash
cargo build --release
```

Create a release archive and checksum.

Explain debug versus release builds.

### 37.2 — GitHub Actions Releases

Build automated release workflows.

Use version tags such as:

```text
v0.1.0
```

Distinguish normal CI from publishing workflows.

### 37.3 — Cross-Platform Builds

Target supported combinations of:

- macOS ARM64.
- macOS x86_64.
- Linux.
- Windows.

Verify the actual targets and artifact compatibility.

### 37.4 — GitHub Releases

Publish:

- Versioned archives.
- Checksums.
- Release notes.
- Installation instructions.

### 37.5 — Homebrew

Teach a custom tap:

```text
homebrew-tap/
└── Formula/
    └── rlox.rb
```

The formula downloads the appropriate release archive and verifies its SHA-256 checksum.

Document installation:

```bash
brew tap OWNER/tap
brew install rlox
```

Use placeholders until a real tap exists.

Teach formula testing, updates, and version consistency.

### 37.6 — crates.io

Explain package metadata and publishing.

Teach:

```bash
cargo install rlox
```

Only demonstrate this as an actual installation command after the package is published and configured correctly.

### 37.7 — VS Code Extension Distribution

Teach:

- VSIX packaging.
- Extension metadata.
- Marketplace publication.
- Executable discovery.
- Compatibility between extension and native binary.
- Installation without Homebrew.

### 37.8 — Documentation and Installation

Produce clear installation documentation for supported platforms.

Do not claim all platform installations are equivalent.

### 37.9 — Release Automation

After building a small manual release workflow, evaluate existing release automation tooling such as `dist`.

Adopt it if it substantially reduces custom maintenance while preserving educational clarity.

**Checkpoint:** Installable, versioned native toolchain and editor extension.

## Chapter 38 — Beyond Lox

**Objective:** Explain how the architecture applies to larger languages.

Discuss:

- Multi-file modules.
- Imports.
- Incremental computation.
- Salsa.
- Rope-based text storage.
- Incremental parsing.
- Formatting.
- Code actions.
- Cross-file semantic analysis.
- Performance scaling.
- Extensible toolchain architecture.

Do not implement all of these in the mandatory curriculum.

Use the completed Lox system to explain architectural tradeoffs.

**Checkpoint:** Reader understands how to apply the architecture to a different language.

**Milestone 7:** Tested, installable, distributable language toolchain.

---

# Optional Appendix: Taking Rlox to the Browser

WASM and Monaco were considered for the mandatory curriculum but deliberately removed.

**Do not reintroduce a mandatory browser playground into Chapters 1–38.**

The book's own interactive React diagrams remain mandatory authoring infrastructure.

The reader-built WASM/Monaco application is optional.

Suggested appendix title:

**Appendix A — Taking Rlox to the Browser**

Scope:

1. Compile the portable syntax crate to WASM.
2. Introduce `wasm-bindgen`.
3. Expose a small syntax-analysis API.
4. Create a minimal Vite/TypeScript application.
5. Integrate Monaco.
6. Display syntax diagnostics.
7. Explain UTF-8 versus UTF-16 source-position conversion.
8. Build and deploy the static application.

Do not reproduce the entire LSP, DAP, or debugger in the browser.

The appendix demonstrates portability, not a second full-featured IDE architecture.

---
