//! Stage 4: Memory 层 — Agent 的"记忆"
//!
//! 对话记忆管理：滑动窗口 + 自动压缩 + Token 预算控制。
//!
//! 对应 Python: 04_memory.py 里的 ConversationMemory 类

use super::model::Message;
use serde::{Deserialize, Serialize};

// ══════════════════════════════════════════════════════════
// 配置常量
// ══════════════════════════════════════════════════════════

/// 最多保留的消息条数（硬上限）
const MAX_MESSAGES: usize = 20;

/// 单条消息的最大字符数（超长截断）
const MAX_MESSAGE_CHARS: usize = 2000;

/// 发给 LLM 时最多使用的字符数（用于 get_context 截断）
/// 粗略对应 ~4000 tokens（中文 1 字 ≈ 2-3 tokens，这里用字符数近似）
const DEFAULT_CONTEXT_BUDGET: usize = 8000;

// ══════════════════════════════════════════════════════════
// Memory 结构体
// ══════════════════════════════════════════════════════════

/// 对话记忆管理器
///
/// 对应 Python: ConversationMemory
/// 职责：维护历史、自动压缩、Token 预算控制
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMemory {
    /// 完整对话历史
    messages: Vec<Message>,

    /// 旧消息的摘要（压缩后生成）
    summary: Option<String>,

    /// 最大消息条数
    max_messages: usize,

    /// 上下文字符预算
    context_budget: usize,
}

impl ConversationMemory {
    /// 创建新的 Memory 实例
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            summary: None,
            max_messages: MAX_MESSAGES,
            context_budget: DEFAULT_CONTEXT_BUDGET,
        }
    }

    /// 用已有历史创建（从前端传入的 history 初始化）
    pub fn from_history(history: &[Message]) -> Self {
        let mut memory = Self::new();
        for msg in history {
            memory.messages.push(Message {
                role: msg.role.clone(),
                content: msg.content.clone(),
                tool_calls: None,      // 历史里不保留工具调用细节
                tool_call_id: None,
                name: None,
            });
        }
        memory
    }

    /// 添加一条用户或助手消息
    pub fn add(&mut self, role: &str, content: &str) {
        // 截断超长消息
        let truncated = if content.len() > MAX_MESSAGE_CHARS {
            format!("{}...", &content[..MAX_MESSAGE_CHARS])
        } else {
            content.to_string()
        };

        self.messages.push(Message {
            role: role.to_string(),
            content: Some(truncated),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        });

        // 超过上限时自动压缩
        if self.messages.len() > self.max_messages {
            self.compact();
        }
    }

    /// 压缩旧消息
    ///
    /// 策略：保留最近一半，旧的一半压缩成摘要。
    /// 对应 Python: _compact()
    fn compact(&mut self) {
        let split_point = self.messages.len() / 2;
        let old_messages: Vec<&Message> = self.messages[..split_point].iter().collect();
        let recent_messages: Vec<Message> = self.messages[split_point..].to_vec();

        // 生成简单摘要（不用 LLM，用规则提取）
        let user_count = old_messages
            .iter()
            .filter(|m| m.role == "user")
            .count();
        let user_previews: Vec<&str> = old_messages
            .iter()
            .filter(|m| m.role == "user")
            .filter_map(|m| m.content.as_deref())
            .map(|c| {
                // 取前 30 个字符
                if c.chars().count() > 30 {
                    let end = c.char_indices().take(30).last().map(|(i, _)| i).unwrap_or(30);
                    &c[..end]
                } else {
                    c
                }
            })
            .take(3)
            .collect();

        let new_summary = if user_previews.is_empty() {
            format!("[早期对话：共{}条消息]", old_messages.len())
        } else {
            format!(
                "[早期对话摘要：用户问了{}个问题，涉及：{}]",
                user_count,
                user_previews.join("、")
            )
        };

        // 合并到已有摘要
        self.summary = if let Some(existing) = &self.summary {
            Some(format!("{} {}", existing, new_summary))
        } else {
            Some(new_summary)
        };

        self.messages = recent_messages;
    }

    /// 获取要在下次 LLM 调用中使用的上下文
    ///
    /// 这是 Memory 层最核心的方法。
    /// 从历史消息中选出不超过预算的部分，优先保留最近的消息。
    ///
    /// 对应 Python: get_context()
    pub fn get_context(&self) -> Vec<Message> {
        let mut result = Vec::new();
        let mut used_chars = 0usize;

        // 如果有摘要，先加上
        if let Some(summary) = &self.summary {
            used_chars += summary.len();
            result.push(Message {
                role: "system".to_string(),
                content: Some(summary.clone()),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            });
        }

        // 从最新的消息往前取，直到预算满了
        for msg in self.messages.iter().rev() {
            let msg_chars = msg.content.as_deref().map(|c| c.len()).unwrap_or(0);
            if used_chars + msg_chars > self.context_budget {
                break;
            }
            result.push(msg.clone());
            used_chars += msg_chars;
        }

        // 反转回正确顺序
        if let Some(summary_end) = result.iter().position(|m| m.role != "system") {
            // 摘要在最前面，消息按时间顺序
            let summary_part: Vec<Message> = result[..summary_end].to_vec();
            let mut msg_part: Vec<Message> = result[summary_end..].to_vec();
            msg_part.reverse();
            let mut final_result = summary_part;
            final_result.extend(msg_part);
            final_result
        } else {
            result.reverse();
            result
        }
    }

    /// 当前消息数量
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// 返回状态统计
    pub fn stats(&self) -> MemoryStats {
        let total_chars: usize = self
            .messages
            .iter()
            .filter_map(|m| m.content.as_deref().map(|c| c.len()))
            .sum();
        let summary_chars = self.summary.as_deref().map(|s| s.len()).unwrap_or(0);

        MemoryStats {
            total_messages: self.messages.len(),
            total_chars: total_chars + summary_chars,
            has_summary: self.summary.is_some(),
            summary_preview: self.summary.as_deref().map(|s| {
                let end = s.char_indices().take(60).last().map(|(i, _)| i).unwrap_or(s.len().min(60));
                s[..end].to_string()
            }),
        }
    }
}

