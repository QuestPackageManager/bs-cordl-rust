#[cfg(feature = "MissionConnectionsGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionConnectionsGenerator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionNodesManager: *mut MissionNodesManager,
    pub _nodeConnectionPref: *mut MissionNodeConnection,
    pub _connectionsCanvas: *mut crate::UnityEngine::GameObject,
    pub _missionNodes: *mut crate::System::Collections::Generic::List_1<
        *mut MissionNode,
    >,
}
#[cfg(feature = "MissionConnectionsGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionConnectionsGenerator => ""
    ."MissionConnectionsGenerator"
);
#[cfg(feature = "MissionConnectionsGenerator")]
impl std::ops::Deref for MissionConnectionsGenerator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionConnectionsGenerator")]
impl std::ops::DerefMut for MissionConnectionsGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionConnectionsGenerator")]
impl MissionConnectionsGenerator {
    pub fn get__rootMissionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MissionNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionNode = __cordl_object
            .invoke("get__rootMissionNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateNodeConnections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateNodeConnections", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveOldConnections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveOldConnections", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateConnections(
        &mut self,
        missionNode: *mut MissionNode,
        visitedNodes: *mut crate::System::Collections::Generic::List_1<*mut MissionNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateConnections", (missionNode, visitedNodes))?;
        Ok(__cordl_ret)
    }
    pub fn CreateConnectionBetweenNodes(
        &mut self,
        parentMissionNode: *mut MissionNode,
        childMissionNode: *mut MissionNode,
    ) -> quest_hook::libil2cpp::Result<*mut MissionNodeConnection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionNodeConnection = __cordl_object
            .invoke(
                "CreateConnectionBetweenNodes",
                (parentMissionNode, childMissionNode),
            )?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MissionConnectionsGenerator")]
impl quest_hook::libil2cpp::ObjectType for MissionConnectionsGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
