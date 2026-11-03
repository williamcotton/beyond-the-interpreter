# Current assignment

## SITE-02 — Numbered chapters and descriptive section paths

**Status:** Accepted; authorized commit closeout.

**Authorization:** the user accepted Section 2.1 (“looks great”) and requested reorganizing content around chapter and section numbers and renaming index.mdx to introduction.mdx before committing. The user then approved the blend of numbered chapter directories and section filenames containing both a section number and a short description. The user accepted the result (“great”) and explicitly requested committing all current work.

**Starting point:** accepted introduction, Chapter 1, and Section 2.1 with its verified Rust endpoint. Existing uncommitted changes are preserved.

**Question and purpose:** organize manuscript source paths and section URLs by chapter and section numbers while retaining short, readable descriptions.

**Scope:** `site/src/content/docs/introduction.mdx`, `chapter-1/1.1-first-program.mdx`, and `chapter-2/2.1-filename.mdx`; update navigation, manuscript links, and author documentation. Keep the introduction at `/` with Starlight’s frontmatter slug override. Descriptive titles stay in frontmatter and navigation labels.

**Excluded:** Rust edits, new lessons, dependency changes, new teaching components, redirects for unpublished old URLs, pushes, and publication.

**Visual approach:** existing page layout suffices; inspect navigation and page rendering after the rename. No new visual is needed.

**Checks:** site type checks/build; current Starlight frontmatter documentation and installed routing code; generated and served routes; local links and absence of stale paths; browser inspection. Confirm lesson bodies and Rust are unchanged by this reorganization.

**Stopping point:** commit the accepted Section 2.1 manuscript, matching Rust, site reorganization, and supporting documentation. Stop after verifying the commit and working tree. Do not begin Section 2.2; await the next assignment.
