#[cfg(feature = "UnityEngine+GradientMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GradientMode {
    #[default]
    Blend = 0i32,
    Fixed = 1i32,
    PerceptualBlend = 2i32,
}
#[cfg(feature = "UnityEngine+GradientMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GradientMode => "UnityEngine"
    ."GradientMode"
);
