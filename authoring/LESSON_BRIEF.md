# Current assignment

## CH04-03 — Extract the workspace (combined review)

**Status:** Accepted; commit closeout authorized.

**Authorization:** the user accepted 4.2 and requested a commit followed by 4.3. Committed the accepted text/library and supporting records as `7ca9e6f` (`feat: pair section 4.2 with source file origins`) before drafting. The user subsequently approved the text (“ok, looks good”) and explicitly requested updating rlox.

**Reader question:** how do we give the working CLI and text library their own packages while keeping one place to build/run/check them?

**Starting point:** one rlox package, main.rs and lib.rs, Clap 4.6.7, six library tests/five binary tests, root lockfile/ignore/hello.lox. Complete 3.2 and 4.2 source listings preserve this accepted endpoint.

**Scope:** page at `site/src/content/docs/chapter-4/4.3-workspace.mdx`, navigation and records. Move unchanged sources into crates/app/src/main.rs and crates/text/src/lib.rs; create complete rlox-app/rlox-text manifests; replace root manifest with virtual workspace and resolver 3. Keep executable rlox, root fixture/ignore/lockfile/target, CLI behavior, source API, and all existing tests. Move Clap declaration unchanged into app; no unused app-to-text dependency.

**Teaching sequence:** baseline tests/CLI; show resulting tree; make exact source moves from rlox/ (macOS/Linux commands or equivalent editor moves); finish grouped child/root manifest edits before Cargo; cargo check --workspace updates local-package lock records; check implicit and explicit root run commands, help, per-package and workspace tests, formatting/Clippy, missing-file failure and recovery. Provide complete child/root manifests and unchanged resulting source views. No Cargo new command or generated scaffolding to overwrite working code.

**New concepts:** package versus binary/library target, virtual workspace/members, explicit binary name/path, Rust library name normalization, resolver selection matching edition 2024, membership versus dependency edges, package/target selection, shared lockfile/target directory. No Rust algorithm/visibility change; existing pub types and super test imports move unchanged. Dependency version/features remain accepted values.

**Important route contract:** all commands run from rlox/. Source arguments stay relative to that current directory, including when -p selects a member. Keep existing Cargo.lock and let Cargo update local package entries; registry versions remain locked. No default-members restriction; default virtual-root selection covers both members, and the sole binary remains runnable without -p. The implementation pass verified this behavior.

**Visual approach:** a static file tree answers where each file goes. Complete code is folded separately from the move instructions; no unchanged code rendered as removals/additions. Existing components only; no routine browser/screenshots.

**Checks observed:** current official Cargo workspace/run/target/resolver references checked; all four reader reference URLs/fragments resolve. All three manifest listings parse and match the intended members/targets/dependency placement. Both full source views exactly match accepted 4.2 files. Site check reported zero diagnostics and build succeeded with existing optional-content/sitemap notices. Eleven local routes and their links/anchors resolve. All six rlox files match HEAD and no crates directory exists; git diff --check passed. No browser/screenshots. These were text-phase checks. The authorized implementation pass followed the entire route in order: baseline tests/CLI, exact moves and all three manifests, workspace check, implicit/explicit root runs and help, member tests, workspace tests/formatting/Clippy, read failure and recovery. All matched. Five manifests/source files match their complete listings; both sources are byte-identical to the accepted originals. Only local package records changed in Cargo.lock; registry entries are identical. Metadata confirms two default members, shared target, rlox binary/rlox_text library names, and intended dependencies. No nested lockfiles. Direct execution confirms source/help/error stdout/stderr/status behavior. Six text tests and five app tests pass. No manuscript corrections or repeated site/browser checks. Existing macOS toolchain only; other platforms unverified.

**Excluded:** Rust behavior/API changes, new dependency versions or path edges, new tests/fixtures, future crates, workspace metadata inheritance, CI (4.4), Git push/publication, commits, 4.4 drafting.

**Stopping point:** the user accepted 4.3 (“great”) and requested committing it before 4.4 text. Commit the accepted endpoint before drafting; no push or publication authorized.
