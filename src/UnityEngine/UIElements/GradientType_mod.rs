#[cfg(feature = "UnityEngine+UIElements+GradientType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GradientType {
    #[default]
    Linear = 0i32,
    Radial = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+GradientType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::GradientType =>
    "UnityEngine.UIElements"."GradientType"
);
