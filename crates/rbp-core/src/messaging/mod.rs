use std::collections::HashMap;

use crate::metadata::{Metadata, Value};

pub trait Message: Sized {
    type Header;

    type Payload;

    /// return the message header
    fn header(&self) -> Self::Header;

    /// set header
    fn with_header(&mut self, header: Self::Header);

    /// return message payload
    fn payload(&self) -> Self::Payload;

    /// set payload
    fn with_payload(&mut self, payload: Self::Payload);

    /// return message timestamp
    fn timestamp(&self) -> i64;
}

/// Message envelope
/// 消息信封
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageEnvelope {
    metadata: Metadata,
    payload: Vec<u8>,
    attributes: HashMap<String, String>,
}

impl MessageEnvelope  {
    pub fn new(payload: Vec<u8>) -> Self {
        Self {
            metadata: Metadata::new(),
            payload,
            attributes: HashMap::new(),
        }
    }

    pub fn insert_metadata(&mut self, key: String, value: Value) {
        self.metadata.insert(key, value);
    }

    pub fn remove_metadata(&mut self, key: String) {
        self.metadata.remove(&key);
    }

    pub fn insert_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    pub fn remove_attribute(&mut self, key: String) {
        self.attributes.remove(&key);
    }
}

