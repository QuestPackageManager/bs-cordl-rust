#[cfg(feature = "JetBrains+Annotations+ImplicitUseKindFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImplicitUseKindFlags {
    #[default]
    Access = 1i32,
    Assign = 2i32,
    Default = 7i32,
    InstantiatedNoFixedConstructorSignature = 8i32,
    InstantiatedWithFixedConstructorSignature = 4i32,
}
#[cfg(feature = "JetBrains+Annotations+ImplicitUseKindFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::JetBrains::Annotations::ImplicitUseKindFlags =>
    "JetBrains.Annotations"."ImplicitUseKindFlags"
);
