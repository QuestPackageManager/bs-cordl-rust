#[cfg(feature = "UnityEngine+Rendering+GraphicsTier")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsTier {
    Tier1 = 0i32,
    Tier2 = 1i32,
    Tier3 = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+GraphicsTier")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::GraphicsTier =>
    "UnityEngine.Rendering"."GraphicsTier"
);
