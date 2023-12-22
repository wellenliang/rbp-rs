use std::future::Future;

use async_trait::async_trait;
use rbp_core::{messaging::MessageEnvelope, identifier::Identifier, named::Named};

use crate::{MessagingResult, topic::Topic};

#[async_trait]
pub trait StreamProducer: Identifier + Named {
    type Error: std::error::Error;

    type Receipt;

    type SendFuture: Future<Output = MessagingResult<Self::Receipt, Self::Error>>;

    fn topic(&self) -> &str;

    async fn send(&self, msg: MessageEnvelope) -> MessagingResult<Self::SendFuture, Self::Error>;
}

#[async_trait]
pub trait StreamProducerBuilder {
    type Error: std::error::Error;

    type StreamProducer: StreamProducer;
    type StreamProducerOptions;

    fn with_topic<T: Topic>(self, topic: T) -> Self;

    fn with_options(self, opts: Self::StreamProducerOptions) -> Self;

    async fn build(self) -> MessagingResult<Self::StreamProducer, Self::Error>;
}
