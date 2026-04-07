import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

test('助手页应渲染工作研究台结构', async () => {
  const source = await readFile(new URL('./Ask.svelte', import.meta.url), 'utf8');

  assert.match(source, /ask-workbench-shell/);
  assert.match(source, /ask-welcome-panel/);
  assert.match(source, /ask-composer-shell/);
  assert.match(source, /ask-reference-card/);
});
