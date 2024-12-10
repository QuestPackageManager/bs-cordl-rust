#[cfg(feature = "HoudiniEngineUnity+HEU_BaseSync")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_BaseSync {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _cookNodeID: i32,
    pub _sessionID: i64,
    pub _nodeName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _initialized: bool,
    pub _syncing: bool,
    pub _deleteParent: bool,
    pub _generatedOutputs: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_GeneratedOutput,
    >,
    pub _outputCacheDirectory: *mut quest_hook::libil2cpp::Il2CppString,
    pub _outputCacheFilePaths: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _generateOptions: crate::HoudiniEngineUnity::HEU_GenerateOptions,
    pub _log: *mut crate::System::Text::StringBuilder,
    pub _error: *mut crate::System::Text::StringBuilder,
    pub _sessionSyncAutoCook: bool,
    pub _loadTask: *mut crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo,
    pub _totalCookCount: i32,
    pub _firstSyncComplete: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_BaseSync")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_BaseSync =>
    "HoudiniEngineUnity"."HEU_BaseSync"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_BaseSync")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_BaseSync {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_BaseSync")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_BaseSync {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_BaseSync")]
impl crate::HoudiniEngineUnity::HEU_BaseSync {
    pub fn AddGeneratedOutputFilePath(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddGeneratedOutputFilePath", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyAttributeModifiersOnGameObjectOutput(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partId: i32,
        go: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ApplyAttributeModifiersOnGameObjectOutput",
                (session, geoID, partId, go),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Bake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Bake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearLog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLog", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNewInstanceFromObject(
        &mut self,
        assetSourceGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        instanceIndex: i32,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        hapiTransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Transform,
        >,
        instancePrefixes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        instanceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        collisionSourceGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CreateNewInstanceFromObject",
                (
                    assetSourceGO,
                    instanceIndex,
                    parentTransform,
                    hapiTransform,
                    instancePrefixes,
                    instanceName,
                    collisionSourceGO,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteSessionData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteSessionData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyGeneratedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyGeneratedData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyOutputs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyOutputs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Error(
        &mut self,
        error: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Error", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateAllInstancers(
        &mut self,
        cookNodeId: i32,
        instancerBuffers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::HoudiniEngineUnity::HEU_LoadBufferInstancer,
            >,
        >,
        loadData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateAllInstancers", (cookNodeId, instancerBuffers, loadData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateGeometry(
        &mut self,
        loadData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
        >,
        objIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateGeometry", (loadData, objIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateInstancer(
        &mut self,
        cookNodeId: i32,
        instancerBuffer: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_LoadBufferInstancer,
        >,
        idBuffersMap: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                i32,
                *mut crate::HoudiniEngineUnity::HEU_LoadBufferBase,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateInstancer", (cookNodeId, instancerBuffer, idBuffersMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateInstancesFromAssetPaths(
        &mut self,
        instancerBuffer: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_LoadBufferInstancer,
        >,
        instanceRootTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GenerateInstancesFromAssetPaths",
                (instancerBuffer, instanceRootTransform),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateInstancesFromNodeIDs(
        &mut self,
        cookNodeId: i32,
        instancerBuffer: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_LoadBufferInstancer,
        >,
        idBuffersMap: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                i32,
                *mut crate::HoudiniEngineUnity::HEU_LoadBufferBase,
            >,
        >,
        instanceRootTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GenerateInstancesFromNodeIDs",
                (cookNodeId, instancerBuffer, idBuffersMap, instanceRootTransform),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateMesh(
        &mut self,
        cookNodeId: i32,
        meshBuffers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::HoudiniEngineUnity::HEU_LoadBufferMesh,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateMesh", (cookNodeId, meshBuffers))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateObjects(
        &mut self,
        loadData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateObjects", (loadData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTerrain(
        &mut self,
        cookNodeId: i32,
        terrainBuffers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::HoudiniEngineUnity::HEU_LoadBufferVolume,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateTerrain", (cookNodeId, terrainBuffers))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHoudiniSession(
        &mut self,
        bCreateIfNotFound: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_SessionBase,
        > = __cordl_object.invoke("GetHoudiniSession", (bCreateIfNotFound))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOutputCacheDirectory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetOutputCacheDirectory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParentNodeID(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetParentNodeID", (session))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLoaded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsLoaded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (msg))?;
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
    pub fn OnLoadComplete(
        &mut self,
        loadData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnLoadComplete", (loadData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStopped(
        &mut self,
        loadData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStopped", (loadData))?;
        Ok(__cordl_ret.into())
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
    pub fn Resync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetOutputCacheDirectory(
        &mut self,
        directory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOutputCacheDirectory", (directory))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetOutputVisiblity(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_LoadBufferBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOutputVisiblity", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupLoadTask(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupLoadTask", (session))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartSync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartSync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopSync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopSync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SyncUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Unload(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", ())?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_BaseSync")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_BaseSync {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
