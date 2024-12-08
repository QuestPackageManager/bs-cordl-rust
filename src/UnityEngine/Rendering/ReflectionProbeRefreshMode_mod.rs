#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeRefreshMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionProbeRefreshMode {
    EveryFrame = 1i32,
    OnAwake = 0i32,
    ViaScripting = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+ReflectionProbeRefreshMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::ReflectionProbeRefreshMode => "UnityEngine.Rendering"
    ."ReflectionProbeRefreshMode"
);