/// Memory 状态统计
#[derive(Debug)]
pub struct MemoryStats {
    pub total_messages: usize,
    pub total_chars: usize,
    pub has_summary: bool,
    pub summary_preview: Option<String>,
}

impl Default for ConversationMemory {
    fn default() -> Self {
        Self::new()
    }
}

// ══════════════════════════════════════════════════════════
// 测试
// ══════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_add_and_len() {
        let mut mem = ConversationMemory::new();
        assert!(mem.is_empty());

        mem.add("user", "你好");
        mem.add("assistant", "你好！");
        assert_eq!(mem.len(), 2);
    }

    #[test]
    fn test_memory_truncates_long_messages() {
        let mut mem = ConversationMemory::new();
        let long_content = "x".repeat(3000);
        mem.add("user", &long_content);

        let msg = &mem.messages[0];
        // 应该被截断到 MAX_MESSAGE_CHARS + "..."
        assert!(msg.content.as_deref().unwrap().len() <= MAX_MESSAGE_CHARS + 3);
    }

    #[test]
    fn test_memory_compact_triggers_at_limit() {
        let mut mem = ConversationMemory::new();
        mem.max_messages = 6; // 降低上限方便测试

        for i in 0..8 {
            mem.add("user", &format!("消息{}", i));
            mem.add("assistant", &format!("回答{}", i));
        }

        // 16 条消息 > max_messages(6)，应该触发压缩
        assert!(mem.summary.is_some(), "超过上限应该生成摘要");
        assert!(mem.messages.len() <= mem.max_messages, "压缩后不应超过上限");

        let stats = mem.stats();
        assert!(stats.has_summary);
        assert!(stats.summary_preview.is_some());
    }

    #[test]
    fn test_memory_get_context_respects_budget() {
        let mut mem = ConversationMemory::new();
        mem.context_budget = 100; // 很小的预算

        // 添加多条消息
        for i in 0..10 {
            mem.add("user", &format!("这是一条比较长的用户消息编号{}", i));
            mem.add("assistant", &format!("这是助手的回答消息编号{}", i));
        }

        let context = mem.get_context();
        let total_chars: usize = context
            .iter()
            .filter_map(|m| m.content.as_deref().map(|c| c.len()))
            .sum();

        // 总字符数不应超过预算太多（摘要可能有额外开销）
        assert!(
            total_chars < 200,
            "上下文应大致在预算范围内，实际 {} 字符",
            total_chars
        );
    }

    #[test]
    fn test_memory_get_context_recent_first() {
        let mut mem = ConversationMemory::new();
        mem.context_budget = 200; // 小预算，只保留最近的消息

        mem.add("user", "最早的消息AAAAAAAAAA");
        mem.add("assistant", "最早的回答BBBBBBBBBB");
        mem.add("user", "最新的消息CCCCCCCCCC");
        mem.add("assistant", "最新的回答DDDDDDDDDD");

        let context = mem.get_context();
        let contents: Vec<&str> = context
            .iter()
            .filter_map(|m| m.content.as_deref())
            .collect();

        // 最新的消息应该在上下文中
        let has_recent = contents.iter().any(|c| c.contains("最新的"));
        assert!(has_recent, "应该保留最新的消息");

        // 最早的消息可能被截断
        // （取决于预算，不严格检查）
    }

    #[test]
    fn test_memory_from_history() {
        let history = vec![
            Message::user("问题1"),
            Message::user("问题2"),
        ];

        let mem = ConversationMemory::from_history(&history);
        assert_eq!(mem.len(), 2);
    }

    #[test]
    fn test_memory_summary_accumulates() {
        let mut mem = ConversationMemory::new();
        mem.max_messages = 4;

        // 第一轮填充 → 触发第一次压缩
        for i in 0..3 {
            mem.add("user", &format!("第{}个问题", i));
            mem.add("assistant", &format!("第{}个回答", i));
        }
        let summary1 = mem.summary.clone();

        // 第二轮填充 → 触发第二次压缩
        for i in 3..6 {
            mem.add("user", &format!("第{}个问题", i));
            mem.add("assistant", &format!("第{}个回答", i));
        }
        let summary2 = mem.summary.clone();

        // 第二次摘要应该包含第一次的内容
        assert!(summary1.is_some());
        assert!(summary2.is_some());
        assert!(summary2.unwrap().len() > summary1.unwrap().len(),
            "摘要应该随压缩轮次累积");
    }
}
