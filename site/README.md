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
file reading and later sections have not been implemented.

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
