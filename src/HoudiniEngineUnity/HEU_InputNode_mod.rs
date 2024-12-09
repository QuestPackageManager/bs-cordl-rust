#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputNode {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _inputNodeType: crate::HoudiniEngineUnity::HEU_InputNode_InputNodeType,
    pub _inputObjectType: crate::HoudiniEngineUnity::HEU_InputNode_InputObjectType,
    pub _pendingInputObjectType: crate::HoudiniEngineUnity::HEU_InputNode_InputObjectType,
    pub _inputObjects: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_InputObjectInfo,
    >,
    pub _inputObjectsConnectedAssetIDs: *mut crate::System::Collections::Generic::List_1<
        i32,
    >,
    pub _inputAsset: *mut crate::UnityEngine::GameObject,
    pub _connectedInputAsset: *mut crate::UnityEngine::GameObject,
    pub _inputAssetInfos: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_InputHDAInfo,
    >,
    pub _nodeID: i32,
    pub _inputIndex: i32,
    pub _requiresCook: bool,
    pub _requiresUpload: bool,
    pub _inputName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _labelName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _paramName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _connectedNodeID: i32,
    pub _keepWorldTransform: bool,
    pub _packGeometryBeforeMerging: bool,
    pub _parentAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    pub _tilemapSettings: *mut crate::HoudiniEngineUnity::HEU_InputInterfaceTilemapSettings,
    pub _uiCache: *mut crate::HoudiniEngineUnity::HEU_InputNodeUICache,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputNode =>
    "HoudiniEngineUnity"."HEU_InputNode"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputNode {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode")]
impl crate::HoudiniEngineUnity::HEU_InputNode {
    #[cfg(feature = "HoudiniEngineUnity+HEU_InputNode+InputActions")]
    pub type InputActions = crate::HoudiniEngineUnity::HEU_InputNode_InputActions;
    #[cfg(feature = "HoudiniEngineUnity+HEU_InputNode+InputNodeType")]
    pub type InputNodeType = crate::HoudiniEngineUnity::HEU_InputNode_InputNodeType;
    #[cfg(feature = "HoudiniEngineUnity+HEU_InputNode+InputObjectType")]
    pub type InputObjectType = crate::HoudiniEngineUnity::HEU_InputNode_InputObjectType;
    #[cfg(feature = "HoudiniEngineUnity+HEU_InputNode+InternalObjectType")]
    pub type InternalObjectType = crate::HoudiniEngineUnity::HEU_InputNode_InternalObjectType;
    pub fn AddInputEntryAtEnd(
        &mut self,
        newEntryGameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddInputEntryAtEnd", (newEntryGameObject))?;
        Ok(__cordl_ret)
    }
    pub fn AddInputEntryAtEndMesh(
        &mut self,
        newEntryGameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_InputObjectInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_InputObjectInfo = __cordl_object
            .invoke("AddInputEntryAtEndMesh", (newEntryGameObject))?;
        Ok(__cordl_ret)
    }
    pub fn AreAnyInputHDAsConnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AreAnyInputHDAsConnected", ())?;
        Ok(__cordl_ret)
    }
    pub fn ChangeInputType(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        newType: crate::HoudiniEngineUnity::HEU_InputNode_InputObjectType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeInputType", (session, newType))?;
        Ok(__cordl_ret)
    }
    pub fn ClearConnectedInputHDAs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearConnectedInputHDAs", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearUICache(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearUICache", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConnectToMergeObject(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConnectToMergeObject", (session))?;
        Ok(__cordl_ret)
    }
    pub fn CopyInputValuesTo(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        destInputNode: *mut crate::HoudiniEngineUnity::HEU_InputNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyInputValuesTo", (session, destInputNode))?;
        Ok(__cordl_ret)
    }
    pub fn CreateInputHDAInfo(
        &mut self,
        inputGameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_InputHDAInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_InputHDAInfo = __cordl_object
            .invoke("CreateInputHDAInfo", (inputGameObject))?;
        Ok(__cordl_ret)
    }
    pub fn CreateInputObjectInfo(
        &mut self,
        inputGameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_InputObjectInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_InputObjectInfo = __cordl_object
            .invoke("CreateInputObjectInfo", (inputGameObject))?;
        Ok(__cordl_ret)
    }
    pub fn DestroyAllData(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyAllData", (session))?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectAndDestroyInputs(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectAndDestroyInputs", (session))?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectConnectedMergeNode(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectConnectedMergeNode", (session))?;
        Ok(__cordl_ret)
    }
    pub fn FindAddToInputHDA(
        &mut self,
        gameObjectName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FindAddToInputHDA", (gameObjectName))?;
        Ok(__cordl_ret)
    }
    pub fn GetConnectedInputCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetConnectedInputCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetConnectedNodeID(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetConnectedNodeID", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetInputEntryGameObject(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("GetInputEntryGameObject", (index))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelectedObjectsForInputHDAs(
        &mut self,
        selectedObjects: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSelectedObjectsForInputHDAs", (selectedObjects))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelectedObjectsForInputObjects(
        &mut self,
        selectedObjects: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSelectedObjectsForInputObjects", (selectedObjects))?;
        Ok(__cordl_ret)
    }
    pub fn HasInputNodeTransformChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasInputNodeTransformChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn InsertInputEntry(
        &mut self,
        index: i32,
        newInputGameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertInputEntry", (index, newInputGameObject))?;
        Ok(__cordl_ret)
    }
    pub fn InternalAddInputHDAAtEnd(
        &mut self,
        newInputHDA: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_InputHDAInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_InputHDAInfo = __cordl_object
            .invoke("InternalAddInputHDAAtEnd", (newInputHDA))?;
        Ok(__cordl_ret)
    }
    pub fn InternalAddInputObjectAtEnd(
        &mut self,
        newInputGameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_InputObjectInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_InputObjectInfo = __cordl_object
            .invoke("InternalAddInputObjectAtEnd", (newInputGameObject))?;
        Ok(__cordl_ret)
    }
    pub fn IsAssetInput(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAssetInput", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: *mut crate::HoudiniEngineUnity::HEU_InputNode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn LoadPreset(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        inputPreset: *mut crate::HoudiniEngineUnity::HEU_InputPreset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadPreset", (session, inputPreset))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn NotifyParentRemovedInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyParentRemovedInput", ())?;
        Ok(__cordl_ret)
    }
    pub fn NumInputEntries(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumInputEntries", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopulateInputPreset(
        &mut self,
        inputPreset: *mut crate::HoudiniEngineUnity::HEU_InputPreset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateInputPreset", (inputPreset))?;
        Ok(__cordl_ret)
    }
    pub fn ReconnectToUpstreamAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReconnectToUpstreamAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveAllInputEntries(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAllInputEntries", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetConnectionForForceUpdate(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetConnectionForForceUpdate", (session))?;
        Ok(__cordl_ret)
    }
    pub fn ResetInputNode(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetInputNode", (session))?;
        Ok(__cordl_ret)
    }
    pub fn ResetInputObjectTransforms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetInputObjectTransforms", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetInputNodeID(
        &mut self,
        nodeID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInputNodeID", (nodeID))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateOnAssetRecreation(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateOnAssetRecreation", (session))?;
        Ok(__cordl_ret)
    }
    pub fn UploadHDAInput(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadHDAInput", (session))?;
        Ok(__cordl_ret)
    }
    pub fn UploadInput(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadInput", (session))?;
        Ok(__cordl_ret)
    }
    pub fn UploadInputObjectTransforms(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadInputObjectTransforms", (session))?;
        Ok(__cordl_ret)
    }
    pub fn UploadObjectMergePackGeometry(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UploadObjectMergePackGeometry", (session))?;
        Ok(__cordl_ret)
    }
    pub fn UploadObjectMergeTransformType(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UploadObjectMergeTransformType", (session))?;
        Ok(__cordl_ret)
    }
    pub fn UploadUnityInput(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadUnityInput", (session))?;
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
    pub fn get_InputName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_InputName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InputNodeID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_InputNodeID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InputObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_InputObjectInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_InputObjectInfo,
        > = __cordl_object.invoke("get_InputObjects", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InputType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::HEU_InputNode_InputNodeType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HEU_InputNode_InputNodeType = __cordl_object
            .invoke("get_InputType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_KeepWorldTransform(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_KeepWorldTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LabelName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_LabelName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PackGeometryBeforeMerging(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_PackGeometryBeforeMerging", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParamName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_ParamName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset = __cordl_object
            .invoke("get_ParentAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PendingInputObjectType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::HEU_InputNode_InputObjectType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HEU_InputNode_InputObjectType = __cordl_object
            .invoke("get_PendingInputObjectType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RequiresCook(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RequiresCook", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RequiresUpload(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RequiresUpload", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ThisInputObjectType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::HEU_InputNode_InputObjectType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HEU_InputNode_InputObjectType = __cordl_object
            .invoke("get_ThisInputObjectType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TilemapSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_InputInterfaceTilemapSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_InputInterfaceTilemapSettings = __cordl_object
            .invoke("get_TilemapSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_KeepWorldTransform(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_KeepWorldTransform", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_PackGeometryBeforeMerging(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PackGeometryBeforeMerging", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ParamName(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ParamName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_PendingInputObjectType(
        &mut self,
        value: crate::HoudiniEngineUnity::HEU_InputNode_InputObjectType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PendingInputObjectType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_RequiresCook(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RequiresCook", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_RequiresUpload(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RequiresUpload", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_InputNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode+InputActions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_InputNode_InputActions {
    ACTION = 0i32,
    DELETE = 1i32,
    INSERT = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode+InputActions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputNode_InputActions
    => "HoudiniEngineUnity"."HEU_InputNode/InputActions"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode+InputNodeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_InputNode_InputNodeType {
    CONNECTION = 0i32,
    NODE = 1i32,
    PARAMETER = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode+InputNodeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputNode_InputNodeType
    => "HoudiniEngineUnity"."HEU_InputNode/InputNodeType"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode+InputObjectType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_InputNode_InputObjectType {
    BOUNDING_BOX = 4i32,
    CURVE = 2i32,
    HDA = 0i32,
    TERRAIN = 3i32,
    TILEMAP = 5i32,
    UNITY_MESH = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode+InputObjectType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_InputNode_InputObjectType => "HoudiniEngineUnity"
    ."HEU_InputNode/InputObjectType"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode+InternalObjectType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_InputNode_InternalObjectType {
    HDA = 1i32,
    UNITY_MESH = 2i32,
    UNKNOWN = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputNode+InternalObjectType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_InputNode_InternalObjectType => "HoudiniEngineUnity"
    ."HEU_InputNode/InternalObjectType"
);
