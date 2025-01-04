#[cfg(feature = "BeatSaber+TrackDefinitions+DataModels+TrackToolbarType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrackToolbarType {
    #[default]
    BtsCharacterSelection = 5i32,
    CarSelection = 6i32,
    FloatValue = 3i32,
    IntValue = 4i32,
    Lights = 0i32,
    None = -1i32,
    Toggle = 1i32,
}
#[cfg(feature = "BeatSaber+TrackDefinitions+DataModels+TrackToolbarType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::TrackDefinitions::DataModels::TrackToolbarType =>
    "BeatSaber.TrackDefinitions.DataModels"."TrackToolbarType"
);
