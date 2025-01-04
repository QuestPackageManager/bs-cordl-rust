#[cfg(feature = "Newtonsoft+Json+PreserveReferencesHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreserveReferencesHandling {
    #[default]
    All = 3i32,
    Arrays = 2i32,
    None = 0i32,
    Objects = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+PreserveReferencesHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::PreserveReferencesHandling =>
    "Newtonsoft.Json"."PreserveReferencesHandling"
);
