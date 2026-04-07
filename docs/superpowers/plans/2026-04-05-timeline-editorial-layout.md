# Timeline Editorial Layout Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将时间线页面和时段摘要页统一升级为 A1 纸感编辑部风格，同时保持信息简洁、重点露图、按小时总览清晰、详情与分类能力不变。

**Architecture:** 继续复用现有时间线数据链路，在 `Timeline.svelte` 内新增重点卡片判定和编辑部轨道布局，并在 `Summary.svelte` 中将按小时摘要升级为阶段带状版。测试仍采用源码级断言，先写失败测试，再实现布局和样式，最后用构建与针对性测试回归验证。

**Tech Stack:** Svelte 4、Tailwind 工具类、组件局部 CSS、Node `node:test`

---

## 文件结构

- Modify: `src/routes/timeline/Timeline.svelte`
- Modify: `src/routes/timeline/Summary.svelte`
- Create: `src/routes/timeline/TimelineLayout.test.js`
- Create: `src/routes/timeline/SummaryLayout.test.js`
- Modify: `src/lib/i18n/index.js`
- Reference: `src/routes/timeline/TimelineScreenshotMode.test.js`
- Reference: `src/routes/timeline/Timeline.category.test.js`

### Task 1: 为编辑部轨道布局补充失败测试

**Files:**
- Create: `src/routes/timeline/TimelineLayout.test.js`
- Modify: 无
- Test: `src/routes/timeline/TimelineLayout.test.js`

- [ ] **Step 1: 写失败测试，约束新布局与重点卡片逻辑**

```js
import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

test('时间线应渲染编辑部轨道布局与重点卡片容器', async () => {
  const source = await readFile(new URL('./Timeline.svelte', import.meta.url), 'utf8');

  assert.match(source, /timeline-editorial-shell/);
  assert.match(source, /timeline-rail/);
  assert.match(source, /timeline-entry-card-featured/);
  assert.match(source, /timeline-entry-card-compact/);
});

test('时间线应通过显式函数判断重点卡片', async () => {
  const source = await readFile(new URL('./Timeline.svelte', import.meta.url), 'utf8');

  assert.match(source, /function selectFeaturedActivityIds/);
  assert.match(source, /featuredActivityIds = new Set/);
  assert.match(source, /getTimelineThumbnail/);
});
```

- [ ] **Step 2: 运行测试并确认失败**

Run: `node --test src/routes/timeline/TimelineLayout.test.js`

Expected: FAIL，提示找不到新类名或新函数。

- [ ] **Step 3: 记录失败原因，保持测试不变，进入实现**

预期失败原因：

```text
The input did not match /timeline-editorial-shell/
```

- [ ] **Step 4: 暂不提交，继续进入布局实现**

```bash
git status --short
```

Expected: 仅出现新测试文件和后续待改源码。

### Task 2: 在时间线组件中实现编辑部轨道结构

**Files:**
- Modify: `src/routes/timeline/Timeline.svelte`
- Test: `src/routes/timeline/TimelineLayout.test.js`

- [ ] **Step 1: 增加重点卡片判定与缩略图读取辅助函数**

```js
const FEATURED_DURATION_THRESHOLD = 20 * 60;
const FEATURED_CONTEXT_THRESHOLD = 10 * 60;

function getTimelineThumbnail(activity) {
  if (!activity?.screenshot_path) {
    return null;
  }
  return thumbnailCache[activity.screenshot_path] || null;
}

function selectFeaturedActivityIds(items) {
  const featuredIds = [];
  let lastFeaturedIndex = -99;

  for (let index = 0; index < items.length; index += 1) {
    const activity = items[index];
    const previous = items[index - 1];
    if (!activity?.id || !activity.screenshot_path) continue;

    const durationScore = activity.duration >= FEATURED_DURATION_THRESHOLD ? 2 : 0;
    const contextScore =
      activity.duration >= FEATURED_CONTEXT_THRESHOLD &&
      previous &&
      (previous.app_name !== activity.app_name || previous.category !== activity.category)
        ? 1
        : 0;

    if (durationScore + contextScore === 0) continue;
    if (index - lastFeaturedIndex < 2) continue;

    featuredIds.push(activity.id);
    lastFeaturedIndex = index;

    if (featuredIds.length >= 4) break;
  }

  return featuredIds;
}
```

- [ ] **Step 2: 将列表结构改为左时间轴 + 右轨迹卡片**

```svelte
<div class="timeline-editorial-shell">
  <div class="timeline-rail" aria-hidden="true"></div>
  <div class="timeline-entry-list">
    {#each activities as activity, i}
      {@const featured = featuredActivityIds.has(activity.id)}
      <button class="timeline-entry timeline-entry-{featured ? 'featured' : 'compact'}">
        ...
      </button>
    {/each}
  </div>
</div>
```

