//! 工作助手流式事件。
//!
//! agent 模块用 `tokio::sync::mpsc` 传递这些事件，commands 层再桥接到
//! Tauri `ipc::Channel`。本文件纯 serde，不依赖 tauri，保持 agent 可单测。

use serde::{Deserialize, Serialize};
use work_review_core::database::MemorySearchItem;

/// 工作助手流式事件（经 Tauri `ipc::Channel` 推送给前端）。
///
/// 前端按 `type` 字段分发：`stepStart` / `stepResult` / `token` / `done` / `error`。
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum StreamEvent {
    /// 工具步骤开始（每个 tool_call 执行前推送）。
    StepStart { tool: String, label: String },
    /// 工具步骤完成，携带本次新增的引用记录。
    StepResult {
        tool: String,
        hits: usize,
        references: Vec<MemorySearchItem>,
    },
    /// LLM 文本增量（阶段 2 token 流式）。阶段 1 不产生此事件。
    Token(String),
    /// 终态：完整答案 + 合并后的全部引用 + 用到的工具标签。
    Done {
        answer: String,
        references: Vec<MemorySearchItem>,
        tool_labels: Vec<String>,
    },
    /// 错误终态。
    Error(String),
}

/// 工具名 → 默认中文标签（前端可按 tool 名覆盖为 i18n 文案）。
///
/// 放在这里而不是前端，是因为 executor 推送 `StepStart` 时需要立即给一个 label，
/// 否则前端在 i18n 未命中时会空白。
pub fn default_tool_label(tool: &str) -> &'static str {
    match tool {
        "search_memory" => "记忆检索",
        "analyze_intents" => "意图分析",
        "aggregate_stats" => "统计聚合",
        "category_search" => "分类检索",
        "trend_comparison" => "趋势对比",
        _ => "处理中",
    }
}
