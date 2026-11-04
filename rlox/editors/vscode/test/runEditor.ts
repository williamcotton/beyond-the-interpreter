import assert from 'node:assert/strict';
import { mkdir, mkdtemp, rm, writeFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { downloadAndUnzipVSCode, runTests, runVSCodeCommand } from '@vscode/test-electron';

async function main(): Promise<void> {
  const root = path.resolve(__dirname, '../..');
  const installed = process.argv.includes('--installed');
  const profile = await mkdtemp(path.join(os.tmpdir(), 'rlox-editor-'));
  const profileArgs = [
    '--user-data-dir', path.join(profile, 'user'),
    '--extensions-dir', path.join(profile, 'extensions'),
  ];

  try {
    const version = process.env.RLOX_VSCODE_VERSION ?? '1.128.0';
    const executable = await downloadAndUnzipVSCode(version);
    let developmentPath = root;

    if (installed) {
      await runVSCodeCommand([
        ...profileArgs,
        '--install-extension', path.join(root, 'rlox-0.1.0.vsix'),
        '--force',
      ], { version });
      const listed = await runVSCodeCommand([
        ...profileArgs, '--list-extensions', '--show-versions',
      ], { version });
      assert.ok(listed.stdout.split(/\r?\n/).includes('beyond-the-interpreter-local.rlox@0.1.0'));

      developmentPath = path.join(profile, 'test-driver');
      await mkdir(developmentPath);
      await writeFile(path.join(developmentPath, 'package.json'), JSON.stringify({
        name: 'rlox-test-driver',
        publisher: 'local-test',
        version: '0.0.0',
        engines: { vscode: '^1.128.0' },
      }));
    }

    await runTests({
      vscodeExecutablePath: executable,
      extensionDevelopmentPath: developmentPath,
      extensionTestsPath: path.join(__dirname, 'editor.js'),
      extensionTestsEnv: {
        RLOX_INSTALLED_EXTENSIONS: installed ? path.join(profile, 'extensions') : '',
      },
      launchArgs: [
        ...profileArgs,
        ...(installed ? [] : ['--disable-extensions']),
        '--skip-welcome',
        '--skip-release-notes',
        '--disable-gpu',
      ],
    });
    console.log(installed ? 'Installed VSIX test passed.' : 'Editor integration test passed.');
  } finally {
    await rm(profile, { recursive: true, force: true });
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

