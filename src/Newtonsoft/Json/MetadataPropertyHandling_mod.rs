#[cfg(feature = "Newtonsoft+Json+MetadataPropertyHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataPropertyHandling {
    Default = 0i32,
    Ignore = 2i32,
    ReadAhead = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+MetadataPropertyHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::MetadataPropertyHandling =>
    "Newtonsoft.Json"."MetadataPropertyHandling"
);
