#[cfg(feature = "System+TypeCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeCode {
    #[default]
    Boolean = 3i32,
    Byte = 6i32,
    Char = 4i32,
    DBNull = 2i32,
    DateTime = 16i32,
    Decimal = 15i32,
    Double = 14i32,
    Empty = 0i32,
    Int16 = 7i32,
    Int32 = 9i32,
    Int64 = 11i32,
    Object = 1i32,
    SByte = 5i32,
    Single = 13i32,
    String = 18i32,
    UInt16 = 8i32,
    UInt32 = 10i32,
    UInt64 = 12i32,
}
#[cfg(feature = "System+TypeCode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::TypeCode => "System"."TypeCode"
);
