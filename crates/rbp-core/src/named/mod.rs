/// bound context named for domain driven design
/// 领域驱动-边界上下文名称
pub trait NamedBoundContext {
    /// return the bound context name
    fn bound_context_name(&self) -> &'static str;
}

/// aggregate named for domain driven design
/// 领域驱动-聚合名称
pub trait NamedAggregate {
    /// return the name of aggregate
    fn aggregate_name(&self) -> &'static str;
}
