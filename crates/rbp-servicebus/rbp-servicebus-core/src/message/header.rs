use std::{collections::HashMap, str::FromStr};

use crate::error::MessageKindError;


/// message header 消息头
pub type Header = HashMap<String, String>;

/// message id key value
/// 消息ID Key 值
pub const MESSAGE_ID_KEY: &str = "RBPServiceBus.MessageIDKey";

/// message broker id key value
/// 消息中间件分配给消息的ID
pub const MESSAGE_NATIVE_ID_KEY: &str = "RBPServiceBus.MessageNativeIDKey";

///  kinds for message
/// 消息类型
pub const MESSAGE_KIND_KEY: &str = "RBPServiceBus.MessageKindKey";

/// 消息类型
/// Enumeration defining different kinds of message like Send and Publish.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageKind {
    /// Regular point-to-point send.
    /// 点对点发送
    Send = 1,
    /// Publish, not a regular point-to-point send.
    /// 发布消息
    Publish = 2,
    /// 订阅消息
    Subscribe = 3,
    /// 取消订阅
    Unsubscribe = 4,
    /// 重试消息
    Reply = 5,
}

impl MessageKind {
    pub fn as_str(&self) -> &str {
        match self {
            MessageKind::Send => "send",
            MessageKind::Publish => "publish",
            MessageKind::Subscribe => "subscribe",
            MessageKind::Unsubscribe => "unsubscribe",
            MessageKind::Reply => "reply",
        }
    }
}

impl FromStr for MessageKind {
    type Err = MessageKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "send" {
            return Ok(MessageKind::Send);
        }

        if s == "publish" {
            return Ok(MessageKind::Publish);
        }

        if s == "subscribe" {
            return Ok(MessageKind::Subscribe);
        }

        if s == "unsubscribe" {
            return Ok(MessageKind::Unsubscribe);
        }

        if s == "reply" {
            return Ok(MessageKind::Reply);
        }

        Err(MessageKindError::StrError)
    }
}
