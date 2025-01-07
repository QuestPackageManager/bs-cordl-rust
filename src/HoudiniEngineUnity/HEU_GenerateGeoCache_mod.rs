#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo+ColliderType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_ColliderInfo_HEU_GenerateGeoCache_ColliderType {
    #[default]
    BOX = 1i32,
    MESH = 3i32,
    NONE = 0i32,
    SIMPLE_BOX = 4i32,
    SIMPLE_CAPSULE = 6i32,
    SIMPLE_SPHERE = 5i32,
    SPHERE = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo+ColliderType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_ColliderInfo_HEU_GenerateGeoCache_ColliderType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "ColliderType";
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
for crate::HoudiniEngineUnity::HEU_ColliderInfo_HEU_GenerateGeoCache_ColliderType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HEU_ColliderInfo_HEU_GenerateGeoCache_ColliderType {
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
for crate::HoudiniEngineUnity::HEU_ColliderInfo_HEU_GenerateGeoCache_ColliderType {
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
for crate::HoudiniEngineUnity::HEU_ColliderInfo_HEU_GenerateGeoCache_ColliderType {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_GenerateGeoCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _AssetID_k__BackingField: i32,
    pub _geoInfo: crate::HoudiniEngineUnity::HAPI_GeoInfo,
    pub _partInfo: crate::HoudiniEngineUnity::HAPI_PartInfo,
    pub _partName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _vertexList: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub _faceCounts: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub _houdiniMaterialIDs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _singleFaceUnityMaterial: bool,
    pub _singleFaceHoudiniMaterial: bool,
    pub _unityMaterialInfos: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_UnityMaterialInfo>,
        >,
    >,
    pub _unityMaterialAttrInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _unityMaterialAttrName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _unityMaterialAttrStringsMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _substanceMaterialAttrNameInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _substanceMaterialAttrName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _substanceMaterialAttrStringsMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _substanceMaterialAttrIndexInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _substanceMaterialAttrIndex: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _inUseMaterials: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
        >,
    >,
    pub _posAttrInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _uvsAttrInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::HoudiniEngineUnity::HAPI_AttributeInfo>,
    >,
    pub _normalAttrInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _colorAttrInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _alphaAttrInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _tangentAttrInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _posAttr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub _uvsAttr: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
    >,
    pub _normalAttr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub _colorAttr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub _alphaAttr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub _tangentAttr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub _groups: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _hasGroupGeometry: bool,
    pub _groupSplitVertexIndices: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
    >,
    pub _groupSplitFaceIndices: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
        >,
    >,
    pub _groupVertexOffsets: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
        >,
    >,
    pub _allCollisionVertexList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _allCollisionFaceIndices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _normalCosineThreshold: f32,
    pub _hasLODGroups: bool,
    pub _LODTransitionValues: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
    pub _isMeshReadWrite: bool,
    pub _colliderInfos: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::HoudiniEngineUnity::HEU_GenerateGeoCache_HEU_ColliderInfo,
            >,
        >,
    >,
    pub _materialCache: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
        >,
    >,
    pub _materialIDToDataMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
        >,
    >,
    pub _assetCacheFolderPath: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _meshIndexFormat: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_MeshIndexFormat,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_GenerateGeoCache {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_GenerateGeoCache";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_GenerateGeoCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_GenerateGeoCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache")]
impl crate::HoudiniEngineUnity::HEU_GenerateGeoCache {
    #[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo")]
    pub type HEU_ColliderInfo = crate::HoudiniEngineUnity::HEU_GenerateGeoCache_HEU_ColliderInfo;
    pub fn CalculateGroupMeshTopology(
        groupFaces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        allFaceCounts: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::MeshTopology> {
        let __cordl_ret: crate::UnityEngine::MeshTopology = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateGroupMeshTopology", (groupFaces, allFaceCounts))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineMeshes(
        subMeshesMap: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                i32,
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MeshData>,
            >,
        >,
        submeshIndices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        bGenerateUVs: bool,
        bGenerateNormals: bool,
        meshIndexFormat: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_MeshIndexFormat,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CombineMeshes",
                (
                    subMeshesMap,
                    submeshIndices,
                    bGenerateUVs,
                    bGenerateNormals,
                    meshIndexFormat,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineQuadMeshes(
        subMeshesMap: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                i32,
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MeshData>,
            >,
        >,
        subMeshIndices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        bGenerateNormals: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CombineQuadMeshes",
                (subMeshesMap, subMeshIndices, bGenerateNormals),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMaterialInfoEntryFromAttributeIndex(
        geoCache: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GenerateGeoCache,
        >,
        materialAttributeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateMaterialInfoEntryFromAttributeIndex",
                (geoCache, materialAttributeIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMeshFromMeshData(
        submesh: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MeshData>,
        bGenerateUVs: bool,
        bGenerateNormals: bool,
        meshIndexFormat: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_MeshIndexFormat,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateMeshFromMeshData",
                (submesh, bGenerateUVs, bGenerateNormals, meshIndexFormat),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateGeoGroupUsingGeoCachePoints(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoCache: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GenerateGeoCache,
        >,
        bGenerateUVs: bool,
        bGenerateTangents: bool,
        bGenerateNormals: bool,
        bUseLODGroups: bool,
        bPartInstanced: bool,
        LODGroupMeshes: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoGroup>,
                >,
            >,
        >,
        defaultMaterialKey: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateGeoGroupUsingGeoCachePoints",
                (
                    session,
                    geoCache,
                    bGenerateUVs,
                    bGenerateTangents,
                    bGenerateNormals,
                    bUseLODGroups,
                    bPartInstanced,
                    LODGroupMeshes,
                    defaultMaterialKey,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateGeoGroupUsingGeoCacheVertices(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoCache: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GenerateGeoCache,
        >,
        bGenerateUVs: bool,
        bGenerateTangents: bool,
        bGenerateNormals: bool,
        bUseLODGroups: bool,
        bPartInstanced: bool,
        LODGroupMeshes: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoGroup>,
                >,
            >,
        >,
        defaultMaterialKey: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateGeoGroupUsingGeoCacheVertices",
                (
                    session,
                    geoCache,
                    bGenerateUVs,
                    bGenerateTangents,
                    bGenerateNormals,
                    bUseLODGroups,
                    bPartInstanced,
                    LODGroupMeshes,
                    defaultMaterialKey,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateLODMeshesFromGeoGroups(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        GeoGroupMeshes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoGroup>,
            >,
        >,
        geoCache: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GenerateGeoCache,
        >,
        generatedOutput: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutput,
        >,
        defaultMaterialKey: i32,
        bGenerateUVs: bool,
        bGenerateTangents: bool,
        bGenerateNormals: bool,
        bPartInstanced: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateLODMeshesFromGeoGroups",
                (
                    session,
                    GeoGroupMeshes,
                    geoCache,
                    generatedOutput,
                    defaultMaterialKey,
                    bGenerateUVs,
                    bGenerateTangents,
                    bGenerateNormals,
                    bPartInstanced,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateMeshFromGeoGroup(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        GeoGroup: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoGroup>,
        geoCache: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GenerateGeoCache,
        >,
        defaultMaterialKey: i32,
        bGenerateUVs: bool,
        bGenerateTangents: bool,
        bGenerateNormals: bool,
        bPartInstanced: bool,
        newMesh: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        >,
        newMaterials: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateMeshFromGeoGroup",
                (
                    session,
                    GeoGroup,
                    geoCache,
                    defaultMaterialKey,
                    bGenerateUVs,
                    bGenerateTangents,
                    bGenerateNormals,
                    bPartInstanced,
                    newMesh,
                    newMaterials,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateMeshFromSingleGroup(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        GeoGroup: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GeoGroup>,
        geoCache: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GenerateGeoCache,
        >,
        generatedOutput: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutput,
        >,
        defaultMaterialKey: i32,
        bGenerateUVs: bool,
        bGenerateTangents: bool,
        bGenerateNormals: bool,
        bPartInstanced: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateMeshFromSingleGroup",
                (
                    session,
                    GeoGroup,
                    geoCache,
                    generatedOutput,
                    defaultMaterialKey,
                    bGenerateUVs,
                    bGenerateTangents,
                    bGenerateNormals,
                    bPartInstanced,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFinalMaterialsFromComparingNewWithPrevious(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        previousMaterials: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
        newMaterials: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
        finalMaterials: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetFinalMaterialsFromComparingNewWithPrevious",
                (gameObject, previousMaterials, newMaterials, finalMaterials),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaterialKeyFromAttributeIndex(
        geoCache: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GenerateGeoCache,
        >,
        attributeIndex: i32,
        unityMaterialName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        substanceName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        substanceIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetMaterialKeyFromAttributeIndex",
                (
                    geoCache,
                    attributeIndex,
                    unityMaterialName,
                    substanceName,
                    substanceIndex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPopulatedGeoCache(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        assetID: i32,
        geoID: i32,
        partID: i32,
        bUseLODGroups: bool,
        materialCache: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_MaterialData>,
            >,
        >,
        assetCacheFolderPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_GenerateGeoCache>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GenerateGeoCache,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetPopulatedGeoCache",
                (
                    session,
                    assetID,
                    geoID,
                    partID,
                    bUseLODGroups,
                    materialCache,
                    assetCacheFolderPath,
                ),
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
    pub fn ParseLODTransitionAttribute(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        LODTransitionValues: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ParseLODTransitionAttribute",
                (session, geoID, partID, LODTransitionValues),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateGeometryData(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        bUseLODGroups: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PopulateGeometryData", (session, bUseLODGroups))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateUnityMaterialData(
        &mut self,
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateUnityMaterialData", (session))?;
        Ok(__cordl_ret.into())
    }
    pub fn TransferRegularAttributesToVertices(
        groupVertexList: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        allFaceCounts: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        groupFaces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        groupVertexOffset: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        attribInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_AttributeInfo,
        >,
        inData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        outData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TransferRegularAttributesToVertices",
                (
                    groupVertexList,
                    allFaceCounts,
                    groupFaces,
                    groupVertexOffset,
                    attribInfo,
                    inData,
                    outData,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCollider(
        geoCache: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GenerateGeoCache,
        >,
        outputData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutputData,
        >,
        colliderInfo: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GenerateGeoCache_HEU_ColliderInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateCollider", (geoCache, outputData, colliderInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateColliders(
        geoCache: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GenerateGeoCache,
        >,
        outputData: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_GeneratedOutputData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateColliders", (geoCache, outputData))?;
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
    pub fn get_AssetID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AssetID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GeoID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_GeoID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PartID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PartID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AssetID(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AssetID", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_GenerateGeoCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_GenerateGeoCache_HEU_ColliderInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _colliderType: crate::HoudiniEngineUnity::HEU_ColliderInfo_HEU_GenerateGeoCache_ColliderType,
    pub _colliderCenter: crate::UnityEngine::Vector3,
    pub _colliderSize: crate::UnityEngine::Vector3,
    pub _colliderRadius: f32,
    pub _convexCollider: bool,
    pub _collisionGroupName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _collisionVertices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
    >,
    pub _collisionIndices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _meshTopology: crate::UnityEngine::MeshTopology,
    pub _isTrigger: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HEU_GenerateGeoCache_HEU_ColliderInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_ColliderInfo";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_GenerateGeoCache_HEU_ColliderInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_GenerateGeoCache_HEU_ColliderInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo")]
impl crate::HoudiniEngineUnity::HEU_GenerateGeoCache_HEU_ColliderInfo {
    #[cfg(
        feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo+ColliderType"
    )]
    pub type ColliderType = crate::HoudiniEngineUnity::HEU_ColliderInfo_HEU_GenerateGeoCache_ColliderType;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_GenerateGeoCache_HEU_ColliderInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
