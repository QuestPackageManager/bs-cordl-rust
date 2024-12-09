#[cfg(feature = "HoudiniEngineUnity+HEU_PartData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PartData {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _partID: i32,
    pub _partName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _objectNodeID: i32,
    pub _geoID: i32,
    pub _partType: crate::HoudiniEngineUnity::HAPI_PartType,
    pub _geoNode: *mut crate::HoudiniEngineUnity::HEU_GeoNode,
    pub _isAttribInstancer: bool,
    pub _isPartInstanced: bool,
    pub _partPointCount: i32,
    pub _isObjectInstancer: bool,
    pub _objectInstancesGenerated: bool,
    pub _objectInstanceInfos: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
    >,
    pub _terrainOffsetPosition: crate::UnityEngine::Vector3,
    pub _assetDBTerrainData: *mut crate::UnityEngine::Object,
    pub _isPartEditable: bool,
    pub _partOutputType: crate::HoudiniEngineUnity::HEU_PartData_PartOutputType,
    pub _curve: *mut crate::HoudiniEngineUnity::HEU_Curve,
    pub _attributesStore: *mut crate::HoudiniEngineUnity::HEU_AttributesStore,
    pub _haveInstancesBeenGenerated: bool,
    pub _meshVertexCount: i32,
    pub _generatedOutput: *mut crate::HoudiniEngineUnity::HEU_GeneratedOutput,
    pub _volumeLayerName: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PartData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_PartData =>
    "HoudiniEngineUnity"."HEU_PartData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_PartData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_PartData {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PartData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_PartData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PartData")]
