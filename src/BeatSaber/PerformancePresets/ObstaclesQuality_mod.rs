#[cfg(feature = "BeatSaber+PerformancePresets+ObstaclesQuality")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObstaclesQuality {
    DefaultObstacleQualityBaseOnDisplacement = 0i32,
    ObstacleHW = 3i32,
    ObstacleLW = 2i32,
    TexturedObstacle = 1i32,
}
#[cfg(feature = "BeatSaber+PerformancePresets+ObstaclesQuality")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::PerformancePresets::ObstaclesQuality
    => "BeatSaber.PerformancePresets"."ObstaclesQuality"
);
