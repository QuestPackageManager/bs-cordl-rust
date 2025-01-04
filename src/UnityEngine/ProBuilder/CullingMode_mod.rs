#[cfg(feature = "UnityEngine+ProBuilder+CullingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CullingMode {
    #[default]
    Back = 1i32,
    Front = 2i32,
    FrontBack = 3i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+CullingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::CullingMode =>
    "UnityEngine.ProBuilder"."CullingMode"
);
