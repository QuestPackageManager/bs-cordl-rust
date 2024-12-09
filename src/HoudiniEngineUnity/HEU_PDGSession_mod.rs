#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PDGSession {
    __cordl_parent: crate::System::Object,
    pub _pdgAssets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_PDGAssetLink,
    >,
    pub _pdgMaxProcessEvents: i32,
    pub _pdgQueryEvents: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::HoudiniEngineUnity::HAPI_PDG_EventInfo,
    >,
    pub _pdgContextSize: i32,
    pub _pdgContextIDs: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _errored: bool,
    pub _errorMsg: *mut crate::System::String,
    pub _pdgState: crate::HoudiniEngineUnity::HAPI_PDG_State,
    pub _pdgEventMessages: *mut crate::System::Text::StringBuilder,
    pub _eventMessageColorCode: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_PDGSession =>
    "HoudiniEngineUnity"."HEU_PDGSession"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_PDGSession {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_PDGSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession")]
impl crate::HoudiniEngineUnity::HEU_PDGSession {
    #[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession+EventMessageColor")]
    pub type EventMessageColor = crate::HoudiniEngineUnity::HEU_PDGSession_EventMessageColor;
    pub fn AddAsset(
        &mut self,
        asset: *mut crate::HoudiniEngineUnity::HEU_PDGAssetLink,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAsset", (asset))?;
        Ok(__cordl_ret)
    }
    pub fn AddEventMessage(
        &mut self,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEventMessage", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn CancelCook(
        &mut self,
        topNetwork: *mut crate::HoudiniEngineUnity::HEU_TOPNetworkData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelCook", (topNetwork))?;
        Ok(__cordl_ret)
    }
    pub fn CleanUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanUp", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearErrorState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearErrorState", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearEventMessages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearEventMessages", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearWorkItemResult(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        contextID: i32,
        eventInfo: crate::HoudiniEngineUnity::HAPI_PDG_EventInfo,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearWorkItemResult", (session, contextID, eventInfo, topNode))?;
        Ok(__cordl_ret)
    }
    pub fn CookTOPNetworkOutputNode(
        &mut self,
        topNetwork: *mut crate::HoudiniEngineUnity::HEU_TOPNetworkData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CookTOPNetworkOutputNode", (topNetwork))?;
        Ok(__cordl_ret)
    }
    pub fn CookTOPNode(&mut self, nodeID: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CookTOPNode", (nodeID))?;
        Ok(__cordl_ret)
    }
    pub fn DirtyAll(&mut self, nodeID: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DirtyAll", (nodeID))?;
        Ok(__cordl_ret)
    }
    pub fn DirtyTOPNode(&mut self, nodeID: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DirtyTOPNode", (nodeID))?;
        Ok(__cordl_ret)
    }
    pub fn GetEventMessages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetEventMessages", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHAPIPDGSession(
        &mut self,
        bCreate: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_SessionBase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_SessionBase = __cordl_object
            .invoke("GetHAPIPDGSession", (bCreate))?;
        Ok(__cordl_ret)
    }
    pub fn GetTOPAssetLinkAndNode(
        &mut self,
        nodeID: i32,
        assetLink: quest_hook::libil2cpp::ByRefMut<
            *mut crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        >,
        topNode: quest_hook::libil2cpp::ByRefMut<
            *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetTOPAssetLinkAndNode", (nodeID, assetLink, topNode))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn NotifyTOPNodeCookedWorkItem(
        &mut self,
        assetLink: *mut crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyTOPNodeCookedWorkItem", (assetLink, topNode))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyTOPNodeCookingWorkItem(
        &mut self,
        assetLink: *mut crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
        inc: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyTOPNodeCookingWorkItem", (assetLink, topNode, inc))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyTOPNodeErrorWorkItem(
        &mut self,
        assetLink: *mut crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyTOPNodeErrorWorkItem", (assetLink, topNode))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyTOPNodePDGStateClear(
        &mut self,
        assetLink: *mut crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyTOPNodePDGStateClear", (assetLink, topNode))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyTOPNodeScheduledWorkItem(
        &mut self,
        assetLink: *mut crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
        inc: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyTOPNodeScheduledWorkItem", (assetLink, topNode, inc))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyTOPNodeTotalWorkItem(
        &mut self,
        assetLink: *mut crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
        inc: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyTOPNodeTotalWorkItem", (assetLink, topNode, inc))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyTOPNodeWaitingWorkItem(
        &mut self,
        assetLink: *mut crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
        inc: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyTOPNodeWaitingWorkItem", (assetLink, topNode, inc))?;
        Ok(__cordl_ret)
    }
    pub fn PauseCook(
        &mut self,
        topNetwork: *mut crate::HoudiniEngineUnity::HEU_TOPNetworkData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PauseCook", (topNetwork))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPDGEvent(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        contextID: i32,
        eventInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PDG_EventInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPDGEvent", (session, contextID, eventInfo))?;
        Ok(__cordl_ret)
    }
    pub fn ReinitializePDGContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReinitializePDGContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveAsset(
        &mut self,
        asset: *mut crate::HoudiniEngineUnity::HEU_PDGAssetLink,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAsset", (asset))?;
        Ok(__cordl_ret)
    }
    pub fn SetErrorState(
        &mut self,
        msg: *mut crate::System::String,
        bLogIt: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetErrorState", (msg, bLogIt))?;
        Ok(__cordl_ret)
    }
    pub fn SetTOPNodePDGState(
        &mut self,
        assetLink: *mut crate::HoudiniEngineUnity::HEU_PDGAssetLink,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
        pdgState: crate::HoudiniEngineUnity::HEU_TOPNodeData_PDGState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTOPNodePDGState", (assetLink, topNode, pdgState))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdatePDGContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePDGContext", ())?;
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
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_PDGSession {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession+EventMessageColor")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_PDGSession_EventMessageColor {
    DEFAULT = 0i32,
    ERROR = 2i32,
    WARNING = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGSession+EventMessageColor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_PDGSession_EventMessageColor => "HoudiniEngineUnity"
    ."HEU_PDGSession/EventMessageColor"
);
