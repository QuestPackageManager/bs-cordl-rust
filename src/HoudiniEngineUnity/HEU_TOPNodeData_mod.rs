#[cfg(feature = "HoudiniEngineUnity+HEU_TOPNodeData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_TOPNodeData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _nodeID: i32,
    pub _nodeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _parentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _workResultParentGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _workResults: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_TOPWorkResult>,
        >,
    >,
    pub _tags: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::TOPNodeTags>,
    pub _showResults: bool,
    pub _pdgState: crate::HoudiniEngineUnity::HEU_TOPNodeData_PDGState,
    pub _workItemTally: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_WorkItemTally,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TOPNodeData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_TOPNodeData =>
    "HoudiniEngineUnity"."HEU_TOPNodeData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_TOPNodeData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_TOPNodeData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TOPNodeData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_TOPNodeData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TOPNodeData")]
impl crate::HoudiniEngineUnity::HEU_TOPNodeData {
    #[cfg(feature = "HoudiniEngineUnity+HEU_TOPNodeData+PDGState")]
    pub type PDGState = crate::HoudiniEngineUnity::HEU_TOPNodeData_PDGState;
    pub fn AnyWorkItemsFailed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AnyWorkItemsFailed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AnyWorkItemsPending(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AnyWorkItemsPending", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AreAllWorkItemsComplete(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AreAllWorkItemsComplete", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
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
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TOPNodeData")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_TOPNodeData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TOPNodeData+PDGState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_TOPNodeData_PDGState {
    #[default]
    COOKING = 3i32,
    COOK_COMPLETE = 4i32,
    COOK_FAILED = 5i32,
    DIRTIED = 1i32,
    DIRTYING = 2i32,
    NONE = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TOPNodeData+PDGState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_TOPNodeData_PDGState =>
    "HoudiniEngineUnity"."HEU_TOPNodeData/PDGState"
);
