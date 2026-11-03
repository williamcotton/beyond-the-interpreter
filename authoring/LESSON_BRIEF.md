# Current assignment

## CH05-PLAN — Frame Chapter 5

**Status:** Accepted when the user approved the two-section breakdown and requested committing it before 5.1.

**Authorization:** the user accepted Section 4.4 and requested a commit followed by outlining Chapter 5. The implementation and hosted evidence were already committed as 96d1d19 and 779f648. Acceptance is recorded in 3eb9e14. This assignment authorizes planning documents, not Chapter 5 manuscript, Rust, or visual implementation. No new push is requested.

**Accepted starting point:** Chapter 4’s app/text workspace, source-display CLI, safe UTF-8 byte ranges and file origins, and passing root GitHub workflow. Actual workspace manifests, text types/tests, and CLI inspected. No syntax crate or scanner exists.

**Reader question:** how do we turn source characters into token kinds while retaining the exact source bytes each token occupies?

**Concrete progression:** `()` establishes token emission and ranges; `!` versus `!=` then reveals why one-character decisions are insufficient. `!==` and `!(` expose conditional consumption and the need to preserve a character after failed lookahead. These are scanner inputs, not yet executable Lox programs.

**Proposed teaching boundaries:**

- **Read a character, emit a token:** a useful rlox-syntax library depending on rlox-text; SyntaxKind and Token; scan a small input into a collection using character iteration and byte offsets. Recognize parentheses and single-character !/=, retain TextRange, and check kinds/source slices. Empty input is a coherent endpoint. Consume unmatched characters into a temporary Error token so the loop is total and makes progress, without expanding into Chapter 6’s error/trivia lesson. Introduce workspace wiring alongside the behavior that needs it.
- **Look ahead before choosing:** distinguish !=/== from !/= without consuming a nonmatching next character. Teach peekable iteration and conditional advancement at need. Check successful/failed lookahead, adjacent operators, trailing operators, and exact source ranges. Extend the cursor visual alongside this explanation.

These are candidate boundaries, not a prescribed section count or detailed backlog. The opening section should deliver a working tiny scanner; review its actual explanation before deciding whether to keep or adjust the next boundary. No separate type-only, crate-only, or testing-only lesson is proposed.

**New Rust reasoning:** enum variants as token categories; a struct pairing kind/range; Vec accumulation; char_indices byte offsets; match dispatch; iteration exhaustion; peekable lookahead and its borrowing pattern. Explain each at its first consequential edit. Reuse existing TextRange construction and slicing; do not add coordinate APIs without a current need. Exact signatures and helper choices are for the text draft, with complete file/import/manifest changes shown there.

**Architecture:** syntax owns scanning and depends on text; app remains unchanged. No new registry dependency is proposed. The handwritten scanner is temporary, extended only to Chapter 6’s limited corpus before the Chapter 7 Logos migration. No file identity is needed inside a scanner of one supplied source string.

**Visual purpose:** answer “which character has been consumed, which is only being inspected, and which bytes belong to the emitted token?” Introduce the character-cursor diagram with the first cursor/range explanation, then show successful and failed lookahead at that mechanism’s first explanation. Use checked authored traces of tiny inputs, not a JavaScript lexer. Include initially useful/static text, keyboard controls, byte labels, non-color state cues, narrow-screen support, and reduced-motion behavior. This is planning only; no component or trace infrastructure is authorized yet.

**References for drafting:** the roadmap’s Crafting Interpreters Chapter 4 reference; precise official Rust Book material for enums, match and vectors, and standard-library documentation for str::char_indices and iterator lookahead. Inspect current official documentation before selecting APIs; planning makes no new version or API compatibility claim.

**Excluded:** identifiers, whitespace/comment handling, developed unknown-character recovery and lossless partition invariants (Chapter 6); numbers/strings/keywords and Logos (Chapter 7); tokens CLI (Chapter 8); parser, EOF convention, diagnostics infrastructure, source database, file IO changes, broad SyntaxKind inventory, generic visualization machinery.

**Planned evidence after text approval:** follow every displayed creation/edit/command from the accepted workspace in order; compare complete files and source-range slices; meaningful scanner tests including byte boundaries and progress; workspace format/Clippy/tests; existing CLI behavior preserved. Verify curated visual observations against native scanner results, and inspect the actual rendered controls/layout when introduced. No new runtime/site tests or builds are needed for this planning task.

**Checks now:** actual starting files and owning guidance inspected; Chapter 5 coverage compared with Chapters 6–8 and architecture; planning links and whitespace checked. No manuscript, Rust, workflow, dependency, or website changes.

**Stopping point:** planning accepted. The user authorized drafting 5.1; Rust implementation still waits for text approval.
