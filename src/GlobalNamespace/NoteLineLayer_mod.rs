#[cfg(feature = "NoteLineLayer")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteLineLayer {
    #[default]
    Base = 0i32,
    Top = 2i32,
    Upper = 1i32,
}
#[cfg(feature = "NoteLineLayer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteLineLayer => ""
    ."NoteLineLayer"
);
