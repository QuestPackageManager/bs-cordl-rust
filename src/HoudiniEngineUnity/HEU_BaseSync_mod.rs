#[cfg(feature = "HoudiniEngineUnity+HEU_BaseSync")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_BaseSync {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _cookNodeID: i32,
    pub _sessionID: i64,
    pub _nodeName: *mut crate::System::String,
    pub _initialized: bool,
    pub _syncing: bool,
    pub _deleteParent: bool,
    pub _generatedOutputs: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_GeneratedOutput,
    >,
    pub _outputCacheDirectory: *mut crate::System::String,
    pub _outputCacheFilePaths: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
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
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddGeneratedOutputFilePath", (path))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyAttributeModifiersOnGameObjectOutput(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
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
    pub fn Bake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Bake", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearLog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLog", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateNewInstanceFromObject(
        &mut self,
        assetSourceGO: *mut crate::UnityEngine::GameObject,
        instanceIndex: i32,
        parentTransform: *mut crate::UnityEngine::Transform,
        hapiTransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Transform,
        >,
        instancePrefixes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
        instanceName: *mut crate::System::String,
        collisionSourceGO: *mut crate::UnityEngine::GameObject,
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
        Ok(__cordl_ret)
    }
    pub fn DeleteSessionData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteSessionData", ())?;
        Ok(__cordl_ret)
    }
    pub fn DestroyGeneratedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyGeneratedData", ())?;
        Ok(__cordl_ret)
    }
    pub fn DestroyOutputs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyOutputs", ())?;
        Ok(__cordl_ret)
    }
    pub fn Error(
        &mut self,
        error: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Error", (error))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateAllInstancers(
        &mut self,
        cookNodeId: i32,
        instancerBuffers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_LoadBufferInstancer,
        >,
        loadData: *mut crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateAllInstancers", (cookNodeId, instancerBuffers, loadData))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateGeometry(
        &mut self,
        loadData: *mut crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
        objIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateGeometry", (loadData, objIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateInstancer(
        &mut self,
        cookNodeId: i32,
        instancerBuffer: *mut crate::HoudiniEngineUnity::HEU_LoadBufferInstancer,
        idBuffersMap: *mut crate::System::Collections::Generic::Dictionary_2<
            i32,
            *mut crate::HoudiniEngineUnity::HEU_LoadBufferBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateInstancer", (cookNodeId, instancerBuffer, idBuffersMap))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateInstancesFromAssetPaths(
        &mut self,
        instancerBuffer: *mut crate::HoudiniEngineUnity::HEU_LoadBufferInstancer,
        instanceRootTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GenerateInstancesFromAssetPaths",
                (instancerBuffer, instanceRootTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GenerateInstancesFromNodeIDs(
        &mut self,
        cookNodeId: i32,
        instancerBuffer: *mut crate::HoudiniEngineUnity::HEU_LoadBufferInstancer,
        idBuffersMap: *mut crate::System::Collections::Generic::Dictionary_2<
            i32,
            *mut crate::HoudiniEngineUnity::HEU_LoadBufferBase,
        >,
        instanceRootTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GenerateInstancesFromNodeIDs",
                (cookNodeId, instancerBuffer, idBuffersMap, instanceRootTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GenerateMesh(
        &mut self,
        cookNodeId: i32,
        meshBuffers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_LoadBufferMesh,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateMesh", (cookNodeId, meshBuffers))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateObjects(
        &mut self,
        loadData: *mut crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateObjects", (loadData))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateTerrain(
        &mut self,
        cookNodeId: i32,
        terrainBuffers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_LoadBufferVolume,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateTerrain", (cookNodeId, terrainBuffers))?;
        Ok(__cordl_ret)
    }
    pub fn GetHoudiniSession(
        &mut self,
        bCreateIfNotFound: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_SessionBase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_SessionBase = __cordl_object
            .invoke("GetHoudiniSession", (bCreateIfNotFound))?;
        Ok(__cordl_ret)
    }
    pub fn GetOutputCacheDirectory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetOutputCacheDirectory", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetParentNodeID(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetParentNodeID", (session))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsLoaded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsLoaded", ())?;
        Ok(__cordl_ret)
    }
    pub fn Log(
        &mut self,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn OnLoadComplete(
        &mut self,
        loadData: *mut crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnLoadComplete", (loadData))?;
        Ok(__cordl_ret)
    }
    pub fn OnStopped(
        &mut self,
        loadData: *mut crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStopped", (loadData))?;
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
    pub fn Resync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resync", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetOutputCacheDirectory(
        &mut self,
        directory: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOutputCacheDirectory", (directory))?;
        Ok(__cordl_ret)
    }
    pub fn SetOutputVisiblity(
        &mut self,
        buffer: *mut crate::HoudiniEngineUnity::HEU_LoadBufferBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOutputVisiblity", (buffer))?;
        Ok(__cordl_ret)
    }
    pub fn SetupLoadTask(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupLoadTask", (session))?;
        Ok(__cordl_ret)
    }
    pub fn StartSync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartSync", ())?;
        Ok(__cordl_ret)
    }
    pub fn StopSync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopSync", ())?;
        Ok(__cordl_ret)
    }
    pub fn SyncUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Unload(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", ())?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_BaseSync")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_BaseSync {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
