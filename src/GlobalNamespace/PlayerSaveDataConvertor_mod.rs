#[cfg(feature = "PlayerSaveDataConvertor")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataConvertor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PlayerSaveDataConvertor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSaveDataConvertor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSaveDataConvertor";
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
#[cfg(feature = "PlayerSaveDataConvertor")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveDataConvertor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataConvertor")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveDataConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataConvertor")]
impl crate::GlobalNamespace::PlayerSaveDataConvertor {
    pub fn GetRuntimeData_PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData2(
        saveData: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ArcVisibilityType> {
        let __cordl_ret: crate::GlobalNamespace::ArcVisibilityType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRuntimeData", (saveData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRuntimeData_PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData0(
        saveData: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    > {
        let __cordl_ret: crate::GlobalNamespace::EnvironmentEffectsFilterPreset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRuntimeData", (saveData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRuntimeData_PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData1(
        saveData: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::NoteJumpDurationTypeSettings,
    > {
        let __cordl_ret: crate::GlobalNamespace::NoteJumpDurationTypeSettings = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRuntimeData", (saveData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSaveData_ArcVisibilityType2(
        data: crate::GlobalNamespace::ArcVisibilityType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData,
    > {
        let __cordl_ret: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_ArcVisibilityTypeSaveData = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSaveData", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSaveData_EnvironmentEffectsFilterPreset0(
        data: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData,
    > {
        let __cordl_ret: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_EnvironmentEffectsFilterPresetSaveData = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSaveData", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSaveData_NoteJumpDurationTypeSettings1(
        data: crate::GlobalNamespace::NoteJumpDurationTypeSettings,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData,
    > {
        let __cordl_ret: crate::GlobalNamespace::PlayerSpecificSettings_PlayerSaveData_NoteJumpDurationTypeSettingsSaveData = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSaveData", (data))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerSaveDataConvertor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
