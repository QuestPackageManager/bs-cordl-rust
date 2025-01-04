#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphLoadFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GlyphLoadFlags {
    #[default]
    LOAD_BITMAP_METRICS_ONLY = 4194304i32,
    LOAD_COLOR = 1048576i32,
    LOAD_COMPUTE_METRICS = 2097152i32,
    LOAD_DEFAULT = 0i32,
    LOAD_FORCE_AUTOHINT = 32i32,
    LOAD_MONOCHROME = 4096i32,
    LOAD_NO_AUTOHINT = 32768i32,
    LOAD_NO_BITMAP = 8i32,
    LOAD_NO_HINTING = 2i32,
    LOAD_NO_SCALE = 1i32,
    LOAD_RENDER = 4i32,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphLoadFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::LowLevel::GlyphLoadFlags
    => "UnityEngine.TextCore.LowLevel"."GlyphLoadFlags"
);
