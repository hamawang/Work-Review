# About Minimal Product Card Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 About 页重构为极简产品名片页，同时保留更新、数据目录、赞助与 Linux 兼容信息能力。

**Architecture:** 保留 About 页现有交互与数据逻辑，只重构页面的展示层。通过新增结构测试约束品牌主卡、操作条与说明卡，再用现有 About 测试和构建验证兜底。

**Tech Stack:** Svelte 4、`src/app.css` 公共样式、Node `node:test`

---

## 文件结构

- Modify: `src/routes/about/About.svelte`
- Modify: `src/app.css`
- Modify: `src/routes/about/AboutEditorial.test.js`

### Task 1: 为极简产品名片页补失败测试

**Files:**
- Modify: `src/routes/about/AboutEditorial.test.js`

- [ ] **Step 1: 约束新结构**

```js
assert.match(source, /about-minimal-shell/);
assert.match(source, /about-brand-card/);
assert.match(source, /about-action-strip/);
assert.match(source, /about-trust-grid/);
assert.match(source, /about-system-note/);
```

- [ ] **Step 2: 运行测试并确认失败**

Run: `node --test src/routes/about/AboutEditorial.test.js`

Expected: FAIL。

### Task 2: 实现 About 页极简产品名片结构

**Files:**
- Modify: `src/routes/about/About.svelte`
- Modify: `src/app.css`

- [ ] **Step 1: 在模板中重组品牌主卡、操作条与说明卡**
- [ ] **Step 2: 保留赞助弹层与 Linux 会话逻辑，但降级为次级区块**
- [ ] **Step 3: 在 `app.css` 中补齐极简产品名片样式**
- [ ] **Step 4: 运行 `node --test src/routes/about/AboutEditorial.test.js src/routes/about/About.test.js` 确认通过**

### Task 3: 进行最终验证

**Files:**
- Modify: `src/routes/about/About.svelte`
- Modify: `src/app.css`

- [ ] **Step 1: 运行 About 相关测试集合**

Run: `node --test src/routes/about/AboutEditorial.test.js src/routes/about/About.test.js src/I18nLayout.test.js`

Expected: PASS。

- [ ] **Step 2: 运行完整构建**

Run: `npm run build`

Expected: exit 0。
