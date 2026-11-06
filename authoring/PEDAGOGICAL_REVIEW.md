# Manuscript teaching review — 2026-11-05

## Assignment and disposition

The user authorized a manuscript-wide additive teaching pass, using the supplied
Chapter 12 discussion as the model. Preserve all chapters, sections, and reader
code; add explanation and useful interaction. This supersedes the older
single-section assignment for this documentation-only task. Ready for human text
and interaction review, not accepted. No Rust implementation changes, commit,
push, or publication.

Coverage is the introduction and all 44 numbered MDX sections currently present,
Chapters 1–12. The roadmap's later chapters have no MDX manuscript to review.

## Editorial diagnosis

The existing manuscript generally does explain individual Rust constructs and
provides concrete commands, observations, failures and complete listings. Its
principal weakness is transfer: being able to follow each edit does not always
prepare a reader to explain why a different input produces a different outcome.
The difficult transitions combine several responsibilities at once. More syntax
commentary alone would not resolve that problem.

The additions make responsibility boundaries explicit, trace an actual example
through the mechanism, and ask for a prediction that distinguishes a plausible
wrong mental model from the intended one. Every numbered section now has a
specific reasoning question and a revealable explanation. These are reading
checks, not missing required implementation steps. They do not change any
exercise into a prerequisite.

## Chapter findings and revisions

1. **First executable.** Cargo can conceal compilation versus execution. Explain
   which artifact changes when the greeting changes. A diagram adds little to
   this one-step distinction, so retain prose.
2. **Source input.** Distinguish shell grouping, Cargo's separator, process
   arguments, path ownership, read text, policy and I/O errors. Add an argument
   trace; make the test boundary and what unit assertions cannot prove explicit.
3. **Commands and Clap.** Explain the selector-to-payload handoff and the value
   produced by a derive parser. Walk source plus filename through Cli and Command
   before main uses them. Help and invalid arguments need different process
   outcomes even though the testing API reports both through its error type.
4. **Coordinates, packages, CI.** Separate ordered coordinates from valid slices,
   file identity from bounds, membership from dependency, and green CI from
   evidence about a particular revision. Add a valid/invalid UTF-8 boundary trace.
5. **Scanner mechanics.** Strengthen the distinction between classification,
   advancement and byte positions. Existing character/lookahead explorers already
   show the right mechanism; add failed-lookahead reasoning rather than duplicate
   their interaction.
6. **Growing tokens.** Explain start versus continuation, coverage versus
   classification, and delimiter ownership. Keep the existing identifier explorer.
   Prose checks suffice for whitespace and comment boundaries at this size.
7. **Logos.** Make generated mechanics, authored language rules and the token
   adapter visibly separate. Add a before/after responsibility trace and follow
   successful and failed lexer outcomes through the adapter. Distinguish keyword
   prefixes, numeric spellings, signs, string context and eventual values. Keep
   lesson-local scope: digits are still unsupported before number recognition.
8. **Token command.** Explain why the command is borrowed before dispatch and why
   display escapes cannot define source coordinates. Treat the error flag as a
   summary that cannot revert from true. Explain independent expected output and
   the three process observations. Retain the existing token-range explorer.
9. **Editor.** Distinguish language registration, editing actions, display scopes,
   multiline state and theme choices. Explain why direct document replacement is
   not an auto-closing test. Make checkout versus installed-archive provenance and
   Node versus editor-host environments explicit. State/action examples answer
   these questions without introducing another editor simulator.
10. **Owned trees.** Separate finite values from recursive type layout, construction
    from parsing, and descent from returning child representations. Add a complete
    mental execution of the prefix printer for -(1 + 2). Keep static tree shapes:
    interaction is more useful when the parser constructs them in Chapter 11.
11. **Recursive descent.** Explain prefix acceptance versus whole-input acceptance,
    the one shared cursor across recursion, progress to a base case, shared depth
    budgets, and precedence as a consequence of delegation and stopping. Add a
    left-fold trace that shows how a loop builds a tall tree. Preserve the existing
    grouping cursor and all grammar/code instructions.
12. **Rowan.** Establish strategy/construction/representation separately before
    coordinate changes. Add a manual number builder/readback example using only
    kinds already available in 12.2. Show normal builder operations, both stacks
    during grouping, error cleanup and delayed checkpoint wrapping. Distinguish
    nodes from branching, bump from mere advancement, checkpoints from cursor
    rewinds, and structural reuse from automatic incremental reparsing. Reinforce
    recovery progress and typed receiver/optional field distinctions. No future
    MissingExpr is smuggled into pre-recovery traces.

## Interaction design and placement

Eight fixed traces, 37 explicit frames, in seven pages: arguments (2.1), byte
boundaries (4.1), Logos responsibility handoff (7.1), left fold (11.4), manual
builder (12.2), grammar/builder stacks and failed-group cleanup (12.3), checkpoint
wrapping (12.5). Frames are authored observations of existing operations, not a
JavaScript scanner or parser. They are embedded beside the relevant mechanism.
Existing explorers remain intact.

