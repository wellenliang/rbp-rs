
use rbp_serialization::Serialization;

#[derive(Debug)]
pub struct Endpoint<T> {
    name: String,
    pub serialization: Box<dyn Serialization<T>>
}
