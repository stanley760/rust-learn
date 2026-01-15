use dioxus::prelude::*;
use std::time::Duration;

/// Message 类型，对应 Ant Design 的 message 类型
#[derive(Clone, PartialEq)]
pub enum MessageType {
    Success,
    Error,
    Warning,
    Info,
    Loading,
}

/// Message 配置
#[derive(Clone, PartialEq)]
pub struct MessageConfig {
    pub content: String,
    pub message_type: MessageType,
    pub duration: u64, // 秒
    pub id: usize,
}

impl MessageConfig {
    pub fn new(content: String, message_type: MessageType, id: usize) -> Self {
        Self {
            content,
            message_type,
            duration: 3, // 默认 3 秒
            id,
        }
    }

    pub fn with_duration(mut self, duration: u64) -> Self {
        self.duration = duration;
        self
    }
}

/// Message 组件（单个消息）
#[component]
fn MessageItem(config: MessageConfig, on_close: EventHandler<usize>) -> Element {
    let class_name = match config.message_type {
        MessageType::Success => "antd-message antd-message-success",
        MessageType::Error => "antd-message antd-message-error",
        MessageType::Warning => "antd-message antd-message-warning",
        MessageType::Info => "antd-message antd-message-info",
        MessageType::Loading => "antd-message antd-message-loading",
    };

    let icon = match config.message_type {
        MessageType::Success => "✓",
        MessageType::Error => "✕",
        MessageType::Warning => "⚠",
        MessageType::Info => "ℹ",
        MessageType::Loading => "⟳",
    };

    // 自动关闭定时器
    use_effect(move || {
        let id = config.id;
        let duration = config.duration;
        spawn(async move {
            tokio::time::sleep(Duration::from_secs(duration)).await;
            on_close.call(id);
        });
    });

    rsx! {
        div {
            class: "{class_name}",
            span { class: "antd-message-icon", "{icon}" }
            span { class: "antd-message-content", "{config.content}" }
        }
    }
}

/// Message 容器（全局提示容器）
#[component]
pub fn MessageContainer(messages: Signal<Vec<MessageConfig>>) -> Element {
    let mut messages_signal = messages;

    let remove_message = move |id: usize| {
        messages_signal.write().retain(|m| m.id != id);
    };

    rsx! {
        div {
            class: "antd-message-container",
            for message in messages.read().iter() {
                MessageItem {
                    key: "{message.id}",
                    config: message.clone(),
                    on_close: remove_message
                }
            }
        }
    }
}

/// Message API - 类似 Ant Design 的全局 API
pub struct MessageApi {
    messages: Signal<Vec<MessageConfig>>,
    id_counter: Signal<usize>,
}

impl MessageApi {
    pub fn new(messages: Signal<Vec<MessageConfig>>, id_counter: Signal<usize>) -> Self {
        Self {
            messages,
            id_counter,
        }
    }

    /// 显示成功消息
    pub fn success(&mut self, content: impl Into<String>) {
        self.show(content.into(), MessageType::Success, 3);
    }

    /// 显示错误消息
    pub fn error(&mut self, content: impl Into<String>) {
        self.show(content.into(), MessageType::Error, 3);
    }

    /// 显示警告消息
    pub fn warning(&mut self, content: impl Into<String>) {
        self.show(content.into(), MessageType::Warning, 3);
    }

    /// 显示信息消息
    pub fn info(&mut self, content: impl Into<String>) {
        self.show(content.into(), MessageType::Info, 3);
    }

    /// 显示加载消息
    pub fn loading(&mut self, content: impl Into<String>) -> usize {
        let id = self.get_next_id();
        let config = MessageConfig {
            content: content.into(),
            message_type: MessageType::Loading,
            duration: 0, // loading 不自动关闭
            id,
        };
        self.messages.write().push(config);
        id
    }

    /// 手动关闭消息
    pub fn close(&mut self, id: usize) {
        self.messages.write().retain(|m| m.id != id);
    }

    /// 自定义持续时间
    pub fn show_with_duration(
        &mut self,
        content: impl Into<String>,
        message_type: MessageType,
        duration: u64,
    ) {
        self.show(content.into(), message_type, duration);
    }

    // 内部方法
    fn show(&mut self, content: String, message_type: MessageType, duration: u64) {
        let id = self.get_next_id();
        let config = MessageConfig {
            content,
            message_type,
            duration,
            id,
        };
        self.messages.write().push(config);
    }

    fn get_next_id(&mut self) -> usize {
        let id = *self.id_counter.read();
        self.id_counter.set(id + 1);
        id
    }
}

/// 创建 Message API 的 Hook
pub fn use_message() -> MessageApi {
    let messages = use_context::<Signal<Vec<MessageConfig>>>();
    let id_counter = use_context::<Signal<usize>>();
    MessageApi::new(messages, id_counter)
}
