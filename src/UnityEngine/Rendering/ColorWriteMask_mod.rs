#[cfg(feature = "UnityEngine+Rendering+ColorWriteMask")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorWriteMask {
    All = 15i32,
    Alpha = 1i32,
    Blue = 2i32,
    Green = 4i32,
    Red = 8i32,
}
#[cfg(feature = "UnityEngine+Rendering+ColorWriteMask")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::ColorWriteMask =>
    "UnityEngine.Rendering"."ColorWriteMask"
);
