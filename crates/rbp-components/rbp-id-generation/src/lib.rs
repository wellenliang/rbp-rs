
#[cfg(feature="uuid")]
pub mod uuid;
#[cfg(feature="ulid")]
pub mod ulid;

pub trait IdGeneratorMut<T> {
    type Error;

    fn generate(&mut self) -> Result<T, Self::Error>;
}

pub trait IdGenerator<T> {
    type Error;

    fn generate(self) -> Result<T, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdGeneratorError {
    UuidError,
    UlidError,
}
