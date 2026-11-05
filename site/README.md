# Local book site

The introduction is authored in `src/content/docs/introduction.mdx` and served at `/`
via its `slug: index` frontmatter override. Sections use
`src/content/docs/chapter-N/N.M-short-description.mdx`, with matching URLs.
Explicit `slug` frontmatter preserves dots in section numbers in the URLs.
Keep descriptive titles in frontmatter and navigation labels.
Chapter 1, Section 1 is in `src/content/docs/chapter-1/1.1-first-program.mdx`
and served at `/chapter-1/1.1-first-program/`. The section and matching
`rlox/` greeting program are accepted. Its instructions were followed successfully
on the local machine; see
[authoring status](../authoring/AUTHORING_STATUS.md) for checks and limits.
Chapter 1 is complete with Section 1.1 only. Chapter 2 introduces source
reading and its first meaningful tests. Its opening section is
`src/content/docs/chapter-2/2.1-filename.mdx`, served at
`/chapter-2/2.1-filename/`. Its text is approved, and the matching Rust changes and ordered reader route
have been verified. Section 2.1 and its code are accepted;
Section 2.2 adds file reading and is accepted. Its manuscript
is `src/content/docs/chapter-2/2.2-read-source.mdx`, served at
`/chapter-2/2.2-read-source/`. Rust and hello.lox match its complete listings;
the ordered reader route is verified. Section 2.3 is at
`src/content/docs/chapter-2/2.3-input-errors.mdx`, served at
`/chapter-2/2.3-input-errors/`. Its text is approved; the matching Rust and
ordered reader route are verified and accepted. The Section 2.4 test manuscript is
`src/content/docs/chapter-2/2.4-input-tests.mdx`, served at
`/chapter-2/2.4-input-tests/`. Its text is approved, and matching Rust tests
and the ordered reader route (including regression/restoration) are verified
and accepted. Production behavior remains as in 2.3. Section 3.1 is
at `src/content/docs/chapter-3/3.1-source-command.mdx`, served at
`/chapter-3/3.1-source-command/`. Its text is approved and matching Rust/ordered
route are verified and accepted. Current invocation:
`cargo run -- source hello.lox` from rlox/. Section 3.2 is a text draft at
`src/content/docs/chapter-3/3.2-clap.mdx`, served at `/chapter-3/3.2-clap/`.
Its text is approved, and its Clap dependency, manifest/lockfile, Rust, and
migrated tests match the verified instructions and are accepted. Section 4.1
is at `src/content/docs/chapter-4/4.1-byte-ranges.mdx`, served at
`/chapter-4/4.1-byte-ranges/`. Its text is approved and matching library/tests
are verified and accepted. Section 4.2 is at
`src/content/docs/chapter-4/4.2-file-origins.mdx`, served at
`/chapter-4/4.2-file-origins/`. Its text is approved and matching file-origin
types/tests are verified and accepted. Section 4.3 is at
`src/content/docs/chapter-4/4.3-workspace.mdx`, served at
`/chapter-4/4.3-workspace/`. Its text is approved and matching workspace
migration is verified and accepted. rlox now contains app/text
workspace packages with unchanged source and CLI behavior. Section 4.4 is
authored at `src/content/docs/chapter-4/4.4-ci.mdx`, served at
`/chapter-4/4.4-ci/`. Its reader workflow assumes the Rust repository root;
our implemented root workflow runs Cargo from rlox/. Local checks and hosted
Rust CI pass; the combined endpoint is accepted.

Astro and Starlight provide the publication; Starlight includes MDX support.
The React integration and strict TypeScript configuration support future
explanations embedded in MDX. This introduction needs no client-side React.

## Run locally

Use Node.js 22.23.4 or newer and npm 9.6.5 or newer. From the repository root:

```sh
cd site
npm ci
npm run dev
```

Open the Local URL printed by Astro, normally <http://localhost:4321/>.
If that port is occupied, Astro prints the alternative port it selected.
Edits to the manuscript refresh the page automatically. Stop the server
with Ctrl+C.

## Check and build

From `site/`:

```sh
npm run check
npm run build
```

