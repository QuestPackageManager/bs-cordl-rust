#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_SessionManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_SessionManager =>
    "HoudiniEngineUnity"."HEU_SessionManager"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_SessionManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_SessionManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager")]
impl crate::HoudiniEngineUnity::HEU_SessionManager {
    #[cfg(
        feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate"
    )]
    pub type CreateSessionFromTypeDelegate = crate::HoudiniEngineUnity::HEU_SessionManager_CreateSessionFromTypeDelegate;
    pub fn CheckAndCloseExistingSession() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckAndCloseExistingSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckVersionMatch() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckVersionMatch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearConnectionError() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearConnectionError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CloseAllSessions() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CloseAllSessions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CloseDefaultSession() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CloseDefaultSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ConnectSessionSyncUsingThriftPipe(
        pipeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        autoClose: bool,
        timeout: f32,
        logError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConnectSessionSyncUsingThriftPipe",
                (pipeName, autoClose, timeout, logError),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConnectSessionSyncUsingThriftSocket(
        hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        serverPort: i32,
        autoClose: bool,
        timeout: f32,
        logError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConnectSessionSyncUsingThriftSocket",
                (hostName, serverPort, autoClose, timeout, logError),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConnectThriftPipeSession(
        pipeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        autoClose: bool,
        timeout: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConnectThriftPipeSession", (pipeName, autoClose, timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConnectThriftSocketSession(
        hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        serverPort: i32,
        autoClose: bool,
        timeout: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConnectThriftSocketSession",
                (hostName, serverPort, autoClose, timeout),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCustomSession() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCustomSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInProcessSession() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInProcessSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSessionFromType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSessionFromType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSessionObject() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSessionObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateThriftPipeSession(
        pipeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        autoClose: bool,
        timeout: f32,
        logError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateThriftPipeSession",
                (pipeName, autoClose, timeout, logError),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateThriftSocketSession(
        hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        serverPort: i32,
        autoClose: bool,
        timeout: f32,
        logError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateThriftSocketSession",
                (hostName, serverPort, autoClose, timeout, logError),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComposedChildNodeList(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        parentNodeID: i32,
        nodeTypeFilter: i32,
        nodeFlagFilter: i32,
        bRecursive: bool,
        childNodeIDs: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetComposedChildNodeList",
                (
                    session,
                    parentNodeID,
                    nodeTypeFilter,
                    nodeFlagFilter,
                    bRecursive,
                    childNodeIDs,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComposedObjectListMemorySafe(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetComposedObjectListMemorySafe",
                (session, nodeID, objectInfos, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComposedObjectTransformsMemorySafe(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetComposedObjectTransformsMemorySafe",
                (session, nodeID, rstOrder, transforms, start, length),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConnectionError(
        clear: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetConnectionError", (clear))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentLicense(
        bLogError: bool,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::HAPI_License> {
        let __cordl_ret: crate::HoudiniEngineUnity::HAPI_License = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentLicense", (bLogError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultSession() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGroupMembership(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        partID: i32,
        groupType: crate::HoudiniEngineUnity::HAPI_GroupType,
        groupName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        membership: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        isInstanced: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetGroupMembership",
                (session, nodeID, partID, groupType, groupName, membership, isInstanced),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGroupNames(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        partID: i32,
        groupType: crate::HoudiniEngineUnity::HAPI_GroupType,
        isInstanced: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGroupNames", (session, nodeID, partID, groupType, isInstanced))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHoudiniPathOnMacOS(
        houdiniPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHoudiniPathOnMacOS", (houdiniPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastSessionError() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLastSessionError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeInputName(
        nodeID: i32,
        inputIndex: i32,
        inputName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodeInputName", (nodeID, inputIndex, inputName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeName(
        nodeID: i32,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodeName", (nodeID, session))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreateDefaultSession(
        bNotifyUserError: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrCreateDefaultSession", (bNotifyUserError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSessionData() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionData,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetSessionData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSessionInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetSessionInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSessionWithID(
        sessionID: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSessionWithID", (sessionID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetString(
        stringHandle: i32,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetString", (stringHandle, session))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringValuesFromStringIndices(
        strIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStringValuesFromStringIndices", (strIndices))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUniqueMaterialShopName(
        assetID: i32,
        materialID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUniqueMaterialShopName", (assetID, materialID))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeDefaultSession() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeDefaultSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalValidateSceneAssets() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalValidateSceneAssets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHARSProcessRunning(processID: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHARSProcessRunning", (processID))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAllSessionData() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAllSessionData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSessionFromHIP(
        bCookNodes: bool,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadSessionFromHIP", (bCookNodes, session))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadStoredDefaultSession() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadStoredDefaultSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenHoudini(
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenHoudini", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenSessionInHoudini(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenSessionInHoudini", (session))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecreateDefaultSessionData() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RecreateDefaultSessionData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterSession(
        sessionID: i64,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterSession", (sessionID, session))?;
        Ok(__cordl_ret.into())
    }
    pub fn RestartSession() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RestartSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveAllSessionData() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveAllSessionData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveSessionToHIP(
        bLockNodes: bool,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveSessionToHIP", (bLockNodes, session))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterSession(
        sessionID: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnregisterSession", (sessionID))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidatePluginSession(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidatePluginSession", (session))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_SessionManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_SessionManager_CreateSessionFromTypeDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_SessionManager_CreateSessionFromTypeDelegate =>
    "HoudiniEngineUnity"."HEU_SessionManager/CreateSessionFromTypeDelegate"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_SessionManager_CreateSessionFromTypeDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_SessionManager_CreateSessionFromTypeDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate")]
impl crate::HoudiniEngineUnity::HEU_SessionManager_CreateSessionFromTypeDelegate {
    pub fn BeginInvoke(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (_cordl_type, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionBase,
        > = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionBase,
        > = __cordl_object.invoke("Invoke", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_SessionManager_CreateSessionFromTypeDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