- [ ] **Step 3: 为重点卡片接入缩略图、紧凑卡片保留精简信息**

```svelte
{#if featured}
  <div class="timeline-entry-card timeline-entry-card-featured">
    {#if getTimelineThumbnail(activity)}
      <img src={getTimelineThumbnail(activity)} alt={t('timeline.detail.screenshotAlt')} />
    {/if}
    ...
  </div>
{:else}
  <div class="timeline-entry-card timeline-entry-card-compact">
    ...
  </div>
{/if}
```

- [ ] **Step 4: 用局部样式实现纸感编辑部视觉**

```svelte
<style>
  .timeline-editorial-shell { ... }
  .timeline-rail { ... }
  .timeline-entry-card-featured { ... }
  .timeline-entry-card-compact { ... }
</style>
```

- [ ] **Step 5: 运行测试，确认由红转绿**

Run: `node --test src/routes/timeline/TimelineLayout.test.js src/routes/timeline/TimelineScreenshotMode.test.js src/routes/timeline/Timeline.category.test.js`

Expected: PASS，时间线新布局测试和既有回归测试均通过。

### Task 3: 为时段摘要页补充失败测试并完成实现

**Files:**
- Modify: `src/routes/timeline/Summary.svelte`
- Modify: `src/lib/i18n/index.js`
- Test: `src/routes/timeline/SummaryLayout.test.js`

- [ ] **Step 1: 写失败测试，约束阶段带状结构与主摘要模式**

```js
test('时段摘要页应渲染编辑部阶段带状布局', async () => {
  const source = await readFile(new URL('./Summary.svelte', import.meta.url), 'utf8');

  assert.match(source, /summary-editorial-shell/);
  assert.match(source, /summary-band/);
  assert.match(source, /summary-band-card/);
  assert.match(source, /summary-app-tags/);
});

test('时段摘要页应提取一句主摘要并替换旧的 bullet 列表模式', async () => {
  const source = await readFile(new URL('./Summary.svelte', import.meta.url), 'utf8');

  assert.match(source, /function getPrimarySummary/);
  assert.match(source, /function getMainApps/);
  assert.match(source, /summary-primary-copy/);
  assert.doesNotMatch(source, /function formatSummary/);
});
```

- [ ] **Step 2: 运行测试并确认失败**

Run: `node --test src/routes/timeline/SummaryLayout.test.js`

Expected: FAIL，提示找不到 `summary-editorial-shell` 或 `getPrimarySummary`。

- [ ] **Step 3: 在 `Summary.svelte` 中实现阶段带状布局和一句主摘要模式**

```svelte
<div class="summary-editorial-shell">
  {#each summaries as summary}
    <section class="summary-band">
      <div class="summary-band-anchor">...</div>
      <div class="summary-band-card">
        <p class="summary-primary-copy">{getPrimarySummary(summary.summary)}</p>
        <div class="summary-app-tags">...</div>
      </div>
    </section>
  {/each}
</div>
```

- [ ] **Step 4: 补充必要的多语言文案并回跑测试**

Run: `node --test src/routes/timeline/SummaryLayout.test.js src/I18nLocaleFlow.test.js`

Expected: PASS，结构测试和 locale 相关测试均通过。

### Task 4: 进行页面级回归验证

**Files:**
- Modify: `src/routes/timeline/Timeline.svelte`
- Modify: `src/routes/timeline/Summary.svelte`
- Test: `src/routes/timeline/TimelineLayout.test.js`
- Test: `src/routes/timeline/SummaryLayout.test.js`

- [ ] **Step 1: 运行时间线与摘要页相关测试集合**

Run: `node --test src/routes/timeline/*.test.js src/I18nLocaleFlow.test.js`

Expected: PASS，无失败用例。

- [ ] **Step 2: 运行完整构建验证页面可编译**

Run: `npm run build`

Expected: exit 0，Vite 构建成功。

- [ ] **Step 3: 检查最终改动范围**

Run: `git diff -- src/routes/timeline/Timeline.svelte src/routes/timeline/Summary.svelte src/routes/timeline/TimelineLayout.test.js src/routes/timeline/SummaryLayout.test.js src/lib/i18n/index.js docs/superpowers/specs/2026-04-05-timeline-editorial-layout-design.md docs/superpowers/plans/2026-04-05-timeline-editorial-layout.md`

Expected: 仅包含时间线布局、测试与文档改动。

- [ ] **Step 4: 用户明确要求直接内联执行，本计划不再等待额外执行方式选择**

```text
执行方式已由用户指定为直接实现。
```
