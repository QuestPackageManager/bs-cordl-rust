#[cfg(feature = "Newtonsoft+Json+NullValueHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NullValueHandling {
    #[default]
    Ignore = 1i32,
    Include = 0i32,
}
#[cfg(feature = "Newtonsoft+Json+NullValueHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::NullValueHandling =>
    "Newtonsoft.Json"."NullValueHandling"
);
