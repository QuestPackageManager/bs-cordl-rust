#[cfg(feature = "UnityEngine+ForceMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ForceMode {
    #[default]
    Acceleration = 5i32,
    Force = 0i32,
    Impulse = 1i32,
    VelocityChange = 2i32,
}
#[cfg(feature = "UnityEngine+ForceMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ForceMode => "UnityEngine"
    ."ForceMode"
);
