use std::marker::PhantomData;

use prost::{Message, bytes::Bytes};
use crate::{Serialization, SerializationError};

#[derive(Debug, Clone, Copy, Default)]
pub struct ProstSerialization<T>(PhantomData<T>)
where
    T: Message + Default;

#[derive(Debug)]
pub enum ProstSerializationError {
    DecodeError(prost::DecodeError),
    EncodeError(prost::EncodeError),
}

impl<T> Serialization<T> for ProstSerialization<T> where T: Message + Default {

    fn serialize(&self, value: T) -> Result<Vec<u8>, SerializationError> {
        let data = value.encode_to_vec();
        Ok(data)
    }

    fn deserialize(&self, data: Vec<u8>) -> Result<T, SerializationError> {
        let buf = Bytes::from(data);
        T::decode(buf).map_err(|err|SerializationError::ProstError(ProstSerializationError::DecodeError(err)))
    }

    fn content_type(&self) -> &str {
        "protobuf"
    }
}
