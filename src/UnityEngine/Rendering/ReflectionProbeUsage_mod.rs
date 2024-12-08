#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeUsage")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionProbeUsage {
    BlendProbes = 1i32,
    BlendProbesAndSkybox = 2i32,
    Off = 0i32,
    Simple = 3i32,
}
#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeUsage")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::ReflectionProbeUsage =>
    "UnityEngine.Rendering"."ReflectionProbeUsage"
);
