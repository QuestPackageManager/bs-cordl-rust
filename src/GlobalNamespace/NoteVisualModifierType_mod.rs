#[cfg(feature = "NoteVisualModifierType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteVisualModifierType {
    DisappearingArrow = 1i32,
    Ghost = 2i32,
    Normal = 0i32,
}
#[cfg(feature = "NoteVisualModifierType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteVisualModifierType => ""
    ."NoteVisualModifierType"
);
