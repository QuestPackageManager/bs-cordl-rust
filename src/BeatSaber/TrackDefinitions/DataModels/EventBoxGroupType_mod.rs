#[cfg(feature = "BeatSaber+TrackDefinitions+DataModels+EventBoxGroupType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventBoxGroupType {
    #[default]
    Color = 0i32,
    FloatFx = 11i32,
    Rotation = 1i32,
    Translation = 2i32,
}
#[cfg(feature = "BeatSaber+TrackDefinitions+DataModels+EventBoxGroupType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::DataModels::EventBoxGroupType =>
    "BeatSaber.TrackDefinitions.DataModels"."EventBoxGroupType"
);
