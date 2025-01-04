#[cfg(feature = "BeatmapSaveDataVersion4+EventBoxGroupType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventBoxGroupType {
    #[default]
    Color = 1i32,
    FloatFx = 4i32,
    None = 0i32,
    Rotation = 2i32,
    Translation = 3i32,
}
#[cfg(feature = "BeatmapSaveDataVersion4+EventBoxGroupType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::EventBoxGroupType =>
    "BeatmapSaveDataVersion4"."EventBoxGroupType"
);
