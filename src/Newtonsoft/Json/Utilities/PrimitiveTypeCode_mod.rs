#[cfg(feature = "Newtonsoft+Json+Utilities+PrimitiveTypeCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrimitiveTypeCode {
    #[default]
    BigInteger = 36i32,
    BigIntegerNullable = 37i32,
    Boolean = 4i32,
    BooleanNullable = 5i32,
    Byte = 14i32,
    ByteNullable = 15i32,
    Bytes = 40i32,
    Char = 2i32,
    CharNullable = 3i32,
    DBNull = 41i32,
    DateTime = 26i32,
    DateTimeNullable = 27i32,
    DateTimeOffset = 28i32,
    DateTimeOffsetNullable = 29i32,
    Decimal = 30i32,
    DecimalNullable = 31i32,
    Double = 24i32,
    DoubleNullable = 25i32,
    Empty = 0i32,
    Guid = 32i32,
    GuidNullable = 33i32,
    Int16 = 8i32,
    Int16Nullable = 9i32,
    Int32 = 12i32,
    Int32Nullable = 13i32,
    Int64 = 18i32,
    Int64Nullable = 19i32,
    Object = 1i32,
    SByte = 6i32,
    SByteNullable = 7i32,
    Single = 22i32,
    SingleNullable = 23i32,
    String = 39i32,
    TimeSpan = 34i32,
    TimeSpanNullable = 35i32,
    UInt16 = 10i32,
    UInt16Nullable = 11i32,
    UInt32 = 16i32,
    UInt32Nullable = 17i32,
    UInt64 = 20i32,
    UInt64Nullable = 21i32,
    Uri = 38i32,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+PrimitiveTypeCode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::PrimitiveTypeCode
    => "Newtonsoft.Json.Utilities"."PrimitiveTypeCode"
);
