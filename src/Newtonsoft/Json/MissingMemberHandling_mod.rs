#[cfg(feature = "Newtonsoft+Json+MissingMemberHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MissingMemberHandling {
    #[default]
    Error = 1i32,
    Ignore = 0i32,
}
#[cfg(feature = "Newtonsoft+Json+MissingMemberHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::MissingMemberHandling =>
    "Newtonsoft.Json"."MissingMemberHandling"
);