The static output is in `dist/`. `npm run preview` serves that build locally;
stop the development server first to reuse port 4321.

`package-lock.json` records exact dependency resolutions. Use `npm ci` to
reproduce them; `.astro/`, `dist/`, and `node_modules/` are generated locally.
No hosting account or deployment configuration is required.

Section 5.1 is drafted at `src/content/docs/chapter-5/5.1-first-tokens.mdx`,
served at `/chapter-5/5.1-first-tokens/`. Its React character-cursor visual
uses two authored examples verified against the matching Rust scanner.
The section, cursor visual, and matching Rust implementation are accepted.

Section 5.2 is drafted at `src/content/docs/chapter-5/5.2-lookahead.mdx`,
served at `/chapter-5/5.2-lookahead/`. Its lookahead cursor contrasts matching,
nonmatching, and exhausted input using fixed authored frames. The section, native implementation, and lookahead visual are accepted;
local checks and native cursor comparisons pass.

Section 6.1 is drafted at `src/content/docs/chapter-6/6.1-identifiers.mdx`,
served at `/chapter-6/6.1-identifiers/`. Its cursor steps through a growing
identifier and the character left unread. The section, identifier scanner, and cursor are accepted;
local checks and native cursor comparisons pass.

Section 6.2 is drafted at `src/content/docs/chapter-6/6.2-whitespace.mdx`,
served at `/chapter-6/6.2-whitespace/`. A static annotated token listing
explains retained whitespace; the text adds classification and source-coverage
checks. The section and matching Rust are accepted; local workspace checks
and native example consistency are verified.

Section 6.3 is drafted at `src/content/docs/chapter-6/6.3-line-comments.mdx`,
served at `/chapter-6/6.3-line-comments/`. Static byte steps explain retained
comments and unread terminating LF. The section and matching Rust are accepted,
completing Chapter 6; local workspace checks and native example consistency pass.

Section 7.1 is drafted at `src/content/docs/chapter-7/7.1-logos.mdx`, served at
`/chapter-7/7.1-logos/`. It preserves the current lexical contract while
replacing scanner mechanics with Logos, with a baseline regression run before
the migration. Text is approved; matching Logos implementation, baseline
comparison and local checks pass. The section and matching scanner, including
the corrected greedy-comment option, are accepted.

Section 7.2 is drafted at `src/content/docs/chapter-7/7.2-punctuation.mdx`,
served at `/chapter-7/7.2-punctuation/`. It completes fixed punctuation and
comparison rules with boundary/reconstruction checks. The section and
matching Rust are accepted; local checks pass.

Section 7.3 is drafted at `src/content/docs/chapter-7/7.3-keywords.mdx`,
served at `/chapter-7/7.3-keywords/`. It distinguishes keywords from names and
explains borrowed spelling through source ranges. Text is approved; matching
Rust and local checks are verified and accepted.

Section 7.4 is drafted at `src/content/docs/chapter-7/7.4-numbers.mdx`,
served at `/chapter-7/7.4-numbers/`. Its number spellings and decimal boundaries
and the matching number scanner/ordered reader-route checks are accepted.

Section 7.5 is drafted at `src/content/docs/chapter-7/7.5-strings.mdx`, served
at `/chapter-7/7.5-strings/`. Quoted strings, multiline contents and unfinished
input and their matching Rust/ordered reader-route checks are accepted,
completing Chapter 7.

Section 8.1 is drafted at `src/content/docs/chapter-8/8.1-token-command.mdx`,
served at `/chapter-8/8.1-token-command/`. The token-command text and fixed
UTF-8 range interactive have approved text. Matching Rust, ordered commands
and native output-to-fixture comparisons pass; the section and inspector
are accepted.

Section 8.2 is drafted at `src/content/docs/chapter-8/8.2-scan-status.mdx`,
served at `/chapter-8/8.2-scan-status/`. Its lexical failure status and separate
stderr summary have approved text; matching Rust and actual CLI streams,
statuses and failure/recovery checks pass. The section and inspector are
accepted.

