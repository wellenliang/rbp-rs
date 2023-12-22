
/// Domain Driven Identifier
/// 领域驱动--标识符
pub trait Identifier {
    type Id;
    /// 返回实体标识符
    fn id(&self) -> Self::Id;
}


pub trait TenantId {

    type TenantId;

    fn tenant_id(&self) ->Self::TenantId;
}


pub trait AggregateId {

    type AggregateId;

    fn aggregate_id(&self) -> Self::AggregateId;
}
