#[cfg(feature = "UnityEngine+Rendering+AmbientMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AmbientMode {
    #[default]
    Custom = 4i32,
    Flat = 3i32,
    Skybox = 0i32,
    Trilight = 1i32,
}
#[cfg(feature = "UnityEngine+Rendering+AmbientMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::AmbientMode =>
    "UnityEngine.Rendering"."AmbientMode"
);
