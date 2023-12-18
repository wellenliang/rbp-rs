use std::collections::HashMap;

/// Metadata 元数据
pub type Metadata = HashMap<String, Value>;

/// metadata value
/// 元数据值
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Byte(i8),
    UByte(u8),
    Short(i16),
    UShort(u16),
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Float(f32),
    Double(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl Eq for Value {}
