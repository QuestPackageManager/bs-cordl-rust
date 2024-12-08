#[cfg(feature = "Newtonsoft+Json+Bson+BsonType")]
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BsonType {
    Array = 4i8,
    Binary = 5i8,
    Boolean = 8i8,
    Code = 13i8,
    CodeWScope = 15i8,
    Date = 9i8,
    Integer = 16i8,
    Long = 18i8,
    MaxKey = 127i8,
    MinKey = -1i8,
    Null = 10i8,
    Number = 1i8,
    Object = 3i8,
    Oid = 7i8,
    Reference = 12i8,
    Regex = 11i8,
    String = 2i8,
    Symbol = 14i8,
    TimeStamp = 17i8,
    Undefined = 6i8,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Bson::BsonType =>
    "Newtonsoft.Json.Bson"."BsonType"
);
