use std::{marker::PhantomData, fmt::Debug};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::SerializationError;

use super::Serialization;

#[derive(Debug, Clone, Copy)]
pub struct JsonSerialization<T>(PhantomData<T>)
where
    T: Serialize + Send + Sync,
    for<'d> T: Deserialize<'d>;

impl<T> Default for JsonSerialization<T>
where
    T: Serialize + Send + Sync,
    for<'d> T: Deserialize<'d>,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Serialization<T> for JsonSerialization<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + Debug,
    for<'d> T: Deserialize<'d>,
{
    fn serialize(&self, value: T) -> Result<Vec<u8>, SerializationError> {
        serde_json::to_vec(&value).map_err(SerializationError::JsonError)
    }

    fn deserialize(&self, data: Vec<u8>) -> Result<T, SerializationError> {
        serde_json::from_slice(&data).map_err(SerializationError::JsonError)
    }

    fn content_type(&self) -> &str {
        "json"
    }
}
