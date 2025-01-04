#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContractType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JsonContractType {
    #[default]
    Array = 2i32,
    Dictionary = 5i32,
    Dynamic = 6i32,
    Linq = 8i32,
    None = 0i32,
    Object = 1i32,
    Primitive = 3i32,
    Serializable = 7i32,
    String = 4i32,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContractType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonContractType =>
    "Newtonsoft.Json.Serialization"."JsonContractType"
);
