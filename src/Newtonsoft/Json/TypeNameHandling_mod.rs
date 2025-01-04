#[cfg(feature = "Newtonsoft+Json+TypeNameHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeNameHandling {
    #[default]
    All = 3i32,
    Arrays = 2i32,
    Auto = 4i32,
    None = 0i32,
    Objects = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+TypeNameHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::TypeNameHandling =>
    "Newtonsoft.Json"."TypeNameHandling"
);
