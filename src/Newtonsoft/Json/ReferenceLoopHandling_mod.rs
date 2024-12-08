#[cfg(feature = "Newtonsoft+Json+ReferenceLoopHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceLoopHandling {
    Error = 0i32,
    Ignore = 1i32,
    Serialize = 2i32,
}
#[cfg(feature = "Newtonsoft+Json+ReferenceLoopHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::ReferenceLoopHandling =>
    "Newtonsoft.Json"."ReferenceLoopHandling"
);
