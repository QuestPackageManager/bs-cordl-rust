#[cfg(feature = "QuestSettingsApplicatorSO")]
#[repr(C)]
#[derive(Debug)]
pub struct QuestSettingsApplicatorSO {
    __cordl_parent: crate::GlobalNamespace::SettingsApplicatorSO,
}
#[cfg(feature = "QuestSettingsApplicatorSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::QuestSettingsApplicatorSO => ""
    ."QuestSettingsApplicatorSO"
);
#[cfg(feature = "QuestSettingsApplicatorSO")]
impl std::ops::Deref for crate::GlobalNamespace::QuestSettingsApplicatorSO {
    type Target = crate::GlobalNamespace::SettingsApplicatorSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuestSettingsApplicatorSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::QuestSettingsApplicatorSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuestSettingsApplicatorSO")]
impl crate::GlobalNamespace::QuestSettingsApplicatorSO {
    pub fn ApplyPerformancePreset(
        &mut self,
        preset: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::PerformancePresets::PerformancePreset,
        >,
        sceneType: crate::GlobalNamespace::SceneType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyPerformancePreset", (preset, sceneType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "QuestSettingsApplicatorSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuestSettingsApplicatorSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
