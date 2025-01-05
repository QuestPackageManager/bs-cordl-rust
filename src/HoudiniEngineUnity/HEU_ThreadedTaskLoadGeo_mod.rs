#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadData+LoadStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_LoadData_HEU_ThreadedTaskLoadGeo_LoadStatus {
    #[default]
    ERROR = 3i32,
    NONE = 0i32,
    STARTED = 1i32,
    SUCCESS = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadData+LoadStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_LoadData_HEU_ThreadedTaskLoadGeo_LoadStatus =>
    "HoudiniEngineUnity"."HEU_ThreadedTaskLoadGeo/HEU_LoadData/LoadStatus"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ThreadedTaskLoadGeo {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_ThreadedTask,
    pub _ownerSync: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_BaseSync>,
    pub _session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    pub _generateOptions: crate::HoudiniEngineUnity::HEU_GenerateOptions,
    pub _loadType: crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_LoadType,
    pub _filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _loadData: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
    >,
    pub _loadCallback: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadCallback,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo =>
    "HoudiniEngineUnity"."HEU_ThreadedTaskLoadGeo"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo {
    type Target = crate::HoudiniEngineUnity::HEU_ThreadedTask;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo")]
impl crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo {
    #[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadCallback")]
    pub type HEU_LoadCallback = crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadCallback;
    #[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadCallbackType")]
    pub type HEU_LoadCallbackType = crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadCallbackType;
    #[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadData")]
    pub type HEU_LoadData = crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData;
    #[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadObject")]
    pub type HEU_LoadObject = crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadObject;
    #[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+LoadType")]
    pub type LoadType = crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_LoadType;
    pub fn AppendLog(
        &mut self,
        status: crate::HoudiniEngineUnity::HEU_LoadData_HEU_ThreadedTaskLoadGeo_LoadStatus,
        logStr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendLog", (status, logStr))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildBufferIDsMap(
        &mut self,
        loadData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildBufferIDsMap", (loadData))?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CookNode(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        cookNodeID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CookNode", (session, cookNodeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFileNode(
        &mut self,
        fileNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CreateFileNode", (fileNodeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLogString(
        &mut self,
        status: crate::HoudiniEngineUnity::HEU_LoadData_HEU_ThreadedTaskLoadGeo_LoadStatus,
        logStr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("CreateLogString", (status, logStr))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoAssetLoad(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DoAssetLoad", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DoFileLoad(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DoFileLoad", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DoWork(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoWork", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateInstancerBuffers(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        instancerParts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::HoudiniEngineUnity::HAPI_PartInfo,
            >,
        >,
        instancerBuffers: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_LoadBufferInstancer,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GenerateInstancerBuffers",
                (session, nodeID, instancerParts, instancerBuffers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateMeshBuffers(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        meshParts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::HoudiniEngineUnity::HAPI_PartInfo,
            >,
        >,
        bSplitPoints: bool,
        bUseLODGroups: bool,
        bGenerateUVs: bool,
        bGenerateTangents: bool,
        bGenerateNormals: bool,
        meshBuffers: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_LoadBufferMesh,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GenerateMeshBuffers",
                (
                    session,
                    nodeID,
                    meshParts,
                    bSplitPoints,
                    bUseLODGroups,
                    bGenerateUVs,
                    bGenerateTangents,
                    bGenerateNormals,
                    meshBuffers,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePartsInstancerBuffer(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        partInfo: crate::HoudiniEngineUnity::HAPI_PartInfo,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_LoadBufferInstancer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_LoadBufferInstancer,
        > = __cordl_object
            .invoke(
                "GeneratePartsInstancerBuffer",
                (session, geoID, partID, partName, partInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePointAttributeInstancerBuffer(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        partInfo: crate::HoudiniEngineUnity::HAPI_PartInfo,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_LoadBufferInstancer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_LoadBufferInstancer,
        > = __cordl_object
            .invoke(
                "GeneratePointAttributeInstancerBuffer",
                (session, geoID, partID, partName, partInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTerrainBuffers(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        volumeParts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::HoudiniEngineUnity::HAPI_PartInfo,
            >,
        >,
        scatterInstancerParts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::HoudiniEngineUnity::HAPI_PartInfo,
            >,
        >,
        volumeBuffers: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::HoudiniEngineUnity::HEU_LoadBufferVolume,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GenerateTerrainBuffers",
                (session, nodeID, volumeParts, scatterInstancerParts, volumeBuffers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCookNodeID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetCookNodeID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDisplayNodeID(
        &mut self,
        objNodeID: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetDisplayNodeID", (objNodeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLoadBufferVolumeFromTileIndex(
        tileIndex: i32,
        buffers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_LoadBufferVolume,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_LoadBufferVolume>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_LoadBufferVolume,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLoadBufferVolumeFromTileIndex", (tileIndex, buffers))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFloatFromAttribute(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        floatValue: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LoadFloatFromAttribute",
                (session, geoID, partID, attrName, floatValue),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadLayerColorFromAttribute(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        colorValue: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LoadLayerColorFromAttribute",
                (session, geoID, partID, attrName, colorValue),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadLayerVector2FromAttribute(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        vectorValue: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LoadLayerVector2FromAttribute",
                (session, geoID, partID, attrName, vectorValue),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadNodeBuffer(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        nodeID: i32,
        loadObject: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadObject,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("LoadNodeBuffer", (session, nodeID, loadObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadObjectBuffers(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        objectInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_ObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("LoadObjectBuffers", (session, objectInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadStringFromAttribute(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        attrName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        strValue: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LoadStringFromAttribute",
                (session, geoID, partID, attrName, strValue),
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
    pub fn OnComplete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnComplete", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStopped(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStopped", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn QueryParts(
        &mut self,
        nodeID: i32,
        meshParts: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::HoudiniEngineUnity::HAPI_PartInfo,
                >,
            >,
        >,
        volumeParts: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::HoudiniEngineUnity::HAPI_PartInfo,
                >,
            >,
        >,
        instancerParts: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::HoudiniEngineUnity::HAPI_PartInfo,
                >,
            >,
        >,
        curveParts: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::HoudiniEngineUnity::HAPI_PartInfo,
                >,
            >,
        >,
        scatterInstancerParts: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::HoudiniEngineUnity::HAPI_PartInfo,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "QueryParts",
                (
                    nodeID,
                    meshParts,
                    volumeParts,
                    instancerParts,
                    curveParts,
                    scatterInstancerParts,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFileParm(
        &mut self,
        fileNodeID: i32,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetFileParm", (fileNodeID, filePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLoadCallback(
        &mut self,
        loadCallback: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLoadCallback", (loadCallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLog(
        &mut self,
        status: crate::HoudiniEngineUnity::HEU_LoadData_HEU_ThreadedTaskLoadGeo_LoadStatus,
        logStr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLog", (status, logStr))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupLoad(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        ownerSync: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_BaseSync>,
        loadType: crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_LoadType,
        cookNodeID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetupLoad",
                (session, ownerSync, loadType, cookNodeID, name, filePath),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupLoadAsset(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        ownerSync: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_BaseSync>,
        assetPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupLoadAsset", (session, ownerSync, assetPath, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupLoadFile(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        ownerSync: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_BaseSync>,
        cookNodeID: i32,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupLoadFile", (session, ownerSync, cookNodeID, filePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupLoadNode(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        ownerSync: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_BaseSync>,
        cookNodeID: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupLoadNode", (session, ownerSync, cookNodeID, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sleep(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Sleep", ())?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ThreadedTaskLoadGeo_HEU_LoadCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadCallback =>
    "HoudiniEngineUnity"."HEU_ThreadedTaskLoadGeo/HEU_LoadCallback"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadCallback")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadCallback")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadCallback")]
impl crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadCallback {
    pub fn BeginInvoke(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        loadData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
        >,
        callbackType: crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadCallbackType,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (session, loadData, callbackType, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        loadData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData,
        >,
        callbackType: crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadCallbackType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (session, loadData, callbackType))?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadCallbackType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_ThreadedTaskLoadGeo_HEU_LoadCallbackType {
    #[default]
    POSTCOOK = 1i32,
    PRECOOK = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadCallbackType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadCallbackType =>
    "HoudiniEngineUnity"."HEU_ThreadedTaskLoadGeo/HEU_LoadCallbackType"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ThreadedTaskLoadGeo_HEU_LoadData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cookNodeID: i32,
    pub _loadStatus: crate::HoudiniEngineUnity::HEU_LoadData_HEU_ThreadedTaskLoadGeo_LoadStatus,
    pub _logStr: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    pub _session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    pub _loadedObjects: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadObject,
            >,
        >,
    >,
    pub _idBuffersMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_LoadBufferBase>,
        >,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData => "HoudiniEngineUnity"
    ."HEU_ThreadedTaskLoadGeo/HEU_LoadData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadData")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadData")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadData")]
impl crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData {
    #[cfg(
        feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadData+LoadStatus"
    )]
    pub type LoadStatus = crate::HoudiniEngineUnity::HEU_LoadData_HEU_ThreadedTaskLoadGeo_LoadStatus;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadData")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadObject")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ThreadedTaskLoadGeo_HEU_LoadObject {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _objectNodeID: i32,
    pub _displayNodeID: i32,
    pub _terrainBuffers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_LoadBufferVolume>,
        >,
    >,
    pub _meshBuffers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_LoadBufferMesh>,
        >,
    >,
    pub _instancerBuffers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_LoadBufferInstancer>,
        >,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadObject => "HoudiniEngineUnity"
    ."HEU_ThreadedTaskLoadGeo/HEU_LoadObject"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadObject")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadObject {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadObject")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadObject")]
impl crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadObject {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+HEU_LoadObject")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_HEU_LoadObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+LoadType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_ThreadedTaskLoadGeo_LoadType {
    #[default]
    ASSET = 2i32,
    FILE = 0i32,
    NODE = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_ThreadedTaskLoadGeo+LoadType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_ThreadedTaskLoadGeo_LoadType => "HoudiniEngineUnity"
    ."HEU_ThreadedTaskLoadGeo/LoadType"
);
