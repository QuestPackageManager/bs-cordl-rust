#[cfg(feature = "UnityEngine+Rendering+ShadowSamplingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShadowSamplingMode {
    #[default]
    CompareDepths = 0i32,
    None = 2i32,
    RawDepth = 1i32,
}
#[cfg(feature = "UnityEngine+Rendering+ShadowSamplingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::ShadowSamplingMode =>
    "UnityEngine.Rendering"."ShadowSamplingMode"
);
