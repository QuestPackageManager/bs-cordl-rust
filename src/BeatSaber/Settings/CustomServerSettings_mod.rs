#[cfg(feature = "BeatSaber+Settings+CustomServerSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CustomServerSettings {
    pub useCustomEnvironment: bool,
    pub forceGameLiftEnvironment: bool,
    pub hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "BeatSaber+Settings+CustomServerSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::CustomServerSettings =>
    "BeatSaber.Settings"."CustomServerSettings"
);
#[cfg(feature = "BeatSaber+Settings+CustomServerSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::CustomServerSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+CustomServerSettings")]
impl crate::BeatSaber::Settings::CustomServerSettings {}
