#[cfg(feature = "BeatSaber+Settings+Settings")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Settings::Settings {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Settings";
    const CLASS_NAME: &'static str = "Settings";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "BeatSaber+Settings+Settings")]
unsafe impl quest_hook::libil2cpp::Argument for crate::BeatSaber::Settings::Settings {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatSaber+Settings+Settings")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::BeatSaber::Settings::Settings {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "BeatSaber+Settings+Settings")]
unsafe impl quest_hook::libil2cpp::Returned for crate::BeatSaber::Settings::Settings {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "BeatSaber+Settings+Settings")]
unsafe impl quest_hook::libil2cpp::Return for crate::BeatSaber::Settings::Settings {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
