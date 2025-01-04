#[cfg(feature = "Newtonsoft+Json+ObjectCreationHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ObjectCreationHandling {
    #[default]
    Auto = 0i32,
    Replace = 2i32,
    Reuse = 1i32,
}
#[cfg(feature = "Newtonsoft+Json+ObjectCreationHandling")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::ObjectCreationHandling =>
    "Newtonsoft.Json"."ObjectCreationHandling"
);
