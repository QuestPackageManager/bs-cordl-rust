#[cfg(feature = "UnityEngine+RenderTextureFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderTextureFormat {
    #[default]
    ARGB1555 = 6i32,
    ARGB2101010 = 8i32,
    ARGB32 = 0i32,
    ARGB4444 = 5i32,
    ARGB64 = 10i32,
    ARGBFloat = 11i32,
    ARGBHalf = 2i32,
    ARGBInt = 17i32,
    BGR101010_XR = 27i32,
    BGRA10101010_XR = 26i32,
    BGRA32 = 20i32,
    Default = 7i32,
    DefaultHDR = 9i32,
    Depth = 1i32,
    R16 = 28i32,
    R8 = 16i32,
    RFloat = 14i32,
    RG16 = 25i32,
    RG32 = 23i32,
    RGB111110Float = 22i32,
    RGB565 = 4i32,
    RGBAUShort = 24i32,
    RGFloat = 12i32,
    RGHalf = 13i32,
    RGInt = 18i32,
    RHalf = 15i32,
    RInt = 19i32,
    Shadowmap = 3i32,
}
#[cfg(feature = "UnityEngine+RenderTextureFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RenderTextureFormat =>
    "UnityEngine"."RenderTextureFormat"
);
