# plan.md — Work Review GitHub Issues 批量优化方案

> 阶段：P2 制定方案 ｜ automation_mode：false（交互模式）
> 测试覆盖率目标：**85%**（语句/分支/函数）
> 涉及 Issue：#98（Bug）、#96、#95、#94
> 注：本 plan 由主模型直接编写——全局规则要求的 plan-down skill 依赖的 zen MCP 服务在本会话未加载，故据实说明。

---

## 第 0 步：已审查并确认的代码事实（不基于猜测）

下列结论均已通过实际读码核实，作为后续方案的依据：

### #98 自动导出偶尔失效
- 没有独立的"10 点定时器"。前端 `src/App.svelte:379-414` 每 60s 轮询，到点调 `generate_report`；导出是 `generate_report_inner` 的副作用。
- `src-tauri/src/commands.rs:4039` 先 `save_report` 存库 → `4042-4046` 再导出，**共用同一个 `?`**（4044）。
- `src/App.svelte:406-408`：轮询发现"已有报告"就设 `lastAutoGenDate=today`，**当天不再触发**。
- **根因坐实**：某天导出失败（如桌面被 OneDrive 重定向/同步占用/杀软锁文件）→ 报告已进库但文件没写 → 下次轮询见"已有报告"→ 当天不再补导出 → 用户看到"偶尔某天桌面没文件"。
- `commands.rs:4010-4016` 的桌宠"成功/失败"气泡在 `4019 report_result?` 后、导出（4044）前，所以**导出失败时桌宠已报"成功"**，导出失败完全静默（仅 `App.svelte:410` console.warn）。
- 无跨午夜/休眠的补偿逻辑。

### #96 浏览器识别（tabbit 无法识别）
- 硬编码白名单 `is_browser_app` **有两份**：`src-tauri/src/monitor.rs:168-202`（前台监控）、`crates/core/src/categorize.rs:51`（入库分类，`database.rs:1262` 调用）。tabbit 不含任何关键词 → 两处都判定"非浏览器"。
- URL 提取：Windows 走通用 UIA（`monitor.rs:1476/1598`，过门禁即可抓 URL）；macOS 走 `browser_url_script_macos` 的 if-else 链（`monitor.rs:2916-3061`，`else => None`，必须有显式分支）；Linux 仅标题解析。

### #95 桌宠鼠标穿透
- 桌宠窗口 `src-tauri/src/avatar_engine.rs:497-529`，`transparent(true)` 透明大窗口；全代码库**从未调用 `set_ignore_cursor_events`**。
- 桌宠是可交互的：拖动 `AvatarWindow.svelte:588-602`、双击开主窗口 `580-586`、跟进卡片按钮；SVG 层 `pointer-events:none`，外层 shell div 承接点击。
- **结论**：简单全局穿透会让桌宠彻底点不动；需"图像区可点、透明区穿透"的智能穿透。

### #94 日报提示词自定义
- 用户以为的"系统预设词"（【核心要求】【输出格式】+ 四标题）其实在 **user message** 里：`crates/core/src/analysis/summary.rs:524-575`（`base_prompt`，四标题在 550-556，格式是 `**加粗**`）。真正的 system message 只是一句话 `summary.rs:66-78`。
- 附加提示词拼接：`crates/core/src/analysis/mod.rs:85` 把 `daily_report_custom_prompt` 挂成"## 额外要求"尾部。
- **下游解析已确认**：`src/routes/report/Report.svelte:397-421` 的 `parseSections` **只认 `## ` 开头的标题，与四个标题名无关**。默认模板用 `**加粗**` 而非 `## `，所以默认报告其实不被分段（已存在瑕疵）。
- 三种 analyzer（summary/cloud/local）各有模板，仅 summary 含四标题块。`daily_report_custom_prompt` 三者共用。不影响工作助手 Ask / 桌宠 followup / 机器人（另一套 `build_assistant_system_prompt` commands.rs:809）。
- config 加字段范式：`crates/core/src/config.rs` 现有 `daily_report_custom_prompt:674`、`PromptPreset:284-291`、avatar 字段 `813-836`；新增字段用 `#[serde(default)]` 即向后兼容（`load`/`save` 无需改，default 块在 `888-917` 区）。

