pub mod error;
pub mod stream_channel;
pub mod topic;

use async_trait::async_trait;
use error::MessagingError;
use rbp_core::messaging::MessageEnvelope;
use stream_channel::StreamChannel;
use topic::Topic;

/// messaging asynchronously results
/// 消息总线统一异步处理结果
pub type MessagingResult<T, E> = Result<T, MessagingError<E>>;



#[async_trait]
pub trait MessageBus {
    type Error: std::error::Error;
    type Dispatcher: MessageBusDispatcher;

    async fn publish<T: Topic>(
        &self,
        topic: T,
        msg: MessageEnvelope,
    ) -> MessagingResult<(), Self::Error>;

    async fn subscribe<H: MessageHandler, T: Topic>(
        &mut self,
        topic: T,
        handler: H,
    ) -> MessagingResult<(), Self::Error>;

    async fn unsubscribe<T: Topic, H: MessageHandler>(
        &mut self,
        topic: T,
        handler: H,
    ) -> MessagingResult<(), Self::Error>;
}

#[async_trait]
pub trait MessageBusDispatcher: Send {
    type Error: std::error::Error;
    type StreamChannelOptions;

    ///  message bus connect to  message broker
    async fn create_stream_channel<T: Topic, SC: StreamChannel>(
        &self,
        urls: Vec<String>,
        topic: T,
        opts: Self::StreamChannelOptions,
    ) -> MessagingResult<SC, Self::Error>;

    async fn dispatcher<T: Topic>(
        &self,
        topic: T,
        msg: MessageEnvelope,
    ) -> MessagingResult<T, Self::Error>;

    /// message bus disconnect from message broker
    async fn close_stream_channel<T: Topic>(&self, topic: T) -> MessagingResult<(), Self::Error>;


    fn stream_channels<SC: StreamChannel>(&self) -> Vec<SC>;

    fn topics<T: Topic>(&self) -> Vec<T>;
}

#[async_trait]
pub trait MessageHandler {
    type Error: std::error::Error;

    async fn handle_message(&self, msg: MessageEnvelope) -> MessagingResult<(), Self::Error>;
}