Section 8.3 is drafted at `src/content/docs/chapter-8/8.3-cli-tests.mdx`,
served at `/chapter-8/8.3-cli-tests/`. Real-executable integration tests and
a deliberate stream regression/restoration have approved text. Matching tests
and the actual dependency/failure/recovery route pass; the section and code
are accepted, completing Chapter 8.

Section 9.1 is drafted at `src/content/docs/chapter-9/9.1-language-registration.mdx`,
served at `/chapter-9/9.1-language-registration/`. Language registration and the
first isolated real-VS-Code test have approved text. Matching extension files,
dependency installation, TypeScript checks and actual editor-host assertions
pass; the section and extension are accepted.

Section 9.2 is drafted at `src/content/docs/chapter-9/9.2-editing-rules.mdx`,
served at `/chapter-9/9.2-editing-rules/`. Editing configuration and actual-host
comment/pair/indentation assertions have approved text. Matching configuration,
TypeScript checking/build and all five actual-host observations pass; the
section and extension are accepted.

Section 9.3 is drafted at `src/content/docs/chapter-9/9.3-syntax-highlighting.mdx`,
served at `/chapter-9/9.3-syntax-highlighting/`. TextMate grammar, scope/state
tests and scope-aware editing checks have approved text. Matching grammar,
all five Node tests and seven actual-host observations pass; the section and
extension are accepted. Manual color/scope inspection is left
to the user through the documented development-host launch configuration.

Section 9.4 is drafted at `src/content/docs/chapter-9/9.4-local-vsix.mdx`,
served at `/chapter-9/9.4-local-vsix/`. Local packaging, archive inspection and
isolated installed-extension tests have approved text, with a scoped dependency
patch and archive README casing corrected during implementation. Actual packaging,
archive inspection and both editor modes pass; the section and extension are
accepted.

Section 9.5 is drafted at `src/content/docs/chapter-9/9.5-editor-ci.mdx`,
served at `/chapter-9/9.5-editor-ci/`. Its approved instructions now match an independent extension job in the root
workflow, using rlox/editors/vscode for shell commands and the lockfile cache
path. Locked installation, TypeScript/grammar checks, packaging and both local
VS Code modes pass. The combined endpoint is accepted and pushed; hosted Ubuntu/Xvfb verification
also passes, completing Chapter 9.

Section 10.1 is drafted at `src/content/docs/chapter-10/10.1-recursive-expressions.mdx`,
served at `/chapter-10/10.1-recursive-expressions/`. Numbers/grouping, the
recursive-size error and Box recovery are explained through static diagrams
and a runnable-example route. Matching reader Rust, error/recovery, example output and workspace checks
pass locally. The section and implementation are accepted.

Section 10.2 is drafted at `src/content/docs/chapter-10/10.2-expression-operators.mdx`,
served at `/chapter-10/10.2-expression-operators/`. Unary/binary arithmetic
nodes, operator types and owned subtrees are introduced through incremental
example edits and a labeled tree. Matching expression types and both example checkpoints pass locally, along
with workspace formatting, Clippy and tests. The combined endpoint is accepted.

Section 10.3 is drafted at `src/content/docs/chapter-10/10.3-expression-printer.mdx`,
served at `/chapter-10/10.3-expression-printer/`. Its borrowed printer, example and four formatting/ownership tests match the
manuscript and pass local workspace checks. The section and code are accepted,
completing Chapter 10.

Section 11.1 is drafted at `src/content/docs/chapter-11/11.1-parse-number.mdx`,
served at `/chapter-11/11.1-parse-number/`. Its number parser, ranged errors, example and five focused tests match the
manuscript and are accepted. Formatting, Clippy and all 55 workspace tests
pass locally.

Section 11.2 is drafted at `src/content/docs/chapter-11/11.2-parse-grouping.mdx`,
served at `/chapter-11/11.2-parse-grouping/`. Its private cursor, recursive grouping, delimiter errors and 64-group policy
match the reader implementation and are accepted. Both checkpoints, example
output, formatting, Clippy and all 60 workspace tests pass locally.

