#[cfg(feature = "Newtonsoft+Json+Linq+MergeNullValueHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MergeNullValueHandling {
    #[default]
    Ignore = 0i32,
    Merge = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+Linq+MergeNullValueHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::MergeNullValueHandling
    => "Newtonsoft.Json.Linq"."MergeNullValueHandling"
);
