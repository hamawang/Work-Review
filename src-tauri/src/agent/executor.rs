//! Stage 3: Agent Loop — Agent 的"大脑"
//!
//! 核心循环：LLM 自主决定调什么工具、调几次、什么时候回答。
//!
//! 对应 Python: 03_agent_loop.py 里的 agent_run() 函数
//! 架构位置：在 Tools (Stage 1) 和 Model (Stage 2) 之上

use super::model::{self, LlmResponse, Message, StopReason, ToolCall};
use super::tools::ToolRegistry;
use crate::config::ModelConfig;
use crate::database::Database;
use crate::error::AppError;
use serde_json::Value;
use std::time::Instant;

// ══════════════════════════════════════════════════════════
// Agent 执行结果
// ══════════════════════════════════════════════════════════

/// Agent 的执行结果
#[derive(Debug)]
pub struct AgentResult {
    /// 最终回答
    pub answer: String,
    /// 使用了几轮
    pub iterations: usize,
    /// 执行过程追踪（用于调试和前端展示）
    pub trace: Vec<TraceStep>,
    /// 是否使用了 AI
    pub used_ai: bool,
    /// 工具调用记录
    pub tool_labels: Vec<String>,
}

/// 执行追踪的每一步
#[derive(Debug, Clone)]
pub enum TraceStep {
    /// LLM 给出最终回答
    FinalAnswer { round: usize, content: String },
    /// LLM 调用了工具
    ToolCall {
        round: usize,
        tool_name: String,
        arguments: Value,
    },
    /// 工具执行结果
    ToolResult {
        round: usize,
        tool_name: String,
        result_preview: String,
    },
    /// 达到最大迭代次数
    MaxIterationsReached { max: usize },
}

// ══════════════════════════════════════════════════════════
// Agent 执行器 — 核心循环
// ══════════════════════════════════════════════════════════

/// 默认最大迭代次数
const DEFAULT_MAX_ITERATIONS: usize = 8;

/// 默认 system prompt
const DEFAULT_SYSTEM_PROMPT: &str =
    "你是 Work Review 的工作助手。你只能基于给定记录回答。\
     请使用简体中文回答，直接回应用户问题，先给结论再给依据。\
     不要编造不存在的事实。";

/// Agent 执行器
///
/// 对应 Python 的 agent_run() 函数
pub struct AgentExecutor;

impl AgentExecutor {
    /// 运行 Agent 循环
    ///
    /// 这是整个 Agent 的心脏。逻辑和 Python 版完全一致：
    /// ```
    /// for i in 0..max_iterations:
    ///     response = llm.chat(messages, tools)
    ///     if response 是最终回答 → 返回
    ///     if response 是工具调用 → 执行工具，结果追加到 messages，继续
    /// 超过 max_iterations → 强制结束
    /// ```
    pub async fn run(
        question: &str,
        model_config: &ModelConfig,
        database: &Database,
        system_prompt: Option<&str>,
        history: &[Message],
        max_iterations: Option<usize>,
    ) -> Result<AgentResult, AppError> {
        let sys = system_prompt.unwrap_or(DEFAULT_SYSTEM_PROMPT);
        let max_iter = max_iterations.unwrap_or(DEFAULT_MAX_ITERATIONS);

        // 工具注册中心（Stage 1）
        let registry = ToolRegistry::new();
        let tools = registry.to_openai_tools();
        let tool_context = super::tools::ToolContext { database };

        // 构造初始消息：历史 + 当前问题
        let mut messages: Vec<Message> = history.to_vec();
        messages.push(Message::user(question));

        let mut trace = Vec::new();
        let mut tool_labels = Vec::new();
        let start = Instant::now();

        for i in 0..max_iter {
            // ── 第 1 步：调用 LLM（Stage 2） ──
            let response = model::chat_with_tools(model_config, sys, &messages, &tools)
                .await
                .map_err(|e| AppError::Analysis(format!("Agent 调用失败: {e}")))?;

            // ── 第 2 步：判断 LLM 的意图 ──
            match response.stop_reason {
                StopReason::Stop => {
                    // LLM 给出最终回答 → 循环结束
                    let content = response.content.unwrap_or_default();
                    trace.push(TraceStep::FinalAnswer {
                        round: i + 1,
                        content: content.chars().take(100).collect(),
                    });

                    return Ok(AgentResult {
                        answer: content,
                        iterations: i + 1,
                        trace,
                        used_ai: true,
                        tool_labels,
                    });
                }

                StopReason::ToolCall => {
                    // LLM 想调工具 → 执行
                    if let Some(calls) = &response.tool_calls {
                        // ① 记录 assistant 的工具调用
                        messages.push(Message::assistant_with_tool_calls(calls));

                        // ② 逐个执行工具
                        for tc in calls {
                            trace.push(TraceStep::ToolCall {
                                round: i + 1,
                                tool_name: tc.name.clone(),
                                arguments: tc.arguments.clone(),
                            });

                            if !tool_labels.contains(&tc.name) {
                                tool_labels.push(tc.name.clone());
                            }

                            // 执行工具（Stage 1）
                            let result = match registry.execute(&tc.name, tc.arguments.clone(), &tool_context) {
                                Ok(r) => r,
                                Err(e) => format!("工具执行失败: {e}"),
                            };

                            trace.push(TraceStep::ToolResult {
                                round: i + 1,
                                tool_name: tc.name.clone(),
                                result_preview: result.chars().take(80).collect(),
                            });

                            // ③ 追加工具结果到对话历史（携带工具名，Gemini 需要）
                            messages.push(Message::tool_result_named(
                                &tc.id,
                                &result,
                                Some(&tc.name),
                            ));
                        }
                    }
                    // 继续循环 → LLM 下一轮能看到工具结果
                }

                StopReason::MaxTokens => {
                    // Token 用完了，用已有内容回答
                    let content = response.content.unwrap_or_else(|| "回答被截断，请尝试缩短问题。".to_string());
                    trace.push(TraceStep::FinalAnswer {
                        round: i + 1,
                        content: content.chars().take(100).collect(),
                    });
                    return Ok(AgentResult {
                        answer: content,
                        iterations: i + 1,
                        trace,
                        used_ai: true,
                        tool_labels,
                    });
                }
            }

            // 安全检查：如果循环超过 30 秒，强制结束
            if start.elapsed().as_secs() > 30 {
                trace.push(TraceStep::MaxIterationsReached { max: max_iter });
                return Ok(AgentResult {
                    answer: "处理超时，请尝试更具体的问题。".to_string(),
                    iterations: i + 1,
                    trace,
                    used_ai: true,
                    tool_labels,
                });
            }
        }

        // ── 超过最大迭代次数 ──
        trace.push(TraceStep::MaxIterationsReached { max: max_iter });
        Ok(AgentResult {
            answer: "抱歉，处理这个问题需要过多步骤。请尝试更具体地描述。".to_string(),
            iterations: max_iter,
            trace,
            used_ai: true,
            tool_labels,
        })
    }
}

