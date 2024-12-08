#[cfg(feature = "MissionStagesManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionStagesManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionStageLockView: *mut MissionStageLockView,
    pub _missionStages: *mut quest_hook::libil2cpp::Il2CppArray<*mut MissionStage>,
    pub _firstLockedMissionStage: *mut MissionStage,
}
#[cfg(feature = "MissionStagesManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionStagesManager => ""."MissionStagesManager"
);
#[cfg(feature = "MissionStagesManager")]
impl std::ops::Deref for MissionStagesManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionStagesManager")]
impl std::ops::DerefMut for MissionStagesManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionStagesManager")]
impl MissionStagesManager {
    #[cfg(feature = "MissionStagesManager+__c")]
    pub type __c = crate::GlobalNamespace::MissionStagesManager___c;
    pub fn InitStages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitStages", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UpdateFirtsLockedMissionStage(
        &mut self,
        numberOfClearedMissions: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateFirtsLockedMissionStage", (numberOfClearedMissions))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateStageLockPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateStageLockPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateStageLockPositionAnimated(
        &mut self,
        animated: bool,
        animationDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateStageLockPositionAnimated", (animated, animationDuration))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateStageLockText(
        &mut self,
        numberOfClearedMissions: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateStageLockText", (numberOfClearedMissions))?;
        Ok(__cordl_ret)
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
    pub fn get_firstLockedMissionStage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MissionStage> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionStage = __cordl_object
            .invoke("get_firstLockedMissionStage", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissionStagesManager")]
impl quest_hook::libil2cpp::ObjectType for MissionStagesManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
