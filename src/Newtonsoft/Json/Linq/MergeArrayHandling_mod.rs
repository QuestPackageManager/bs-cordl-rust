#[cfg(feature = "Newtonsoft+Json+Linq+MergeArrayHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MergeArrayHandling {
    #[default]
    Concat = 0i32,
    Merge = 3i32,
    Replace = 2i32,
    Union = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+Linq+MergeArrayHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::MergeArrayHandling =>
    "Newtonsoft.Json.Linq"."MergeArrayHandling"
);
