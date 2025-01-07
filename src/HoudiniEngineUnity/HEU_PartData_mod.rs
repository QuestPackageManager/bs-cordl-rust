#[cfg(feature = "HoudiniEngineUnity+HEU_PartData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_PartData {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _partID: i32,
    pub _partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _objectNodeID: i32,
    pub _geoID: i32,
    pub _partType: crate::HoudiniEngineUnity::HAPI_PartType,
    pub _geoNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoNode>,
    pub _isAttribInstancer: bool,
    pub _isPartInstanced: bool,
    pub _partPointCount: i32,
    pub _isObjectInstancer: bool,
    pub _objectInstancesGenerated: bool,
    pub _objectInstanceInfos: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo>,
        >,
    >,
    pub _terrainOffsetPosition: crate::UnityEngine::Vector3,
    pub _assetDBTerrainData: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    pub _isPartEditable: bool,
    pub _partOutputType: crate::HoudiniEngineUnity::HEU_PartData_PartOutputType,
    pub _curve: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Curve>,
    pub _attributesStore: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_AttributesStore,
    >,
    pub _haveInstancesBeenGenerated: bool,
    pub _meshVertexCount: i32,
    pub _generatedOutput: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_GeneratedOutput,
    >,
    pub _volumeLayerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PartData")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HEU_PartData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_PartData";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn AppendBakedCloneName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendBakedCloneName", (name))?;
        Ok(__cordl_ret.into())
    }
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
        Ok(__cordl_ret.into())
    }
    pub fn BakePartToGameObject_GameObject__cordl_bool__cordl_bool__cordl_bool_ByRefMut_Dictionary_2_Dictionary_2_ByRefMut_Il2CppString__cordl_bool__cordl_bool1(
        &mut self,
        targetGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        bDeleteExistingComponents: bool,
        bDontDeletePersistantResources: bool,
        bWriteMeshesToAssetDatabase: bool,
        bakedAssetPath: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        sourceToTargetMeshMap: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
            >,
        >,
        sourceToCopiedMaterials: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
        assetDBObject: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        >,
        assetObjectFileName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn BakePartToGameObject_HEU_PartData_GameObject_GameObject_Il2CppString__cordl_bool__cordl_bool__cordl_bool__cordl_bool_ByRefMut_Dictionary_2_Dictionary_2_ByRefMut_Il2CppString__cordl_bool__cordl_bool0(
        partData: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
        srcGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        targetGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bIsInstancer: bool,
        bDeleteExistingComponents: bool,
        bDontDeletePersistantResources: bool,
        bWriteMeshesToAssetDatabase: bool,
        bakedAssetPath: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        sourceToTargetMeshMap: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
            >,
        >,
        sourceToCopiedMaterials: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
        assetDBObject: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        >,
        assetObjectFileName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        bReconnectPrefabInstances: bool,
        bKeepPreviousTransformValues: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "BakePartToGameObject",
                (
                    partData,
                    srcGO,
                    targetGO,
                    assetName,
                    bIsInstancer,
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
        Ok(__cordl_ret.into())
    }
    pub fn BakePartToNewGameObject(
        &mut self,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        bWriteMeshesToAssetDatabase: bool,
        bakedAssetPath: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        sourceToTargetMeshMap: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
            >,
        >,
        sourceToCopiedMaterials: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
        assetDBObject: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        >,
        assetObjectFileName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        bReconnectPrefabInstances: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
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
        Ok(__cordl_ret.into())
    }
    pub fn CalculateColliderState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateColliderState", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ClearGeneratedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearGeneratedData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearGeneratedMeshOutput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearGeneratedMeshOutput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearGeneratedVolumeOutput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearGeneratedVolumeOutput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearInstances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearInstances", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearInvalidObjectInstanceInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearInvalidObjectInstanceInfos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearObjectInstanceInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearObjectInstanceInfos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyChildGameObjects(
        partData: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
        sourceGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        targetGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sourceToTargetMeshMap: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
            >,
        >,
        sourceToCopiedMaterials: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
        bWriteMeshesToAssetDatabase: bool,
        bakedAssetPath: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        assetDBObject: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        >,
        assetObjectFileName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        bDeleteExistingComponents: bool,
        bDontDeletePersistantResources: bool,
        bKeepPreviousTransformValues: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CopyChildGameObjects",
                (
                    partData,
                    sourceGO,
                    targetGO,
                    assetName,
                    sourceToTargetMeshMap,
                    sourceToCopiedMaterials,
                    bWriteMeshesToAssetDatabase,
                    bakedAssetPath,
                    assetDBObject,
                    assetObjectFileName,
                    bDeleteExistingComponents,
                    bDontDeletePersistantResources,
                    bKeepPreviousTransformValues,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyGameObjectComponents(
        partData: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
        sourceGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        targetGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        assetName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sourceToTargetMeshMap: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
            >,
        >,
        sourceToCopiedMaterials: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
        bWriteMeshesToAssetDatabase: bool,
        bakedAssetPath: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        assetDBObject: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        >,
        assetObjectFileName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        bDeleteExistingComponents: bool,
        bDontDeletePersistantResources: bool,
        lodTransformValues: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::HoudiniEngineUnity::TransformData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CopyGameObjectComponents",
                (
                    partData,
                    sourceGO,
                    targetGO,
                    assetName,
                    sourceToTargetMeshMap,
                    sourceToCopiedMaterials,
                    bWriteMeshesToAssetDatabase,
                    bakedAssetPath,
                    assetDBObject,
                    assetObjectFileName,
                    bDeleteExistingComponents,
                    bDontDeletePersistantResources,
                    lodTransformValues,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNewInstanceFromObject(
        &mut self,
        sourceObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        instanceIndex: i32,
        parentTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        hapiTransform: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_Transform,
        >,
        instancedObjectNodeID: i32,
        instancedObjectPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        rotationOffset: crate::UnityEngine::Vector3,
        scaleOffset: crate::UnityEngine::Vector3,
        instancePrefixes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        collisionSrcGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn CreateObjectInstanceInfo(
        &mut self,
        instancedObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        instancedObjectNodeID: i32,
        instancedObjectPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
        > = __cordl_object
            .invoke(
                "CreateObjectInstanceInfo",
                (instancedObject, instancedObjectNodeID, instancedObjectPath),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyAllData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyAllData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyAttributesStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyAttributesStore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyPart(
        part: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyPart", (part))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyParts(
        parts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyParts", (parts))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateAttributesStore(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateAttributesStore", (session))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateInstancesFromObject(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        sourceObject: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ObjectNode,
        >,
        instancePrefixes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn GenerateInstancesFromObjectID(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        objectNodeID: i32,
        instancePrefixes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn GenerateInstancesFromObjectIds(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        instancePrefixes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateInstancesFromObjectIds", (session, instancePrefixes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateInstancesFromUnityAssetPathAttribute(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        unityInstanceAttr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GenerateInstancesFromUnityAssetPathAttribute",
                (session, unityInstanceAttr),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateMesh(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
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
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePartInstances(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GeneratePartInstances", (session))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClonableObjects(
        &mut self,
        clonableObjects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetClonableObjects", (clonableObjects))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClonableParts(
        &mut self,
        clonableParts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetClonableParts", (clonableParts))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurve(
        &mut self,
        bEditableOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Curve>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_Curve,
        > = __cordl_object.invoke("GetCurve", (bEditableOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDebugInfo(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDebugInfo", (sb))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHDAPartWithGameObject(
        &mut self,
        inGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_PartData,
        > = __cordl_object.invoke("GetHDAPartWithGameObject", (inGameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectInstanceInfoWithObjectID(
        &mut self,
        objNodeID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
        > = __cordl_object.invoke("GetObjectInstanceInfoWithObjectID", (objNodeID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectInstanceInfoWithObjectPath(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
        > = __cordl_object.invoke("GetObjectInstanceInfoWithObjectPath", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectInstanceInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
                >,
            >,
        > = __cordl_object.invoke("GetObjectInstanceInfos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOutput(
        &mut self,
        outputs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeneratedOutput>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetOutput", (outputs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOutputGameObjects(
        &mut self,
        outputObjects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetOutputGameObjects", (outputObjects))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPartPointCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPartPointCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVolumeLayerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetVolumeLayerName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HaveInstancesBeenGenerated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HaveInstancesBeenGenerated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        partID: i32,
        geoID: i32,
        objectNodeID: i32,
        geoNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoNode>,
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
        Ok(__cordl_ret.into())
    }
    pub fn IsAttribInstancer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAttribInstancer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInstancerAnyType(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsInstancerAnyType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsObjectInstancer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsObjectInstancer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPartCurve(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPartCurve", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPartEditable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPartEditable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPartInstanced(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPartInstanced", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPartInstancer(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPartInstancer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPartMesh(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPartMesh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPartVolume(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPartVolume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsUsingMaterial(
        &mut self,
        materialData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_MaterialData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsUsingMaterial", (materialData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PopulateObjectInstanceInfos(
        &mut self,
        objInstanceInfos: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateObjectInstanceInfos", (objInstanceInfos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCurvePart(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCurvePart", (session))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetGameObject(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameObject", (gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGameObjectName(
        &mut self,
        partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameObjectName", (partName))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetObjectInstanceInfos(
        &mut self,
        sourceObjectInstanceInfos: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::HoudiniEngineUnity::HEU_ObjectInstanceInfo,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetObjectInstanceInfos", (sourceObjectInstanceInfos))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetTerrainData(
        &mut self,
        terrainData: quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainData>,
        exportPathRelative: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        exportPathUser: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetTerrainData",
                (terrainData, exportPathRelative, exportPathUser),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetVolumeLayerName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVolumeLayerName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupAttributeGeometry(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupAttributeGeometry", (session))?;
        Ok(__cordl_ret.into())
    }
    pub fn SyncAttributesStore(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
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
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
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
    pub fn get_GeneratedOutput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeneratedOutput>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutput,
        > = __cordl_object.invoke("get_GeneratedOutput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MeshVertexCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MeshVertexCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectInstancesBeenGenerated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ObjectInstancesBeenGenerated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OutputGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("get_OutputGameObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_HoudiniAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        > = __cordl_object.invoke("get_ParentAsset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentGeoNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeoNode,
        > = __cordl_object.invoke("get_ParentGeoNode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PartID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PartID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PartName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_PartName", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "HoudiniEngineUnity+HEU_PartData")]
impl AsRef<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
    >,
> for crate::HoudiniEngineUnity::HEU_PartData {
    fn as_ref(
        &self,
    ) -> &crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PartData")]
impl AsMut<
    crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
    >,
> for crate::HoudiniEngineUnity::HEU_PartData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::HoudiniEngineUnity::IEquivable_1<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_PartData>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PartData+PartOutputType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_PartData_PartOutputType {
    #[default]
    CURVE = 3i32,
    INSTANCER = 4i32,
    MESH = 1i32,
    NONE = 0i32,
    VOLUME = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_PartData+PartOutputType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_PartData_PartOutputType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "PartOutputType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HEU_PartData_PartOutputType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HEU_PartData_PartOutputType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HEU_PartData_PartOutputType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::HoudiniEngineUnity::HEU_PartData_PartOutputType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
