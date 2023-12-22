mod consumer;
mod producer;
mod url;

use async_trait::async_trait;
use rbp_core::{identifier::Identifier, named::Named};


use crate::topic::Topic;
use crate::MessagingResult;

pub use self::consumer::*;
pub use self::producer::*;
pub use self::url::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalKind {
    InMemory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistributedKind {
    Pulsar,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamChannelKind {
    Local(LocalKind),
    Distributed(DistributedKind),
}

#[async_trait]
pub trait StreamChannel: Identifier + Named + Sized {
    type Error: std::error::Error;

    type ConnectOptions;

    type StreamProducer: StreamProducer;
    type StreamProducerOptions;

    type StreamConsumer;
    type StreamConsumerOptions;

    async fn connect(
        url: StreamChannelUrl,
        conn_opts: Self::ConnectOptions,
    ) -> MessagingResult<Self, Self::Error>;

    async fn create_stream_producer<T: Topic>(
        &self,
        topic: T,
        opts: Self::StreamProducerOptions,
    ) -> MessagingResult<Self::StreamProducer, Self::Error>;

    async fn create_stream_consumer<T: Topic>(
        &self,
        topic: Vec<T>,
        opts: Self::StreamConsumerOptions,
    ) -> MessagingResult<Self::StreamConsumer, Self::Error>;

    async fn disconnect(&mut self) -> Result<(), Self::Error>;

    fn kind(&self) -> StreamChannelKind;
}
