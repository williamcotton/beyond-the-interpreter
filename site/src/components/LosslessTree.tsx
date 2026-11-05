import { useId, useState } from 'react';
import './lossless-tree.css';

// Fixed observations, not a parser. Both AST values and every Rowan element,
// parent, depth, text and byte range were checked against the 12.2 native
// implementation after following its reader route.
const examples = [
  {
    "label": "Number spelling",
    "source": "001.50",
    "ast": "1.5",
    "elements": [
      {
        "kind": "Root",
        "start": 0,
        "end": 6,
        "depth": 0,
        "parent": null
      },
      {
        "kind": "NumberExpr",
        "start": 0,
        "end": 6,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "Number",
        "start": 0,
        "end": 6,
        "depth": 2,
        "parent": "NumberExpr"
      }
    ]
  },
  {
    "label": "Comments and Unicode",
    "source": " // é\n12.5 // end",
    "ast": "12.5",
    "elements": [
      {
        "kind": "Root",
        "start": 0,
        "end": 18,
        "depth": 0,
        "parent": null
      },
      {
        "kind": "Whitespace",
        "start": 0,
        "end": 1,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "LineComment",
        "start": 1,
        "end": 6,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "Whitespace",
        "start": 6,
        "end": 7,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "NumberExpr",
        "start": 7,
        "end": 11,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "Number",
        "start": 7,
        "end": 11,
        "depth": 2,
        "parent": "NumberExpr"
      },
      {
        "kind": "Whitespace",
        "start": 11,
        "end": 12,
        "depth": 1,
        "parent": "Root"
      },
      {
        "kind": "LineComment",
        "start": 12,
        "end": 18,
        "depth": 1,
        "parent": "Root"
      }
    ]
  }
];

export default function LosslessTree() {
  const id = useId();
  const [exampleIndex, setExampleIndex] = useState(1);
  const [selected, setSelected] = useState(4);
  const example = examples[exampleIndex];
  const element = example.elements[selected];
  const bytes = new TextEncoder().encode(example.source);
  const fragment = (start: number, end: number) => new TextDecoder().decode(bytes.slice(start, end));
  return (
    <figure className="lossless-tree" aria-labelledby={`${id}-caption`}>
      <figcaption id={`${id}-caption`}>A number value and the source that produced it</figcaption>
      <label htmlFor={`${id}-example`}>Choose a source: </label>
      <select id={`${id}-example`} value={exampleIndex} onChange={(event) => {
        setExampleIndex(Number(event.target.value));
        setSelected(0);
      }}>
        {examples.map((item, index) => <option value={index} key={item.label}>{item.label}</option>)}
      </select>
      <p>Owned AST: <code>Number({example.ast})</code>. Prefix output: <code>{example.ast}</code>.</p>
      <p>The stored value has no original spelling, comments or positions. The concrete tree keeps them:</p>
      <pre aria-label="Source with selected tree element highlighted"><code>
        {fragment(0, element.start)}<mark>{fragment(element.start, element.end)}</mark>{fragment(element.end, bytes.length)}
      </code></pre>
      <p>Exact source, with line breaks escaped: <code>{JSON.stringify(example.source)}</code></p>
      <p id={`${id}-help`}>Select a node or token to inspect its byte range. Indentation shows parent and child relationships.</p>
      <ol aria-describedby={`${id}-help`}>
        {example.elements.map((item, index) => <li key={index}>
          <button type="button" aria-pressed={selected === index} onClick={() => setSelected(index)}>
            <code>{'  '.repeat(item.depth)}{item.kind} [{item.start}..{item.end})</code>
            {selected === index ? ' — selected' : ''}
          </button>
        </li>)}
      </ol>
      <p aria-live="polite" aria-atomic="true">
        {element.kind}: bytes {element.start} through {element.end}, excluding {element.end}.
        {' '}Parent: {element.parent ?? 'none (the root)'}. Text: <code>{JSON.stringify(fragment(element.start, element.end))}</code>.
      </p>
      <details>
        <summary>Both examples as text</summary>
        {examples.map((item) => <div key={item.label}>
          <p>{item.label}: <code>{JSON.stringify(item.source)}</code>; AST <code>Number({item.ast})</code>.</p>
          <pre><code>{item.elements.map((node) => `${'  '.repeat(node.depth)}${node.kind} [${node.start}..${node.end})`).join('\n')}</code></pre>
        </div>)}
      </details>
    </figure>
  );
}
