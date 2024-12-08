#[cfg(feature = "UnityEngine+AudioVelocityUpdateMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioVelocityUpdateMode {
    Auto = 0i32,
    Dynamic = 2i32,
    Fixed = 1i32,
}
#[cfg(feature = "UnityEngine+AudioVelocityUpdateMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AudioVelocityUpdateMode =>
    "UnityEngine"."AudioVelocityUpdateMode"
);
