import { useId, useState } from 'react';
import './character-cursor.css';

// Fixed 6.1 observations checked against native scanner and iterator behavior.
// This component selects authored frames and does not scan source.
const characters = [
  { text: 'a', start: 0, end: 1 },
  { text: '_', start: 1, end: 2 },
  { text: '2', start: 2, end: 3 },
  { text: '!', start: 3, end: 4 },
];
const tokens = [
  { kind: 'Identifier', start: 0, end: 3 },
  { kind: 'Bang', start: 3, end: 4 },
];
const frames: { cursor: number; end: number | null; inspected?: number; emitted: number; note: string }[] = [
  { cursor: 0, end: null, emitted: 0, note: 'Start at byte 0. No token has been started.' },
  { cursor: 1, end: 1, emitted: 0, note: 'Read a. It can start an identifier. The token starts at byte 0.' },
  { cursor: 2, end: 2, emitted: 0, note: 'Consume _: an underscore can continue the name. Extend its end to byte 2.' },
  { cursor: 3, end: 3, emitted: 0, note: 'Consume 2: a digit can continue the name. Extend its end to byte 3.' },
  { cursor: 3, end: 3, inspected: 3, emitted: 0, note: 'Inspect !. It cannot continue the name, so leave it unread. The cursor and token end stay at byte 3.' },
  { cursor: 3, end: 3, emitted: 1, note: 'Emit Identifier [0, 3). The next outer iteration can still read !.' },
  { cursor: 4, end: null, emitted: 2, note: 'Read ! and emit Bang [3, 4). No characters remain.' },
];

export default function IdentifierCursor() {
  const id = useId();
  const [step, setStep] = useState(0);
  const frame = frames[step];
  return (
    <figure className="character-cursor identifier-cursor" aria-labelledby={`${id}-caption`}>
      <figcaption id={`${id}-caption`}>One name, a growing range</figcaption>
      <p>Source: <code>a_2!</code></p>
      <ol className="character-cursor-cells" aria-label="Characters and their byte ranges">
        {characters.map((character) => {
          const consumed = character.end <= frame.cursor;
          const inspected = character.start === frame.inspected;
          return <li key={character.start} data-consumed={consumed} data-inspected={inspected}>
            <code>{character.text}</code>
            <span>bytes [{character.start}, {character.end})</span>
            <span>{consumed ? 'Consumed' : inspected ? 'Inspected · unread' : 'Unread'}</span>
          </li>;
        })}
      </ol>
      <p className="character-cursor-state" aria-live="polite" aria-atomic="true">
        Step {step} of {frames.length - 1}. Cursor: byte {frame.cursor}. {frame.note}
        {frame.end !== null && <> Name range: [0, {frame.end}).</>}
      </p>
      <div className="character-cursor-controls">
        <button type="button" disabled={step === 0} onClick={() => setStep(step - 1)}>Previous</button>
        <button type="button" disabled={step === frames.length - 1} onClick={() => setStep(step + 1)}>Next step</button>
      </div>
      <p>Emitted tokens: {frame.emitted === 0 ? 'none' : tokens.slice(0, frame.emitted).map((token) => (
        <code key={token.start}>{token.kind} [{token.start}, {token.end}) </code>
      ))}</p>
      <details>
        <summary>Complete trace in text</summary>
        <ol>{frames.map((item, index) => <li key={index}>
          Cursor at byte {item.cursor}. {item.note}
          {item.end !== null && <> Name range: [0, {item.end}).</>}
        </li>)}</ol>
      </details>
    </figure>
  );
}
