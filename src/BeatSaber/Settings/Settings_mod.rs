#[cfg(feature = "BeatSaber+Settings+Settings")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Settings {
    pub room: crate::BeatSaber::Settings::RoomSettings,
    pub controller: crate::BeatSaber::Settings::ControllerSettings,
    pub smoothCamera: crate::BeatSaber::Settings::SmoothCameraSettings,
    pub audio: crate::BeatSaber::Settings::AudioSettings,
    pub misc: crate::BeatSaber::Settings::MiscSettings,
    pub quality: crate::BeatSaber::Settings::QualitySettings,
    pub quest: crate::BeatSaber::Settings::QuestSettings,
    pub window: crate::BeatSaber::Settings::WindowSettings,
    pub customServer: crate::BeatSaber::Settings::CustomServerSettings,
    pub performanceTools: crate::BeatSaber::Settings::PerformanceToolSettings,
    pub debug: crate::BeatSaber::Settings::DebugSettings,
}
#[cfg(feature = "BeatSaber+Settings+Settings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::Settings =>
    "BeatSaber.Settings"."Settings"
);
#[cfg(feature = "BeatSaber+Settings+Settings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::Settings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+Settings")]
impl crate::BeatSaber::Settings::Settings {}
