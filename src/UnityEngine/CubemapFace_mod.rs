#[cfg(feature = "UnityEngine+CubemapFace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubemapFace {
    NegativeX = 1i32,
    NegativeY = 3i32,
    NegativeZ = 5i32,
    PositiveX = 0i32,
    PositiveY = 2i32,
    PositiveZ = 4i32,
    Unknown = -1i32,
}
#[cfg(feature = "UnityEngine+CubemapFace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CubemapFace => "UnityEngine"
    ."CubemapFace"
);
