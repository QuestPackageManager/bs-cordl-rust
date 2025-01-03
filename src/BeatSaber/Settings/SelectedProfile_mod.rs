#[cfg(feature = "BeatSaber+Settings+SelectedProfile")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SelectedProfile {
    pub builtIn: bool,
    pub index: i32,
}
#[cfg(feature = "BeatSaber+Settings+SelectedProfile")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::SelectedProfile =>
    "BeatSaber.Settings"."SelectedProfile"
);
#[cfg(feature = "BeatSaber+Settings+SelectedProfile")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::SelectedProfile {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+SelectedProfile")]
impl crate::BeatSaber::Settings::SelectedProfile {}
