#[cfg(feature = "Newtonsoft+Json+JsonToken")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonToken {
    Boolean = 10i32,
    Bytes = 17i32,
    Comment = 5i32,
    Date = 16i32,
    EndArray = 14i32,
    EndConstructor = 15i32,
    EndObject = 13i32,
    Float = 8i32,
    Integer = 7i32,
    None = 0i32,
    Null = 11i32,
    PropertyName = 4i32,
    Raw = 6i32,
    StartArray = 2i32,
    StartConstructor = 3i32,
    StartObject = 1i32,
    String = 9i32,
    Undefined = 12i32,
}
#[cfg(feature = "Newtonsoft+Json+JsonToken")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::JsonToken => "Newtonsoft.Json"
    ."JsonToken"
);
