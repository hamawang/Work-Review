# Core Workflow UI Upgrade Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将侧边栏、概览、日报、助手四个核心工作流页面统一升级为纸感编辑部风格，并对设置/关于做轻量统一优化，同时保持原有数据功能和页面职责差异。

**Architecture:** 保留现有路由、数据接口和业务逻辑，重构核心工作流四页的展示层与公共样式，同时对设置/关于增加轻量统一壳层。实现顺序按 `Sidebar → Overview → Report → Ask` 推进，并在侧边栏落地时同步处理品牌标签、编号移除，以及设置/关于的轻量统一，再以现有回归测试和构建验证兜底。

**Tech Stack:** Svelte 4、Tailwind 工具类、`src/app.css` 公共样式、Node `node:test`

---

## 文件结构

- Modify: `src/lib/components/Sidebar.svelte`
- Modify: `src/routes/Overview.svelte`
- Modify: `src/routes/report/Report.svelte`
- Modify: `src/routes/ask/Ask.svelte`
- Modify: `src/routes/settings/Settings.svelte`
- Modify: `src/routes/about/About.svelte`
- Modify: `src/app.css`
- Create: `src/lib/components/SidebarEditorial.test.js`
- Create: `src/routes/OverviewEditorial.test.js`
- Create: `src/routes/report/ReportEditorial.test.js`
- Create: `src/routes/ask/AskEditorial.test.js`
- Create: `src/routes/settings/SettingsEditorial.test.js`
- Create: `src/routes/about/AboutEditorial.test.js`

### Task 1: 为 A 包补充失败测试

**Files:**
- Create: `src/lib/components/SidebarEditorial.test.js`
- Create: `src/routes/OverviewEditorial.test.js`
- Create: `src/routes/report/ReportEditorial.test.js`
- Create: `src/routes/ask/AskEditorial.test.js`

- [ ] **Step 1: 写侧边栏失败测试**

```js
test('侧边栏应提供编辑部导航框架', async () => {
  const source = await readFile(new URL('./Sidebar.svelte', import.meta.url), 'utf8');
  assert.match(source, /sidebar-editorial-shell/);
  assert.match(source, /sidebar-nav-section/);
  assert.match(source, /sidebar-brand-panel/);
});
```

- [ ] **Step 2: 写概览页失败测试**

```js
test('概览页应渲染总编台式分区布局', async () => {
  const source = await readFile(new URL('./Overview.svelte', import.meta.url), 'utf8');
  assert.match(source, /overview-editorial-shell/);
  assert.match(source, /overview-command-deck/);
  assert.match(source, /overview-section-grid/);
});
```

- [ ] **Step 3: 写日报页失败测试**

```js
test('日报页应渲染纸感成稿容器', async () => {
  const source = await readFile(new URL('./Report.svelte', import.meta.url), 'utf8');
  assert.match(source, /report-editorial-shell/);
  assert.match(source, /report-sheet/);
  assert.match(source, /report-article-card/);
});
```

- [ ] **Step 4: 写助手页失败测试**

```js
test('助手页应渲染工作研究台结构', async () => {
  const source = await readFile(new URL('./Ask.svelte', import.meta.url), 'utf8');
  assert.match(source, /ask-workbench-shell/);
  assert.match(source, /ask-welcome-panel/);
  assert.match(source, /ask-composer-shell/);
});
```

- [ ] **Step 5: 运行测试并确认失败**

Run: `node --test src/lib/components/SidebarEditorial.test.js src/routes/OverviewEditorial.test.js src/routes/report/ReportEditorial.test.js src/routes/ask/AskEditorial.test.js`

Expected: FAIL，提示找不到新增结构类名。

### Task 2: 重构侧边栏为编辑部导航基座

**Files:**
- Modify: `src/lib/components/Sidebar.svelte`
- Modify: `src/app.css`
- Test: `src/lib/components/SidebarEditorial.test.js`

- [ ] **Step 1: 在侧边栏模板中补充编辑部导航容器**
- [ ] **Step 2: 调整品牌区、状态区、导航项与底部工具区层次**
- [ ] **Step 2.1: 将品牌副标题改为标签式信息并移除导航编号**
- [ ] **Step 3: 在 `app.css` 中补齐侧边栏的纸感导航样式**
- [ ] **Step 4: 运行 `node --test src/lib/components/SidebarEditorial.test.js src/I18nLayout.test.js` 确认通过**

### Task 5.5: 为设置与关于页补充轻量统一壳层

