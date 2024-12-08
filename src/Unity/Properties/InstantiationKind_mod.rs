#[cfg(feature = "Unity+Properties+InstantiationKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstantiationKind {
    Activator = 0i32,
    NotInstantiatable = 2i32,
    PropertyBagOverride = 1i32,
}
#[cfg(feature = "Unity+Properties+InstantiationKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::InstantiationKind =>
    "Unity.Properties"."InstantiationKind"
);
