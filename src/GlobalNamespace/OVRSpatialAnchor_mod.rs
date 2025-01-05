#[cfg(feature = "OVRSpatialAnchor")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSpatialAnchor {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _startCalled: bool,
    pub _requestId: u64,
    pub _defaultSaveOptions: crate::GlobalNamespace::OVRSpatialAnchor_SaveOptions,
    pub _defaultEraseOptions: crate::GlobalNamespace::OVRSpatialAnchor_EraseOptions,
    pub OnLocalize: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
    >,
    pub _Space_k__BackingField: crate::GlobalNamespace::OVRSpace,
    pub _Uuid_k__BackingField: crate::System::Guid,
}
#[cfg(feature = "OVRSpatialAnchor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSpatialAnchor => ""
    ."OVRSpatialAnchor"
);
#[cfg(feature = "OVRSpatialAnchor")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSpatialAnchor {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSpatialAnchor")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRSpatialAnchor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSpatialAnchor")]
impl crate::GlobalNamespace::OVRSpatialAnchor {
    #[cfg(feature = "OVRSpatialAnchor+Development")]
    pub type Development = crate::GlobalNamespace::OVRSpatialAnchor_Development;
    #[cfg(feature = "OVRSpatialAnchor+EraseOptions")]
    pub type EraseOptions = crate::GlobalNamespace::OVRSpatialAnchor_EraseOptions;
    #[cfg(feature = "OVRSpatialAnchor+InvertedCapture_2")]
    pub type InvertedCapture_2<
        TResult: quest_hook::libil2cpp::Type,
        TCapture: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::OVRSpatialAnchor_InvertedCapture_2<TResult, TCapture>;
    #[cfg(feature = "OVRSpatialAnchor+LoadOptions")]
    pub type LoadOptions = crate::GlobalNamespace::OVRSpatialAnchor_LoadOptions;
    #[cfg(feature = "OVRSpatialAnchor+MultiAnchorActionType")]
    pub type MultiAnchorActionType = crate::GlobalNamespace::OVRSpatialAnchor_MultiAnchorActionType;
    #[cfg(feature = "OVRSpatialAnchor+MultiAnchorDelegatePair")]
    pub type MultiAnchorDelegatePair = crate::GlobalNamespace::OVRSpatialAnchor_MultiAnchorDelegatePair;
    #[cfg(feature = "OVRSpatialAnchor+OperationResult")]
    pub type OperationResult = crate::GlobalNamespace::OVRSpatialAnchor_OperationResult;
    #[cfg(feature = "OVRSpatialAnchor+SaveOptions")]
    pub type SaveOptions = crate::GlobalNamespace::OVRSpatialAnchor_SaveOptions;
    #[cfg(feature = "OVRSpatialAnchor+UnboundAnchor")]
    pub type UnboundAnchor = crate::GlobalNamespace::OVRSpatialAnchor_UnboundAnchor;
    pub fn AreSortedUserListsEqual(
        sortedList1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpaceUser>,
        sortedList2: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpaceUser>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreSortedUserListsEqual", (sortedList1, sortedList2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyAnchorListIntoListFromPool(
        anchorList: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyAnchorListIntoListFromPool", (anchorList))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpatialAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateSpatialAnchor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EraseAsync_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = __cordl_object
            .invoke("EraseAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EraseAsync_OVRSpatialAnchor_EraseOptions1(
        &mut self,
        eraseOptions: crate::GlobalNamespace::OVRSpatialAnchor_EraseOptions,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = __cordl_object
            .invoke("EraseAsync", (eraseOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn Erase_Gc0(
        &mut self,
        onComplete: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
            bool,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Erase", (onComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn Erase_OVRSpatialAnchor_EraseOptions_Gc1(
        &mut self,
        eraseOptions: crate::GlobalNamespace::OVRSpatialAnchor_EraseOptions,
        onComplete: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
            bool,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Erase", (eraseOptions, onComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetListToStoreTheShareRequest(
        &mut self,
        users: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpaceUser>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
        > = __cordl_object.invoke("GetListToStoreTheShareRequest", (users))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackingSpacePose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPose = __cordl_object
            .invoke("GetTrackingSpacePose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeFromExisting(
        &mut self,
        space: crate::GlobalNamespace::OVRSpace,
        uuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeFromExisting", (space, uuid))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeOnLoad() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeOnLoad", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeUnchecked(
        &mut self,
        space: crate::GlobalNamespace::OVRSpace,
        uuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeUnchecked", (space, uuid))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeMultiAnchorDelegate(
        requestId: u64,
        result: crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        actionType: crate::GlobalNamespace::OVRSpatialAnchor_MultiAnchorActionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeMultiAnchorDelegate", (requestId, result, actionType))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadUnboundAnchors(
        options: crate::GlobalNamespace::OVRSpatialAnchor_LoadOptions,
        onComplete: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::GlobalNamespace::OVRSpatialAnchor_UnboundAnchor,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadUnboundAnchors", (options, onComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadUnboundAnchorsAsync(
        options: crate::GlobalNamespace::OVRSpatialAnchor_LoadOptions,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::GlobalNamespace::OVRSpatialAnchor_UnboundAnchor,
                >,
            >,
        >,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::GlobalNamespace::OVRSpatialAnchor_UnboundAnchor,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadUnboundAnchorsAsync", (options))?;
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
    pub fn OnShareSpacesComplete(
        requestId: u64,
        result: crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnShareSpacesComplete", (requestId, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSpaceEraseComplete(
        requestId: u64,
        result: bool,
        uuid: crate::System::Guid,
        location: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSpaceEraseComplete", (requestId, result, uuid, location))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSpaceListSaveComplete(
        requestId: u64,
        result: crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSpaceListSaveComplete", (requestId, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSpaceQueryComplete(
        requestId: u64,
        queryResult: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSpaceQueryComplete", (requestId, queryResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSpaceSaveComplete(
        requestId: u64,
        space: crate::GlobalNamespace::OVRSpace,
        result: bool,
        uuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSpaceSaveComplete", (requestId, space, result, uuid))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSpaceSetComponentStatusComplete(
        requestId: u64,
        result: bool,
        space: crate::GlobalNamespace::OVRSpace,
        uuid: crate::System::Guid,
        componentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OnSpaceSetComponentStatusComplete",
                (requestId, result, space, uuid, componentType, enabled),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSpatialAnchorCreateComplete(
        requestId: u64,
        success: bool,
        space: crate::GlobalNamespace::OVRSpace,
        uuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSpatialAnchorCreateComplete", (requestId, success, space, uuid))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateUnbound(
        uuid: crate::System::Guid,
        space: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PopulateUnbound", (uuid, space))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveAsync_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = __cordl_object
            .invoke("SaveAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveAsync_OVRSpatialAnchor_SaveOptions1(
        &mut self,
        saveOptions: crate::GlobalNamespace::OVRSpatialAnchor_SaveOptions,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = __cordl_object
            .invoke("SaveAsync", (saveOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveBatchAnchors() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveBatchAnchors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Save_Gc0(
        &mut self,
        onComplete: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
            bool,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", (onComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn Save_Gc_OVRSpatialAnchor_SaveOptions_Gc2(
        anchors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
        >,
        saveOptions: crate::GlobalNamespace::OVRSpatialAnchor_SaveOptions,
        onComplete: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
            >,
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Save", (anchors, saveOptions, onComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn Save_OVRSpatialAnchor_SaveOptions_Gc1(
        &mut self,
        saveOptions: crate::GlobalNamespace::OVRSpatialAnchor_SaveOptions,
        onComplete: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
            bool,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", (saveOptions, onComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShareAsyncInternal(
        &mut self,
        users: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpaceUser>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        > = __cordl_object.invoke("ShareAsyncInternal", (users))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShareAsync_Gc4(
        &mut self,
        users: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpaceUser>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        > = __cordl_object.invoke("ShareAsync", (users))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShareAsync_OVRSpaceUser0(
        &mut self,
        user: crate::GlobalNamespace::OVRSpaceUser,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        > = __cordl_object.invoke("ShareAsync", (user))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShareAsync_OVRSpaceUser_OVRSpaceUser1(
        &mut self,
        user1: crate::GlobalNamespace::OVRSpaceUser,
        user2: crate::GlobalNamespace::OVRSpaceUser,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        > = __cordl_object.invoke("ShareAsync", (user1, user2))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShareAsync_OVRSpaceUser_OVRSpaceUser_OVRSpaceUser2(
        &mut self,
        user1: crate::GlobalNamespace::OVRSpaceUser,
        user2: crate::GlobalNamespace::OVRSpaceUser,
        user3: crate::GlobalNamespace::OVRSpaceUser,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        > = __cordl_object.invoke("ShareAsync", (user1, user2, user3))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShareAsync_OVRSpaceUser_OVRSpaceUser_OVRSpaceUser_OVRSpaceUser3(
        &mut self,
        user1: crate::GlobalNamespace::OVRSpaceUser,
        user2: crate::GlobalNamespace::OVRSpaceUser,
        user3: crate::GlobalNamespace::OVRSpaceUser,
        user4: crate::GlobalNamespace::OVRSpaceUser,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        > = __cordl_object.invoke("ShareAsync", (user1, user2, user3, user4))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShareBatchAnchors() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShareBatchAnchors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Share_Gc_Gc4(
        &mut self,
        users: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpaceUser>,
        onComplete: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Share", (users, onComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn Share_Gc_Gc_Gc5(
        anchors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
        >,
        users: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpaceUser>,
        onComplete: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
            >,
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Share", (anchors, users, onComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn Share_OVRSpaceUser_Gc0(
        &mut self,
        user: crate::GlobalNamespace::OVRSpaceUser,
        onComplete: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Share", (user, onComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn Share_OVRSpaceUser_OVRSpaceUser_Gc1(
        &mut self,
        user1: crate::GlobalNamespace::OVRSpaceUser,
        user2: crate::GlobalNamespace::OVRSpaceUser,
        onComplete: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Share", (user1, user2, onComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn Share_OVRSpaceUser_OVRSpaceUser_OVRSpaceUser_Gc2(
        &mut self,
        user1: crate::GlobalNamespace::OVRSpaceUser,
        user2: crate::GlobalNamespace::OVRSpaceUser,
        user3: crate::GlobalNamespace::OVRSpaceUser,
        onComplete: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Share", (user1, user2, user3, onComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn Share_OVRSpaceUser_OVRSpaceUser_OVRSpaceUser_OVRSpaceUser_Gc3(
        &mut self,
        user1: crate::GlobalNamespace::OVRSpaceUser,
        user2: crate::GlobalNamespace::OVRSpaceUser,
        user3: crate::GlobalNamespace::OVRSpaceUser,
        user4: crate::GlobalNamespace::OVRSpaceUser,
        onComplete: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Share", (user1, user2, user3, user4, onComplete))?;
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
    pub fn ThrowIfBound(
        uuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowIfBound", (uuid))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToNativeArray(
        anchors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<u64>> {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<u64> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToNativeArray", (anchors))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryExtractValue<TKey, TValue>(
        dict: quest_hook::libil2cpp::Gc<TKey, TValue>,
        key: TKey,
        value: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryExtractValue", (dict, key, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPose(
        space: crate::GlobalNamespace::OVRSpace,
        pose: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPose>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetPose", (space, pose))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTransform", ())?;
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
    pub fn add_OnLocalize(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_OnLocalize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Created(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Created", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Localized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Localized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PendingCreation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_PendingCreation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Space(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRSpace> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRSpace = __cordl_object
            .invoke("get_Space", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Uuid(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object.invoke("get_Uuid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_OnLocalize(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_OnLocalize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Space(
        &mut self,
        value: crate::GlobalNamespace::OVRSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Space", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Uuid(
        &mut self,
        value: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Uuid", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSpatialAnchor")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRSpatialAnchor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSpatialAnchor+Development")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSpatialAnchor_Development {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "OVRSpatialAnchor+Development")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSpatialAnchor_Development =>
    ""."OVRSpatialAnchor/Development"
);
#[cfg(feature = "OVRSpatialAnchor+Development")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSpatialAnchor_Development {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSpatialAnchor+Development")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRSpatialAnchor_Development {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSpatialAnchor+Development")]
impl crate::GlobalNamespace::OVRSpatialAnchor_Development {
    pub fn Log(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogError(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogError", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogRequest(
        requestId: u64,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogRequest", (requestId, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogRequestResult(
        requestId: u64,
        result: bool,
        successMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        failureMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LogRequestResult",
                (requestId, result, successMessage, failureMessage),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LogWarning(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogWarning", (message))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSpatialAnchor+Development")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRSpatialAnchor_Development {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRSpatialAnchor+EraseOptions")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OVRSpatialAnchor_EraseOptions {
    pub Storage: crate::GlobalNamespace::OVRSpace_StorageLocation,
}
#[cfg(feature = "OVRSpatialAnchor+EraseOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSpatialAnchor_EraseOptions
    => ""."OVRSpatialAnchor/EraseOptions"
);
#[cfg(feature = "OVRSpatialAnchor+EraseOptions")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSpatialAnchor_EraseOptions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSpatialAnchor+EraseOptions")]
impl crate::GlobalNamespace::OVRSpatialAnchor_EraseOptions {}
#[cfg(feature = "OVRSpatialAnchor+InvertedCapture_2")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OVRSpatialAnchor_InvertedCapture_2<
    TResult: quest_hook::libil2cpp::Type,
    TCapture: quest_hook::libil2cpp::Type,
> {
    pub _capture: TCapture,
    pub _callback: quest_hook::libil2cpp::Gc<TCapture, TResult>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
    __cordl_phantom_TCapture: std::marker::PhantomData<TCapture>,
}
#[cfg(feature = "OVRSpatialAnchor+InvertedCapture_2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSpatialAnchor_InvertedCapture_2 < TResult, TCapture > => ""
    ."OVRSpatialAnchor/InvertedCapture`2<TResult,TCapture>" < TResult, TCapture >
);
#[cfg(feature = "OVRSpatialAnchor+InvertedCapture_2")]
unsafe impl<
    TResult: quest_hook::libil2cpp::Type,
    TCapture: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSpatialAnchor_InvertedCapture_2<TResult, TCapture> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSpatialAnchor+InvertedCapture_2")]
impl<
    TResult: quest_hook::libil2cpp::Type,
    TCapture: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::OVRSpatialAnchor_InvertedCapture_2<TResult, TCapture> {
    pub fn ContinueTaskWith(
        task: crate::GlobalNamespace::OVRTask_1<TResult>,
        onCompleted: quest_hook::libil2cpp::Gc<TCapture, TResult>,
        state: TCapture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCapture: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContinueTaskWith", (task, onCompleted, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        result: TResult,
        invertedCapture: crate::GlobalNamespace::OVRSpatialAnchor_InvertedCapture_2<
            TResult,
            TCapture,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCapture: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Invoke", (result, invertedCapture))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<TCapture, TResult>,
        capture: TCapture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TCapture: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (callback, capture),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSpatialAnchor+LoadOptions")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OVRSpatialAnchor_LoadOptions {
    pub _StorageLocation_k__BackingField: crate::GlobalNamespace::OVRSpace_StorageLocation,
    pub _MaxAnchorCount_k__BackingField: i32,
    pub _Timeout_k__BackingField: f64,
    pub _uuids: quest_hook::libil2cpp::Gc<crate::System::Guid>,
}
#[cfg(feature = "OVRSpatialAnchor+LoadOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSpatialAnchor_LoadOptions =>
    ""."OVRSpatialAnchor/LoadOptions"
);
#[cfg(feature = "OVRSpatialAnchor+LoadOptions")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSpatialAnchor_LoadOptions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSpatialAnchor+LoadOptions")]
impl crate::GlobalNamespace::OVRSpatialAnchor_LoadOptions {
    pub const MaxSupported: i32 = 1024i32;
    pub fn ToQueryOptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRSpaceQuery_Options> {
        let __cordl_ret: crate::GlobalNamespace::OVRSpaceQuery_Options = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToQueryOptions",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxAnchorCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_MaxAnchorCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StorageLocation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRSpace_StorageLocation,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRSpace_StorageLocation = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_StorageLocation",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Timeout(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Timeout",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Uuids(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Guid>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Guid> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Uuids",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MaxAnchorCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_MaxAnchorCount",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_StorageLocation(
        &mut self,
        value: crate::GlobalNamespace::OVRSpace_StorageLocation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_StorageLocation",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Timeout(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Timeout",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Uuids(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Uuids",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRSpatialAnchor+MultiAnchorActionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRSpatialAnchor_MultiAnchorActionType {
    #[default]
    Save = 0i32,
    Share = 1i32,
}
#[cfg(feature = "OVRSpatialAnchor+MultiAnchorActionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSpatialAnchor_MultiAnchorActionType => ""
    ."OVRSpatialAnchor/MultiAnchorActionType"
);
#[cfg(feature = "OVRSpatialAnchor+MultiAnchorDelegatePair")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OVRSpatialAnchor_MultiAnchorDelegatePair {
    pub Anchors: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
    >,
    pub Delegate: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
        >,
        crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
    >,
}
#[cfg(feature = "OVRSpatialAnchor+MultiAnchorDelegatePair")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSpatialAnchor_MultiAnchorDelegatePair => ""
    ."OVRSpatialAnchor/MultiAnchorDelegatePair"
);
#[cfg(feature = "OVRSpatialAnchor+MultiAnchorDelegatePair")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSpatialAnchor_MultiAnchorDelegatePair {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSpatialAnchor+MultiAnchorDelegatePair")]
impl crate::GlobalNamespace::OVRSpatialAnchor_MultiAnchorDelegatePair {}
#[cfg(feature = "OVRSpatialAnchor+OperationResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRSpatialAnchor_OperationResult {
    #[default]
    Failure = -1000i32,
    Failure_SpaceCloudStorageDisabled = -2000i32,
    Failure_SpaceLocalizationFailed = -2002i32,
    Failure_SpaceMappingInsufficient = -2001i32,
    Failure_SpaceNetworkRequestFailed = -2004i32,
    Failure_SpaceNetworkTimeout = -2003i32,
    Success = 0i32,
}
#[cfg(feature = "OVRSpatialAnchor+OperationResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRSpatialAnchor_OperationResult => ""
    ."OVRSpatialAnchor/OperationResult"
);
#[cfg(feature = "OVRSpatialAnchor+SaveOptions")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OVRSpatialAnchor_SaveOptions {
    pub Storage: crate::GlobalNamespace::OVRSpace_StorageLocation,
}
#[cfg(feature = "OVRSpatialAnchor+SaveOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSpatialAnchor_SaveOptions =>
    ""."OVRSpatialAnchor/SaveOptions"
);
#[cfg(feature = "OVRSpatialAnchor+SaveOptions")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSpatialAnchor_SaveOptions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSpatialAnchor+SaveOptions")]
impl crate::GlobalNamespace::OVRSpatialAnchor_SaveOptions {}
#[cfg(feature = "OVRSpatialAnchor+UnboundAnchor")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OVRSpatialAnchor_UnboundAnchor {
    pub _space: crate::GlobalNamespace::OVRSpace,
    pub _Uuid_k__BackingField: crate::System::Guid,
}
#[cfg(feature = "OVRSpatialAnchor+UnboundAnchor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRSpatialAnchor_UnboundAnchor
    => ""."OVRSpatialAnchor/UnboundAnchor"
);
#[cfg(feature = "OVRSpatialAnchor+UnboundAnchor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRSpatialAnchor_UnboundAnchor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSpatialAnchor+UnboundAnchor")]
impl crate::GlobalNamespace::OVRSpatialAnchor_UnboundAnchor {
    pub fn AddStorableAndShareableComponents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddStorableAndShareableComponents",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn BindTo(
        &mut self,
        spatialAnchor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSpatialAnchor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "BindTo",
            (spatialAnchor),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Localize(
        &mut self,
        onComplete: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRSpatialAnchor_UnboundAnchor,
            bool,
        >,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Localize",
            (onComplete, timeout),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalizeAsync(
        &mut self,
        timeout: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<bool>> {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<bool> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "LocalizeAsync",
            (timeout),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateLocalization(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ValidateLocalization",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        space: crate::GlobalNamespace::OVRSpace,
        uuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (space, uuid),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Localized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Localized",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Localizing(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Localizing",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Pose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_ret: crate::UnityEngine::Pose = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Pose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Uuid(&mut self) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_ret: crate::System::Guid = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Uuid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