---

## 目标与非目标

**目标**
- #98：让自动导出稳定，失败可感知、可重试、可补偿。
- #96：浏览器识别可由用户扩展（通用），并内置 tabbit。
- #95：桌宠支持智能鼠标穿透（透明区穿透、本体可交互），可开关。
- #94：日报模板可编辑 + 一键还原默认 + 数据占位符化 + 标题解析对齐。

**非目标（本次不做）**
- #96 不为每个浏览器逐像素逆向；macOS 自定义浏览器默认尝试 Chromium AppleScript 模板，不保证全部生效。
- #95 首版命中测试用桌宠 bounding box（矩形/近似）而非逐像素 alpha；逐像素 alpha 留作后续增强。
- #94 模板 override 首版只作用于 **summary 模式**；cloud/local 模式沿用各自模板。

---

## 风险与假设
- **R1（#95 关键）**：`set_ignore_cursor_events(true)` 开启后窗口收不到鼠标事件 → 前端无法靠自身 mousemove 检测光标移回本体。**对策**：命中测试由后端基于全局鼠标坐标 + 窗口位置/尺寸驱动（复用/扩展 `avatar_input.rs` 的鼠标采集），动态切换穿透。
- **R2（#94）**：让用户编辑整段模板后，原 `format!` 注入的数据（日期/应用/网站/关键词/时间线）需改为**占位符渲染**，否则数据无处注入。
- **R3（#96）**：两份 `is_browser_app` 必须同时接入自定义列表，否则"前台识别"与"入库分类"不一致。
- **R4（#98）**：判断"当天是否已成功导出"采用**导出目录内文件存在性**作为幂等判据，最简单且跨重启可靠。
- 假设：tabbit 为 Chromium 内核（小众浏览器多为此）；Windows 端过门禁后通用 UIA 可抓 URL。

---

## 各 Issue 详细方案

### #98 自动导出稳定性修复（Bug，优先级 P0）

**思路**：导出与"报告是否已生成"解耦；导出失败不再被吞、可重试、启动补偿、用户可感知。

- 后端 `src-tauri/src/commands.rs`
  - 将 `generate_report_inner`（4042-4046）的导出错误与生成错误解耦：导出失败时 `log::warn` + 发一个失败事件（emit），**不**让 `generate_report` 整体返回 Err（报告已生成成功）。
  - 新增命令 `export_saved_report(date, locale)`：从库读已存报告并导出，返回 Ok/Err，供前端补导出与重试调用。
  - 新增命令 `is_report_exported(date)`：检查导出目录内 `{date}.md` 是否存在（幂等判据）。
- 前端 `src/App.svelte`
  - 自动轮询逻辑：当 `existingReport` 存在但 `config.daily_report_auto_export` 开启且 `is_report_exported(today)` 为 false 时，调用 `export_saved_report` 补导出，成功后再设 `lastAutoGenDate`。
  - 启动补偿：`onMount` 时检查今天/昨天"有报告但未导出"，补导出。
  - 导出失败 → 应用内 toast / 系统通知（含目标路径，便于排查 OneDrive 重定向等）。
- i18n：导出失败/补导出提示文案（zh-CN/zh-TW/en）。

**改动文件**：`commands.rs`、`main.rs`(注册命令)、`App.svelte`、`src/lib/i18n/locales/*.js`

**子任务**
- [ ] 后端：导出错误与生成解耦（emit 失败事件，不污染生成返回）
- [ ] 后端：新增 `export_saved_report` + `is_report_exported` 命令并注册
- [ ] 前端：补导出逻辑（已有报告但未导出时触发）
- [ ] 前端：启动补偿 + 失败 toast/通知
- [ ] i18n 文案

### #96 浏览器通用可扩展支持 + 内置 tabbit

