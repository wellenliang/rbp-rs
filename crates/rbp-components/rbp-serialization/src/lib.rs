use serde::{Serialize, de::DeserializeOwned};

#[cfg(feature="serialization-json")]
pub mod json;

#[cfg(feature="serialization-prost")]
pub mod prost;

pub trait Serialization<T>: Sync + Send where T: Serialize + DeserializeOwned{
    type Error: Send+ Sync;

    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error>;

    fn deserialize(&self, data: Vec<u8>) -> Result<T, Self::Error>;

    fn content_type(&self) -> &str;
}
