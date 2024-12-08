#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PDGAssetLink {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _assetPath: *mut crate::System::String,
    pub _assetGO: *mut crate::UnityEngine::GameObject,
    pub _assetName: *mut crate::System::String,
    pub _assetID: i32,
    pub _heu: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    pub _topNetworks: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_TOPNetworkData,
    >,
    pub _topNetworkNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub _selectedTOPNetwork: i32,
    pub _linkState: crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState,
    pub _autoCook: bool,
    pub _useHEngineData: bool,
    pub _repaintUIDelegate: *mut crate::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate,
    pub _numWorkItems: i32,
    pub _workItemTally: *mut crate::HoudiniEngineUnity::HEU_WorkItemTally,
    pub _loadRootGameObject: *mut crate::UnityEngine::GameObject,
    pub _outputCachePathRoot: *mut crate::System::String,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_PDGAssetLink =>
    "HoudiniEngineUnity"."HEU_PDGAssetLink"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_PDGAssetLink {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_PDGAssetLink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink")]
impl crate::HoudiniEngineUnity::HEU_PDGAssetLink {
    #[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+LinkState")]
    pub type LinkState = crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState;
    #[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
    pub type UpdateUIDelegate = crate::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate;
    pub fn PauseCook(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PauseCook", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSelectedTOPNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_TOPNodeData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData = __cordl_object
            .invoke("GetSelectedTOPNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SelectedTOPNetwork(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_SelectedTOPNetwork", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopulateTOPNetworks(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("PopulateTOPNetworks", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnBeforeSerialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeforeSerialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHAPISession(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_SessionBase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_SessionBase = __cordl_object
            .invoke("GetHAPISession", ())?;
        Ok(__cordl_ret)
    }
    pub fn SelectTOPNetwork(
        &mut self,
        newIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectTOPNetwork", (newIndex))?;
        Ok(__cordl_ret)
    }
    pub fn OnAfterDeserialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAfterDeserialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTOPNetwork(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_TOPNetworkData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_TOPNetworkData = __cordl_object
            .invoke("GetTOPNetwork", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetTOPNode(
        &mut self,
        nodeID: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_TOPNodeData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData = __cordl_object
            .invoke("GetTOPNode", (nodeID))?;
        Ok(__cordl_ret)
    }
    pub fn RepaintUI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RepaintUI", ())?;
        Ok(__cordl_ret)
    }
    pub fn CookOutput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CookOutput", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateWorkItemTally(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateWorkItemTally", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadResults(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
        workItemInfo: crate::HoudiniEngineUnity::HAPI_PDG_WorkitemInfo,
        resultInfos: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::HoudiniEngineUnity::HAPI_PDG_WorkitemResultInfo,
        >,
        workItemID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LoadResults",
                (session, topNode, workItemInfo, resultInfos, workItemID),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetTOPNodeStatus(
        &mut self,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetTOPNodeStatus", (topNode))?;
        Ok(__cordl_ret)
    }
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLoadRootTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("GetLoadRootTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AssetLinkState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState = __cordl_object
            .invoke("get_AssetLinkState", ())?;
        Ok(__cordl_ret)
    }
    pub fn DirtyAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DirtyAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn NotifyAssetCooked_HEU_HoudiniAsset__cordl_bool_List_1_0(
        &mut self,
        asset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        bSuccess: bool,
        generatedOutputs: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyAssetCooked", (asset, bSuccess, generatedOutputs))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyAssetCooked_HEU_CookedEventData1(
        &mut self,
        cookedEventData: *mut crate::HoudiniEngineUnity::HEU_CookedEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyAssetCooked", (cookedEventData))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyAssetCooked_HEU_ReloadEventData2(
        &mut self,
        reloadEventData: *mut crate::HoudiniEngineUnity::HEU_ReloadEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyAssetCooked", (reloadEventData))?;
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
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleInitialLoad(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInitialLoad", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearAllTOPData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearAllTOPData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AssetName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AssetName", ())?;
        Ok(__cordl_ret)
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
    pub fn UpdateTOPNodeResultsVisibility(
        &mut self,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTOPNodeResultsVisibility", (topNode))?;
        Ok(__cordl_ret)
    }
    pub fn PopulateFromHDA(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateFromHDA", ())?;
        Ok(__cordl_ret)
    }
    pub fn SelectTOPNode(
        &mut self,
        network: *mut crate::HoudiniEngineUnity::HEU_TOPNetworkData,
        newIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectTOPNode", (network, newIndex))?;
        Ok(__cordl_ret)
    }
    pub fn CancelCook(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelCook", ())?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        hdaAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (hdaAsset))?;
        Ok(__cordl_ret)
    }
    pub fn DirtyTOPNode(
        &mut self,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DirtyTOPNode", (topNode))?;
        Ok(__cordl_ret)
    }
    pub fn ResetTOPNetworkWorkItemTally(
        &mut self,
        topNetwork: *mut crate::HoudiniEngineUnity::HEU_TOPNetworkData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetTOPNetworkWorkItemTally", (topNetwork))?;
        Ok(__cordl_ret)
    }
    pub fn GetSelectedTOPNetwork(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_TOPNetworkData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_TOPNetworkData = __cordl_object
            .invoke("GetSelectedTOPNetwork", ())?;
        Ok(__cordl_ret)
    }
    pub fn CookTOPNode(
        &mut self,
        topNode: *mut crate::HoudiniEngineUnity::HEU_TOPNodeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CookTOPNode", (topNode))?;
        Ok(__cordl_ret)
    }
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_PDGAssetLink {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+LinkState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_PDGAssetLink_LinkState {
    ERROR_NOT_LINKED = 3i32,
    INACTIVE = 0i32,
    LINKED = 2i32,
    LINKING = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+LinkState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_PDGAssetLink_LinkState
    => "HoudiniEngineUnity"."HEU_PDGAssetLink/LinkState"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PDGAssetLink_UpdateUIDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate => "HoudiniEngineUnity"
    ."HEU_PDGAssetLink/UpdateUIDelegate"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
impl crate::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate {
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn BeginInvoke(
        &mut self,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PDGAssetLink+UpdateUIDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_PDGAssetLink_UpdateUIDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
