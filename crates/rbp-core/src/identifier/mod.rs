
/// Domain Driven Identifier
/// 领域驱动--标识符
pub trait Identifier<T> {
    /// 返回实体标识符
    fn id(&self) -> T;
}
