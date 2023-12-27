use std::marker::PhantomData;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

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
    T: Serialize + DeserializeOwned + Send + Sync,
    for<'d> T: Deserialize<'d>,
{
    type Error = serde_json::Error;

    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(&value)
    }

    fn deserialize(&self, data: Vec<u8>) -> Result<T, Self::Error> {
        serde_json::from_slice(&data)
    }

    fn content_type(&self) -> &str {
        "json"
    }
}
