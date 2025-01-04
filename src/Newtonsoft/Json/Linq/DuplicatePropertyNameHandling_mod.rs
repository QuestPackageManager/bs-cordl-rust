#[cfg(feature = "Newtonsoft+Json+Linq+DuplicatePropertyNameHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DuplicatePropertyNameHandling {
    #[default]
    Error = 2i32,
    Ignore = 1i32,
    Replace = 0i32,
}
#[cfg(feature = "Newtonsoft+Json+Linq+DuplicatePropertyNameHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling => "Newtonsoft.Json.Linq"
    ."DuplicatePropertyNameHandling"
);
