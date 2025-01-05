#[cfg(feature = "BeatSaber+Settings+SettingValidations")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingValidations {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+Settings+SettingValidations")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::SettingValidations =>
    "BeatSaber.Settings"."SettingValidations"
);
#[cfg(feature = "BeatSaber+Settings+SettingValidations")]
impl std::ops::Deref for crate::BeatSaber::Settings::SettingValidations {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingValidations")]
impl std::ops::DerefMut for crate::BeatSaber::Settings::SettingValidations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingValidations")]
impl crate::BeatSaber::Settings::SettingValidations {
    pub fn AdjustAudioSettings(
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::BeatSaber::Settings::AudioSettings,
        >,
        presetLatency: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustAudioSettings", (settings, presetLatency))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustControllerSettings(
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::BeatSaber::Settings::ControllerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustControllerSettings", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustPlayStation4(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustPlayStation4", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustPlayStation4Pro(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustPlayStation4Pro", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustPlayStation5(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustPlayStation5", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustQuest1(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustQuest1", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustQuest2(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustQuest2", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustQuest3(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustQuest3", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustQuestPro(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustQuestPro", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustRoomSettings(
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::BeatSaber::Settings::RoomSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustRoomSettings", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustSettings(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        presetLatency: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustSettings", (settings, presetLatency))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustSmoothCameraSettings(
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::BeatSaber::Settings::SmoothCameraSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustSmoothCameraSettings", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustStandalone(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        forceApplyQualityAll: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustStandalone", (settings, forceApplyQualityAll))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clamp(
        value: crate::Unity::Mathematics::float3,
        min: f32,
        max: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clamp", (value, min, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn Pick(value: i32, min: i32, max: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Pick", (value, min, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn Wrap_f32_1(
        value: f32,
        min: f32,
        max: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Wrap", (value, min, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn Wrap_float3_0(
        value: crate::Unity::Mathematics::float3,
        min: f32,
        max: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Wrap", (value, min, max))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingValidations")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Settings::SettingValidations {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
