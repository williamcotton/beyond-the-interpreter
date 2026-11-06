import { useId, useState } from 'react';
import './character-cursor.css';
import './mechanism-trace.css';

type Example = { source: string; structure: string; observation: string };
type Comparison = { title: string; prompt: string; examples: Example[] };

// Fixed examples from the lesson rules. Selecting a case only displays its data.
export const comparisons = {
  numbers: {
    title: 'Where does the number end?',
    prompt: 'Choose a spelling and follow the dot into its token.',
    examples: [
      { source: '12.5', structure: 'Number [0, 4) "12.5"', observation: 'The dot and a following digit complete the fractional suffix.' },
      { source: '12.', structure: 'Number [0, 2) "12"\nDot    [2, 3) "."', observation: 'The integer ends at byte 2. The dot remains available to its own rule.' },
      { source: '.5', structure: 'Dot    [0, 1) "."\nNumber [1, 2) "5"', observation: 'The number rule requires a digit at its start. Recognition resumes after the dot.' },
      { source: '12.5.6', structure: 'Number [0, 4) "12.5"\nDot    [4, 5) "."\nNumber [5, 6) "6"', observation: 'The optional fractional group occurs once. The next dot begins a new token.' },
      { source: '-12.5', structure: 'Minus  [0, 1) "-"\nNumber [1, 5) "12.5"', observation: 'The sign has its own token. The parser will determine how that minus participates in the expression.' },
    ],
  },
  precedence: {
    title: 'Which rule finishes the operand?',
    prompt: 'Compare a precedence boundary, explicit grouping and a subtraction chain.',
    examples: [
      { source: '1 + 2 * 3', structure: '+\n├─ 1\n└─ *\n   ├─ 2\n   └─ 3', observation: 'The right factor consumes 2 * 3 before returning to term. Addition receives the entire product as its right operand.' },
      { source: '(1 + 2) * 3', structure: '*\n├─ group\n│  └─ +\n│     ├─ 1\n│     └─ 2\n└─ 3', observation: 'The group invokes expression for 1 + 2. The completed group then serves as the left operand of multiplication.' },
      { source: '8 - 3 - 1', structure: '-\n├─ -\n│  ├─ 8\n│  └─ 3\n└─ 1', observation: 'Both minus signs belong to the same term loop. Its second iteration uses the entire 8 - 3 tree as the new left operand.' },
      { source: '8 - (3 - 1)', structure: '-\n├─ 8\n└─ group\n   └─ -\n      ├─ 3\n      └─ 1', observation: 'The inner term loop belongs to the grouped expression. It finishes before the outer subtraction can acquire its right operand.' },
    ],
  },
  slots: {
    title: 'Read a field from the actual child slot',
    prompt: 'Choose complete or partial source, then follow the accessor through the immediate children.',
    examples: [
      { source: '* 2', structure: 'BinaryExpr\n├─ slot 0 → MissingExpr\n└─ slot 1 → NumberExpr("2")\n\nleft()  → None\nright() → Some(NumberExpr)', observation: 'Choosing slot 0 before casting preserves the absent left operand. Filtering to successful casts first would move the number into that position.' },
      { source: '1 +', structure: 'BinaryExpr\n├─ slot 0 → NumberExpr("1")\n└─ slot 1 → MissingExpr\n\nleft()  → Some(NumberExpr)\nright() → None', observation: 'The missing node holds the right-hand position. Its empty text contributes zero bytes to reconstruction.' },
      { source: '1 + 2 * 3', structure: 'Outer BinaryExpr\n├─ slot 0 → NumberExpr("1")\n├─ direct operator token → Plus\n└─ slot 1 → BinaryExpr("2 * 3")\n\noperator_token() → Plus', observation: 'The outer accessor examines direct token children. The star is nested inside the right binary node, so that child owns its operator lookup. Spaces are omitted from this outline.' },
      { source: '2', structure: 'NumberExpr("2")\n\nBinaryExpr::cast(number) → None\nNumberExpr::cast(number) → Some(NumberExpr)', observation: 'The receiver kind is checked before field access. A number provides its number-token accessor through the successful number view.' },
    ],
  },
} satisfies Record<string, Comparison>;

export default function BoundaryCases({ variant }: { variant: keyof typeof comparisons }) {
  const id = useId();
  const [selected, setSelected] = useState(0);
  const comparison = comparisons[variant];
  const example = comparison.examples[selected];
  return <figure className="character-cursor mechanism-trace" aria-labelledby={`${id}-title`}>
    <figcaption id={`${id}-title`}>{comparison.title}</figcaption>
    <p>{comparison.prompt}</p>
    <div className="character-cursor-controls" role="group" aria-label="Source examples">
      {comparison.examples.map((item, index) => <button type="button" key={item.source}
        aria-pressed={selected === index} onClick={() => setSelected(index)}>{item.source}</button>)}
    </div>
    <div aria-live="polite" aria-atomic="true">
      <p>Source <code>{example.source}</code></p>
      <pre tabIndex={0} aria-label="Structure and result">{example.structure}</pre>
      <p>{example.observation}</p>
    </div>
    <details><summary>Read all examples</summary>
      {comparison.examples.map(item => <div key={item.source}>
        <p><code>{item.source}</code></p><pre tabIndex={0}>{item.structure}</pre><p>{item.observation}</p>
      </div>)}
    </details>
  </figure>;
}