All traces have initially rendered content, Previous/Next/Start again controls,
disabled boundary controls, a polite live region and a complete text disclosure.
State is text rather than color-only encoding; wrapped layouts use existing theme
variables, and no animation is required. Diagram operations abbreviate kind
conversions where explicitly stated. The checkpoint example has no trivia and
matches the manuscript's delayed wrapping, not an alternative eager algorithm.

## Verification and limits

- Installed site dependencies with the existing lockfile; no dependency changes.
- Site type/content check: zero errors, warnings or hints. Static build: 46 pages,
  including the existing fallback 404. Existing optional i18n/404/sitemap notices.
- Exercised every new frame through a source-level hook harness: forward,
  backward, reset, boundary controls, displayed state, live region and complete
  server-rendered text. This does not establish real-browser hydration or layout.
- Compared all MDX to the checkout baseline: all original code fences remain
  byte-identical and in order; original H2 headings and frontmatter remain intact.
  The only new Rust listing is the explicitly labeled reading experiment.
- All 3,066 built local links/anchors resolve; every numbered page contains its
  rendered reasoning question and answer.
- Native source, checkpoints, workflow, manifests, lockfiles and existing diagram
  data are unchanged. No Rust tests or ordered implementation route were repeated;
  the task changes explanation and author publishing components only.
- Public Rust args documentation and Logos handbook consulted for their roles.
  The versioned Rowan web endpoint was inaccessible; APIs were checked against
  the installed Rowan 0.17.0 builder/api source, including first_token and
  checkpoint behavior. No dependency versions changed.
- Appearance and learning effectiveness require human review. Repository guidance
  excludes screenshots and automated visual inspection without an explicit request.

## Review focus

Read the progression through 7.1, 11.4, and 12.2–12.5. After stepping each trace,
can a reader predict the next state and explain which component makes the choice?
Then sample early chapters for pacing: the additional reasoning should clarify a
real distinction without turning routine setup into a second lesson.

## Second examination — 2026-11-06

The user requested another manuscript-wide examination, emphasizing repetitive
section flow and allowing demonstrations where they help. This pass revises the
first pass's approach. Its uniform closing questions repeated nearby explanations,
and paragraphs inserted immediately before headings sometimes explained the same
mechanism again in the following subsection. The consistency had become a pacing
problem.

### Editorial changes

Removed the 44 uniform closing checks. Nine specific prediction exercises now
appear before an observation or implementation that answers them. They cover a
regression in argument policy, UTF-8 boundaries, failed lookahead, classification
versus coverage, process output, allocation ownership, mixed recursive depth,
trivia parentage and recovery progress. The existing bordered treatment groups
those questions with their answers.

Consolidated repeated first-pass summaries and routine unchanged-file inventories.
Scanner endings now help diagnose boundary failures. Parser endings connect test
observations to the shape or call that produced them. Setup sections rely on their
existing experiments and observations. Retained commands, edit ordering, complete
reference listings and meaningful success/failure observations.

Added explanations where the required inference was previously implicit. These
include the lifetime of a filename borrowed through a helper, manifest-relative
paths and package/binary names, CRLF ownership, the optional fractional suffix,
Node-to-editor-host execution, package/test provenance, EOF versus token errors,
recursive result height and the single-kind expansion of the Rowan adapter macro.
Reworked 12.2 to begin constructing its first concrete tree after the motivation
already supplied in 12.1.

### Demonstration decisions

Added selectable examples for number boundaries (7.4), precedence/grouping/
association (11.5), and typed child slots/operator ownership (12.7). Added traces
for multiline TextMate context (9.3) and recovery through a closing delimiter
(12.6). All are fixed authored examples of the lesson rules. No second language
implementation is introduced.

Replaced the Logos responsibility stepper with ordinary connected prose. Corrected
the two-stack trace to show the number branch inside primary(), matching 12.3's
actual parser. Clarified that the builder demo title refers to the numeric literal
12.5. Made the checkpoint's unread source consistently use its unspaced spelling.

### Verification

Site check reports zero errors, warnings and hints. Static build produces 46 pages.
All 3,030 local links and anchors resolve. Every original fenced listing, H2 heading
and frontmatter block matches the start-of-pass baseline. The MDX is approximately
3,229 words shorter, excluding the new demonstration source.

Source-level interaction harnesses exercise all nine traces / 43 frames and all
three case selectors / 13 examples, including backward/reset, selected-state
feedback, live content and server-rendered complete fallbacks. Existing native
source and tests supply the grammar/token/recovery contracts for the authored
cases. No reader implementation replay or browser hydration/appearance validation
was performed. Human review of pacing and presentation remains pending.

Review 7.4, 9.3, 11.5 and 12.6–12.7 for the revised teaching approaches. Ready for
review. No commit or publication authorization has been inferred.
