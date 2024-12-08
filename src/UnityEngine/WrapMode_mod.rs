#[cfg(feature = "UnityEngine+WrapMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapMode {
    Clamp = 1i32,
    ClampForever = 8i32,
    Default = 0i32,
    Loop = 2i32,
    PingPong = 4i32,
}
#[cfg(feature = "UnityEngine+WrapMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::WrapMode => "UnityEngine"
    ."WrapMode"
);
