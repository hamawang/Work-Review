import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

test('概览页应渲染总编台式分区布局', async () => {
  const source = await readFile(new URL('./Overview.svelte', import.meta.url), 'utf8');

  assert.match(source, /overview-editorial-shell/);
  assert.match(source, /overview-lead-card/);
  assert.match(source, /overview-summary-grid/);
  assert.match(source, /overview-command-deck/);
  assert.match(source, /overview-section-grid/);
  assert.match(source, /overview-section-card/);
  assert.match(source, /overview-browser-gallery/);
  assert.doesNotMatch(source, /overview-single-card/);
  assert.doesNotMatch(source, /<StatsCard[^>]*embedded/);
  assert.match(source, /<AppUsageChart[\s\S]*embedded/);
  assert.match(source, /<ActivityHourlyChart[\s\S]*embedded/);
});
