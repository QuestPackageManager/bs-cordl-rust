#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+ObstacleType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObstacleType {
    Free = 2i32,
    FullHeight = 0i32,
    Top = 1i32,
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+ObstacleType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleType =>
    "BeatmapSaveDataVersion2_6_0AndEarlier"."ObstacleType"
);
