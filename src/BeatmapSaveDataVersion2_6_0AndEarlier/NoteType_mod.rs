#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+NoteType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteType {
    #[default]
    Bomb = 3i32,
    GhostNote = 2i32,
    None = -1i32,
    NoteA = 0i32,
    NoteB = 1i32,
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+NoteType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion2_6_0AndEarlier::NoteType
    => "BeatmapSaveDataVersion2_6_0AndEarlier"."NoteType"
);
