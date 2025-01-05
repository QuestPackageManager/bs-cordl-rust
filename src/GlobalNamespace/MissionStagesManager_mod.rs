#[cfg(feature = "MissionStagesManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionStagesManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionStageLockView: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionStageLockView,
    >,
    pub _missionStages: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::MissionStage>,
    >,
    pub _firstLockedMissionStage: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionStage,
    >,
}
#[cfg(feature = "MissionStagesManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionStagesManager => ""
    ."MissionStagesManager"
);
#[cfg(feature = "MissionStagesManager")]
impl std::ops::Deref for crate::GlobalNamespace::MissionStagesManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionStagesManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionStagesManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionStagesManager")]
impl crate::GlobalNamespace::MissionStagesManager {
    pub fn InitStages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitStages", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn UpdateStageLockPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateStageLockPosition", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
    pub fn get_firstLockedMissionStage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionStage>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionStage,
        > = __cordl_object.invoke("get_firstLockedMissionStage", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionStagesManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MissionStagesManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
