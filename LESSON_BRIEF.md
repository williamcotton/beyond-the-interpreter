# Current assignment

## SITE-01 — Local publication and introduction

**Status:** Ready for review; user acceptance pending.

**Authorization:** On 2026-11-02 the user requested the site and an introduction visible on localhost using the documented publication stack. This authorizes the bounded implementation directly; earlier DOCS-02 wording remains pending acceptance in status.

**Audience and question:** reader-facing introduction plus author-only site setup. How does source text become execution, diagnostics, and editor understanding, and what will this book help the reader build?

**Starting point:** nine root guidance/roadmap documents; no site, Rust implementation, or accepted lesson endpoint. The broad architecture is agreed; no prose/style example has been accepted. The user identified Crafting Interpreters as the preferred teaching approach and supplied PROJECT_OUTLINE_DETAILED.md as a roadmap.

**Scope:** create `site/` with Astro, Starlight, MDX, React integration, strict TypeScript, a single introduction at `/`, minimal navigation, restrained book styling, a lockfile, and reproducible local commands. Add a root ignore file and update orientation/status. Verify current official setup documentation and registry compatibility before adoption.

**Reader outcome:** understand the book’s relationship to Crafting Interpreters, the two Lox engines and surrounding tools, assumed experience, and how examples and explanations will work. The `6 * 7` Lox example illustrates eventual behavior; it is not a claim that an engine exists.

**Teaching sequence:** begin with a short source example; connect execution and editor questions; explain the companion relationship and destination; describe learning Rust at point of need and following the book. No Rust edits or commands are taught here.

**Visual approach:** no algorithm visual is needed for this orientation. Use a concrete code example and readable prose. Do not add a diagram framework or demonstration engine. React is configured as requested, without inventing an interaction to exercise it.

**References:** Crafting Interpreters introduction and Lox tour; Rust Book. Author setup follows current official Astro, React integration, and Starlight manual-setup documentation. Exact selected dependencies are recorded in `site/package.json` and its lockfile.

**Excluded:** Rust scaffolding, scanner lessons, curriculum reordering, custom diff/replay/trace infrastructure, a future chapter navigation tree, hosted CI, commits, pushes, and publication.

**Checks:** install from the lockfile using the README route; run Astro/TypeScript checks and a production build; launch the local server and verify its response, rendered introduction content, assets, and internal anchors. Inspect desktop/mobile presentation and keyboard navigation if browser access is available; report any unavailable visual checks explicitly.

**Endpoint and stopping point:** leave the introduction available on localhost, deliver actual checks and limits, update this brief and status to Ready for review, and stop for human review. Do not begin the first implementation lesson. Next scope requires user authorization.
