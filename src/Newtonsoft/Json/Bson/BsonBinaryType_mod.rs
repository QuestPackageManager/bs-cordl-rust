#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinaryType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BsonBinaryType {
    Binary = 0u8,
    BinaryOld = 2u8,
    Function = 1u8,
    Md5 = 5u8,
    UserDefined = 128u8,
    Uuid = 4u8,
    UuidOld = 3u8,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinaryType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Bson::BsonBinaryType =>
    "Newtonsoft.Json.Bson"."BsonBinaryType"
);
