import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { test } from 'node:test';
import { INITIAL, Registry, parseRawGrammar, type StateStack } from 'vscode-textmate';
import { loadWASM, OnigScanner, OnigString } from 'vscode-oniguruma';

const root = path.resolve(__dirname, '../..');
const grammarPromise = (async () => {
  const manifest = JSON.parse(readFileSync(path.join(root, 'package.json'), 'utf8'));
  const declaration = manifest.contributes.grammars[0];
  assert.equal(declaration.language, 'lox');
  assert.equal(declaration.scopeName, 'source.lox');
  const grammarPath = path.resolve(root, declaration.path);

  const wasm = readFileSync(require.resolve('vscode-oniguruma/release/onig.wasm'));
  await loadWASM(Uint8Array.from(wasm).buffer);
  const registry = new Registry({
    onigLib: Promise.resolve({
      createOnigScanner: (patterns: string[]) => new OnigScanner(patterns),
      createOnigString: (text: string) => new OnigString(text),
    }),
    loadGrammar: async (scope) => scope === declaration.scopeName
      ? parseRawGrammar(readFileSync(grammarPath, 'utf8'), grammarPath)
      : null,
  });
  const grammar = await registry.loadGrammar('source.lox');
  assert.ok(grammar, 'the contributed grammar must load');
  return grammar;
})();

async function scoped(line: string, state: StateStack = INITIAL) {
  const result = (await grammarPromise).tokenizeLine(line, state);
  return {
    state: result.ruleStack,
    tokens: result.tokens.map((token) => ({
      text: line.slice(token.startIndex, Math.min(token.endIndex, line.length)),
      scopes: token.scopes,
    })),
  };
}

test('keywords leave longer names whole', async () => {
  const words = 'and class else false for fun if nil or print return super this true var while'.split(' ');
  const result = await scoped(words.join(' '));
  assert.deepEqual(
    result.tokens.filter((token) => token.scopes.includes('keyword.control.lox')).map((token) => token.text),
    words,
  );
  for (const name of ['forest', 'printer', 'true_value', '_if', 'Print']) {
    const result = await scoped(name);
    assert.equal(result.tokens[0].text, name);
    assert.ok(result.tokens.every((token) => !token.scopes.includes('keyword.control.lox')));
    assert.ok(result.tokens[0].scopes.includes('variable.other.lox'));
  }
});

test('strings carry state across lines and shield comment-like text', async () => {
  const first = await scoped('print "é🦀 //');
  assert.ok(first.tokens.find((token) => token.text === 'é🦀 //')?.scopes.includes('string.quoted.double.lox'));
  assert.ok(first.tokens.every((token) => !token.scopes.includes('comment.line.double-slash.lox')));

  const second = await scoped('var while" print', first.state);
  assert.ok(second.tokens.find((token) => token.text === 'var while')?.scopes.includes('string.quoted.double.lox'));
  assert.ok(second.tokens.find((token) => token.text === 'print')?.scopes.includes('keyword.control.lox'));
});

test('backslashes do not escape quotes and unfinished strings stay open', async () => {
  const closed = await scoped('"a\\"print');
  assert.ok(closed.tokens.find((token) => token.text === 'print')?.scopes.includes('keyword.control.lox'));

  const first = await scoped('"unfinished');
  const second = await scoped('print // still string', first.state);
  assert.ok(second.tokens.every((token) => token.scopes.includes('string.quoted.double.lox')));
});

test('numbers leave signs, dots and following names separate', async () => {
  const result = await scoped('-12.5 123. .5 1e3 1.2.3 123print');
  assert.deepEqual(
    result.tokens.filter((token) => token.scopes.includes('constant.numeric.lox')).map((token) => token.text),
    ['12.5', '123', '5', '1', '1.2', '3', '123'],
  );
  assert.ok(result.tokens.find((token) => token.text === '-')?.scopes.includes('keyword.operator.lox'));
  assert.ok(result.tokens.find((token) => token.text === 'e3')?.scopes.includes('variable.other.lox'));
  assert.ok(result.tokens.find((token) => token.text === 'print')?.scopes.includes('keyword.control.lox'));
});

test('line comments stop at the line boundary', async () => {
  const comment = await scoped('// print "text"');
  assert.ok(comment.tokens.every((token) => token.scopes.includes('comment.line.double-slash.lox')));
  const next = await scoped('print != == <= >= / * + - ! = < > (){}.,;', comment.state);
  assert.ok(next.tokens.find((token) => token.text === 'print')?.scopes.includes('keyword.control.lox'));
  assert.deepEqual(
    next.tokens.filter((token) => token.scopes.includes('keyword.operator.lox')).map((token) => token.text),
    ['!=', '==', '<=', '>=', '/', '*', '+', '-', '!', '=', '<', '>'],
  );
  assert.ok(next.tokens.some((token) => token.scopes.includes('punctuation.lox')));
});
