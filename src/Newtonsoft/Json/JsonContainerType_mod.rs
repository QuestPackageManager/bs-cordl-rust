#[cfg(feature = "Newtonsoft+Json+JsonContainerType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JsonContainerType {
    #[default]
    Array = 2i32,
    Constructor = 3i32,
    None = 0i32,
    Object = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+JsonContainerType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::JsonContainerType =>
    "Newtonsoft.Json"."JsonContainerType"
);
