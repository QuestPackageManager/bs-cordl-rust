#[cfg(feature = "MissionConnectionsGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionConnectionsGenerator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionNodesManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionNodesManager,
    >,
    pub _nodeConnectionPref: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionNodeConnection,
    >,
    pub _connectionsCanvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _missionNodes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::MissionNode,
        >,
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
        parentMissionNode: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionNode,
        >,
        childMissionNode: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNodeConnection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionNodeConnection,
        > = __cordl_object
            .invoke(
                "CreateConnectionBetweenNodes",
                (parentMissionNode, childMissionNode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateConnections(
        &mut self,
        missionNode: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
        visitedNodes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::MissionNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateConnections", (missionNode, visitedNodes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNodeConnections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateNodeConnections", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveOldConnections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveOldConnections", ())?;
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
    pub fn get__rootMissionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionNode,
        > = __cordl_object.invoke("get__rootMissionNode", ())?;
        Ok(__cordl_ret.into())
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
