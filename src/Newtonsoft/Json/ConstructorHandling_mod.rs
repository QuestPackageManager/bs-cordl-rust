#[cfg(feature = "Newtonsoft+Json+ConstructorHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructorHandling {
    AllowNonPublicDefaultConstructor = 1i32,
    Default = 0i32,
}
#[cfg(feature = "Newtonsoft+Json+ConstructorHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::ConstructorHandling =>
    "Newtonsoft.Json"."ConstructorHandling"
);
