# Current assignment

## CH10-02 — Connect operands with operators (combined review)

**Authorization:** User accepted 10.1 with “ok great, commit and let’s move on to section 10.2”. Accepted pair committed as 96c0b0c (feat: pair section 10.1 with recursive expression shapes). User subsequently said “ok, looks good, now update rlox”, approving the text and authorizing its exact reader route. No commit/push authorized.

**Status:** Accepted when the user said “ok great, commit and let’s move on to section 10.3”. Commit accepted pair, then draft 10.3 text only; no push.

**Actual starting state:** expr.rs contains Debug-derived Number(f64)/Grouping(Box<Expr>); public module already wired in syntax lib.rs. expression_shape example prints number/one-group/two-group views. All 46 workspace tests and local Cargo checks passed in 10.1. No operators, parser or evaluation exists. Root workflow/extension unchanged.

**Reader question/example:** How can a tree record that minus has one operand while multiplication/addition have two, and preserve which operand is left/right? Build -123 first, then -(1 + 2) * 3, retaining an explicit grouping node.

**Proposed route:** Append UnaryOp::Negate, add Expr::Unary named fields, update example import/append unary construction and display. Check syntax crate/run example. Append BinaryOp Add/Subtract/Multiply/Divide, add Expr::Binary named fields, extend example import and append sum/group/negation/product construction and pretty Debug output. Check/run again, then workspace formatting/Clippy/tests. Complete final expr/example views supplied. Exact snippets/expected observations to verify after approval, not executed in text phase.

**New Rust ideas:** Enum variants with named fields; separate operator enum types, explicit operand roles; ownership moves through branching tree. Reuse existing Box/Debug/module lessons. Pretty diagnostic formatting introduced only to inspect the larger constructed tree. No Copy/Clone derives or generic traversal API needed.

**Behavior/boundaries:** Hand-built arithmetic trees, not source parsing, precedence algorithm or evaluation. No custom printer until 10.3, full Lox AST, scanner/app changes, new crate/dependency, extension/CI changes or constructor-mirroring unit tests. Example outputs and structure diagrams provide proportional observations until operations introduce meaningful behavior to test.

**References/visual purpose:** Rust Book associated enum data/named variants and ownership moves; Crafting Interpreters expression grammar/tree section. One labeled unary tree and one mixed arithmetic tree explain arity/roles/grouping. Static accessible text with labeled branches, no color dependence, new component or JavaScript engine. Human owns visual appearance; no screenshots.

**Stopping point:** Commit accepted 10.2 pair, then draft 10.3 text only. No future reader implementation or push authorized.

**Text-phase evidence:** Site check zero errors/warnings/hints; build succeeds with 33 pages including 404 and existing notices. Ordered operator appends, variant diffs, import replacements and main insertions reconstruct both complete resulting files exactly from accepted 10.1. Resulting Rust listings match stdin-only rustfmt; shell snippets parse. Three external references and anchors resolve; local 10.2 returns HTTP 200 and affected rendered local links/anchors pass. Accepted 10.1 body preserved apart from next navigation. Actual Rust/extension/workflow/dependencies/components/styles unchanged from 96c0b0c. Expected native check/run/pretty-output behavior remains unverified until implementation authorization. No future reader code generated/compiled/tested, screenshots, push, new commit or 10.3 draft.

**Implementation evidence:** Started from actual accepted 10.1. Appended UnaryOp, applied the focused Unary variant edit, and ran cargo check -p rlox-syntax successfully. Updated example import and appended unary construction; actual four-line stdout matches the manuscript exactly. Then appended BinaryOp, applied Binary variant edit, and repeated cargo check successfully. Updated import and appended mixed-expression construction in the displayed order; example stdout preserves the first four lines and exactly matches all additional pretty Debug lines. Both resulting reader files match complete manuscript listings. Formatting and workspace/all-target Clippy with warnings denied pass; workspace tests pass (46 unchanged tests). Example structure/output matches the labeled unary and mixed-tree diagrams; no manuscript correction needed. Syntax root/scanner/tests, manifests/lockfile, app/text crates, extension and root workflow unchanged from 96c0b0c. Previous site/reference checks remain applicable; no repeated site build, screenshots/visual review, constructor-mirroring tests, new infrastructure, commit/push or 10.3 work. Verified on local macOS ARM64/Rust 1.95.0; no new hosted or other-platform evidence claimed.
