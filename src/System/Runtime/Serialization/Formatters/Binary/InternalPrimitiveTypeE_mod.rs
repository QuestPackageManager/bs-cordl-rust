#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+InternalPrimitiveTypeE")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InternalPrimitiveTypeE {
    #[default]
    Boolean = 1i32,
    Byte = 2i32,
    Char = 3i32,
    Currency = 4i32,
    DateTime = 13i32,
    Decimal = 5i32,
    Double = 6i32,
    Int16 = 7i32,
    Int32 = 8i32,
    Int64 = 9i32,
    Invalid = 0i32,
    Null = 17i32,
    SByte = 10i32,
    Single = 11i32,
    String = 18i32,
    TimeSpan = 12i32,
    UInt16 = 14i32,
    UInt32 = 15i32,
    UInt64 = 16i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+InternalPrimitiveTypeE")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE =>
    "System.Runtime.Serialization.Formatters.Binary"."InternalPrimitiveTypeE"
);
