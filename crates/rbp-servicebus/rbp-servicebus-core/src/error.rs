use thiserror::Error;

#[derive(Debug, Error)]
pub enum MessageKindError {
    #[error("invalid message kind string")]
    StrError
}
