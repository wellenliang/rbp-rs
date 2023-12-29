use self::header::Header;

pub mod header;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message<T>
{
    header: Header,
    body: T,
}
