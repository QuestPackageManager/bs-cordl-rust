#[cfg(feature = "MissionConnectionsGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionConnectionsGenerator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionNodesManager: *mut crate::GlobalNamespace::MissionNodesManager,
    pub _nodeConnectionPref: *mut crate::GlobalNamespace::MissionNodeConnection,
    pub _connectionsCanvas: *mut crate::UnityEngine::GameObject,
    pub _missionNodes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::MissionNode,
    >,
}
#[cfg(feature = "MissionConnectionsGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionConnectionsGenerator =>
    ""."MissionConnectionsGenerator"
);
#[cfg(feature = "MissionConnectionsGenerator")]
impl std::ops::Deref for crate::GlobalNamespace::MissionConnectionsGenerator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionConnectionsGenerator")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionConnectionsGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionConnectionsGenerator")]
impl crate::GlobalNamespace::MissionConnectionsGenerator {
    pub fn CreateConnectionBetweenNodes(
        &mut self,
        parentMissionNode: *mut crate::GlobalNamespace::MissionNode,
        childMissionNode: *mut crate::GlobalNamespace::MissionNode,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::MissionNodeConnection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MissionNodeConnection = __cordl_object
            .invoke(
                "CreateConnectionBetweenNodes",
                (parentMissionNode, childMissionNode),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateConnections(
        &mut self,
        missionNode: *mut crate::GlobalNamespace::MissionNode,
        visitedNodes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::MissionNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateConnections", (missionNode, visitedNodes))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get__rootMissionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::MissionNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MissionNode = __cordl_object
            .invoke("get__rootMissionNode", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissionConnectionsGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionConnectionsGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
