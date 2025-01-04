#[cfg(feature = "Newtonsoft+Json+Linq+JTokenType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JTokenType {
    #[default]
    Array = 2i32,
    Boolean = 9i32,
    Bytes = 14i32,
    Comment = 5i32,
    Constructor = 3i32,
    Date = 12i32,
    Float = 7i32,
    Guid = 15i32,
    Integer = 6i32,
    None = 0i32,
    Null = 10i32,
    Object = 1i32,
    Property = 4i32,
    Raw = 13i32,
    String = 8i32,
    TimeSpan = 17i32,
    Undefined = 11i32,
    Uri = 16i32,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JTokenType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::JTokenType =>
    "Newtonsoft.Json.Linq"."JTokenType"
);
