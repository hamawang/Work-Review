# Avatar Visual Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将桌宠升级为分层矢量职场搭子猫，并让状态差异从“表情切换”升级为“角色化表现”。

**Architecture:** 保留现有 `AvatarWindow -> AvatarCanvas -> avatarStateMeta/avatarOutline` 架构，在不调整 Tauri 事件链的前提下，重构 SVG 分层与状态 token。通过测试先约束轮廓、状态和组件结构，再最小化实现新形象与动效。

**Tech Stack:** Svelte 4, SVG, Tailwind utility classes, Node test runner, Vite

---

### Task 1: 固化桌宠视觉规格

**Files:**
- Modify: `docs/superpowers/specs/2026-04-08-avatar-visual-redesign-design.md`

- [ ] **Step 1: 自检规格是否覆盖核心范围**

检查项：

```text
1. 是否明确了视觉方向与角色设定
2. 是否明确了状态策略与实现边界
3. 是否明确了不做项，避免实现蔓延
```

- [ ] **Step 2: 确认规格可直接映射到代码边界**

对齐目标文件：

```text
src/lib/components/Avatar/avatarOutline.js
src/lib/components/Avatar/avatarAppearance.js
src/lib/components/Avatar/avatarStateMeta.js
src/lib/components/Avatar/AvatarCanvas.svelte
src/lib/components/Avatar/avatarOutline.test.js
src/lib/components/Avatar/avatarAppearance.test.js
src/lib/components/Avatar/avatarStateMeta.test.js
```

### Task 2: 先写回归测试约束新形象

**Files:**
- Modify: `src/lib/components/Avatar/avatarOutline.test.js`
- Modify: `src/lib/components/Avatar/avatarAppearance.test.js`
- Modify: `src/lib/components/Avatar/avatarStateMeta.test.js`

- [ ] **Step 1: 为新轮廓结构写失败测试**

要增加的断言方向：

```js
assert.ok(outline.facePatchPath);
assert.ok(outline.bellyPath);
assert.ok(outline.tailTipPath);
assert.ok(outline.shadowPath);
assert.ok(outline.leftEyeHighlightPath);
```

- [ ] **Step 2: 运行单测并确认红灯**

Run:

```bash
node --test src/lib/components/Avatar/avatarOutline.test.js
```

Expected:

```text
FAIL，提示新分层路径不存在或断言不匹配
```

- [ ] **Step 3: 为外观 token 与状态扩展写失败测试**

要增加的断言方向：

```js
assert.match(appearance.baseFur, /#|rgb|hsl/);
assert.match(appearance.line, /#|rgb|hsl/);
assert.ok(getAvatarModeMeta('meeting', '开会中').accessory === 'headset');
assert.ok(getAvatarModeMeta('generating', '生成中').auraVisible);
```

- [ ] **Step 4: 运行状态测试并确认红灯**

Run:

```bash
node --test src/lib/components/Avatar/avatarAppearance.test.js src/lib/components/Avatar/avatarStateMeta.test.js
```

Expected:

```text
FAIL，提示新 token 或 accessory 字段不存在
```

### Task 3: 实现新的轮廓与外观 token

**Files:**
- Modify: `src/lib/components/Avatar/avatarOutline.js`
- Modify: `src/lib/components/Avatar/avatarAppearance.js`

- [ ] **Step 1: 为桌宠新增分层轮廓路径**

核心新增结构：

```js
return {
  headPath,
  bodyPath,
  bellyPath,
  facePatchPath,
  muzzlePath,
  tailPath,
  tailTipPath,
  shadowPath,
  leftPawPath,
  rightPawPath,
  leftEarInnerPath,
  rightEarInnerPath,
  leftEyeHighlightPath,
  rightEyeHighlightPath,
};
```

- [ ] **Step 2: 为状态新增颜色 token**

核心方向：

```js
idle: {
  baseFur: '#f5f1eb',
  belly: '#fffaf4',
  line: '#425466',
  shadow: 'rgba(59, 79, 99, 0.18)',
  accent: '#f19cb4'
}
```

- [ ] **Step 3: 运行轮廓与外观测试确认转绿**

Run:

```bash
node --test src/lib/components/Avatar/avatarOutline.test.js src/lib/components/Avatar/avatarAppearance.test.js
```

Expected:

```text
PASS
```

### Task 4: 扩展状态元信息

**Files:**
- Modify: `src/lib/components/Avatar/avatarStateMeta.js`
- Modify: `src/lib/components/Avatar/avatarStateMeta.test.js`

- [ ] **Step 1: 给模式元信息补充角色化字段**

核心字段：

```js
{
  accessory: 'none',
  auraVisible: false,
  blushOpacity: 0.22,
  eyeScale: 1,
  gazeX: 0,
  gazeY: 0
}
```

- [ ] **Step 2: 给重点状态接入配件和氛围**

映射方向：

```js
meeting -> accessory: 'headset'
reading -> accessory: 'glasses'
generating -> auraVisible: true
music -> accessory: 'headphones'
```

- [ ] **Step 3: 运行状态测试确认转绿**

Run:

```bash
node --test src/lib/components/Avatar/avatarStateMeta.test.js
```

Expected:

```text
PASS
```

### Task 5: 改造 AvatarCanvas 结构与样式

**Files:**
- Modify: `src/lib/components/Avatar/AvatarCanvas.svelte`
- Modify: `src/lib/components/Avatar/avatarOutline.test.js`

- [ ] **Step 1: 为组件结构写失败测试**

要增加的源码断言方向：

```js
assert.match(source, /face-patch/);
assert.match(source, /belly-fill/);
assert.match(source, /tail-tip/);
assert.match(source, /avatar-shadow/);
assert.match(source, /accessory-headset|accessory-glasses|accessory-headphones/);
```

- [ ] **Step 2: 运行组件测试确认红灯**

Run:

```bash
node --test src/lib/components/Avatar/avatarOutline.test.js
```

Expected:

```text
FAIL，提示新类名尚未出现在 AvatarCanvas.svelte
```

- [ ] **Step 3: 实现新分层 SVG 结构**

实现方向：

```svelte
<path d={outline.shadowPath} class="avatar-shadow" />
<path d={outline.bodyPath} class="body-fill avatar-stroke" />
<path d={outline.bellyPath} class="belly-fill" />
<path d={outline.facePatchPath} class="face-patch" />
<path d={outline.tailTipPath} class="tail-tip" />
```

- [ ] **Step 4: 接入 accessory 与 aura 条件渲染**

实现方向：

```svelte
{#if resolvedMeta.accessory === 'headset'}
  <g class="accessory accessory-headset">...</g>
{/if}

{#if resolvedMeta.auraVisible}
  <g class="avatar-aura">...</g>
{/if}
```

- [ ] **Step 5: 运行桌宠相关测试确认通过**

Run:

```bash
node --test src/lib/components/Avatar/avatarOutline.test.js src/lib/components/Avatar/avatarAppearance.test.js src/lib/components/Avatar/avatarStateMeta.test.js
```

Expected:

```text
PASS
```

### Task 6: 完整验证

**Files:**
- Verify only

- [ ] **Step 1: 运行桌宠相关测试集合**

Run:

```bash
node --test src/lib/components/Avatar/avatarOutline.test.js src/lib/components/Avatar/avatarAppearance.test.js src/lib/components/Avatar/avatarStateMeta.test.js src/lib/components/Avatar/avatarWindow.test.js
```

Expected:

```text
PASS，0 failures
```

- [ ] **Step 2: 运行构建验证**

Run:

```bash
npm run build
```

Expected:

```text
构建成功，exit code 0
```
