import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

test('日报页应渲染纸感成稿容器', async () => {
  const source = await readFile(new URL('./Report.svelte', import.meta.url), 'utf8');

  assert.match(source, /report-editorial-shell/);
  assert.match(source, /report-sheet/);
  assert.match(source, /report-article-card/);
});
