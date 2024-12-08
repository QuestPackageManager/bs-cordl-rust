#[cfg(feature = "BeatmapDifficulty")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeatmapDifficulty {
    Easy = 0i32,
    Expert = 3i32,
    ExpertPlus = 4i32,
    Hard = 2i32,
    Normal = 1i32,
}
#[cfg(feature = "BeatmapDifficulty")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for BeatmapDifficulty => ""."BeatmapDifficulty"
);
