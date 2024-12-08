#[cfg(feature = "System+Resources+ResourceTypeCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceTypeCode {
    Boolean = 2i32,
    Byte = 4i32,
    ByteArray = 32i32,
    Char = 3i32,
    DateTime = 15i32,
    Decimal = 14i32,
    Double = 13i32,
    Int16 = 6i32,
    Int32 = 8i32,
    Int64 = 10i32,
    LastPrimitive = 16i32,
    Null = 0i32,
    SByte = 5i32,
    Single = 12i32,
    StartOfUserTypes = 64i32,
    Stream = 33i32,
    String = 1i32,
    UInt16 = 7i32,
    UInt32 = 9i32,
    UInt64 = 11i32,
}
#[cfg(feature = "System+Resources+ResourceTypeCode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Resources::ResourceTypeCode =>
    "System.Resources"."ResourceTypeCode"
);
