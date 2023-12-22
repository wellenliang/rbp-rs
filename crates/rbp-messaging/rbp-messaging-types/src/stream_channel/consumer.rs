use rbp_core::{identifier::Identifier, named::Named};

pub trait SingleTopicStreamConsumer: Identifier + Named {
    type Error: std::error::Error;


}
