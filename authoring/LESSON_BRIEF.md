# Current assignment

## CH07-03 — Tell keywords from names (implementation phase)

**Status:** Accepted. The user said “ok great, commit and let’s move on to section 7.4” after combined review.

**Authorization:** the user accepted 7.2 and requested committing it before moving on to 7.3. Paired changes committed as `125aeb9` (`feat: pair section 7.2 with complete punctuation`). The user subsequently approved the text and requested updating rlox, authorizing its displayed Rust edits and checks. No new commit/push or 7.4 draft requested. Explain identifier-spelling retrieval here as explicitly agreed with the user.

**Accepted starting point:** actual Logos 0.16.1 scanner inspected: complete punctuation, ASCII Identifier, retained trivia, Error adapter and source coverage. Nineteen syntax/thirty workspace tests previously pass. Existing identifier EOF test includes print as an Identifier; that classification intentionally changes in this lesson. Source slice API already exists.

**Reader question/example:** how do print and printer get different kinds while their original spellings remain available? Target Print [0,5), Whitespace [5,6), Identifier [6,13) for print printer; range.slice(source) retrieves each exact borrowed source fragment.

**Behavior taught:** sixteen canonical lowercase keyword spellings added as fixed variants beside the existing identifier regex. Same-length exact keyword wins specificity; a longer identifier wins length; matching is case-sensitive. No keyword map, callbacks or numeric priority values. Token remains kind/range; source characters live in caller-retained original source and are borrowed on request. Later semantic name representation is not chosen here.

**Reader route:** add complete keyword variants immediately before Identifier's regex; change print to printer in the existing identifier-only EOF test; append exact keyword, longer-name/case and kind/range/spelling tests; extend existing coverage corpus; run syntax tests then formatting/Clippy/workspace tests. Complete resulting library separately accessible; keep all other expectations and the adapter/regexes unchanged.

**Tests proposed:** all sixteen reserved words with whole-source ranges; forest/printer/classify/true_value/_if/Print remain identifiers; explicit print printer kinds/ranges plus source slices for keyword, whitespace and name; coverage across keywords/prefix names, punctuation and comments.

**New Rust reasoning:** reuse token attributes, enum alternatives, test arrays and existing TextRange::slice. Explain Option<&str>, borrowing without per-token allocation, and why caller/source association matters at the point of retrieving a name. No new ownership field, lifetime-bearing Token type, source database, helper/API or interning scheme.

**Visual approach:** static kind/range/spelling listing plus prefix comparison. Existing listings suffice; no component, animation, JavaScript lexer, browser review or screenshots. User provides visual confirmation.

**References:** official Crafting Interpreters Scanner keyword map checked for all sixteen canonical spellings; Logos disambiguation rechecked for specificity/longer matches; Rust Book string slices for borrowed spelling. Logos version/API and dependencies unchanged.

**Excluded:** numeric/string tokenization (7.4–7.5), keyword semantics/evaluation/resolution, parser/HIR/name storage, owned or interned token payloads, new dependency/module/CLI, custom priority framework, unrelated accepted Rust changes, later drafts, commit or push.

**Text-phase checks (before implementation):** final site check zero errors/warnings/hints; build succeeds (21 generated pages including 404) with existing optional-content/sitemap and npm user-config notices. Local section HTTP 200. Three manuscript reference URLs/fragments and affected rendered local links/anchors resolve. Displayed edits applied to accepted source in memory agree with complete resulting library; diff context and rustfmt stdin formatting checked. Sixteen keyword patterns and expected table agree with official Lox Scanner map. Existing nineteen punctuation rules, all three regexes and scan adapter unchanged. Static kind/range/spelling listing agrees with displayed test; byte intervals and all three source-slice expectations reconstruct print printer exactly. Accepted 7.2 body preserved; next link updated. rlox/.github/site dependencies/components/styles match HEAD. git diff --check passes. No future Rust files generated/compiled/tested, dependency install, browser review/screenshots, new commit/push or 7.4 authoring. Native keyword resolution and actual command-route observations remain pending text approval.

**Implementation evidence:** started from actual accepted 7.2; inserted all keyword variants before Identifier, changed the identifier-only EOF input from print to printer, appended the displayed keyword/name/spelling tests and extended the coverage corpus in order. Actual lib.rs exactly matches complete listing. Ran displayed syntax test, formatting, workspace Clippy with warnings denied, and workspace tests in order: twenty-two syntax tests and all thirty-three workspace tests pass. All sixteen keyword spellings, longer-name/case boundaries, intentional print reclassification, exact range.slice(source) results for keyword/whitespace/name and source reconstruction match expectations. Static listing agrees with native-tested kinds/ranges/source slices. Existing regexes/adapter, manifests/lockfile, app/text, sample/ignore/workflow unchanged. CLI source bytes/help/usage/read-error streams/statuses retain expected behavior. No manuscript correction needed. Prior site/source checks remain applicable; no repeated site build, browser review/screenshots, validation infrastructure, commit/push or hosted run.

**Stopping point:** 7.3 accepted; commit this pair, then draft 7.4 text only under the user’s current authorization. No push.
