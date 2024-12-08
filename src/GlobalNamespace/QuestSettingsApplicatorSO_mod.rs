#[cfg(feature = "QuestSettingsApplicatorSO")]
#[repr(C)]
#[derive(Debug)]
pub struct QuestSettingsApplicatorSO {
    __cordl_parent: SettingsApplicatorSO,
}
#[cfg(feature = "QuestSettingsApplicatorSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for QuestSettingsApplicatorSO => ""
    ."QuestSettingsApplicatorSO"
);
#[cfg(feature = "QuestSettingsApplicatorSO")]
impl std::ops::Deref for QuestSettingsApplicatorSO {
    type Target = SettingsApplicatorSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuestSettingsApplicatorSO")]
impl std::ops::DerefMut for QuestSettingsApplicatorSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuestSettingsApplicatorSO")]
impl QuestSettingsApplicatorSO {
    pub fn ApplyPerformancePreset(
        &mut self,
        preset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
        sceneType: SceneType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyPerformancePreset", (preset, sceneType))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "QuestSettingsApplicatorSO")]
impl quest_hook::libil2cpp::ObjectType for QuestSettingsApplicatorSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
