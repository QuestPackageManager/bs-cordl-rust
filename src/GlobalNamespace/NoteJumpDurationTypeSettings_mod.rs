#[cfg(feature = "NoteJumpDurationTypeSettings")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteJumpDurationTypeSettings {
    Dynamic = 0i32,
    Static = 1i32,
}
#[cfg(feature = "NoteJumpDurationTypeSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for NoteJumpDurationTypeSettings => ""
    ."NoteJumpDurationTypeSettings"
);
