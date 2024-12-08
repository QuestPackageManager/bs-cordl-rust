#[cfg(feature = "UnityEngine+UIElements+ColumnDataType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnDataType {
    CellTemplate = 12i32,
    HeaderTemplate = 11i32,
    Icon = 2i32,
    MaxWidth = 5i32,
    MinWidth = 6i32,
    Name = 0i32,
    Optional = 9i32,
    Resizable = 10i32,
    Sortable = 8i32,
    Stretchable = 7i32,
    Title = 1i32,
    Visibility = 3i32,
    Width = 4i32,
}
#[cfg(feature = "UnityEngine+UIElements+ColumnDataType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ColumnDataType =>
    "UnityEngine.UIElements"."ColumnDataType"
);
