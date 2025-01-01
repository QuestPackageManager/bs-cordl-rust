#[cfg(feature = "BeatSaber+Settings+MiscSettings")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MiscSettings {
    pub language: *mut quest_hook::libil2cpp::Il2CppString,
    pub pauseButtonResponse: crate::BeatSaber::Settings::MiscSettings_ResponseLevel,
}
#[cfg(feature = "BeatSaber+Settings+MiscSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::MiscSettings =>
    "BeatSaber.Settings"."MiscSettings"
);
#[cfg(feature = "BeatSaber+Settings+MiscSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::MiscSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+MiscSettings")]
impl crate::BeatSaber::Settings::MiscSettings {
    #[cfg(feature = "BeatSaber+Settings+MiscSettings+ResponseLevel")]
    pub type ResponseLevel = crate::BeatSaber::Settings::MiscSettings_ResponseLevel;
}
#[cfg(feature = "BeatSaber+Settings+MiscSettings+ResponseLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiscSettings_ResponseLevel {
    Instant = 0i32,
    Long = 1i32,
}
#[cfg(feature = "BeatSaber+Settings+MiscSettings+ResponseLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::MiscSettings_ResponseLevel
    => "BeatSaber.Settings"."MiscSettings/ResponseLevel"
);
