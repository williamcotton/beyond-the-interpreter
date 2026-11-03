import { useId, useState } from 'react';
import './character-cursor.css';

type Frame = { cursor: number; inspected?: number; emitted: number; note: string };
type Observation = {
  source: string;
  characters: { text: string; start: number; end: number }[];
  tokens: { kind: string; start: number; end: number }[];
  frames: Frame[];
};

// Authored 5.2 observations checked against native scanner and iterator behavior.
// The UI selects frames; it contains no scanning or lookahead algorithm.
const observations: Observation[] = [
  {
    source: '!=',
    characters: [{ text: '!', start: 0, end: 1 }, { text: '=', start: 1, end: 2 }],
    tokens: [{ kind: 'BangEqual', start: 0, end: 2 }],
    frames: [
      { cursor: 0, emitted: 0, note: 'Start at byte 0. No token has been started.' },
      { cursor: 1, emitted: 0, note: 'Read !. The token starts at byte 0; its end is provisionally byte 1.' },
      { cursor: 1, inspected: 1, emitted: 0, note: 'Inspect = at byte 1. It matches, but inspection alone has not consumed it.' },
      { cursor: 2, emitted: 0, note: 'Consume the matching =. Extend the token end to byte 2.' },
      { cursor: 2, emitted: 1, note: 'Emit BangEqual [0, 2). No characters remain; the next loop condition ends the scan.' },
    ],
  },
  {
    source: '!(',
    characters: [{ text: '!', start: 0, end: 1 }, { text: '(', start: 1, end: 2 }],
    tokens: [{ kind: 'Bang', start: 0, end: 1 }, { kind: 'LeftParen', start: 1, end: 2 }],
    frames: [
      { cursor: 0, emitted: 0, note: 'Start at byte 0. No token has been started.' },
      { cursor: 1, emitted: 0, note: 'Read !. The token starts at byte 0; its end is provisionally byte 1.' },
      { cursor: 1, inspected: 1, emitted: 0, note: 'Inspect ( at byte 1. It does not match =. Leave it unread; the cursor stays at byte 1.' },
      { cursor: 1, emitted: 1, note: 'Emit Bang [0, 1). The ( is still available to the next loop iteration.' },
      { cursor: 2, emitted: 1, note: 'The next iteration reads (. This token starts at byte 1 and ends at byte 2.' },
      { cursor: 2, emitted: 2, note: 'Emit LeftParen [1, 2). No characters remain.' },
    ],
  },
  {
    source: '=',
    characters: [{ text: '=', start: 0, end: 1 }],
    tokens: [{ kind: 'Equal', start: 0, end: 1 }],
    frames: [
      { cursor: 0, emitted: 0, note: 'Start at byte 0. No token has been started.' },
      { cursor: 1, emitted: 0, note: 'Read =. The token starts at byte 0; its end is provisionally byte 1.' },
      { cursor: 1, emitted: 0, note: 'There is no following character. Conditional consumption returns None; the token end stays at byte 1.' },
      { cursor: 1, emitted: 1, note: 'Emit Equal [0, 1). The next loop condition also finds no character and ends the scan.' },
    ],
  },
];

export default function LookaheadCursor() {
  const id = useId();
  const [exampleIndex, setExampleIndex] = useState(0);
  const [step, setStep] = useState(0);
  const example = observations[exampleIndex];
  const frame = example.frames[step];
  return (
    <figure className="character-cursor lookahead-cursor" aria-labelledby={`${id}-caption`}>
      <figcaption id={`${id}-caption`}>Inspect before consuming</figcaption>
      <label htmlFor={`${id}-example`}>Source example </label>
      <select id={`${id}-example`} value={exampleIndex} onChange={(event) => {
        setExampleIndex(Number(event.target.value));
        setStep(0);
      }}>
        {observations.map((item, index) => <option key={item.source} value={index}>{item.source}</option>)}
      </select>
      <ol className="character-cursor-cells" aria-label="Characters and their byte ranges">
        {example.characters.map((character) => {
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
        Step {step} of {example.frames.length - 1}. Cursor: byte {frame.cursor}. {frame.note}
      </p>
      <div className="character-cursor-controls">
        <button type="button" disabled={step === 0} onClick={() => setStep(step - 1)}>Previous</button>
        <button type="button" disabled={step === example.frames.length - 1} onClick={() => setStep(step + 1)}>Next step</button>
      </div>
      <p>Emitted tokens: {frame.emitted === 0 ? 'none' : example.tokens.slice(0, frame.emitted).map((token) => (
        <code key={token.start}>{token.kind} [{token.start}, {token.end}) </code>
      ))}</p>
      <details>
        <summary>Complete trace in text</summary>
        <ol>{example.frames.map((item, index) => <li key={index}>
          Cursor at byte {item.cursor}. {item.note}
        </li>)}</ol>
      </details>
    </figure>
  );
}
