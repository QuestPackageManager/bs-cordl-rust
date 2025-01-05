#[cfg(feature = "HoudiniEngineUnity+HEU_SessionBase")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_SessionBase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _sessionData: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_SessionData,
    >,
    pub _UserNotifiedSessionInvalid_k__BackingField: bool,
    pub _sessionErrorMsg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _LogErrorOverride_k__BackingField: bool,
    pub _ThrowErrorOverride_k__BackingField: bool,
    pub _LastCallResultCode_k__BackingField: crate::HoudiniEngineUnity::HAPI_Result,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_SessionBase =>
    "HoudiniEngineUnity"."HEU_SessionBase"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionBase")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_SessionBase {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionBase")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_SessionBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionBase")]
impl crate::HoudiniEngineUnity::HEU_SessionBase {
    pub fn AddAttribute(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddAttribute", (nodeID, partID, name, attrInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddGroup(
        &mut self,
        nodeID: i32,
        partID: i32,
        groupType: crate::HoudiniEngineUnity::HAPI_GroupType,
        groupName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddGroup", (nodeID, partID, groupType, groupName))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckAndCloseExistingSession(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckAndCloseExistingSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckForSpecificErrors(
        &mut self,
        nodeID: i32,
        errorsToCheck: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CheckForSpecificErrors", (nodeID, errorsToCheck))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckVersionMatch(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckVersionMatch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearSessionInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSessionInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CloseSession(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CloseSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CommitGeo(&mut self, nodeID: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CommitGeo", (nodeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComposeChildNodeList(
        &mut self,
        parentNodeID: i32,
        nodeTypeFilter: i32,
        nodeFlagFilter: i32,
        bRecursive: bool,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ComposeChildNodeList",
                (parentNodeID, nodeTypeFilter, nodeFlagFilter, bRecursive, count),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ComposeNodeCookResult(
        &mut self,
        nodeId: i32,
        verbosity: crate::HoudiniEngineUnity::HAPI_StatusVerbosity,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ComposeNodeCookResult", (nodeId, verbosity))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComposeObjectList(
        &mut self,
        nodeID: i32,
        objectCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ComposeObjectList", (nodeID, objectCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConnectNodeInput(
        &mut self,
        nodeID: i32,
        inputIndex: i32,
        nodeIDToConnect: i32,
        outputIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ConnectNodeInput",
                (nodeID, inputIndex, nodeIDToConnect, outputIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConnectThriftPipeSession(
        &mut self,
        bIsDefaultSession: bool,
        pipeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        autoClose: bool,
        timeout: f32,
        logError: bool,
        autoInitialize: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ConnectThriftPipeSession",
                (
                    bIsDefaultSession,
                    pipeName,
                    autoClose,
                    timeout,
                    logError,
                    autoInitialize,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConnectThriftSocketSession(
        &mut self,
        bIsDefaultSession: bool,
        hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        serverPort: i32,
        autoClose: bool,
        timeout: f32,
        logError: bool,
        autoInitialize: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ConnectThriftSocketSession",
                (
                    bIsDefaultSession,
                    hostName,
                    serverPort,
                    autoClose,
                    timeout,
                    logError,
                    autoInitialize,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertTransform(
        &mut self,
        inTransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_TransformEuler,
        >,
        RSTOrder: crate::HoudiniEngineUnity::HAPI_RSTOrder,
        ROTOrder: crate::HoudiniEngineUnity::HAPI_XYZOrder,
        outTransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_TransformEuler,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ConvertTransform",
                (inTransform, RSTOrder, ROTOrder, outTransform),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CookNode(
        &mut self,
        nodeID: i32,
        bCookTemplatedGeos: bool,
        bSplitGeosByGroup: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CookNode", (nodeID, bCookTemplatedGeos, bSplitGeosByGroup))?;
        Ok(__cordl_ret.into())
    }
    pub fn CookNodeWithOptions(
        &mut self,
        nodeID: i32,
        cookOptions: crate::HoudiniEngineUnity::HAPI_CookOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CookNodeWithOptions", (nodeID, cookOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCustomSession(
        &mut self,
        bIsDefaultSession: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CreateCustomSession", (bIsDefaultSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHeightFieldInput(
        &mut self,
        parentNodeID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xSize: i32,
        ySize: i32,
        voxelSize: f32,
        sampling: crate::HoudiniEngineUnity::HAPI_HeightFieldSampling,
        heightfieldNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
        heightNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
        maskNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
        mergeNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CreateHeightFieldInput",
                (
                    parentNodeID,
                    name,
                    xSize,
                    ySize,
                    voxelSize,
                    sampling,
                    heightfieldNodeID,
                    heightNodeID,
                    maskNodeID,
                    mergeNodeID,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHeightfieldInputVolumeNode(
        &mut self,
        parentNodeID: i32,
        newNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        xSize: i32,
        ySize: i32,
        voxelSize: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CreateHeightfieldInputVolumeNode",
                (parentNodeID, newNodeID, name, xSize, ySize, voxelSize),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInProcessSession(
        &mut self,
        bIsDefaultSession: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CreateInProcessSession", (bIsDefaultSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInputNode(
        &mut self,
        nodeID: quest_hook::libil2cpp::ByRefMut<i32>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CreateInputNode", (nodeID, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNode(
        &mut self,
        parentNodeID: i32,
        operatorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nodeLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bCookOnCreation: bool,
        newNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CreateNode",
                (parentNodeID, operatorName, nodeLabel, bCookOnCreation, newNodeID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSessionData(
        &mut self,
        bOverwriteExisting: bool,
        bIsDefaultSession: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CreateSessionData", (bOverwriteExisting, bIsDefaultSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateThriftPipeSession(
        &mut self,
        bIsDefaultSession: bool,
        pipeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        autoClose: bool,
        timeout: f32,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CreateThriftPipeSession",
                (bIsDefaultSession, pipeName, autoClose, timeout, bLogError),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateThriftSocketSession(
        &mut self,
        bIsDefaultSession: bool,
        hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        serverPort: i32,
        autoClose: bool,
        timeout: f32,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CreateThriftSocketSession",
                (bIsDefaultSession, hostName, serverPort, autoClose, timeout, bLogError),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteGroup(
        &mut self,
        nodeID: i32,
        partID: i32,
        groupType: crate::HoudiniEngineUnity::HAPI_GroupType,
        groupName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("DeleteGroup", (nodeID, partID, groupType, groupName))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteNode(
        &mut self,
        nodeID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteNode", (nodeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisconnectNodeInput(
        &mut self,
        nodeID: i32,
        inputIndex: i32,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("DisconnectNodeInput", (nodeID, inputIndex, bLogError))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractImageToFile(
        &mut self,
        nodeID: i32,
        fileFormat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        imagePlanes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destinationFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        destinationFilePath: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ExtractImageToFile",
                (
                    nodeID,
                    fileFormat,
                    imagePlanes,
                    destinationFolderPath,
                    destinationFilePath,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractImageToMemory(
        &mut self,
        nodeID: i32,
        fileFormat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        imagePlanes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ExtractImageToMemory", (nodeID, fileFormat, imagePlanes, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActiveCacheCount(
        &mut self,
        activeCacheCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetActiveCacheCount", (activeCacheCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActiveCacheNames(
        &mut self,
        cacheNamesArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        activeCacheCount: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetActiveCacheNames", (cacheNamesArray, activeCacheCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetInfo(
        &mut self,
        nodeID: i32,
        assetInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AssetInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetAssetInfo", (nodeID, assetInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeFloat64Data(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f64>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetAttributeFloat64Data",
                (nodeID, partID, name, attributeInfo, data, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeFloatData(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetAttributeFloatData",
                (nodeID, partID, name, attributeInfo, data, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeInfo(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        owner: crate::HoudiniEngineUnity::HAPI_AttributeOwner,
        attributeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetAttributeInfo", (nodeID, partID, name, owner, attributeInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeInt16Data(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetAttributeInt16Data",
                (nodeID, partID, name, attributeInfo, data, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeInt64Data(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetAttributeInt64Data",
                (nodeID, partID, name, attributeInfo, data, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeInt8Data(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetAttributeInt8Data",
                (nodeID, partID, name, attributeInfo, data, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeIntData(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetAttributeIntData",
                (nodeID, partID, name, attributeInfo, data, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeNames(
        &mut self,
        nodeID: i32,
        partID: i32,
        owner: crate::HoudiniEngineUnity::HAPI_AttributeOwner,
        attributeNames: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetAttributeNames",
                (nodeID, partID, owner, attributeNames, count),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeStringData(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        dataArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetAttributeStringData",
                (nodeID, partID, name, attributeInfo, dataArray, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeUInt8Data(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attributeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetAttributeUInt8Data",
                (nodeID, partID, name, attributeInfo, data, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAvailableAssetCount(
        &mut self,
        libraryID: i32,
        assetCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetAvailableAssetCount", (libraryID, assetCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAvailableAssets(
        &mut self,
        libraryID: i32,
        assetNames: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        assetCount: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetAvailableAssets", (libraryID, assetNames, assetCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoxInfo(
        &mut self,
        nodeID: i32,
        partID: i32,
        boxInfo: quest_hook::libil2cpp::ByRefMut<crate::HoudiniEngineUnity::HAPI_BoxInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetBoxInfo", (nodeID, partID, boxInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCacheProperty(
        &mut self,
        cacheName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cacheProperty: crate::HoudiniEngineUnity::HAPI_CacheProperty,
        propertyValue: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetCacheProperty", (cacheName, cacheProperty, propertyValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCallResult(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<crate::HoudiniEngineUnity::HAPI_Result>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetCallResult", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComposedChildNodeList(
        &mut self,
        parentNodeID: i32,
        childNodeIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetComposedChildNodeList", (parentNodeID, childNodeIDs, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComposedObjectList(
        &mut self,
        nodeID: i32,
        objectInfos: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::HoudiniEngineUnity::HAPI_ObjectInfo,
                >,
            >,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetComposedObjectList", (nodeID, objectInfos, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComposedObjectTransforms(
        &mut self,
        nodeID: i32,
        rstOrder: crate::HoudiniEngineUnity::HAPI_RSTOrder,
        transforms: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::HoudiniEngineUnity::HAPI_Transform,
                >,
            >,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetComposedObjectTransforms",
                (nodeID, rstOrder, transforms, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCookResult(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<crate::HoudiniEngineUnity::HAPI_Result>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetCookResult", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCookState(
        &mut self,
        state: quest_hook::libil2cpp::ByRefMut<crate::HoudiniEngineUnity::HAPI_State>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetCookState", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurveCounts(
        &mut self,
        nodeID: i32,
        partID: i32,
        counts: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetCurveCounts", (nodeID, partID, counts, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurveInfo(
        &mut self,
        nodeID: i32,
        partID: i32,
        curveInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_CurveInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetCurveInfo", (nodeID, partID, curveInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurveKnots(
        &mut self,
        nodeID: i32,
        partID: i32,
        knots: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetCurveKnots", (nodeID, partID, knots, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurveOrders(
        &mut self,
        nodeID: i32,
        partID: i32,
        orders: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetCurveOrders", (nodeID, partID, orders, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDisplayGeoInfo(
        &mut self,
        nodeID: i32,
        geoInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_GeoInfo,
        >,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetDisplayGeoInfo", (nodeID, geoInfo, bLogError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvInt(
        &mut self,
        intType: crate::HoudiniEngineUnity::HAPI_EnvIntType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetEnvInt", (intType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceCounts__cordl_bool1(
        &mut self,
        nodeID: i32,
        partID: i32,
        faceCounts: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetFaceCounts",
                (nodeID, partID, faceCounts, start, length, bLogError),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceCounts_i32_i32_ByRefMut_i32_i32_0(
        &mut self,
        nodeID: i32,
        partID: i32,
        faceCounts: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetFaceCounts", (nodeID, partID, faceCounts, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGeoInfo(
        &mut self,
        nodeID: i32,
        geoInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_GeoInfo,
        >,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetGeoInfo", (nodeID, geoInfo, bLogError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGeoSize(
        &mut self,
        nodeID: i32,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_size: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetGeoSize", (nodeID, format, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGroupCountOnPackedInstancePart(
        &mut self,
        nodeID: i32,
        partID: i32,
        pointGroupCount: quest_hook::libil2cpp::ByRefMut<i32>,
        primitiveGroupCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetGroupCountOnPackedInstancePart",
                (nodeID, partID, pointGroupCount, primitiveGroupCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGroupMembership(
        &mut self,
        nodeID: i32,
        partID: i32,
        groupType: crate::HoudiniEngineUnity::HAPI_GroupType,
        groupName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        membershipArrayAllEqual: quest_hook::libil2cpp::ByRefMut<bool>,
        membershipArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetGroupMembership",
                (
                    nodeID,
                    partID,
                    groupType,
                    groupName,
                    membershipArrayAllEqual,
                    membershipArray,
                    start,
                    length,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGroupMembershipOnPackedInstancePart(
        &mut self,
        nodeID: i32,
        partID: i32,
        groupType: crate::HoudiniEngineUnity::HAPI_GroupType,
        groupName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        membershipArrayAllEqual: quest_hook::libil2cpp::ByRefMut<bool>,
        membershipArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetGroupMembershipOnPackedInstancePart",
                (
                    nodeID,
                    partID,
                    groupType,
                    groupName,
                    membershipArrayAllEqual,
                    membershipArray,
                    start,
                    length,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGroupNames(
        &mut self,
        nodeID: i32,
        groupType: crate::HoudiniEngineUnity::HAPI_GroupType,
        names: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetGroupNames", (nodeID, groupType, names, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGroupNamesOnPackedInstancePart(
        &mut self,
        nodeID: i32,
        partID: i32,
        groupType: crate::HoudiniEngineUnity::HAPI_GroupType,
        groupNamesArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        groupCount: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetGroupNamesOnPackedInstancePart",
                (nodeID, partID, groupType, groupNamesArray, groupCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHandleBindingInfo(
        &mut self,
        nodeID: i32,
        handleIndex: i32,
        handleBindingInfos: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::HoudiniEngineUnity::HAPI_HandleBindingInfo,
                >,
            >,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetHandleBindingInfo",
                (nodeID, handleIndex, handleBindingInfos, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHandleInfo(
        &mut self,
        nodeID: i32,
        handleInfos: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::HoudiniEngineUnity::HAPI_HandleInfo,
                >,
            >,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetHandleInfo", (nodeID, handleInfos, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHeightFieldData(
        &mut self,
        nodeID: i32,
        partID: i32,
        valuesArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetHeightFieldData", (nodeID, partID, valuesArray, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetImageInfo(
        &mut self,
        materialNodeID: i32,
        imageInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_ImageInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetImageInfo", (materialNodeID, imageInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetImagePlanes(
        &mut self,
        nodeID: i32,
        imagePlanes: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        numImagePlanes: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetImagePlanes", (nodeID, imagePlanes, numImagePlanes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstanceTransformsOnPart(
        &mut self,
        nodeID: i32,
        partID: i32,
        rstOrder: crate::HoudiniEngineUnity::HAPI_RSTOrder,
        transformsArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::HoudiniEngineUnity::HAPI_Transform,
                >,
            >,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetInstanceTransformsOnPart",
                (nodeID, partID, rstOrder, transformsArray, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstancedObjectIds(
        &mut self,
        nodeID: i32,
        instanced_node_id_array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetInstancedObjectIds",
                (nodeID, instanced_node_id_array, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstancedPartIds(
        &mut self,
        nodeID: i32,
        partID: i32,
        instancedPartsArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetInstancedPartIds",
                (nodeID, partID, instancedPartsArray, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstancerPartTransforms(
        &mut self,
        nodeID: i32,
        partID: i32,
        rstOrder: crate::HoudiniEngineUnity::HAPI_RSTOrder,
        transformsArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::HoudiniEngineUnity::HAPI_Transform,
                >,
            >,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetInstancerPartTransforms",
                (nodeID, partID, rstOrder, transformsArray, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastSessionError(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetLastSessionError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaterialInfo(
        &mut self,
        materialNodeID: i32,
        materialInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_MaterialInfo,
        >,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetMaterialInfo", (materialNodeID, materialInfo, bLogError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaterialNodeIDsOnFaces(
        &mut self,
        nodeID: i32,
        partID: i32,
        bSingleFaceMaterial: quest_hook::libil2cpp::ByRefMut<bool>,
        materialNodeIDs: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        faceCount: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetMaterialNodeIDsOnFaces",
                (nodeID, partID, bSingleFaceMaterial, materialNodeIDs, faceCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaterialOnPart(
        &mut self,
        nodeID: i32,
        partID: i32,
        materialInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_MaterialInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetMaterialOnPart", (nodeID, partID, materialInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeInfo(
        &mut self,
        nodeID: i32,
        nodeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_NodeInfo,
        >,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetNodeInfo", (nodeID, nodeInfo, bLogError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeInputName(
        &mut self,
        nodeID: i32,
        inputIndex: i32,
        nodeNameIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetNodeInputName", (nodeID, inputIndex, nodeNameIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodePath(
        &mut self,
        nodeID: i32,
        relativeNodeID: i32,
        path: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetNodePath", (nodeID, relativeNodeID, path))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectInfo(
        &mut self,
        nodeID: i32,
        objectInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_ObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetObjectInfo", (nodeID, objectInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectTransform(
        &mut self,
        nodeID: i32,
        relativeToNodeID: i32,
        rstOrder: crate::HoudiniEngineUnity::HAPI_RSTOrder,
        hapiTransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Transform,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetObjectTransform",
                (nodeID, relativeToNodeID, rstOrder, hapiTransform),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParamChoiceValues(
        &mut self,
        nodeID: i32,
        values: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::HoudiniEngineUnity::HAPI_ParmChoiceInfo,
                >,
            >,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParamChoiceValues", (nodeID, values, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParamFloatValue(
        &mut self,
        nodeID: i32,
        parmName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        value: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParamFloatValue", (nodeID, parmName, index, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParamFloatValues(
        &mut self,
        nodeID: i32,
        values: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParamFloatValues", (nodeID, values, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParamIntValue(
        &mut self,
        nodeID: i32,
        parmName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParamIntValue", (nodeID, parmName, index, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParamIntValues(
        &mut self,
        nodeID: i32,
        values: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParamIntValues", (nodeID, values, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParamNodeValue(
        &mut self,
        nodeID: i32,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nodeValue: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParamNodeValue", (nodeID, paramName, nodeValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParamStringValue(
        &mut self,
        nodeID: i32,
        parmName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParamStringValue", (nodeID, parmName, index, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParamStringValues(
        &mut self,
        nodeID: i32,
        values: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParamStringValues", (nodeID, values, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParams(
        &mut self,
        nodeID: i32,
        parmInfos: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::HoudiniEngineUnity::HAPI_ParmInfo,
                >,
            >,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParams", (nodeID, parmInfos, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParmIDFromName(
        &mut self,
        nodeID: i32,
        parmName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parmID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParmIDFromName", (nodeID, parmName, parmID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParmStringValue(
        &mut self,
        nodeID: i32,
        parmName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        evaluate: bool,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParmStringValue", (nodeID, parmName, index, evaluate, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParmTagName(
        &mut self,
        nodeID: i32,
        parmID: i32,
        tagIndex: i32,
        tagName: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParmTagName", (nodeID, parmID, tagIndex, tagName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParmTagValue(
        &mut self,
        nodeID: i32,
        parmID: i32,
        tagName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tagValue: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParmTagValue", (nodeID, parmID, tagName, tagValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParmWithTag(
        &mut self,
        nodeID: i32,
        tagName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parmID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetParmWithTag", (nodeID, tagName, parmID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPartInfo(
        &mut self,
        nodeID: i32,
        partID: i32,
        partInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PartInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetPartInfo", (nodeID, partID, partInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreset(
        &mut self,
        nodeID: i32,
        presetData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetPreset", (nodeID, presetData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRegisteredAssetFromID(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        > = __cordl_object.invoke("GetRegisteredAssetFromID", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetServerEnvString(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetServerEnvString", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetServerEnvVarCount(
        &mut self,
        env_count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetServerEnvVarCount", (env_count))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSessionData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionData,
        > = __cordl_object.invoke("GetSessionData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSessionEnvInt(
        &mut self,
        intType: crate::HoudiniEngineUnity::HAPI_SessionEnvIntType,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetSessionEnvInt", (intType, bLogError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSessionErrorMsg(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetSessionErrorMsg", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSessionInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetSessionInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSessionSyncInfo(
        &mut self,
        syncInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_SessionSyncInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetSessionSyncInfo", (syncInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSphereInfo(
        &mut self,
        nodeID: i32,
        partID: i32,
        sphereInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_SphereInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetSphereInfo", (nodeID, partID, sphereInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStatusString(
        &mut self,
        statusType: crate::HoudiniEngineUnity::HAPI_StatusType,
        verbosity: crate::HoudiniEngineUnity::HAPI_StatusVerbosity,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetStatusString", (statusType, verbosity))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetString(
        &mut self,
        stringHandle: i32,
        resultString: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        bufferLength: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetString", (stringHandle, resultString, bufferLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringBufferLength(
        &mut self,
        stringHandle: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetStringBufferLength", (stringHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTotalCookCount(
        &mut self,
        nodeID: i32,
        nodeTypeFilter: i32,
        nodeFlagFilter: i32,
        includeChildren: bool,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetTotalCookCount",
                (nodeID, nodeTypeFilter, nodeFlagFilter, includeChildren, count),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUseHoudiniTime(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetUseHoudiniTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexList(
        &mut self,
        nodeID: i32,
        partID: i32,
        vertexList: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetVertexList", (nodeID, partID, vertexList, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetViewport(
        &mut self,
        viewport: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Viewport,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetViewport", (viewport))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVolumeBounds(
        &mut self,
        nodeID: i32,
        partID: i32,
        x_min: quest_hook::libil2cpp::ByRefMut<f32>,
        y_min: quest_hook::libil2cpp::ByRefMut<f32>,
        z_min: quest_hook::libil2cpp::ByRefMut<f32>,
        x_max: quest_hook::libil2cpp::ByRefMut<f32>,
        y_max: quest_hook::libil2cpp::ByRefMut<f32>,
        z_max: quest_hook::libil2cpp::ByRefMut<f32>,
        x_center: quest_hook::libil2cpp::ByRefMut<f32>,
        y_center: quest_hook::libil2cpp::ByRefMut<f32>,
        z_center: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetVolumeBounds",
                (
                    nodeID,
                    partID,
                    x_min,
                    y_min,
                    z_min,
                    x_max,
                    y_max,
                    z_max,
                    x_center,
                    y_center,
                    z_center,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVolumeInfo(
        &mut self,
        nodeID: i32,
        partID: i32,
        volumeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_VolumeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetVolumeInfo", (nodeID, partID, volumeInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleStatusResult(
        &mut self,
        result: crate::HoudiniEngineUnity::HAPI_Result,
        prependMsg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bThrowError: bool,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HandleStatusResult", (result, prependMsg, bThrowError, bLogError))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeSession(
        &mut self,
        sessionData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InitializeSession", (sessionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertMultiparmInstance(
        &mut self,
        nodeID: i32,
        parmID: i32,
        instancePosition: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InsertMultiparmInstance", (nodeID, parmID, instancePosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAssetRegistered(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAssetRegistered", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNodeValid(
        &mut self,
        nodeID: i32,
        uniqueNodeID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsNodeValid", (nodeID, uniqueNodeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSessionSync(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSessionSync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSessionValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSessionValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetLibraryFromFile(
        &mut self,
        assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bAllowOverwrite: bool,
        libraryID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "LoadAssetLibraryFromFile",
                (assetPath, bAllowOverwrite, libraryID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetLibraryFromMemory(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        bAllowOverwrite: bool,
        libraryID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("LoadAssetLibraryFromMemory", (buffer, bAllowOverwrite, libraryID))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadGeoFromFile(
        &mut self,
        nodeID: i32,
        file_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("LoadGeoFromFile", (nodeID, file_name))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadHIPFile(
        &mut self,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bCookOnLoad: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("LoadHIPFile", (fileName, bCookOnLoad))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadNodeFromFile(
        &mut self,
        file_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parentNodeID: i32,
        nodeLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cook_on_load: bool,
        newNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "LoadNodeFromFile",
                (file_name, parentNodeID, nodeLabel, cook_on_load, newNodeID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ParmHasTag(
        &mut self,
        nodeID: i32,
        parmID: i32,
        tagName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hasTag: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ParmHasTag", (nodeID, parmID, tagName, hasTag))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueryNodeInput(
        &mut self,
        nodeID: i32,
        inputIndex: i32,
        connectedNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("QueryNodeInput", (nodeID, inputIndex, connectedNodeID, bLogError))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterAsset(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterAsset", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveMultiParmInstance(
        &mut self,
        nodeID: i32,
        parmID: i32,
        instancePosition: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RemoveMultiParmInstance", (nodeID, parmID, instancePosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn RenameNode(
        &mut self,
        nodeID: i32,
        newName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RenameNode", (nodeID, newName))?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderCOPToImage(
        &mut self,
        copNodeID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RenderCOPToImage", (copNodeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderTextureToImage(
        &mut self,
        materialNodeID: i32,
        parmID: i32,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RenderTextureToImage", (materialNodeID, parmID, bLogError))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReregisterOnAwake(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReregisterOnAwake", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn RestartSession(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RestartSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RevertGeo(&mut self, nodeID: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RevertGeo", (nodeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn RevertParmToDefault(
        &mut self,
        nodeID: i32,
        parm_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RevertParmToDefault", (nodeID, parm_name, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn RevertParmToDefaults(
        &mut self,
        nodeID: i32,
        parm_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RevertParmToDefaults", (nodeID, parm_name))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveGeoToFile(
        &mut self,
        nodeID: i32,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SaveGeoToFile", (nodeID, fileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveHIPFile(
        &mut self,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bLockNodes: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SaveHIPFile", (fileName, bLockNodes))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveNodeToFile(
        &mut self,
        nodeID: i32,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SaveNodeToFile", (nodeID, fileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAttributeFloatData(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetAttributeFloatData",
                (nodeID, partID, name, attrInfo, data, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAttributeInt16Data(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetAttributeInt16Data",
                (nodeID, partID, name, attrInfo, data, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAttributeInt64Data(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i64>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetAttributeInt64Data",
                (nodeID, partID, name, attrInfo, data, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAttributeInt8Data(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i8>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetAttributeInt8Data",
                (nodeID, partID, name, attrInfo, data, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAttributeIntData(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetAttributeIntData",
                (nodeID, partID, name, attrInfo, data, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAttributeStringData(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        data: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetAttributeStringData",
                (nodeID, partID, name, attrInfo, data, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCacheProperty(
        &mut self,
        cacheName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cacheProperty: crate::HoudiniEngineUnity::HAPI_CacheProperty,
        propertyValue: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetCacheProperty", (cacheName, cacheProperty, propertyValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCurveCounts(
        &mut self,
        nodeID: i32,
        partID: i32,
        counts: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetCurveCounts", (nodeID, partID, counts, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCurveInfo(
        &mut self,
        nodeID: i32,
        partID: i32,
        curveInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_CurveInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetCurveInfo", (nodeID, partID, curveInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCurveKnots(
        &mut self,
        nodeID: i32,
        partID: i32,
        knots: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetCurveKnots", (nodeID, partID, knots, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCurveOrders(
        &mut self,
        nodeID: i32,
        partID: i32,
        orders: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetCurveOrders", (nodeID, partID, orders, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFaceCount(
        &mut self,
        nodeID: i32,
        partID: i32,
        faceCounts: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetFaceCount", (nodeID, partID, faceCounts, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGroupMembership(
        &mut self,
        nodeID: i32,
        partID: i32,
        groupType: crate::HoudiniEngineUnity::HAPI_GroupType,
        groupName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        membershipArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetGroupMembership",
                (nodeID, partID, groupType, groupName, membershipArray, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetHeightFieldData(
        &mut self,
        nodeID: i32,
        partID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        valuesArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetHeightFieldData",
                (nodeID, partID, name, valuesArray, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetImageInfo(
        &mut self,
        materialNodeID: i32,
        imageInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_ImageInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetImageInfo", (materialNodeID, imageInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLibraryErrorMsg(
        &mut self,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLibraryErrorMsg", (bLogError))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNodeDisplay(
        &mut self,
        node_id: i32,
        onOff: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetNodeDisplay", (node_id, onOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetObjectTransform(
        &mut self,
        nodeID: i32,
        hapiTransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_TransformEuler,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetObjectTransform", (nodeID, hapiTransform))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParamFloatValue(
        &mut self,
        nodeID: i32,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetParamFloatValue", (nodeID, paramName, index, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParamFloatValues(
        &mut self,
        nodeID: i32,
        values: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetParamFloatValues", (nodeID, values, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParamIntValue(
        &mut self,
        nodeID: i32,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetParamIntValue", (nodeID, paramName, index, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParamIntValues(
        &mut self,
        nodeID: i32,
        values: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetParamIntValues", (nodeID, values, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParamNodeValue(
        &mut self,
        nodeID: i32,
        paramName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nodeValueID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetParamNodeValue", (nodeID, paramName, nodeValueID))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParamStringValue_Il2CppString1(
        &mut self,
        nodeID: i32,
        parmName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parmValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetParamStringValue", (nodeID, parmName, parmValue, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParamStringValue_i32_0(
        &mut self,
        nodeID: i32,
        strValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parmID: i32,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetParamStringValue", (nodeID, strValue, parmID, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPartInfo(
        &mut self,
        nodeID: i32,
        partID: i32,
        partInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PartInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetPartInfo", (nodeID, partID, partInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPreset(
        &mut self,
        nodeID: i32,
        presetData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetPreset", (nodeID, presetData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetServerEnvString(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetServerEnvString", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSessionConnectionErrorMsg(
        &mut self,
        introMsg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: crate::HoudiniEngineUnity::HAPI_Result,
        bIsHARSRunning: bool,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetSessionConnectionErrorMsg",
                (introMsg, result, bIsHARSRunning, bLogError),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSessionData(
        &mut self,
        sessionData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSessionData", (sessionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSessionErrorMsg(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSessionErrorMsg", (msg, bLogError))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSessionSync(
        &mut self,
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SetSessionSync", (enable))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSessionSyncInfo(
        &mut self,
        syncInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_SessionSyncInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SetSessionSyncInfo", (syncInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTime(&mut self, _cordl_time: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SetTime", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUseHoudiniTime(
        &mut self,
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SetUseHoudiniTime", (enable))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertexList(
        &mut self,
        nodeID: i32,
        partID: i32,
        vertexList: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetVertexList", (nodeID, partID, vertexList, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetViewport(
        &mut self,
        viewport: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Viewport,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SetViewport", (viewport))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVolumeInfo(
        &mut self,
        nodeID: i32,
        partID: i32,
        volumeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_VolumeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetVolumeInfo", (nodeID, partID, volumeInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVolumeTileFloatData(
        &mut self,
        nodeID: i32,
        partID: i32,
        tileInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_VolumeTileInfo,
        >,
        valuesArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetVolumeTileFloatData",
                (nodeID, partID, tileInfo, valuesArray, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterAsset(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterAsset", (id))?;
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
    pub fn get_ConnectionState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::SessionConnectionState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::SessionConnectionState = __cordl_object
            .invoke("get_ConnectionState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LastCallResultCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::HAPI_Result> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::HAPI_Result = __cordl_object
            .invoke("get_LastCallResultCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LogErrorOverride(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_LogErrorOverride", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ThisSessionMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::SessionMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::SessionMode = __cordl_object
            .invoke("get_ThisSessionMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ThrowErrorOverride(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ThrowErrorOverride", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserNotifiedSessionInvalid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_UserNotifiedSessionInvalid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ConnectionState(
        &mut self,
        value: crate::HoudiniEngineUnity::SessionConnectionState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ConnectionState", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LastCallResultCode(
        &mut self,
        value: crate::HoudiniEngineUnity::HAPI_Result,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LastCallResultCode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LogErrorOverride(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LogErrorOverride", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ThisSessionMode(
        &mut self,
        value: crate::HoudiniEngineUnity::SessionMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ThisSessionMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ThrowErrorOverride(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ThrowErrorOverride", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UserNotifiedSessionInvalid(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UserNotifiedSessionInvalid", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionBase")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_SessionBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
