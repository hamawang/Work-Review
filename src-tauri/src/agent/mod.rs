//! Agent 模块 — 第三代 Agentic 架构
//!
//! 五层结构：Tools → Model → Executor → Memory → Orchestrator
//! 当前进度：Stage 1-5 全部完成 ✅

pub mod executor;
pub mod memory;
pub mod model;
pub mod orchestrator;
pub mod tools;

pub use executor::{AgentExecutor, AgentResult, TraceStep};
pub use memory::ConversationMemory;
pub use model::{chat_with_tools, LlmResponse, Message, StopReason, ToolCall};
pub use orchestrator::{direct_answer, fast_answer, route_query, Orchestrator, OrchestratorResult, QueryPath};
pub use tools::{ToolDefinition, ToolRegistry};
