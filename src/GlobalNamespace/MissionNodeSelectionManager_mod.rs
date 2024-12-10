#[cfg(feature = "MissionNodeSelectionManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionNodeSelectionManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _missionNodesManager: *mut crate::GlobalNamespace::MissionNodesManager,
    pub didSelectMissionNodeEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::MissionNodeVisualController,
    >,
    pub _missionNodes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MissionNode,
    >,
    pub _selectedNode: *mut crate::GlobalNamespace::MissionNodeVisualController,
}
#[cfg(feature = "MissionNodeSelectionManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionNodeSelectionManager =>
    ""."MissionNodeSelectionManager"
);
#[cfg(feature = "MissionNodeSelectionManager")]
impl std::ops::Deref for crate::GlobalNamespace::MissionNodeSelectionManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNodeSelectionManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionNodeSelectionManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionNodeSelectionManager")]
impl crate::GlobalNamespace::MissionNodeSelectionManager {
    pub fn DeselectSelectedNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeselectSelectedNode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNodeWasDisplayed(
        &mut self,
        missionNode: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionNodeVisualController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNodeWasDisplayed", (missionNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNodeWasSelect(
        &mut self,
        missionNode: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionNodeVisualController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNodeWasSelect", (missionNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn add_didSelectMissionNodeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::MissionNodeVisualController,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectMissionNodeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectMissionNodeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::MissionNodeVisualController,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectMissionNodeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionNodeSelectionManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionNodeSelectionManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
