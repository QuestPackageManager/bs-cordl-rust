#[cfg(feature = "BeatSaberSessionEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BeatSaberSessionEventType {
    #[default]
    SessionFinish = 2i32,
    SessionStart = 1i32,
}
#[cfg(feature = "BeatSaberSessionEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatSaberSessionEventType => ""
    ."BeatSaberSessionEventType"
);