**思路**：用户可在设置里维护"自定义浏览器进程名/关键词"列表；两份 `is_browser_app` 合并内置 + 自定义；内置 tabbit；macOS 自定义/tabbit 尝试 Chromium AppleScript 模板。

- 配置 `crates/core/src/config.rs`：新增 `#[serde(default)] pub custom_browser_apps: Vec<String>`（default `Vec::new()`，normalize 去空白/小写/去重）。
- 识别 `src-tauri/src/monitor.rs:168` 与 `crates/core/src/categorize.rs:51`：
  - 内置列表加 `contains("tabbit")`。
  - 引入自定义列表来源：用 `RwLock<Vec<String>>`/`OnceCell` 全局缓存，config 加载/保存时刷新；两处 `is_browser_app` 合并匹配内置 + 自定义。
- macOS URL：`browser_url_script_macos`（monitor.rs:2916）加 tabbit 分支；对"自定义浏览器"默认套用 Chromium AppleScript 模板（`URL of active tab of front window`）。
- 前端：设置内新增"自定义浏览器"列表编辑 UI（建议放 `SettingsPrivacy`/分类相关组件），调用保存命令刷新后端缓存。
- i18n 文案。

**改动文件**：`config.rs`、`monitor.rs`、`categorize.rs`、`commands.rs`(读写配置/刷新缓存)、对应设置 `.svelte`、`i18n/*.js`

**子任务**
- [ ] config 新增 `custom_browser_apps` + default + normalize
- [ ] 两份 `is_browser_app` 接入自定义列表 + 内置 tabbit
- [ ] 全局缓存随 config 刷新
- [ ] macOS：tabbit 分支 + 自定义走 Chromium 模板
- [ ] 前端设置 UI + i18n

### #95 桌宠智能鼠标穿透

**思路**：开关 + 后端驱动命中测试（透明区穿透、本体/交互区不穿透）。

- 配置 `crates/core/src/config.rs`：新增 `#[serde(default)] pub avatar_mouse_passthrough: bool`（default false）。
- 后端 `src-tauri/src/avatar_engine.rs`：
  - 新增穿透控制（`window.set_ignore_cursor_events(bool)`）。
  - 命中测试：基于全局鼠标坐标（复用/扩展 `avatar_input.rs` 采集）+ 桌宠窗口 position/size，判断光标是否落在桌宠本体 bounding box（首版矩形/近似）内；在区内→关闭穿透，区外→开启穿透；节流。
  - 仅当 `avatar_mouse_passthrough` 开启时启用该动态逻辑；关闭时恢复正常（不穿透）。
- 命令 `src-tauri/src/commands.rs` + `main.rs`：新增 `set_avatar_mouse_passthrough`；`persist_app_config` 的 `avatar_window_changed` 纳入新字段。
- 前端 `src/routes/settings/components/SettingsAppearance.svelte`：仿 `toggleBreakReminder` 加开关；i18n。

**改动文件**：`config.rs`、`avatar_engine.rs`、`avatar_input.rs`、`commands.rs`、`main.rs`、`SettingsAppearance.svelte`、`i18n/*.js`

**子任务**
- [ ] config 新增 `avatar_mouse_passthrough` + default + normalize
- [ ] 后端：全局鼠标坐标 → 命中测试 → 动态 `set_ignore_cursor_events`（节流）
- [ ] 命令 `set_avatar_mouse_passthrough` + 注册 + 配置同步
- [ ] 前端开关 + i18n
- [ ] 验证：透明区可点穿、本体可拖动/双击/点按钮

### #94 日报模板可编辑 + 一键还原 + 占位符化 + 解析对齐

**思路**：抽取默认模板为单一数据源（含占位符）→ 用户可编辑 override → build_ai_prompt 渲染占位符 → 一键还原 → 默认标题统一为 `## ` 让段落编辑生效。

