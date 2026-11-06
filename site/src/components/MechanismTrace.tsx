import { useId, useState } from 'react';
import './character-cursor.css';
import './mechanism-trace.css';

type Frame = { operation: string; state: string; why: string };
type Trace = { title: string; question: string; frames: Frame[] };

// Fixed teaching traces, not a second scanner/parser. Each frame is an explicit
// observation of the corresponding lesson's operations; no input is evaluated.
export const traces = {
  arguments: {
    title: 'From the terminal to the selected filename',
    question: 'At which boundary do the separator and the executable name disappear?',
    frames: [
      { operation: 'cargo run --quiet -- "my program.lox"', state: 'Shell groups one filename:\nmy program.lox', why: 'The surrounding quotes group the argument; they are not characters in its value.' },
      { operation: 'Cargo consumes its own options', state: 'Cargo option: --quiet\nSeparator: --\nArgument for rlox: my program.lox', why: 'The separator tells Cargo where the program arguments begin.' },
      { operation: 'std::env::args()', state: 'Position 0: executable name/path\nPosition 1: my program.lox', why: 'Our iterator begins with the executable argument, followed by the filename.' },
      { operation: '.nth(1)', state: 'Skip position 0\nReturn Some("my program.lox")', why: 'The returned String is argument text. No file has been opened.' },
    ],
  },
  bytes: {
    title: 'A range selects byte boundaries',
    question: 'Predict whether each range selects valid text before advancing.',
    frames: [
      { operation: 'Source: aé!', state: 'Offset:  0    1    2    3    4\nBytes:   | a  | é       | !  |\nBoundary yes  yes  no   yes  yes', why: 'é occupies bytes 1 and 2. Offset 2 is inside its UTF-8 encoding.' },
      { operation: 'Select [1, 3)', state: 'a | é | !\n    selected: é\nLength: 3 - 1 = 2 bytes', why: 'Both endpoints are valid boundaries; the end is excluded.' },
      { operation: 'Select [1, 2)', state: 'Start: valid boundary\nEnd: inside é\nResult: no string slice', why: 'Ordering alone is insufficient. Rust string slices must contain complete UTF-8 encodings.' },
      { operation: 'Select [4, 4)', state: 'Position: just after !\nSelected text: empty string\nLength: 0 bytes', why: 'The end of the string is a valid boundary. An empty selection can represent an insertion position.' },
    ],
  },
  fold: {
    title: 'A loop grows a left-associated tree',
    question: 'What does the accumulated left tree contain after each iteration?',
    frames: [
      { operation: 'Read the first unary operand', state: 'Source: 8 / 4 / 2\nLeft tree: 8\nUnread: /4/2', why: 'The loop begins with a complete operand, not an empty binary node.' },
      { operation: 'Consume /, then read unary 4', state: 'Left tree: (/ 8 4)\nUnread: /2\nHeight: 1', why: 'The previous left tree and the new operand become children of one new parent.' },
      { operation: 'Consume /, then read unary 2', state: 'Left tree: (/ (/ 8 4) 2)\nUnread: nothing\nHeight: 2', why: 'The entire previous binary tree becomes the next left child. That is left association.' },
      { operation: 'Return the accumulated tree', state: 'Root: /\n├─ Left: /\n│  ├─ 8\n│  └─ 4\n└─ Right: 2', why: 'The loop has finished, but its result is nested. A loop can build a tall tree without deeply recursive parsing calls.' },
    ],
  },
  builder: {
    title: 'Build the number 12.5 with the builder',
    question: 'Before each step, name the container that will receive the next token.',
    frames: [
      { operation: 'builder.start_node(Root)', state: 'Open stack (outer → inner): Root\nTree under construction:\nRoot', why: 'Open the root container. These operation labels abbreviate the into_raw() conversions used below.' },
      { operation: 'builder.start_node(NumberExpr)', state: 'Open stack: Root → NumberExpr\nRoot\n└─ NumberExpr', why: 'A node represents structure, even when it will contain only one token.' },
      { operation: 'builder.token(Number, "12.5")', state: 'Open stack: Root → NumberExpr\nRoot\n└─ NumberExpr\n   └─ Number "12.5"', why: 'The token goes into the innermost open container. This operation does not close NumberExpr.' },
      { operation: 'builder.finish_node() // NumberExpr', state: 'Open stack: Root\nCompleted child: NumberExpr\n└─ Number "12.5"', why: 'Close NumberExpr. It is now a completed child of Root.' },
      { operation: 'builder.finish_node() // Root; builder.finish()', state: 'Open stack: empty\nImmutable green tree:\nRoot → NumberExpr → Number "12.5"', why: 'All containers must be closed before extracting the completed green storage.' },
      { operation: 'SyntaxNode::new_root(green)', state: 'Positioned view:\nRoot [0, 4)\n└─ NumberExpr [0, 4)\n   └─ Number "12.5" [0, 4)', why: 'The syntax handle adds navigation and positions. It does not parse the spelling or change the tree.' },
    ],
  },
  twoStacks: {
    title: 'Two stacks while parsing (1)',
    question: 'The grammar stack says where we are; the builder stack says where emitted text goes.',
    frames: [
      { operation: 'Begin the outer expression', state: 'Grammar: expression → primary\nBuilder: Root\nUnread: (1)', why: 'Delegating through grammar methods does not itself open Rowan nodes.' },
      { operation: 'group opens GroupingExpr, then bump consumes (', state: 'Grammar: expression → primary → group\nBuilder: Root → GroupingExpr\nEmitted: (\nUnread: 1)', why: 'bump emits the current token into the open group and advances the shared cursor.' },
      { operation: 'primary opens NumberExpr and emits 1', state: 'Grammar: … group → expression → primary\nBuilder: Root → GroupingExpr → NumberExpr\nEmitted in NumberExpr: 1\nUnread: )', why: 'The Rust call stack and Rowan stack are related but not identical: several grammar calls share one structural node.' },
      { operation: 'Close NumberExpr; inner expression returns', state: 'Grammar: outer … group\nBuilder: Root → GroupingExpr\nUnread: )', why: 'Returning does not rewind the cursor. The group that opened the parenthesis owns the close.' },
      { operation: 'bump consumes ); close GroupingExpr', state: 'Grammar: return to outer expression\nBuilder: Root\nGroup text: (1)\nUnread: nothing', why: 'The close is emitted before the group container closes, so both parentheses belong to that group.' },
    ],
  },
  cleanup: {
    title: 'An error must still close the group',
    question: 'For (1, what must happen before the failed group returns?',
    frames: [
      { operation: 'Open the group and emit (', state: 'Builder: Root → GroupingExpr\nUnread: 1', why: 'The missing closing parenthesis is not known yet.' },
      { operation: 'Parse and close the number', state: 'Builder: Root → GroupingExpr\nGroup children: "(" and NumberExpr("1")\nUnread: end of input', why: 'The number succeeds; the enclosing group is still open.' },
      { operation: 'Save Err: expected closing parenthesis', state: 'Builder: Root → GroupingExpr\nSaved result: Err\nNo ) token emitted', why: 'Saving the error rather than immediately returning leaves us able to complete the container.' },
      { operation: 'finish_node(), then return the saved error', state: 'Builder: Root\nCompleted partial group: (1\nDiagnostic: expected closing parenthesis at EOF', why: 'The partial tree preserves the supplied text. Error propagation happens after structural cleanup.' },
      { operation: 'Finish the root', state: 'Builder: empty\nTree text: (1\nErrors: one missing-close diagnostic', why: 'The public result includes the tree and its syntax error. No closing source character was invented.' },
    ],
  },
  recovery: {
    title: 'Return control to the owner of the closing parenthesis',
    question: 'Follow (1 + ) through the missing operand and back to its group.',
    frames: [
      { operation: 'group opens its node and consumes (', state: 'Builder: Root → GroupingExpr\nUnread: 1 + )', why: 'The group asks expression to parse its contents.' },
      { operation: 'term obtains the number 1', state: 'Builder: Root → GroupingExpr\nChildren: "(", NumberExpr(1)\nUnread:  + )', why: 'The first operand has finished. Lookahead finds an additive operator.' },
      { operation: 'term consumes the space and +', state: 'Builder: Root → GroupingExpr\nChildren: "(", NumberExpr(1), " ", "+"\nUnread:  )', why: 'The right factor calls unary to obtain the next operand.' },
      { operation: 'unary emits trivia and creates MissingExpr', state: 'Cursor: byte 5, at )\nMissingExpr range: [5, 5)\nDiagnostic range: [5, 6)\nUnread: )', why: 'The empty slot records the missing operand. The diagnostic points to the close encountered where an operand was expected.' },
      { operation: 'Return height 0 and finish the partial sum', state: 'Builder: Root → GroupingExpr\nChildren: "(", BinaryExpr(1 + MissingExpr)\nUnread: )', why: 'The ordinary recovery result lets term wrap both operand slots. The nested expression returns with the close still available.' },
      { operation: 'group consumes ) and closes', state: 'Builder: Root\nCompleted group text: (1 + )\nUnread: nothing', why: 'The group owns this delimiter and advances the cursor. The source is retained along with the missing-operand diagnostic.' },
    ],
  },
  stringContext: {
    title: 'Carry a string context from one line to the next',
    question: 'Which words become keywords depends on the context at the start of the line.',
    frames: [
      { operation: 'Start the first line', state: 'Line 1: print "first\nIncoming context: ordinary source', why: 'print is recognized as a keyword before the opening quote begins a string.' },
      { operation: 'Reach the end of the first line', state: 'Line 1: print "first\nOutgoing context: inside string', why: 'The line ends while the string is open. The grammar engine returns that context for the next line.' },
      { operation: 'Read the next line with the saved context', state: 'Line 2: var // text"; print 2;\nIncoming context: inside string\nString contents: var // text', why: 'The word var and the two slashes remain string content until the closing quote.' },
      { operation: 'Continue after the closing quote', state: 'Line 2 suffix: ; print 2;\nContext: ordinary source\nprint → keyword\n2 → number', why: 'Closing the string restores the surrounding context, making the remaining source eligible for the ordinary rules.' },
    ],
  },
  checkpoint: {
    title: 'Wrap already-emitted children at a checkpoint',
    question: 'The checkpoint marks a child boundary; it never rewinds the input.',
    frames: [
      { operation: 'Save checkpoint C before reading 8', state: 'Open container: Root\nChildren: C |\nUnread: 8/4/2', why: 'The source is 8/4/2. Save the child boundary before emitting the left number.' },
      { operation: 'Parse unary 8', state: 'Children: C | NumberExpr(8)\nUnread: /4/2', why: 'The left node is finished before a binary parent is opened.' },
      { operation: 'Emit /; parse unary 4; check height', state: 'Children: C | NumberExpr(8), "/", NumberExpr(4)\nUnread: /2', why: 'The manuscript waits for a successful right operand and height check before wrapping.' },
      { operation: 'start_node_at(C, BinaryExpr); finish_node()', state: 'Children: C | BinaryExpr(8 / 4)\nUnread: /2', why: 'Everything after C becomes children of the new binary node. The parser cursor does not move during wrapping.' },
      { operation: 'Emit /; parse 2; wrap again at C', state: 'Children: C | BinaryExpr\n                  ├─ BinaryExpr(8 / 4)\n                  ├─ "/"\n                  └─ NumberExpr(2)', why: 'The same bookmark wraps the previous binary as the left child of the next one, preserving left association.' },
    ],
  },
} satisfies Record<string, Trace>;

