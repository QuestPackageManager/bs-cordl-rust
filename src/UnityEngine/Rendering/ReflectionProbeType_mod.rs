#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReflectionProbeType {
    #[default]
    Card = 1i32,
    Cube = 0i32,
}
#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::ReflectionProbeType =>
    "UnityEngine.Rendering"."ReflectionProbeType"
);
