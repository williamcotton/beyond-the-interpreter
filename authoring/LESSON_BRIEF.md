# Current assignment

## CH07-05 — Keep strings together (implementation phase)

**Status:** Accepted when the user said “great, commit and then outline chapter 8 with numbered sections”.

**Authorization:** the user accepted 7.4 and requested committing it and moving to 7.5. The accepted number pair is committed as `bebc8e2` (`feat: pair section 7.4 with number scanning`). The user subsequently said “ok, looks good, now update rlox”, approving the text and authorizing its implementation. No commit or push requested.

**Accepted starting state:** inspected actual Logos 0.16.1 enum/rules/adapter and tests. Twenty-four syntax and thirty-five workspace tests pass at 7.4. Scanner has numbers, reserved words, punctuation, ASCII identifiers, retained trivia and Error fallback; quotes are currently unsupported.

**Reader question/example:** how can print "é"; keep its quoted contents together, and what happens if the final quote is absent? String [6,10) includes quotes/two-byte é. A lone opening quote plus é becomes one Error [0,3).

**Behavior:** String rule matches opening quote, zero or more non-quote characters, closing quote. Empty, UTF-8 and multiline contents allowed; backslashes do not escape anything. Preserve source spelling/delimiters without a value payload. At EOF an incomplete match becomes Error from opening quote through EOF via existing adapter; ordinary unsupported characters retain their existing behavior.

**Ordered reader route:** insert String before Number; append complete-spelling/adjacency/backslash tests; explain unfinished-input ranges then append EOF/error-offset tests; extend source reconstruction corpus; syntax tests, formatting, workspace Clippy and tests. Complete resulting syntax library separately accessible. No existing test expectation changes.

**New Rust ideas:** hash-delimited raw string literals needed to express quotes plainly; distinguish Rust escapes used to construct test input from Lox source characters. Reuse enum attributes, tests, range slicing and adapter. No callback, helper, new import, module, dependency or manifest edit.

**References/compatibility:** official Crafting Interpreters chapter cached HTML/string-literals anchor and live Scanner.java confirm first-quote termination, multiline and lack of escapes. Official Logos common-regex/error semantics and installed pinned 0.16.1 pattern/generator source inspected read-only: non-quote class is not rejected as dot repetition; EOF without an accepting token takes the consumed error range. This supports proposed behavior; native verification remains for implementation phase. Rust Reference raw-string syntax checked. No dependency/version change.

**Visual purpose:** static annotated successful UTF-8 token stream and unfinished-string byte range at the relevant explanations. No custom component/animation/second scanner. User owns visual confirmation; no screenshots/browser visual review.

**Excluded:** escape extensions, runtime string-value extraction/storage, typed diagnostics, parser/CLI, later chapters, commit or push. Chapter 7 completion remains pending combined acceptance.

**Text-phase checks:** Site check has zero errors/warnings/hints; build succeeds with 23 pages including 404 and existing npm/optional-content/sitemap notices. Displayed edits applied in order to accepted source in memory exactly match the formatted complete library; rustfmt used stdin only. Existing source/tests preserved apart from the proposed String addition and appended tests/corpus. Static UTF-8 source/range/spelling examples reconstruct both samples exactly. Local 7.5 page returns HTTP 200; affected rendered local links/anchors resolve. git diff --check passes. Official Logos/Rust reference URLs and anchors resolve; Crafting Interpreters string anchor/semantics checked against cached official chapter and live official Scanner.java. Accepted 7.4 body preserved; only next navigation updated. Rust, workflow, dependencies, components and styles unchanged from bebc8e2. Native regex derivation, EOF error behavior and actual reader commands remain pending approval/implementation; no future Rust files generated, compiled or tested. No browser review/screenshots, validation framework, push or Chapter 8 draft.

**Implementation evidence:** Started from actual accepted 7.4 and followed the displayed String rule, complete-spelling/adjacency/backslash tests, unfinished-input tests and reconstruction corpus extension in order. Actual syntax library exactly matches the complete listing. Ran displayed commands from rlox in order: syntax tests pass (28), formatting passes, workspace Clippy with warnings denied passes, workspace tests pass (39: 28 syntax, 6 text, 5 app). Empty/multiline/UTF-8 strings, comment-like contents, retained quotes/spelling, adjacent boundaries and literal backslashes match expectations. EOF errors cover the opening quote through EOF, including the exact offset after an earlier complete string; source reconstruction and all prior tests pass. No manuscript correction needed. Only syntax lib.rs changed under rlox; existing rules/adapter, manifests/lockfile, app/text and workflow unchanged. Prior site/text/link checks remain applicable. No repeated site build, screenshots/browser review, new validation infrastructure, commit/push, hosted CI run or Chapter 8 work. Other platforms/minimum compiler remain untested.

**Stopping point:** commit accepted 7.5 pair, then outline Chapter 8 with numbered sections. No lesson draft, Rust changes or push authorized.
