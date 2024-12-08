#[cfg(feature = "LevelScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelScenesTransitionSetupDataSO {
    __cordl_parent: ScenesTransitionSetupDataSO,
    pub _gameplayCoreSceneSetupData_k__BackingField: *mut GameplayCoreSceneSetupData,
}
#[cfg(feature = "LevelScenesTransitionSetupDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LevelScenesTransitionSetupDataSO => ""
    ."LevelScenesTransitionSetupDataSO"
);
#[cfg(feature = "LevelScenesTransitionSetupDataSO")]
impl std::ops::Deref for LevelScenesTransitionSetupDataSO {
    type Target = ScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelScenesTransitionSetupDataSO")]
impl std::ops::DerefMut for LevelScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelScenesTransitionSetupDataSO")]
impl LevelScenesTransitionSetupDataSO {
    pub fn BeforeScenesWillBeActivated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeforeScenesWillBeActivated", ())?;
        Ok(__cordl_ret)
    }
    pub fn BeforeScenesWillBeActivatedAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("BeforeScenesWillBeActivatedAsync", ())?;
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
    pub fn get_gameplayCoreSceneSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut GameplayCoreSceneSetupData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut GameplayCoreSceneSetupData = __cordl_object
            .invoke("get_gameplayCoreSceneSetupData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_transformedBeatmapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IReadonlyBeatmapData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IReadonlyBeatmapData = __cordl_object
            .invoke("get_transformedBeatmapData", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_gameplayCoreSceneSetupData(
        &mut self,
        value: *mut GameplayCoreSceneSetupData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gameplayCoreSceneSetupData", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LevelScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType for LevelScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}