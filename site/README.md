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
