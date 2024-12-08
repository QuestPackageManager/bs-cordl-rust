#[cfg(feature = "UnityEngine+UIElements+TransformOriginOffset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformOriginOffset {
    Bottom = 4i32,
    Center = 5i32,
    Left = 1i32,
    Right = 2i32,
    Top = 3i32,
}
#[cfg(feature = "UnityEngine+UIElements+TransformOriginOffset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TransformOriginOffset
    => "UnityEngine.UIElements"."TransformOriginOffset"
);
