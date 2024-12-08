#[cfg(feature = "Newtonsoft+Json+DefaultValueHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultValueHandling {
    Ignore = 1i32,
    IgnoreAndPopulate = 3i32,
    Include = 0i32,
    Populate = 2i32,
}
#[cfg(feature = "Newtonsoft+Json+DefaultValueHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::DefaultValueHandling =>
    "Newtonsoft.Json"."DefaultValueHandling"
);
