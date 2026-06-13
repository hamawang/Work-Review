import { test } from 'node:test';
import assert from 'node:assert';
import { assistantStore } from './lib/stores/assistant.js';

function snapshot() {
  let s;
  const unsub = assistantStore.subscribe((v) => {
    s = v;
  });
  unsub();
  return s;
}

test('normalizeMessage 为消息补齐 id/steps/streaming 默认值（兼容旧 localStorage）', () => {
  assistantStore.reset();
  assistantStore.appendMessage({ role: 'assistant', content: 'hi' });
  const m = snapshot().messages[0];
  assert.ok(m.id, '应自动生成 id');
  assert.deepEqual(m.steps, []);
  assert.equal(m.streaming, false);
  assert.deepEqual(m.references, []);
  assert.deepEqual(m.toolLabels, []);
});

test('updateLastStreaming 增量更新当前 streaming 消息（步骤 → 命中 → 收尾）', () => {
  assistantStore.reset();
  assistantStore.appendMessage({ role: 'user', content: '今天做了什么' });
  assistantStore.appendMessage({ role: 'assistant', content: '', streaming: true, steps: [] });

  // StepStart
  assistantStore.updateLastStreaming((m) => ({
    ...m,
    steps: [...m.steps, { tool: 'search_memory', label: '记忆检索', status: 'running' }],
  }));
  // StepResult
  assistantStore.updateLastStreaming((m) => ({
    ...m,
    steps: m.steps.map((s, i) =>
      i === m.steps.length - 1 ? { ...s, status: 'done', hits: 3 } : s
    ),
    references: [{ title: 'r1', timestamp: 1 }],
  }));
  // Done
  assistantStore.updateLastStreaming((m) => ({ ...m, content: '答案', streaming: false }));

  const msgs = snapshot().messages;
  assert.equal(msgs.length, 2);
  assert.equal(msgs[1].content, '答案');
  assert.equal(msgs[1].streaming, false);
  assert.equal(msgs[1].steps.length, 1);
  assert.equal(msgs[1].steps[0].status, 'done');
  assert.equal(msgs[1].steps[0].hits, 3);
  assert.equal(msgs[1].references.length, 1);
});

test('updateLastStreaming 在无 streaming 消息时不改动状态', () => {
  assistantStore.reset();
  assistantStore.appendMessage({ role: 'user', content: '问' });
  const beforeLen = snapshot().messages.length;
  assistantStore.updateLastStreaming((m) => ({ ...m, content: '不应出现' }));
  const after = snapshot();
  assert.equal(after.messages.length, beforeLen);
  assert.ok(!after.messages.some((m) => m.content === '不应出现'));
});
