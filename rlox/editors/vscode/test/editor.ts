import assert from 'node:assert/strict';
import path from 'node:path';
import * as vscode from 'vscode';

export async function run(): Promise<void> {
  const root = path.resolve(__dirname, '../..');
  const extension = vscode.extensions.getExtension('beyond-the-interpreter-local.rlox');
  assert.ok(extension, 'the development extension must be registered');
  assert.equal(extension.extensionPath, root);

  const fixture = vscode.Uri.file(path.join(root, 'fixtures', 'hello.lox'));
  const document = await vscode.workspace.openTextDocument(fixture);
  assert.equal(document.languageId, 'lox');

  const editor = await vscode.window.showTextDocument(document);
  assert.equal(editor.document.uri.toString(), document.uri.toString());
  console.log('EDITOR CHECK: hello.lox opened as Lox');

  const settings = vscode.workspace.getConfiguration('editor');
  await settings.update('autoClosingBrackets', 'always', vscode.ConfigurationTarget.Global);
  await settings.update('autoClosingQuotes', 'always', vscode.ConfigurationTarget.Global);
  await settings.update('autoIndent', 'full', vscode.ConfigurationTarget.Global);

  async function openAtEnd(content: string): Promise<vscode.TextEditor> {
    const document = await vscode.workspace.openTextDocument({ language: 'lox', content });
    const editor = await vscode.window.showTextDocument(document);
    editor.options = { insertSpaces: true, tabSize: 2, indentSize: 2 };
    const end = document.positionAt(content.length);
    editor.selection = new vscode.Selection(end, end);
    return editor;
  }

  const comment = await openAtEnd('print 1;');
  await vscode.commands.executeCommand('editor.action.commentLine');
  assert.equal(comment.document.getText(), '// print 1;');
  await vscode.commands.executeCommand('editor.action.commentLine');
  assert.equal(comment.document.getText(), 'print 1;');
  console.log('EDITOR CHECK: line comment toggles and restores source');

  const pair = await openAtEnd('');
  await vscode.commands.executeCommand('type', { text: '(' });
  assert.equal(pair.document.getText(), '()');
  await vscode.commands.executeCommand('type', { text: ')' });
  assert.equal(pair.document.getText(), '()');
  console.log('EDITOR CHECK: parenthesis closes and existing close is skipped');

  const quote = await openAtEnd('');
  await vscode.commands.executeCommand('type', { text: '"' });
  assert.equal(quote.document.getText(), '""');
  console.log('EDITOR CHECK: quotes auto-close in empty Lox source');

  const indent = await openAtEnd('if (true) {');
  await vscode.commands.executeCommand('type', { text: '\n' });
  assert.equal(indent.document.getText(), 'if (true) {\n  ');
  console.log('EDITOR CHECK: opening brace increases indentation by two spaces');

  const inComment = await openAtEnd('// ');
  await vscode.commands.executeCommand('type', { text: '(' });
  assert.equal(inComment.document.getText(), '// (');
  console.log('EDITOR CHECK: comment scope suppresses auto-closing');

  const inString = await openAtEnd('"text ');
  await vscode.commands.executeCommand('type', { text: '(' });
  assert.equal(inString.document.getText(), '"text (');
  console.log('EDITOR CHECK: string scope suppresses auto-closing');
}