// ══════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_step_final_answer_format() {
        let step = TraceStep::FinalAnswer {
            round: 1,
            content: "今天你主要在编码".to_string(),
        };
        if let TraceStep::FinalAnswer { round, content } = step {
            assert_eq!(round, 1);
            assert_eq!(content, "今天你主要在编码");
        }
    }

    #[test]
    fn test_trace_step_tool_call_format() {
        let step = TraceStep::ToolCall {
            round: 2,
            tool_name: "search_memory".to_string(),
            arguments: serde_json::json!({"query": "debug"}),
        };
        if let TraceStep::ToolCall {
            round,
            tool_name,
            arguments,
        } = step
        {
            assert_eq!(round, 2);
            assert_eq!(tool_name, "search_memory");
            assert_eq!(arguments["query"], "debug");
        }
    }

    #[test]
    fn test_agent_result_trace_ordering() {
        // 模拟一次完整的 trace：调工具 → 拿结果 → 最终回答
        let trace = vec![
            TraceStep::ToolCall {
                round: 1,
                tool_name: "analyze_intents".to_string(),
                arguments: serde_json::json!({"date_from": "2026-06-01", "date_to": "2026-06-09"}),
            },
            TraceStep::ToolResult {
                round: 1,
                tool_name: "analyze_intents".to_string(),
                result_preview: "编码 8h (73%)...".to_string(),
            },
            TraceStep::FinalAnswer {
                round: 2,
                content: "本周你主要在编码...".to_string(),
            },
        ];

        // 验证 trace 的顺序正确
        assert_eq!(trace.len(), 3);
        assert!(matches!(&trace[0], TraceStep::ToolCall { round: 1, .. }));
        assert!(matches!(&trace[1], TraceStep::ToolResult { round: 1, .. }));
        assert!(matches!(&trace[2], TraceStep::FinalAnswer { round: 2, .. }));
    }

    #[test]
    fn test_max_iterations_default() {
        assert_eq!(DEFAULT_MAX_ITERATIONS, 8);
    }

    #[test]
    fn test_message_construction() {
        let user_msg = Message::user("今天做了什么");
        assert_eq!(user_msg.role, "user");
        assert_eq!(user_msg.content.as_deref(), Some("今天做了什么"));

        let tool_msg = Message::tool_result("call_123", "结果");
        assert_eq!(tool_msg.role, "tool");
        assert_eq!(tool_msg.tool_call_id.as_deref(), Some("call_123"));

        let tc = ToolCall {
            id: "call_456".to_string(),
            name: "search_memory".to_string(),
            arguments: serde_json::json!({"query": "debug"}),
        };
        let assistant_msg = Message::assistant_with_tool_calls(&[tc]);
        assert_eq!(assistant_msg.role, "assistant");
        assert!(assistant_msg.tool_calls.is_some());
    }
}
