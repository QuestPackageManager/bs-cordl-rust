#[cfg(feature = "UnityEngine+UIElements+ColumnsDataType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnsDataType {
    #[default]
    PrimaryColumn = 0i32,
    Reorderable = 2i32,
    Resizable = 3i32,
    ResizePreview = 4i32,
    StretchMode = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+ColumnsDataType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ColumnsDataType =>
    "UnityEngine.UIElements"."ColumnsDataType"
);
