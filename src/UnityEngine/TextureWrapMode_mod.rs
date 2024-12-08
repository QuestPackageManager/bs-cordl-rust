#[cfg(feature = "UnityEngine+TextureWrapMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureWrapMode {
    Clamp = 1i32,
    Mirror = 2i32,
    MirrorOnce = 3i32,
    Repeat = 0i32,
}
#[cfg(feature = "UnityEngine+TextureWrapMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextureWrapMode => "UnityEngine"
    ."TextureWrapMode"
);
