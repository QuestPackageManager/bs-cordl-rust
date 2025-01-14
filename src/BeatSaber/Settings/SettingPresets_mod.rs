#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingPresets {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Settings::SettingPresets {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Settings";
    const CLASS_NAME: &'static str = "SettingPresets";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
impl std::ops::Deref for crate::BeatSaber::Settings::SettingPresets {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
impl std::ops::DerefMut for crate::BeatSaber::Settings::SettingPresets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
impl crate::BeatSaber::Settings::SettingPresets {
    pub fn DefaultAudioSettingsWithLatency(
        latency: f32,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Settings::AudioSettings> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                crate::BeatSaber::Settings::AudioSettings,
                1usize,
            >("DefaultAudioSettingsWithLatency")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DefaultAudioSettingsWithLatency", 1usize
                )
            });
        let __cordl_ret: crate::BeatSaber::Settings::AudioSettings = unsafe {
            method.invoke_unchecked((), (latency))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultControllerSettings() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Settings::ControllerSettings,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::BeatSaber::Settings::ControllerSettings,
                0usize,
            >("DefaultControllerSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DefaultControllerSettings", 0usize
                )
            });
        let __cordl_ret: crate::BeatSaber::Settings::ControllerSettings = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultMiscSettings() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Settings::MiscSettings,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::BeatSaber::Settings::MiscSettings,
                0usize,
            >("DefaultMiscSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DefaultMiscSettings", 0usize
                )
            });
        let __cordl_ret: crate::BeatSaber::Settings::MiscSettings = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultQuestSettings() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Settings::QuestSettings,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::BeatSaber::Settings::QuestSettings,
                0usize,
            >("DefaultQuestSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DefaultQuestSettings", 0usize
                )
            });
        let __cordl_ret: crate::BeatSaber::Settings::QuestSettings = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultSmoothCameraSettings() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Settings::SmoothCameraSettings,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::BeatSaber::Settings::SmoothCameraSettings,
                0usize,
            >("DefaultSmoothCameraSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DefaultSmoothCameraSettings", 0usize
                )
            });
        let __cordl_ret: crate::BeatSaber::Settings::SmoothCameraSettings = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultWindowSettings() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Settings::WindowSettings,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::BeatSaber::Settings::WindowSettings,
                0usize,
            >("DefaultWindowSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DefaultWindowSettings", 0usize
                )
            });
        let __cordl_ret: crate::BeatSaber::Settings::WindowSettings = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Settings::SettingPresets {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
