# Current assignment

## CH04-04 — Run the checks automatically (implementation and hosted verification)

**Status:** Local implementation verified; authorized commit/push and hosted verification in progress.

**Authorization:** the user accepted 4.3 and requested committing it before 4.4. Accepted endpoint committed as `13c8e8e` (`feat: pair section 4.3 with app and text workspace`). During drafting, the user identified the reader/repository-root mismatch and approved separating the reader’s Rust repository from this book’s monorepo. The user confirmed our workflow belongs at this repository root, approved the text, and explicitly requested updating the codebase, pushing to origin, and observing CI from the CLI.

**Reader question:** how can the same workspace checks run on a GitHub revision and show what failed?

**Scope:** manuscript at `site/src/content/docs/chapter-4/4.4-ci.mdx`, navigation and records. Propose one GitHub Actions job: checkout, stable Rust with rustfmt/Clippy, formatting, workspace Clippy, workspace tests. Keep Rust sources, manifests, lockfile, and CLI unchanged. No CI caches, matrix, deployment, artifacts, or extra helper scripts.

**Root distinction (agreed):** the reader’s Rust workspace directory is their Git repository root. Their workflow is `.github/workflows/ci.yml` beside Cargo.toml and crates/, with shell steps at that root; no cd .. or working-directory: rlox. Our book repository contains site/, authoring/, and rlox/; its actual workflow will instead be `/Users/williamcotton/Projects/beyond-the-interpreter/.github/workflows/ci.yml` with this job setting:

```yaml
defaults:
  run:
    working-directory: rlox
```

That is the sole authoring-layout adaptation to the reader workflow. Document and verify this difference explicitly during implementation; do not put an inert workflow under rlox/.github or teach the book’s folder layout as a reader prerequisite. Our accepted code already supplies the workspace-root reader checks.

**Workflow choices for review:** ubuntu-24.04 hosted image; actions/checkout v7.0.1 pinned to verified official commit 3d3c42e5aac5ba805825da76410c181273ba90b1; contents: read; persist-credentials: false. Rustup is present in the current official image; install/select stable with minimal profile plus rustfmt/clippy and print versions in job logs. Setup runs on the GitHub runner, not the user’s local machine. Current official action/release/tag, runner-image, syntax, secure-use, and rustup docs checked. No compiler pin or new Rust dependency.

**Teaching route:** rerun known checks locally; confirm reader workspace Git root/remote (link precise setup prerequisite); create complete workflow; explain events/job/steps/setup and read permission; commit/push reader’s source and workflow to their own GitHub repo; inspect exact commit’s Actions job/logs; reproduce a failing command and recover if needed. No deliberately broken push or forced failure is needed. Complete YAML is the lesson artifact.

**Verification boundaries:** text phase checks YAML syntax/declared steps, action SHA/release provenance, documentation/links, site check/build, and unchanged actual Rust/workflow state. Text approval has now been followed by local implementation. After implementation authorization, write our adapted root workflow, validate the sole layout difference, run local checks, and prepare the concrete result for review. The current user request supplies explicit commit/push authorization. Only an observed passing run establishes hosted CI success.

**Checks observed:** reader YAML parses with the existing site YAML parser; events, permission, checkout pin/input, runner, root directory, Rust setup, and all three exact check commands match the intended configuration. Confirmed official v7.0.1 tag/commit and runner/toolchain documentation; all seven reader reference URLs/fragments resolve. Site check reported zero diagnostics and build succeeded with existing optional-content/sitemap notices. Twelve local routes return HTTP 200 and local links/anchors resolve. All tracked rlox files match HEAD. Neither root nor nested actual CI workflow exists; no proposed Rust/check/setup/publication commands executed. git diff --check passed; no browser/screenshots. Hosted behavior and reader commit/push route remain unverified.

**Visual approach:** existing workflow listing and named job/step logs suffice; no custom UI or routine browser/screenshots.

**Implementation observed:** created .github/workflows/ci.yml at our repository root. YAML parses; removing the agreed defaults.run.working-directory: rlox makes its parsed structure identical to the reader workflow. Local formatting, workspace Clippy with warnings denied, and all eleven tests pass. Rust sources/manifests/lockfile remain unchanged; no local Rustup/default-toolchain changes. CLI authentication and remote access confirmed. The current request authorizes the lesson/workflow commit, origin push, CI observation, and necessary corrections within this scope.

**Excluded:** deployment, unrelated repository/remote changes, local global-toolchain changes, Chapter 5 authoring.

**Stopping point:** push the concrete section/workflow changes, observe the CI run for that commit, resolve failures within scope, and deliver as Ready for combined review with actual hosted evidence. Do not begin Chapter 5.
