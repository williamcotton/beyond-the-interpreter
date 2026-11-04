import { useId, useState } from 'react';
import './token-ranges.css';

// Fixed observations checked against 8.1 native tokens output.
// This component selects source ranges and does not scan source.
const source = 'print "é";\n';
const bytes = new TextEncoder().encode(source);
const tokens = [
  { kind: 'Print', start: 0, end: 5, row: 'Print [0..5) "print"' },
  { kind: 'Whitespace', start: 5, end: 6, row: 'Whitespace [5..6) " "' },
  { kind: 'String', start: 6, end: 10, row: 'String [6..10) "\\"é\\""' },
  { kind: 'Semicolon', start: 10, end: 11, row: 'Semicolon [10..11) ";"' },
  { kind: 'Whitespace', start: 11, end: 12, row: 'Whitespace [11..12) "\\n"' },
];
const decode = (start: number, end: number) => new TextDecoder().decode(bytes.slice(start, end));

export default function TokenRanges() {
  const id = useId();
  const [selected, setSelected] = useState(2);
  const token = tokens[selected];
  return (
    <figure className="token-ranges" aria-labelledby={`${id}-caption`}>
      <figcaption id={`${id}-caption`}>Match a token row to its source bytes</figcaption>
      <p>Original source (with a final newline):</p>
      <pre aria-label="Original source with the selected token highlighted"><code>
        {decode(0, token.start)}<mark>{decode(token.start, token.end)}</mark>{decode(token.end, bytes.length)}
      </code></pre>
      <p id={`${id}-help`}>Select a row. Ranges count UTF-8 bytes; the last column escapes characters for display.</p>
      <ol className="token-ranges-rows" aria-describedby={`${id}-help`}>
        {tokens.map((item, index) => <li key={item.start}>
          <button type="button" aria-pressed={selected === index} onClick={() => setSelected(index)}>
            <span className="token-ranges-indicator">{selected === index ? 'Selected: ' : ''}</span>
            <code>{item.row}</code>
          </button>
        </li>)}
      </ol>
      <p aria-live="polite" aria-atomic="true">
        Selected {token.kind}: bytes {token.start} up to {token.end}, excluding {token.end}.
        {' '}{token.end - token.start} byte{token.end - token.start === 1 ? '' : 's'}.
      </p>
      <p>Selected bytes in hexadecimal: <code>{Array.from(bytes.slice(token.start, token.end), (byte) => byte.toString(16).padStart(2, '0')).join(' ')}</code></p>
      <details>
        <summary>Complete mapping in text</summary>
        <p>The source has 12 bytes. The string occupies bytes 6 through 9:
          an opening quote, two UTF-8 bytes for é, and a closing quote.
          Its row displays backslashes to escape those quotes. The final source byte
          is a newline, displayed as <code>\n</code> in the last row.</p>
        <pre><code>{tokens.map((item) => item.row).join('\n')}</code></pre>
      </details>
    </figure>
  );
}