Section 11.3 is drafted at `src/content/docs/chapter-11/11.3-parse-unary.mdx`,
served at `/chapter-11/11.3-parse-unary/`. Its unary rule, grouped operands and shared nesting budget match the
implementation and are accepted. Example output, formatting, Clippy and all
64 workspace tests pass locally.

Section 11.4 is drafted at `src/content/docs/chapter-11/11.4-parse-factors.mdx`,
served at `/chapter-11/11.4-parse-factors/`. Its multiplicative loop and binary-tree height checks match the implementation
and are accepted. Example output, formatting, Clippy and all 69 workspace
tests pass locally.

Section 11.5 is drafted at `src/content/docs/chapter-11/11.5-parse-terms.mdx`,
served at `/chapter-11/11.5-parse-terms/`. Its arithmetic parser and source-loss/error examples match the implementation
and are accepted, completing Chapter 11. Example output, formatting, Clippy
and all 74 workspace tests pass locally.

Section 12.1 is drafted at `src/content/docs/chapter-12/12.1-rowan-coordinates.mdx`,
served at `/chapter-12/12.1-rowan-coordinates/`. It introduces Rowan-compatible byte coordinates and explicit source-capacity errors, with every scanner/parser/app/example/test caller migration and complete resulting files. Section 12.1 and matching Rust are accepted. All seven complete listings match; the documented native checks and all 75 tests pass locally. Ordinary scanner/parser/CLI outputs are unchanged. Appearance review belongs to the user.

Section 12.2 is drafted at `src/content/docs/chapter-12/12.2-lossless-tree.mdx`, served at `/chapter-12/12.2-lossless-tree/`. It introduces one kind declaration, safe raw-kind conversion, Rowan Language/aliases and a small source-preserving token tree. Its `LosslessTree` component inspects fixed scanner/AST examples without parsing JavaScript. Its matching Rust is accepted: all 79 tests pass locally, complete files match and both comparison fixtures agree with native tree/AST observations. Appearance review belongs to the user.

Section 12.3 is drafted at `src/content/docs/chapter-12/12.3-lossless-groups.mdx`, served at `/chapter-12/12.3-lossless-groups/`. It introduces a concrete number/group parser with explicit trivia ownership, a root plus located errors, balanced partial nodes and complete suffix retention. Section 12.3 and matching Rust are accepted; all 83 tests pass locally. Complete files, example observations and group diagram match native behavior; original arithmetic/token-tree/CLI output is unchanged. No interactive component changed.

Section 12.4 is drafted at `src/content/docs/chapter-12/12.4-lossless-unary.mdx`, served at `/chapter-12/12.4-lossless-unary/`. It introduces concrete unary negation with retained operator/trivia, balanced partial nodes and a shared group/unary depth budget. Section 12.4 and matching Rust are accepted; all 87 tests pass locally. Complete files, example observations and both unary diagrams match native behavior; prior CST/arithmetic/CLI output remains unchanged. No components changed.

Section 12.5 is drafted at `src/content/docs/chapter-12/12.5-lossless-infix.mdx`, served at `/chapter-12/12.5-lossless-infix/`. It introduces checkpoint wrapping, factor/term loops, non-emitting operator lookahead and subtree-height results, with fixed arithmetic comparisons in `LosslessTree` arithmetic mode. Original token mode renders unchanged. Section 12.5 and matching Rust are accepted; all 92 tests pass locally. Three complete views and pretty-debug observations match native behavior, as do all four arithmetic comparison AST values and 66 CST entries. Original arithmetic/token-tree/CLI observations are unchanged.

Section 12.6 is drafted at `src/content/docs/chapter-12/12.6-incomplete-syntax.mdx`, served at `/chapter-12/12.6-incomplete-syntax/`. It adds explicit missing operand slots and bounded local recovery. LosslessTree recovery mode includes zero-width markers, errors and full text fallbacks; earlier modes retain identical initial SSR. Section 12.6 and matching Rust are accepted; all 96 tests pass locally. Three complete listings and example observations match, and all four recovery fixtures (39 CST entries/five diagnostics) agree with native Rust. Prior arithmetic fixtures/CLI/owned-parser observations remain unchanged.
