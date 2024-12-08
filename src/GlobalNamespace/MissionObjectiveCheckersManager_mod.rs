#[cfg(feature = "MissionObjectiveCheckersManager+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionObjectiveCheckersManager_InitData {
    __cordl_parent: crate::System::Object,
    pub missionObjectives: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MissionObjective,
    >,
}
#[cfg(feature = "MissionObjectiveCheckersManager+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MissionObjectiveCheckersManager_InitData => ""
    ."MissionObjectiveCheckersManager/InitData"
);
#[cfg(feature = "MissionObjectiveCheckersManager+InitData")]
impl std::ops::Deref
for crate::GlobalNamespace::MissionObjectiveCheckersManager_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveCheckersManager+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MissionObjectiveCheckersManager_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveCheckersManager+InitData")]
impl crate::GlobalNamespace::MissionObjectiveCheckersManager_InitData {
    pub fn New(
        missionObjectives: *mut quest_hook::libil2cpp::Il2CppArray<*mut MissionObjective>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (missionObjectives))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        missionObjectives: *mut quest_hook::libil2cpp::Il2CppArray<*mut MissionObjective>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (missionObjectives))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissionObjectiveCheckersManager+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionObjectiveCheckersManager_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MissionObjectiveCheckersManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionObjectiveCheckersManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionObjectiveCheckers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MissionObjectiveChecker,
    >,
    pub _initData: *mut crate::GlobalNamespace::MissionObjectiveCheckersManager_InitData,
    pub _gameplayManager: *mut ILevelEndActions,
    pub objectiveDidFailEvent: *mut crate::System::Action,
    pub objectiveWasClearedEvent: *mut crate::System::Action,
    pub objectivesListDidChangeEvent: *mut crate::System::Action,
    pub _activeMissionObjectiveCheckers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MissionObjectiveChecker,
    >,
}
#[cfg(feature = "MissionObjectiveCheckersManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionObjectiveCheckersManager => ""
    ."MissionObjectiveCheckersManager"
);
#[cfg(feature = "MissionObjectiveCheckersManager")]
impl std::ops::Deref for MissionObjectiveCheckersManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveCheckersManager")]
impl std::ops::DerefMut for MissionObjectiveCheckersManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveCheckersManager")]
impl MissionObjectiveCheckersManager {
    #[cfg(feature = "MissionObjectiveCheckersManager+InitData")]
    pub type InitData = crate::GlobalNamespace::MissionObjectiveCheckersManager_InitData;
    pub fn GetMissionObjectiveChecker(
        &mut self,
        missionObjectiveType: *mut MissionObjectiveTypeSO,
    ) -> quest_hook::libil2cpp::Result<*mut MissionObjectiveChecker> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionObjectiveChecker = __cordl_object
            .invoke("GetMissionObjectiveChecker", (missionObjectiveType))?;
        Ok(__cordl_ret)
    }
    pub fn GetResults(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut MissionObjectiveResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut MissionObjectiveResult,
        > = __cordl_object.invoke("GetResults", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelFailed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLevelFailed", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelFinished(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLevelFinished", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMissionObjectiveCheckerStatusDidChange(
        &mut self,
        missionObjectiveChecker: *mut MissionObjectiveChecker,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMissionObjectiveCheckerStatusDidChange",
                (missionObjectiveChecker),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopChecking(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopChecking", ())?;
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
    pub fn add_objectiveDidFailEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_objectiveDidFailEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_objectiveWasClearedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_objectiveWasClearedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_objectivesListDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_objectivesListDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_activeMissionObjectiveCheckers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut MissionObjectiveChecker>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut MissionObjectiveChecker,
        > = __cordl_object.invoke("get_activeMissionObjectiveCheckers", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_objectiveDidFailEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_objectiveDidFailEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_objectiveWasClearedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_objectiveWasClearedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_objectivesListDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_objectivesListDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissionObjectiveCheckersManager")]
impl quest_hook::libil2cpp::ObjectType for MissionObjectiveCheckersManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
