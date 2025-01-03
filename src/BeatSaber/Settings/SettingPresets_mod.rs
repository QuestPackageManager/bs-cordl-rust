#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingPresets {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::SettingPresets =>
    "BeatSaber.Settings"."SettingPresets"
);
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
        let __cordl_ret: crate::BeatSaber::Settings::AudioSettings = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefaultAudioSettingsWithLatency", (latency))?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultControllerSettings() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Settings::ControllerSettings,
    > {
        let __cordl_ret: crate::BeatSaber::Settings::ControllerSettings = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefaultControllerSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultMiscSettings() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Settings::MiscSettings,
    > {
        let __cordl_ret: crate::BeatSaber::Settings::MiscSettings = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefaultMiscSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultQuestSettings() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Settings::QuestSettings,
    > {
        let __cordl_ret: crate::BeatSaber::Settings::QuestSettings = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefaultQuestSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultSmoothCameraSettings() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Settings::SmoothCameraSettings,
    > {
        let __cordl_ret: crate::BeatSaber::Settings::SmoothCameraSettings = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefaultSmoothCameraSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultWindowSettings() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Settings::WindowSettings,
    > {
        let __cordl_ret: crate::BeatSaber::Settings::WindowSettings = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefaultWindowSettings", ())?;
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
