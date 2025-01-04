#[cfg(feature = "UnityEngine+UIElements+RenderHints")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderHints {
    #[default]
    BoneTransform = 2i32,
    ClipWithScissors = 4i32,
    DirtyAll = 992i32,
    DirtyBoneTransform = 64i32,
    DirtyClipWithScissors = 128i32,
    DirtyDynamicColor = 512i32,
    DirtyGroupTransform = 32i32,
    DirtyMaskContainer = 256i32,
    DirtyOffset = 5i32,
    DynamicColor = 16i32,
    GroupTransform = 1i32,
    MaskContainer = 8i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+RenderHints")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RenderHints =>
    "UnityEngine.UIElements"."RenderHints"
);
