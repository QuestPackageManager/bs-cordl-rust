#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphRenderMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlyphRenderMode {
    COLOR = 69652i32,
    COLOR_HINTED = 69656i32,
    RASTER = 4118i32,
    RASTER_HINTED = 4122i32,
    SDF = 4134i32,
    SDF16 = 16422i32,
    SDF32 = 32806i32,
    SDF8 = 8230i32,
    SDFAA = 4165i32,
    SDFAA_HINTED = 4169i32,
    SMOOTH = 4117i32,
    SMOOTH_HINTED = 4121i32,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+GlyphRenderMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::LowLevel::GlyphRenderMode
    => "UnityEngine.TextCore.LowLevel"."GlyphRenderMode"
);
