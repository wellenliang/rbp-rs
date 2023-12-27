use serde::{Serialize, de::DeserializeOwned};

use super::headers::MessageHeader;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message<T> where T: Serialize + DeserializeOwned  {
    header: MessageHeader,
    body: T
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportMessage {
    header: MessageHeader,
    body: Vec<u8>
}

