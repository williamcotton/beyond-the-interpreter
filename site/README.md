# Local book site

The introduction is authored in `src/content/docs/index.mdx` and served at `/`.
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
Edits to the introduction refresh the page automatically. Stop the server
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