**Files:**
- Modify: `src/routes/settings/Settings.svelte`
- Modify: `src/routes/about/About.svelte`
- Modify: `src/app.css`
- Test: `src/routes/settings/SettingsEditorial.test.js`
- Test: `src/routes/about/AboutEditorial.test.js`

- [ ] **Step 1: 为设置页增加统一壳层与保存操作区**
- [ ] **Step 2: 为关于页增加统一壳层与封面卡样式**
- [ ] **Step 3: 运行 `node --test src/routes/settings/SettingsEditorial.test.js src/routes/about/AboutEditorial.test.js src/routes/about/About.test.js` 确认通过**

### Task 3: 重构概览页为今日工作总编台

**Files:**
- Modify: `src/routes/Overview.svelte`
- Modify: `src/app.css`
- Test: `src/routes/OverviewEditorial.test.js`

- [ ] **Step 1: 将顶部区域改为总编台式封面和命令区**
- [ ] **Step 2: 重新组织统计卡片、网站访问、应用使用、小时活跃度章节**
- [ ] **Step 3: 在 `app.css` 中补齐概览页分区、卡片和弹层样式**
- [ ] **Step 4: 运行 `node --test src/routes/OverviewEditorial.test.js src/routes/Overview.test.js` 确认通过**

### Task 4: 重构日报页为纸感成稿页

**Files:**
- Modify: `src/routes/report/Report.svelte`
- Modify: `src/app.css`
- Test: `src/routes/report/ReportEditorial.test.js`

- [ ] **Step 1: 保留 `report-hero` 结构前提下增加成稿页壳层**
- [ ] **Step 2: 将生成选项、状态提示和正文区切成更清晰的阅读结构**
- [ ] **Step 3: 强化 Markdown 容器、章节间距和页纸观感**
- [ ] **Step 4: 运行 `node --test src/routes/report/ReportEditorial.test.js src/routes/report/ReportLayout.test.js src/routes/report/ReportExport.test.js` 确认通过**

### Task 5: 重构助手页为工作研究台

**Files:**
- Modify: `src/routes/ask/Ask.svelte`
- Modify: `src/app.css`
- Test: `src/routes/ask/AskEditorial.test.js`

- [ ] **Step 1: 为空状态、消息区、证据区、输入区增加研究台结构容器**
- [ ] **Step 2: 保持消息逻辑不变，只升级视觉与信息层级**
- [ ] **Step 3: 在 `app.css` 中补齐工作研究台样式**
- [ ] **Step 4: 运行 `node --test src/routes/ask/AskEditorial.test.js src/I18nLocaleFlow.test.js` 确认通过**

### Task 6: 进行整组验证

**Files:**
- Modify: `src/app.css`
- Modify: `src/lib/components/Sidebar.svelte`
- Modify: `src/routes/Overview.svelte`
- Modify: `src/routes/report/Report.svelte`
- Modify: `src/routes/ask/Ask.svelte`
- Modify: `src/routes/settings/Settings.svelte`
- Modify: `src/routes/about/About.svelte`

- [ ] **Step 1: 运行核心工作流相关测试集合**

Run: `node --test src/lib/components/SidebarEditorial.test.js src/routes/OverviewEditorial.test.js src/routes/report/ReportEditorial.test.js src/routes/ask/AskEditorial.test.js src/routes/settings/SettingsEditorial.test.js src/routes/about/AboutEditorial.test.js src/routes/about/About.test.js src/routes/Overview.test.js src/routes/report/ReportLayout.test.js src/routes/report/ReportExport.test.js src/I18nLayout.test.js src/I18nLocaleFlow.test.js`

Expected: PASS。

- [ ] **Step 2: 运行完整构建**

Run: `npm run build`

Expected: exit 0，构建成功。

- [ ] **Step 3: 检查最终改动范围**

Run: `git diff -- src/lib/components/Sidebar.svelte src/routes/Overview.svelte src/routes/report/Report.svelte src/routes/ask/Ask.svelte src/routes/settings/Settings.svelte src/routes/about/About.svelte src/app.css src/lib/components/SidebarEditorial.test.js src/routes/OverviewEditorial.test.js src/routes/report/ReportEditorial.test.js src/routes/ask/AskEditorial.test.js src/routes/settings/SettingsEditorial.test.js src/routes/about/AboutEditorial.test.js docs/superpowers/specs/2026-04-05-core-workflow-ui-upgrade-design.md docs/superpowers/plans/2026-04-05-core-workflow-ui-upgrade.md`

Expected: 仅包含 A 包页面、样式、测试与文档改动。

- [ ] **Step 4: 用户已明确要求直接内联执行**

```text
执行方式已由用户指定为直接实现。
```
