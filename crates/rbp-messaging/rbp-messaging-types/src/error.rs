use thiserror::Error;


/// messaging error
/// 消息总线错误
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MessagingError<E> {

    #[error("backend error: {0}")]
    Backend(E)
}
