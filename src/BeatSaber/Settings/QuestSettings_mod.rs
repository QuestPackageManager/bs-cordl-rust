#[cfg(feature = "BeatSaber+Settings+QuestSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct QuestSettings {
    pub cpuLevel: crate::BeatSaber::Settings::QuestSettings_SuggestedPerformanceLevel,
    pub gpuLevel: crate::BeatSaber::Settings::QuestSettings_SuggestedPerformanceLevel,
    pub foveatedRenderingMenu: crate::BeatSaber::Settings::QuestSettings_FoveatedRenderingLevel,
    pub foveatedRenderingGameplay: crate::BeatSaber::Settings::QuestSettings_FoveatedRenderingLevel,
    pub eyeTrackedFoveatedRendering: bool,
    pub dynamicFoveatedRendering: bool,
}
#[cfg(feature = "BeatSaber+Settings+QuestSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::QuestSettings =>
    "BeatSaber.Settings"."QuestSettings"
);
#[cfg(feature = "BeatSaber+Settings+QuestSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::QuestSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+QuestSettings")]
impl crate::BeatSaber::Settings::QuestSettings {
    #[cfg(feature = "BeatSaber+Settings+QuestSettings+FoveatedRenderingLevel")]
    pub type FoveatedRenderingLevel = crate::BeatSaber::Settings::QuestSettings_FoveatedRenderingLevel;
    #[cfg(feature = "BeatSaber+Settings+QuestSettings+SuggestedPerformanceLevel")]
    pub type SuggestedPerformanceLevel = crate::BeatSaber::Settings::QuestSettings_SuggestedPerformanceLevel;
}
#[cfg(feature = "BeatSaber+Settings+QuestSettings+FoveatedRenderingLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestSettings_FoveatedRenderingLevel {
    High = 3i32,
    HighTop = 4i32,
    Low = 1i32,
    Medium = 2i32,
    Off = 0i32,
}
#[cfg(feature = "BeatSaber+Settings+QuestSettings+FoveatedRenderingLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::QuestSettings_FoveatedRenderingLevel => "BeatSaber.Settings"
    ."QuestSettings/FoveatedRenderingLevel"
);
#[cfg(feature = "BeatSaber+Settings+QuestSettings+SuggestedPerformanceLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestSettings_SuggestedPerformanceLevel {
    Boost = 4i32,
    Default = 0i32,
    PowerSavings = 1i32,
    SustainedHigh = 3i32,
    SustainedLow = 2i32,
}
#[cfg(feature = "BeatSaber+Settings+QuestSettings+SuggestedPerformanceLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::Settings::QuestSettings_SuggestedPerformanceLevel =>
    "BeatSaber.Settings"."QuestSettings/SuggestedPerformanceLevel"
);