impl crate::HoudiniEngineUnity::HEU_PartData {
    #[cfg(feature = "HoudiniEngineUnity+HEU_PartData+PartOutputType")]
    pub type PartOutputType = crate::HoudiniEngineUnity::HEU_PartData_PartOutputType;
    #[cfg(feature = "HoudiniEngineUnity+HEU_PartData+__c")]
    pub type __c = crate::HoudiniEngineUnity::HEU_PartData___c;
    #[cfg(feature = "HoudiniEngineUnity+HEU_PartData+__c__DisplayClass86_0")]
    pub type __c__DisplayClass86_0 = crate::HoudiniEngineUnity::HEU_PartData___c__DisplayClass86_0;
    #[cfg(feature = "HoudiniEngineUnity+HEU_PartData+__c__DisplayClass88_0")]
    pub type __c__DisplayClass88_0 = crate::HoudiniEngineUnity::HEU_PartData___c__DisplayClass88_0;
    #[cfg(feature = "HoudiniEngineUnity+HEU_PartData+__c__DisplayClass88_1")]
    pub type __c__DisplayClass88_1 = crate::HoudiniEngineUnity::HEU_PartData___c__DisplayClass88_1;
    pub fn ApplyHAPITransform(
        &mut self,
        hapiTransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Transform,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyHAPITransform", (hapiTransform))?;
        Ok(__cordl_ret)
    }
    pub fn BakePartToGameObject(
        &mut self,
        targetGO: *mut crate::UnityEngine::GameObject,
        bDeleteExistingComponents: bool,
        bDontDeletePersistantResources: bool,
        bWriteMeshesToAssetDatabase: bool,
        bakedAssetPath: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        sourceToTargetMeshMap: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::UnityEngine::Mesh,
            *mut crate::UnityEngine::Mesh,
        >,
        sourceToCopiedMaterials: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::UnityEngine::Material,
            *mut crate::UnityEngine::Material,
        >,
        assetDBObject: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Object>,
        assetObjectFileName: *mut quest_hook::libil2cpp::Il2CppString,
        bReconnectPrefabInstances: bool,
        bKeepPreviousTransformValues: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "BakePartToGameObject",
                (
                    targetGO,
                    bDeleteExistingComponents,
                    bDontDeletePersistantResources,
                    bWriteMeshesToAssetDatabase,
                    bakedAssetPath,
                    sourceToTargetMeshMap,
                    sourceToCopiedMaterials,
                    assetDBObject,
                    assetObjectFileName,
                    bReconnectPrefabInstances,
                    bKeepPreviousTransformValues,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn BakePartToNewGameObject(
        &mut self,
        parentTransform: *mut crate::UnityEngine::Transform,
        bWriteMeshesToAssetDatabase: bool,
        bakedAssetPath: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        sourceToTargetMeshMap: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::UnityEngine::Mesh,
            *mut crate::UnityEngine::Mesh,
        >,
        sourceToCopiedMaterials: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::UnityEngine::Material,
            *mut crate::UnityEngine::Material,
        >,
        assetDBObject: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Object>,
        assetObjectFileName: *mut quest_hook::libil2cpp::Il2CppString,
        bReconnectPrefabInstances: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke(
                "BakePartToNewGameObject",
                (
                    parentTransform,
                    bWriteMeshesToAssetDatabase,
                    bakedAssetPath,
                    sourceToTargetMeshMap,
                    sourceToCopiedMaterials,
                    assetDBObject,
                    assetObjectFileName,
                    bReconnectPrefabInstances,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CalculateColliderState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateColliderState", ())?;
        Ok(__cordl_ret)
    }
    pub fn CalculateVisibility(
        &mut self,
        bParentVisibility: bool,
        bParentDisplayGeo: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateVisibility", (bParentVisibility, bParentDisplayGeo))?;
        Ok(__cordl_ret)
    }
    pub fn ClearGeneratedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearGeneratedData", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearGeneratedMeshOutput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearGeneratedMeshOutput", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearGeneratedVolumeOutput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearGeneratedVolumeOutput", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearInstances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearInstances", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearInvalidObjectInstanceInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearInvalidObjectInstanceInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearObjectInstanceInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearObjectInstanceInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateNewInstanceFromObject(
        &mut self,
        sourceObject: *mut crate::UnityEngine::GameObject,
        instanceIndex: i32,
        parentTransform: *mut crate::UnityEngine::Transform,
        hapiTransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Transform,
        >,
        instancedObjectNodeID: i32,
        instancedObjectPath: *mut quest_hook::libil2cpp::Il2CppString,
        rotationOffset: crate::UnityEngine::Vector3,
        scaleOffset: crate::UnityEngine::Vector3,
        instancePrefixes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        collisionSrcGO: *mut crate::UnityEngine::GameObject,
        copyParentFlags: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CreateNewInstanceFromObject",
                (
                    sourceObject,
                    instanceIndex,
                    parentTransform,
                    hapiTransform,
                    instancedObjectNodeID,
                    instancedObjectPath,
                    rotationOffset,
                    scaleOffset,
                    instancePrefixes,
                    collisionSrcGO,
                    copyParentFlags,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateObjectInstanceInfo(
        &mut self,
        instancedObject: *mut crate::UnityEngine::GameObject,
        instancedObjectNodeID: i32,
        instancedObjectPath: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo = __cordl_object
            .invoke(
                "CreateObjectInstanceInfo",
                (instancedObject, instancedObjectNodeID, instancedObjectPath),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DestroyAllData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyAllData", ())?;
        Ok(__cordl_ret)
    }
    pub fn DestroyAttributesStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyAttributesStore", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateAttributesStore(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateAttributesStore", (session))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateInstancesFromObject(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        sourceObject: *mut crate::HoudiniEngineUnity::HEU_ObjectNode,
        instancePrefixes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GenerateInstancesFromObject",
                (session, sourceObject, instancePrefixes),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GenerateInstancesFromObjectID(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        objectNodeID: i32,
        instancePrefixes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GenerateInstancesFromObjectID",
                (session, objectNodeID, instancePrefixes),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GenerateInstancesFromObjectIds(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        instancePrefixes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateInstancesFromObjectIds", (session, instancePrefixes))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateInstancesFromUnityAssetPathAttribute(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        unityInstanceAttr: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GenerateInstancesFromUnityAssetPathAttribute",
                (session, unityInstanceAttr),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GenerateMesh(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        bGenerateUVs: bool,
        bGenerateTangents: bool,
        bGenerateNormals: bool,
        bUseLODGroups: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GenerateMesh",
                (
                    session,
                    bGenerateUVs,
                    bGenerateTangents,
                    bGenerateNormals,
                    bUseLODGroups,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GeneratePartInstances(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GeneratePartInstances", (session))?;
        Ok(__cordl_ret)
    }
    pub fn GetClonableObjects(
        &mut self,
        clonableObjects: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetClonableObjects", (clonableObjects))?;
        Ok(__cordl_ret)
    }
    pub fn GetClonableParts(
        &mut self,
        clonableParts: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_PartData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetClonableParts", (clonableParts))?;
        Ok(__cordl_ret)
    }
    pub fn GetCurve(
        &mut self,
        bEditableOnly: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_Curve> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_Curve = __cordl_object
            .invoke("GetCurve", (bEditableOnly))?;
        Ok(__cordl_ret)
    }
    pub fn GetDebugInfo(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDebugInfo", (sb))?;
        Ok(__cordl_ret)
    }
    pub fn GetHDAPartWithGameObject(
        &mut self,
        inGameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_PartData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_PartData = __cordl_object
            .invoke("GetHDAPartWithGameObject", (inGameObject))?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectInstanceInfoWithObjectID(
        &mut self,
        objNodeID: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo = __cordl_object
            .invoke("GetObjectInstanceInfoWithObjectID", (objNodeID))?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectInstanceInfoWithObjectPath(
        &mut self,
        path: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo = __cordl_object
            .invoke("GetObjectInstanceInfoWithObjectPath", (path))?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectInstanceInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
        > = __cordl_object.invoke("GetObjectInstanceInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetOutput(
        &mut self,
        outputs: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_GeneratedOutput,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetOutput", (outputs))?;
        Ok(__cordl_ret)
    }
    pub fn GetOutputGameObjects(
        &mut self,
        outputObjects: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetOutputGameObjects", (outputObjects))?;
        Ok(__cordl_ret)
    }
    pub fn GetPartPointCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPartPointCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetVolumeLayerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetVolumeLayerName", ())?;
        Ok(__cordl_ret)
    }
    pub fn HaveInstancesBeenGenerated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HaveInstancesBeenGenerated", ())?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        partID: i32,
        geoID: i32,
        objectNodeID: i32,
        geoNode: *mut crate::HoudiniEngineUnity::HEU_GeoNode,
        partInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PartInfo,
        >,
        partOutputType: crate::HoudiniEngineUnity::HEU_PartData_PartOutputType,
        isEditable: bool,
        isObjectInstancer: bool,
        isAttribInstancer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Initialize",
                (
                    session,
                    partID,
                    geoID,
                    objectNodeID,
                    geoNode,
                    partInfo,
                    partOutputType,
                    isEditable,
                    isObjectInstancer,
                    isAttribInstancer,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn IsAttribInstancer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAttribInstancer", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: *mut crate::HoudiniEngineUnity::HEU_PartData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn IsInstancerAnyType(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsInstancerAnyType", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsObjectInstancer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsObjectInstancer", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsPartCurve(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPartCurve", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsPartEditable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPartEditable", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsPartInstanced(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPartInstanced", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsPartInstancer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPartInstancer", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsPartMesh(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPartMesh", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsPartVolume(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPartVolume", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsUsingMaterial(
        &mut self,
        materialData: *mut crate::HoudiniEngineUnity::HEU_MaterialData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsUsingMaterial", (materialData))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PopulateObjectInstanceInfos(
        &mut self,
        objInstanceInfos: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateObjectInstanceInfos", (objInstanceInfos))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessCurvePart(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCurvePart", (session))?;
        Ok(__cordl_ret)
    }
    pub fn SetColliderState(
        &mut self,
        bEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColliderState", (bEnabled))?;
        Ok(__cordl_ret)
    }
    pub fn SetGameObject(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameObject", (gameObject))?;
        Ok(__cordl_ret)
    }
    pub fn SetGameObjectName(
        &mut self,
        partName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameObjectName", (partName))?;
        Ok(__cordl_ret)
    }
    pub fn SetObjectInstanceInfos(
        &mut self,
        sourceObjectInstanceInfos: *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetObjectInstanceInfos", (sourceObjectInstanceInfos))?;
        Ok(__cordl_ret)
    }
    pub fn SetObjectInstancer(
        &mut self,
        bObjectInstancer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetObjectInstancer", (bObjectInstancer))?;
        Ok(__cordl_ret)
    }
    pub fn SetTerrainData(
        &mut self,
        terrainData: *mut crate::UnityEngine::TerrainData,
        exportPathRelative: *mut quest_hook::libil2cpp::Il2CppString,
        exportPathUser: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetTerrainData",
                (terrainData, exportPathRelative, exportPathUser),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetTerrainOffsetPosition(
        &mut self,
        offsetPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTerrainOffsetPosition", (offsetPosition))?;
        Ok(__cordl_ret)
    }
    pub fn SetVisiblity(
        &mut self,
        bVisibility: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVisiblity", (bVisibility))?;
        Ok(__cordl_ret)
    }
    pub fn SetVolumeLayerName(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVolumeLayerName", (name))?;
        Ok(__cordl_ret)
    }
    pub fn SetupAttributeGeometry(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupAttributeGeometry", (session))?;
        Ok(__cordl_ret)
    }
    pub fn SyncAttributesStore(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoID: i32,
        partInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_PartInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncAttributesStore", (session, geoID, partInfo))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", ())?;
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
    pub fn get_GeneratedOutput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_GeneratedOutput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_GeneratedOutput = __cordl_object
            .invoke("get_GeneratedOutput", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MeshVertexCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MeshVertexCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectInstancesBeenGenerated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ObjectInstancesBeenGenerated", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OutputGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_OutputGameObject", ())?;
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
    pub fn get_ParentGeoNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_GeoNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_GeoNode = __cordl_object
            .invoke("get_ParentGeoNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PartID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PartID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PartName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_PartName", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ObjectInstancesBeenGenerated(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ObjectInstancesBeenGenerated", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PartData")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_PartData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PartData+PartOutputType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_PartData_PartOutputType {
    CURVE = 3i32,
    INSTANCER = 4i32,
    MESH = 1i32,
    NONE = 0i32,
    VOLUME = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PartData+PartOutputType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_PartData_PartOutputType
    => "HoudiniEngineUnity"."HEU_PartData/PartOutputType"
);
