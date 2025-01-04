#[cfg(feature = "UnityEngine+UIElements+FlexDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexDirection {
    #[default]
    Column = 0i32,
    ColumnReverse = 1i32,
    Row = 2i32,
    RowReverse = 3i32,
}
#[cfg(feature = "UnityEngine+UIElements+FlexDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::FlexDirection =>
    "UnityEngine.UIElements"."FlexDirection"
);
