#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionProbeMode {
    Baked = 0i32,
    Custom = 2i32,
    Realtime = 1i32,
}
#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::ReflectionProbeMode =>
    "UnityEngine.Rendering"."ReflectionProbeMode"
);
