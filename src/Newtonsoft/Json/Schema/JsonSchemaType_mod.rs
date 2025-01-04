#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JsonSchemaType {
    #[default]
    Any = 127i32,
    Array = 32i32,
    Boolean = 8i32,
    Float = 2i32,
    Integer = 4i32,
    None = 0i32,
    Null = 64i32,
    Object = 16i32,
    String = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+Schema+JsonSchemaType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Schema::JsonSchemaType =>
    "Newtonsoft.Json.Schema"."JsonSchemaType"
);