export default function MechanismTrace({ variant }: { variant: keyof typeof traces }) {
  const id = useId();
  const [step, setStep] = useState(0);
  const trace = traces[variant];
  const frame = trace.frames[step];
  return <figure className="character-cursor mechanism-trace" aria-labelledby={`${id}-title`}>
    <figcaption id={`${id}-title`}>{trace.title}</figcaption>
    <p>{trace.question}</p>
    <div aria-live="polite" aria-atomic="true">
      <p><strong>Step {step + 1} of {trace.frames.length}</strong></p>
      <p><code>{frame.operation}</code></p>
      <pre tabIndex={0} aria-label="State at this step">{frame.state}</pre>
      <p>{frame.why}</p>
    </div>
    <div className="character-cursor-controls">
      <button type="button" disabled={step === 0} onClick={() => setStep(step - 1)}>Previous</button>
      <button type="button" disabled={step === trace.frames.length - 1} onClick={() => setStep(step + 1)}>Next step</button>
      <button type="button" disabled={step === 0} onClick={() => setStep(0)}>Start again</button>
    </div>
    <details>
      <summary>Read the complete trace</summary>
      <ol>{trace.frames.map((item, index) => <li key={index}>
        <p><code>{item.operation}</code></p><pre tabIndex={0}>{item.state}</pre><p>{item.why}</p>
      </li>)}</ol>
    </details>
  </figure>;
}
