# Current assignment

## CH07-04 — Recognize number spellings (implementation phase)

**Status:** Accepted when the user said “ok great, commit and let’s move on to section 7.5”.

**Authorization:** the user accepted 7.3 and said “ok great, commit and let’s move on to section 7.4”. The accepted keyword pair is committed as `dc2d748` (`feat: pair section 7.3 with keyword recognition`). The user subsequently said “ok, looks good, now update rlox”, approving the text and authorizing its implementation. No commit/push requested.

**Accepted starting state:** inspected actual Logos 0.16.1 SyntaxKind and scan adapter, twenty-two syntax tests, and existing source-slicing API. Keywords/punctuation/ASCII identifiers/trivia/Error are implemented; digits still produce Error. Prior implementation checks pass all thirty-three workspace tests.

**Reader question/example:** when does a dot belong to a number? Recognize 123.45 as one Number, 123. as Number then Dot, .5 as Dot then Number, and 1.2.3 as Number/Dot/Number. Preserve exact source spelling through ranges.

**Behavior and ordered route:** insert Number with ASCII digit regex and optional dot-plus-digits before Identifier. Rename the existing 2aé_b boundary test and change only its leading Error to Number; preserve the UTF-8 error and all ranges. Append whole-number/spelling and exact-boundary tests, extend reconstruction inputs, then run syntax tests, formatting, workspace Clippy and tests. Complete resulting library available separately.

**New Rust ideas:** reuse regex attribute/raw strings and range slicing. Explain optional regex group versus required digits. Boundary test uses tuple expectations, length assertion and zip; explain why length must be checked separately. No new API, dependency, module or token payload.

**Visual purpose:** static annotated source/kind/byte-range cases explain dot boundaries at the relevant paragraph. No component or animation needed; visual confirmation belongs to the user. No screenshots/browser visual review.

**References:** official Logos attributes/common regex documentation rechecked; canonical Crafting Interpreters number rule checked through cached official chapter HTML (live chapter fetch failed). Existing dependency/compiler unchanged. No new version compatibility requirement.

**Excluded:** runtime f64 conversion/storage, exponent/hex syntax, strings, parser validity, CLI changes, future sections, commit of 7.4 or push.

**Checks:** Site check: zero errors/warnings/hints. Build: 22 pages including 404, with existing npm/optional-content/sitemap notices. Local 7.4 route HTTP 200. Displayed ordered edits reconstructed in memory exactly match the rustfmt-formatted complete library; no future Rust files generated or compiled. Static source/range examples reconstruct all four inputs. Affected rendered local links/anchors checked. Accepted 7.3 body preserved, only next navigation changed. Rust and workflow unchanged from dc2d748. git diff --check passes. Native number behavior and reader command route remain unverified until implementation authorization. No screenshots, browser review, push or 7.5 draft.

**Implementation evidence:** Followed the displayed enum insertion, existing boundary-test rename/leading-kind change, appended tests and reconstruction corpus additions in order from accepted 7.3. Actual lib.rs exactly matches the complete manuscript listing. Syntax tests pass (24); formatting passes; workspace Clippy with warnings denied passes; workspace tests pass (35: 24 syntax, 6 text, 5 app). Exact decimal/dot/sign/name boundaries, preserved leading-zero spellings, UTF-8 error ranges and contiguous reconstruction match expectations. No manuscript correction needed. Only syntax lib.rs changed under rlox; adapter, manifests/lockfile, app/text and workflow unchanged. Prior site/text checks remain applicable; no repeated publication build, screenshots/browser review, new validation infrastructure, commit/push or hosted CI run. Other platforms/minimum compiler were not tested.

**Stopping point:** commit the accepted 7.4 pair, then draft 7.5 text only. No push.
