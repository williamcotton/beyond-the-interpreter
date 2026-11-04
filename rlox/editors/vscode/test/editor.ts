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
}

