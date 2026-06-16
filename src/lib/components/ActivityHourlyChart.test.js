import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

test('按小时活跃度图表应将所选时段直接显示在柱状图内', async () => {
  const source = await readFile(new URL('./ActivityHourlyChart.svelte', import.meta.url), 'utf8');

  assert.match(source, /let selectedHour = null/);
  assert.match(source, /function selectHour\(hour\)/);
  assert.match(source, /aria-pressed=\{selectedHour === bucket\.hour\}/);
  assert.match(source, /on:click=\{\(\) => selectHour\(bucket\.hour\)\}/);
  // 点击柱状图高亮选中（ring），详情显示在下方信息条而非浮动弹窗
  assert.match(source, /ring-2 ring-sky-300/);
  assert.doesNotMatch(source, /tooltipAlignmentClass/);
  assert.doesNotMatch(source, /hourlyChart\.selectedHour/);
  assert.doesNotMatch(source, /hourlyChart\.selectedHourHint/);
});

test('按小时活跃度图表文案不应继续维护独立所选时段提示', async () => {
  const source = await readFile(new URL('../i18n/locales/zh-CN.js', import.meta.url), 'utf8');

  assert.equal((source.match(/selectedHour:/g) || []).length, 0);
  assert.equal((source.match(/selectedHourHint:/g) || []).length, 0);
});

test('按小时活跃度图表应在图表下方显示当前选中时段信息条', async () => {
  const source = await readFile(new URL('./ActivityHourlyChart.svelte', import.meta.url), 'utf8');

  assert.match(source, /selectedBucket = buckets\[selectedHour\] \|\| null/);
  assert.match(source, /\{#if selectedBucket\}/);
  assert.match(source, /当前选中/);
  assert.match(source, /\{formatHourRangeLabel\(selectedBucket\.hour\)\}/);
  assert.match(source, /\{formatCompact\(selectedBucket\.duration\)\}/);
});
