use std::marker::PhantomData;

use prost::{Message, bytes::Bytes};

use crate::Serialization;

#[derive(Debug, Clone, Copy, Default)]
pub struct ProstSerialization<T>(PhantomData<T>)
where
    T: Message + Default;

pub enum ProstSerializationError {
    DecodeError(prost::DecodeError),
    EncodeError(prost::EncodeError),
}

impl<T> Serialization<T> for ProstSerialization<T> where T: Message + Default {
    type Error=ProstSerializationError;

    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error> {
        let data = value.encode_to_vec();
        Ok(data)
    }

    fn deserialize(&self, data: Vec<u8>) -> Result<T, Self::Error> {
        let buf = Bytes::from(data);
        T::decode(buf).map_err(ProstSerializationError::DecodeError)
    }

    fn content_type(&self) -> &str {
        "prost"
    }
}
