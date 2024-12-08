#[cfg(feature = "UnityEngine+MaterialGlobalIlluminationFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialGlobalIlluminationFlags {
    AnyEmissive = 3i32,
    BakedEmissive = 2i32,
    EmissiveIsBlack = 4i32,
    None = 0i32,
    RealtimeEmissive = 1i32,
}
#[cfg(feature = "UnityEngine+MaterialGlobalIlluminationFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::MaterialGlobalIlluminationFlags =>
    "UnityEngine"."MaterialGlobalIlluminationFlags"
);
