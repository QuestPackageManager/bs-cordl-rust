#[cfg(feature = "UnityEngine+DrivenTransformProperties")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrivenTransformProperties {
    #[default]
    All = -1i32,
    AnchorMax = 3072i32,
    AnchorMaxX = 1024i32,
    AnchorMaxY = 2048i32,
    AnchorMin = 768i32,
    AnchorMinX = 256i32,
    AnchorMinY = 512i32,
    AnchoredPosition = 6i32,
    AnchoredPosition3D = 14i32,
    AnchoredPositionX = 2i32,
    AnchoredPositionY = 4i32,
    AnchoredPositionZ = 8i32,
    Anchors = 3840i32,
    None = 0i32,
    Pivot = 49152i32,
    PivotX = 16384i32,
    PivotY = 32768i32,
    Rotation = 16i32,
    Scale = 224i32,
    ScaleX = 32i32,
    ScaleY = 64i32,
    ScaleZ = 128i32,
    SizeDelta = 12288i32,
    SizeDeltaX = 4096i32,
    SizeDeltaY = 8192i32,
}
#[cfg(feature = "UnityEngine+DrivenTransformProperties")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::DrivenTransformProperties =>
    "UnityEngine"."DrivenTransformProperties"
);
