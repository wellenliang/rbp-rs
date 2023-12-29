#[cfg(feature = "serialization-json")]
pub mod json;

#[cfg(feature = "serialization-prost")]
pub mod prost;

use std::fmt::Debug;

#[cfg(feature="serialization-prost")]
use prost::ProstSerializationError;

pub trait Serialization<T>: Sync + Send + Debug
{

    fn serialize(&self, value: T) -> Result<Vec<u8>, SerializationError>;

    fn deserialize(&self, data: Vec<u8>) -> Result<T,SerializationError>;

    fn content_type(&self) -> &str;
}

#[derive(Debug)]
pub enum SerializationError {
    #[cfg(feature="serialization-json")]
    JsonError(serde_json::Error),
    #[cfg(feature="serialization-prost")]
    ProstError(ProstSerializationError)
}
