#[cfg(feature = "BeatmapDifficultyMask")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeatmapDifficultyMask {
    All = 31u8,
    Easy = 1u8,
    Expert = 8u8,
    ExpertPlus = 16u8,
    Hard = 4u8,
    Normal = 2u8,
}
#[cfg(feature = "BeatmapDifficultyMask")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDifficultyMask => ""
    ."BeatmapDifficultyMask"
);
