#[cfg(feature = "UnityEngine+Rendering+DefaultReflectionMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultReflectionMode {
    Custom = 1i32,
    Skybox = 0i32,
}
#[cfg(feature = "UnityEngine+Rendering+DefaultReflectionMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::DefaultReflectionMode =>
    "UnityEngine.Rendering"."DefaultReflectionMode"
);
