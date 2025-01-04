#[cfg(feature = "UnityEngine+UIElements+DragAndDropPosition")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DragAndDropPosition {
    #[default]
    BetweenItems = 1i32,
    OutsideItems = 2i32,
    OverItem = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+DragAndDropPosition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DragAndDropPosition =>
    "UnityEngine.UIElements"."DragAndDropPosition"
);
