#[cfg(feature = "System+Data+Common+StorageType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StorageType {
    #[default]
    BigInteger = 24i32,
    Boolean = 3i32,
    Byte = 6i32,
    ByteArray = 20i32,
    Char = 4i32,
    CharArray = 21i32,
    DBNull = 2i32,
    DateTime = 16i32,
    DateTimeOffset = 23i32,
    Decimal = 15i32,
    Double = 14i32,
    Empty = 0i32,
    Guid = 19i32,
    Int16 = 7i32,
    Int32 = 9i32,
    Int64 = 11i32,
    Object = 1i32,
    SByte = 5i32,
    Single = 13i32,
    SqlBinary = 26i32,
    SqlBoolean = 27i32,
    SqlByte = 28i32,
    SqlBytes = 29i32,
    SqlChars = 30i32,
    SqlDateTime = 31i32,
    SqlDecimal = 32i32,
    SqlDouble = 33i32,
    SqlGuid = 34i32,
    SqlInt16 = 35i32,
    SqlInt32 = 36i32,
    SqlInt64 = 37i32,
    SqlMoney = 38i32,
    SqlSingle = 39i32,
    SqlString = 40i32,
    String = 18i32,
    TimeSpan = 17i32,
    Type = 22i32,
    UInt16 = 8i32,
    UInt32 = 10i32,
    UInt64 = 12i32,
    Uri = 25i32,
}
#[cfg(feature = "System+Data+Common+StorageType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::Common::StorageType =>
    "System.Data.Common"."StorageType"
);
