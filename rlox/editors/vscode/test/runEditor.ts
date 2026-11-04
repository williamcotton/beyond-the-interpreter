import { mkdtemp, rm } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { runTests } from '@vscode/test-electron';

async function main(): Promise<void> {
  const root = path.resolve(__dirname, '../..');
  const profile = await mkdtemp(path.join(os.tmpdir(), 'rlox-editor-'));

  try {
    await runTests({
      version: '1.144.1',
      extensionDevelopmentPath: root,
      extensionTestsPath: path.join(__dirname, 'editor.js'),
      launchArgs: [
        '--user-data-dir', path.join(profile, 'user'),
        '--extensions-dir', path.join(profile, 'extensions'),
        '--disable-extensions',
        '--skip-welcome',
        '--skip-release-notes',
        '--disable-gpu',
      ],
    });
    console.log('Editor integration test passed.');
  } finally {
    await rm(profile, { recursive: true, force: true });
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

