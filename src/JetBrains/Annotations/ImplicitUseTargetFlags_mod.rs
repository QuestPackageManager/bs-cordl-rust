#[cfg(feature = "JetBrains+Annotations+ImplicitUseTargetFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImplicitUseTargetFlags {
    #[default]
    Default = 1i32,
    Members = 2i32,
    WithMembers = 3i32,
}
#[cfg(feature = "JetBrains+Annotations+ImplicitUseTargetFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::JetBrains::Annotations::ImplicitUseTargetFlags
    => "JetBrains.Annotations"."ImplicitUseTargetFlags"
);
