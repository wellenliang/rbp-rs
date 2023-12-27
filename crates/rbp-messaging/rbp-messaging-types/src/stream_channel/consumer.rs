use rbp_core::{identifier::Identifier, named::Named};

pub trait StreamConsumer: Identifier + Named {
    type Error: std::error::Error;


}
