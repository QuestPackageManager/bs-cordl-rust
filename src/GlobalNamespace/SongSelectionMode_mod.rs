#[cfg(feature = "SongSelectionMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SongSelectionMode {
    OwnerPicks = 2i32,
    Random = 1i32,
    RandomPlayerPicks = 3i32,
    Vote = 0i32,
}
#[cfg(feature = "SongSelectionMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for SongSelectionMode => ""."SongSelectionMode"
);
