#[cfg(feature = "LIV+SDK+Unity+FEATURES")]
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FEATURES {
    BACKGROUND_DEPTH_RENDER = 8u64,
    BACKGROUND_RENDER = 1u64,
    COMPLEX_CLIP_PLANE = 4u64,
    DEBUG_CLIP_PLANE = 281474976710656u64,
    FIX_FOREGROUND_ALPHA = 32u64,
    FOREGROUND_RENDER = 2u64,
    GROUND_CLIP_PLANE = 64u64,
    INTERLACED_RENDER = 536870912u64,
    NONE = 0u64,
    OPTIMIZED_RENDER = 268435456u64,
    OVERRIDE_POST_PROCESSING = 16u64,
    RELEASE_CONTROL = 32768u64,
}
#[cfg(feature = "LIV+SDK+Unity+FEATURES")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::FEATURES => "LIV.SDK.Unity"
    ."FEATURES"
);
