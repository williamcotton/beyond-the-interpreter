import { useId, useState } from 'react';
import './character-cursor.css';

// Authored observations for Section 5.1, checked against the native scanner.
// This component displays those observations; it does not scan source.
const examples = [
  {
    source: '()!=',
    tokens: [
      { text: '(', kind: 'LeftParen', start: 0, end: 1 },
      { text: ')', kind: 'RightParen', start: 1, end: 2 },
      { text: '!', kind: 'Bang', start: 2, end: 3 },
      { text: '=', kind: 'Equal', start: 3, end: 4 },
    ],
  },
  {
    source: 'é!',
    tokens: [
      { text: 'é', kind: 'Error', start: 0, end: 2 },
      { text: '!', kind: 'Bang', start: 2, end: 3 },
    ],
  },
];

export default function CharacterCursor() {
  const id = useId();
  const [exampleIndex, setExampleIndex] = useState(0);
  const [step, setStep] = useState(0);
  const example = examples[exampleIndex];
  const current = step > 0 ? example.tokens[step - 1] : undefined;
  const cursor = current?.end ?? 0;

  return (
    <figure className="character-cursor" aria-labelledby={`${id}-caption`}>
      <figcaption id={`${id}-caption`}>One iteration, one character</figcaption>
      <label htmlFor={`${id}-example`}>Source example </label>
      <select id={`${id}-example`} value={exampleIndex} onChange={(event) => {
        setExampleIndex(Number(event.target.value));
        setStep(0);
      }}>
        {examples.map((item, index) => <option key={item.source} value={index}>{item.source}</option>)}
      </select>
      <ol className="character-cursor-cells" aria-label="Characters and their byte ranges">
        {example.tokens.map((token, index) => (
          <li key={token.start} data-consumed={index < step}>
            <code>{token.text}</code>
            <span>bytes [{token.start}, {token.end})</span>
            <span>{index < step ? 'Consumed' : 'Unread'}</span>
          </li>
        ))}
      </ol>
      <p className="character-cursor-state" aria-live="polite" aria-atomic="true">
        Step {step} of {example.tokens.length}. Cursor: byte {cursor}.
        {' '}{current
          ? `Read ${current.text}; emit ${current.kind} [${current.start}, ${current.end}).`
          : 'No characters consumed; no tokens emitted.'}
        {' '}{step === example.tokens.length
          ? 'No characters remain. The loop stops.'
          : `Next character: ${example.tokens[step].text}.`}
      </p>
      <div className="character-cursor-controls">
        <button type="button" disabled={step === 0} onClick={() => setStep(step - 1)}>Previous</button>
        <button type="button" disabled={step === example.tokens.length} onClick={() => setStep(step + 1)}>Next character</button>
      </div>
      <p>Emitted tokens: {step === 0 ? 'none' : example.tokens.slice(0, step).map((token) => (
        <code key={token.start}>{token.kind} [{token.start}, {token.end}) </code>
      ))}</p>
      <details>
        <summary>Complete trace in text</summary>
        <p>Start at byte 0 with no emitted tokens.</p>
        <ol>
          {example.tokens.map((token) => <li key={token.start}>
            Read <code>{token.text}</code> at byte {token.start}; emit <code>{token.kind}</code> with
            range [{token.start}, {token.end}); advance to byte {token.end}.
          </li>)}
        </ol>
        <p>No characters remain; return the collected tokens.</p>
      </details>
    </figure>
  );
}
