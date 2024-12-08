#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPackingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlyphPackingMode {
    BestAreaFit = 2i32,
    BestLongSideFit = 1i32,
    BestShortSideFit = 0i32,
    BottomLeftRule = 3i32,
    ContactPointRule = 4i32,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphPackingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::LowLevel::GlyphPackingMode =>
    "UnityEngine.TextCore.LowLevel"."GlyphPackingMode"
);
