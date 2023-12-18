
/// Domain Driven Identifier
/// 领域驱动--标识符
pub trait Identifier {
    type Id;

    fn id(&self) -> Self::Id;
}
