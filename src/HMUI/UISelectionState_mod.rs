#[cfg(feature = "HMUI+UISelectionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UISelectionState {
    #[default]
    Disabled = 5i32,
    Highlighted = 1i32,
    Normal = 0i32,
    Pressed = 2i32,
    Selected = 3i32,
    SelectedAndHighlighted = 4i32,
}
#[cfg(feature = "HMUI+UISelectionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::UISelectionState => "HMUI"
    ."UISelectionState"
);
