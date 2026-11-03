# Beyond the Interpreter

**Building Modern Language Tools in Rust**

An interactive Rust companion to Robert Nystrom’s *Crafting Interpreters*. Readers build `rlox`: two Lox execution engines and the tools to inspect, edit, debug, test, and distribute a language.

The introduction and Chapter 1 are accepted. Chapter 1 consists of Section 1.1 only; tests will enter in Chapter 2 alongside reading source. Section 2.1 and its matching Rust endpoint are accepted. Section 2.2 and its matching Rust/sample are accepted. Section 2.3 and its matching Rust are accepted. Section 2.4 and its matching unit tests are accepted. Section 3.1 and its matching Rust are accepted. Section 3.2 and its matching Clap implementation are accepted. Section 4.1 and its source-coordinate library are accepted. Section 4.2 and its file-origin types/tests are accepted. Section 4.3 and its workspace migration are accepted. Section 4.4 and the root Rust CI workflow are accepted and passing on GitHub. Section 5.1 and its Rust scanner/cursor visual are accepted. Section 5.2 and its scanner/lookahead visual are accepted. Section 6.1 and its identifier scanner/cursor are accepted. Section 6.2 and its whitespace scanner/source-coverage checks are accepted. Section 6.3 and its slash/comment scanner/source-coverage checks are accepted, completing Chapter 6. Section 7.1 and its Logos migration/regression checks are accepted. Section 7.2 and its punctuation rules/range checks are accepted. The current `source FILE` command reads exactly one supplied file, prints its source text, and reports input errors on stderr with a failure status. Later chapters describe planned work, not implemented capabilities.

## Read the book locally

From the repository root:

```sh
cd site
npm ci
npm run dev
```

Open Astro’s Local URL, normally <http://localhost:4321/>. [Section 1.1](http://localhost:4321/chapter-1/1.1-first-program/) is linked from the introduction. See [site/README.md](site/README.md) for Node/npm prerequisites and site checks.

## Run the current Rust program

With Rust 1.85 or newer installed, from the repository root:

```sh
cd rlox
cargo run -- source hello.lox
```

Expected program output:

```text
print "Hello, Lox!";
```

The local endpoint was verified using rustc and Cargo 1.95.0 on macOS. Formatting, Clippy, and Cargo test passed; six library tests check ranges/UTF-8 slicing and file origins, and five binary tests check the declared command interface and help status. The Section 2.3 route established usage/read diagnostics and recovery; Section 2.4 verified the test additions, a deliberate regression, restoration, and unchanged CLI behavior. Section 3.2 verified dependency resolution, generated help, usage status 2, file-read failure status 1, source output, and migrated tests; see authoring status for details. The workspace migration preserves source contents and registry lock entries; package/workspace tests and root run commands were verified. From `rlox/`, use `cargo test --workspace` to check both packages. Fresh installation and other platforms have not been tested.

## How we work

Draft and render one section, review its text and scope with the user, then follow those same instructions to update the Rust implementation. Correct the manuscript wherever that process reveals a problem. Verify the reader’s route and code, then obtain acceptance of the combined endpoint before moving on. [AGENTS.md](AGENTS.md) contains the operating agreement.

## Project guidance

The guidance lives in `authoring/`, with one document per concern:

1. [AUTHORING_STATUS.md](authoring/AUTHORING_STATUS.md) — actual state, accepted examples, decisions, and verification evidence.
2. [LESSON_BRIEF.md](authoring/LESSON_BRIEF.md) — the current authorized assignment and stopping point.
3. [PROJECT_OUTLINE.md](authoring/PROJECT_OUTLINE.md) — the single five-part, 38-chapter synopsis, preserved from the supplied very detailed outline.
4. [ARCHITECTURE.md](authoring/ARCHITECTURE.md) — technical destination, responsibilities, dependency directions, and transitions to explain.
5. [REVIEW_GUIDE.md](authoring/REVIEW_GUIDE.md) — chapter planning, text-first review, reader-route verification, visuals, and acceptance.

Read these and inspect the actual files before substantive work. The roadmap is not a standing assignment. Existing conversation authorization remains valid even when a brief has not yet caught up.

## Repository

- `site/` — MDX manuscript and the Astro/Starlight publication, with React available for explanations that need interaction.
- `rlox/` — the Rust program readers build, a workspace containing `rlox-app` (CLI using Clap) and `rlox-text` (source-coordinate library).
- `authoring/` — the author-facing guidance and records. Add fixtures or validation tools here only when a lesson needs them.

Readers build Rust; the publishing stack is author infrastructure. Git is an ordinary development tool, not the book’s instructional history. Accepted endpoints must remain reconstructible from the book and appropriate reference files.

This restart carries no implementation or verification claims from earlier attempts. Historical material at <https://github.com/williamcotton/beyond-the-interpreter> is reference only; inspect specific material before reusing it. The current book and Rust workspace have been pushed to origin; Rust CI passes. The site has not been deployed.
