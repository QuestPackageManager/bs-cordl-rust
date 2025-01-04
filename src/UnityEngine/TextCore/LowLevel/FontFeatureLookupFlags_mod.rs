#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontFeatureLookupFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontFeatureLookupFlags {
    #[default]
    IgnoreLigatures = 4i32,
    IgnoreSpacingAdjustments = 256i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontFeatureLookupFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::LowLevel::FontFeatureLookupFlags =>
    "UnityEngine.TextCore.LowLevel"."FontFeatureLookupFlags"
);
