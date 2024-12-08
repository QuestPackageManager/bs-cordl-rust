#[cfg(feature = "MissionNodesManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionNodesManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _rootMissionNode: *mut MissionNode,
    pub _finalMissionNode: *mut MissionNode,
    pub _missionStagesManager: *mut MissionStagesManager,
    pub _connectionsParentObject: *mut crate::UnityEngine::GameObject,
    pub _missionNodesParentObject: *mut crate::UnityEngine::GameObject,
    pub _missionProgressModel: *mut CampaignProgressModel,
    pub _allMissionNodeConnections: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MissionNodeConnection,
    >,
    pub _allMissionNodes: *mut quest_hook::libil2cpp::Il2CppArray<*mut MissionNode>,
    pub _isInitialized: bool,
}
#[cfg(feature = "MissionNodesManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionNodesManager => ""."MissionNodesManager"
);
#[cfg(feature = "MissionNodesManager")]
impl std::ops::Deref for MissionNodesManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNodesManager")]
impl std::ops::DerefMut for MissionNodesManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNodesManager")]
impl MissionNodesManager {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn DidFirstLockedMissionStageChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("DidFirstLockedMissionStageChange", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAllMissionNodes_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAllMissionNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAllMissionNodes_MissionNode_HashSet_1_1(
        &mut self,
        node: *mut MissionNode,
        visited: *mut crate::System::Collections::Generic::HashSet_1<*mut MissionNode>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::HashSet_1<*mut MissionNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::HashSet_1<
            *mut MissionNode,
        > = __cordl_object.invoke("GetAllMissionNodes", (node, visited))?;
        Ok(__cordl_ret)
    }
    pub fn GetMissionNodeWithModelClearedStateInconsistency(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MissionNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionNode = __cordl_object
            .invoke("GetMissionNodeWithModelClearedStateInconsistency", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNewEnabledConnection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut MissionNodeConnection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut MissionNodeConnection,
        > = __cordl_object.invoke("GetNewEnabledConnection", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTopMostNotClearedMissionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MissionNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionNode = __cordl_object
            .invoke("GetTopMostNotClearedMissionNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn IMissionNodesManager_get_allMissionNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
            *mut IMissionNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
            *mut IMissionNode,
        > = __cordl_object.invoke("IMissionNodesManager.get_allMissionNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn IMissionNodesManager_get_finalMissionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IMissionNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IMissionNode = __cordl_object
            .invoke("IMissionNodesManager.get_finalMissionNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsNodeInteractable(
        &mut self,
        node: *mut MissionNodeVisualController,
        parentCleared: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsNodeInteractable", (node, parentCleared))?;
        Ok(__cordl_ret)
    }
    pub fn MissionWasCleared(
        &mut self,
        missionNode: *mut MissionNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MissionWasCleared", (missionNode))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RegisterAllNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterAllNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetAllNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetAllNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetupNodeConnections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupNodeConnections", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetupNodeMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupNodeMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetupNodeTree(
        &mut self,
        node: *mut MissionNodeVisualController,
        parentCleared: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupNodeTree", (node, parentCleared))?;
        Ok(__cordl_ret)
    }
    pub fn SetupStages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupStages", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateStageLockText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateStageLockText", ())?;
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
    pub fn get_IsInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInitialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_allMissionNodes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut MissionNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<*mut MissionNode> = __cordl_object
            .invoke("get_allMissionNodes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_finalMissionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MissionNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionNode = __cordl_object
            .invoke("get_finalMissionNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_missionProgressModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut CampaignProgressModel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut CampaignProgressModel = __cordl_object
            .invoke("get_missionProgressModel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_missionStagesManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MissionStagesManager> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionStagesManager = __cordl_object
            .invoke("get_missionStagesManager", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rootMissionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MissionNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionNode = __cordl_object
            .invoke("get_rootMissionNode", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissionNodesManager")]
impl quest_hook::libil2cpp::ObjectType for MissionNodesManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