- `crates/core/src/analysis/summary.rs`：
  - 抽 `pub fn default_report_prompt_template(locale) -> String`（三语言），用占位符 `{{date}} {{apps}} {{websites}} {{hourly}} {{keywords}} {{timeline}}`，**标题改为 `## ` 格式**（与 `parseSections` 对齐，修掉现有 `**加粗**` 不被分段的瑕疵）。
  - `build_ai_prompt`：若 override 非空用 override，否则用默认模板；统一做占位符替换后再 `append_custom_prompt_for_locale`（保留附加提示词兼容）。
- 配置 `crates/core/src/config.rs`：新增 `#[serde(default)] pub daily_report_template_override: String`（default ""，normalize trim）。
- 后端命令：`get_default_report_template(locale)`（供"还原默认"取唯一真源）；保存 override 走现有配置保存。
- 前端 `src/routes/report/Report.svelte`：在附加提示词区附近加"编辑日报模板"折叠区 + "还原默认"按钮（复用现有 preset UI 风格）；i18n。
- 同步：`generate_fallback_ai_content`（summary.rs:683）与测试夹具中的标题统一为 `## `，避免不一致。

**改动文件**：`summary.rs`、`mod.rs`、`config.rs`、`commands.rs`、`main.rs`、`Report.svelte`、`i18n/*.js`

**子任务**
- [ ] summary.rs：抽默认模板（占位符 + `## ` 标题）为单一数据源
- [ ] build_ai_prompt 改占位符渲染 + override 优先
- [ ] config 新增 `daily_report_template_override`
- [ ] 命令 `get_default_report_template` + 注册
- [ ] 前端编辑模板 + 还原默认 + i18n
- [ ] fallback/测试夹具标题对齐 `## `

---

## 跨 Issue：文档与变更日志联动（G1/G2）
- [ ] 更新 `PROJECTWIKI.md`（缺失则按模板新建基础版）：受影响模块（monitor/avatar/analysis/report 导出）、新增配置字段、新增命令、ADR（智能穿透后端驱动、模板占位符化）。
- [ ] 更新 `CHANGELOG.md`（Keep a Changelog）：Fixed（#98）、Added（#96/#95/#94），与提交 SHA 双向关联。

## 验证计划（目标覆盖率 85%）
- 单元/逻辑测试：
  - #98：`build_daily_report_export_path`、补导出幂等、导出失败不污染生成返回。
  - #96：`is_browser_app` 内置 tabbit + 自定义列表命中/不误伤（仿 categorize.rs:835 现有测试）。
  - #94：占位符渲染、override 优先、`default_report_prompt_template` 与 parseSections 标题对齐（前端 `parseSections` 测试）。
  - #95：命中测试纯函数（坐标 in/out bounding box）。
- 手动验证：
  - #98：设置自动导出，模拟导出目录不可写→恢复→确认补导出 + 有失败提示。
  - #95：开穿透，透明区点穿到下层应用；桌宠本体仍可拖动/双击/点跟进按钮。
  - #96：把某进程名加入自定义列表→前台与统计均识别为浏览器。
  - #94：编辑模板→生成日报样式随之变；一键还原；段落可编辑。
- 全量回归：`cargo test`（Rust）+ 前端 `*.test.js`（vitest）。
- 代码质量：每个 issue 改完做静态检查（`cargo clippy`）；按规则做两轮审查（codex skill 若可用，否则主模型 + clippy 双轮）。

## 回滚策略
- 所有新增 config 字段均 `#[serde(default)]`，旧配置文件可正常加载；移除字段不破坏旧数据。
- 每个 issue 独立提交（Conventional Commits），便于单独 revert。
- 智能穿透/模板 override 均为开关/可空，默认行为与现状一致（关闭即旧行为）。

## 执行顺序（建议）
1. [ ] #98（P0 Bug，价值最高、风险最低）
2. [ ] #96（通用浏览器列表）
3. [ ] #94（模板占位符化）
4. [ ] #95（智能穿透，最复杂，放最后）
5. [ ] 文档 + CHANGELOG + 全量回归 + 双轮审查

---

## 评审（执行后填写）
> 完成后在此总结实际改动、偏差、遗留项。
