#[cfg(feature = "UnityEngine+RenderMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    ScreenSpaceCamera = 1i32,
    ScreenSpaceOverlay = 0i32,
    WorldSpace = 2i32,
}
#[cfg(feature = "UnityEngine+RenderMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RenderMode => "UnityEngine"
    ."RenderMode"
);
