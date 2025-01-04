#[cfg(feature = "UnityEngine+SkinWeights")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkinWeights {
    #[default]
    FourBones = 4i32,
    None = 0i32,
    OneBone = 1i32,
    TwoBones = 2i32,
    Unlimited = 255i32,
}
#[cfg(feature = "UnityEngine+SkinWeights")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SkinWeights => "UnityEngine"
    ."SkinWeights"
);
