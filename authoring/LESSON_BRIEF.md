# Current assignment

## CH09-05 — Run editor checks in CI (combined review)

**Status:** Combined local endpoint accepted when the user said “ok, commit and push and then check the status of the build/ci run”. Commit/push and hosted verification now authorized; no Chapter 10 work.

**Accepted starting state:** Section 9.4 and its compatibility correction are committed as a32dd10. Root Rust-only workflow used Cargo working directory rlox/. Extension has five grammar tests, disposable development/installed modes, ^1.128.0 engine with 1.125.0 types, default VS Code 1.128.0 host and optional newer-host selection. User confirmed the corrected package works in Cursor; leave further Cursor checks to the user.

**Reader question/example:** How can a clean hosted checkout check the same extension and installed VSIX that work locally? Add an independent Ubuntu extension job beside the existing Rust job, using locked npm installation and Xvfb for actual editor processes.

**Ordered route followed:** Renamed workflow Project CI; appended the three displayed groups in order (checkout/Node setup, install/check/grammar/package/archive, display tools/development/installed tests). Applied both documented layout adjustments: shell working directory rlox/editors/vscode and Node action cache-dependency-path rlox/editors/vscode/package-lock.json. Existing Rust job, triggers and read-only permissions unchanged. No rlox/.github created. Followed local npm ci, check, test, package, unzip inspection and both editor modes.

**Behavior and exclusions:** Existing tests/package reused; no new Rust or extension behavior. No matrix, publishing, deployment, screenshots, personal-profile installation or Chapter 10 work. Hosted execution requires explicit push authorization and an observed run; older Rust-only green runs are not evidence.

**New concepts and references:** Independent jobs, locked install versus download cache, shell directory versus action input paths, Xvfb for Electron, failed/skipped steps and commit-specific CI evidence. No new Rust concepts. Official VS Code CI guide and Ubuntu 24.04 inventory support Xvfb; setup-node v7.1.0 commit/inputs previously verified, existing checkout pin retained. Node 26.0.0 matches local verification. References remain in the manuscript.

**Visual purpose:** Focused YAML and complete workflow show job boundaries/paths; no new visual component. User owns rendered appearance. Actual editor assertions require no screenshot.

**Observed checks:** Actual YAML 1.2 matches the complete reader listing with exactly the documented layout adjustments; unchanged Rust job/triggers/permissions verified. Shell steps parse. Locked install succeeds with zero vulnerabilities and no manifest/lockfile changes. TypeScript check/build and all five grammar tests pass. VSIX has exactly four extension payload files plus two metadata entries. Development host passes all seven observations; installed host passes identity/location/version plus the same seven, both status 0 on VS Code 1.128.0/macOS ARM64. Temporary profiles cleaned. Actual rlox/ unchanged from a32dd10; git diff --check passes. Existing site checks/build remain applicable because manuscript/navigation did not change during implementation. No unnecessary Rust/site rerun.

**Limits:** Local npm ci proves locked reinstall on the existing checkout, not a fresh hosted Git checkout. GitHub checkout/cache actions, Ubuntu display prerequisites and Linux editor execution remain unobserved. No hosted result claimed; no push, commit, Cursor testing or screenshots.

**Stopping point:** Commit and push the accepted 9.5 pair, then inspect the CI run for that exact commit. Correct any failure within this section and report observed hosted evidence. Do not start Chapter 10.
