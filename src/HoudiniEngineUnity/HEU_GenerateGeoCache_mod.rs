#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo+ColliderType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HEU_ColliderInfo_ColliderType {
    BOX = 1i32,
    MESH = 3i32,
    NONE = 0i32,
    SIMPLE_BOX = 4i32,
    SIMPLE_CAPSULE = 6i32,
    SIMPLE_SPHERE = 5i32,
    SPHERE = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo+ColliderType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_ColliderInfo_ColliderType => "HoudiniEngineUnity"
    ."HEU_GenerateGeoCache/HEU_ColliderInfo/ColliderType"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_GenerateGeoCache_HEU_ColliderInfo {
    __cordl_parent: crate::System::Object,
    pub _colliderType: crate::HoudiniEngineUnity::HEU_ColliderInfo_ColliderType,
    pub _colliderCenter: crate::UnityEngine::Vector3,
    pub _colliderSize: crate::UnityEngine::Vector3,
    pub _colliderRadius: f32,
    pub _convexCollider: bool,
    pub _collisionGroupName: *mut crate::System::String,
    pub _collisionVertices: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector3,
    >,
    pub _collisionIndices: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _meshTopology: crate::UnityEngine::MeshTopology,
    pub _isTrigger: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_GenerateGeoCache_HEU_ColliderInfo => "HoudiniEngineUnity"
    ."HEU_GenerateGeoCache/HEU_ColliderInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+HEU_ColliderInfo")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_GenerateGeoCache_HEU_ColliderInfo {
    type Target = crate::System::Object;
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
    pub type ColliderType = crate::HoudiniEngineUnity::HEU_ColliderInfo_ColliderType;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_GenerateGeoCache {
    __cordl_parent: crate::System::Object,
    pub _AssetID_k__BackingField: i32,
    pub _geoInfo: crate::HoudiniEngineUnity::HAPI_GeoInfo,
    pub _partInfo: crate::HoudiniEngineUnity::HAPI_PartInfo,
    pub _partName: *mut crate::System::String,
    pub _vertexList: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _faceCounts: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _houdiniMaterialIDs: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _singleFaceUnityMaterial: bool,
    pub _singleFaceHoudiniMaterial: bool,
    pub _unityMaterialInfos: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::HoudiniEngineUnity::HEU_UnityMaterialInfo,
    >,
    pub _unityMaterialAttrInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _unityMaterialAttrName: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _unityMaterialAttrStringsMap: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::System::String,
    >,
    pub _substanceMaterialAttrNameInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _substanceMaterialAttrName: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _substanceMaterialAttrStringsMap: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::System::String,
    >,
    pub _substanceMaterialAttrIndexInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _substanceMaterialAttrIndex: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _inUseMaterials: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_MaterialData,
    >,
    pub _posAttrInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _uvsAttrInfo: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    >,
    pub _normalAttrInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _colorAttrInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _alphaAttrInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _tangentAttrInfo: crate::HoudiniEngineUnity::HAPI_AttributeInfo,
    pub _posAttr: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _uvsAttr: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
    pub _normalAttr: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _colorAttr: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _alphaAttr: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _tangentAttr: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _groups: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub _hasGroupGeometry: bool,
    pub _groupSplitVertexIndices: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _groupSplitFaceIndices: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::Collections::Generic::List_1<i32>,
    >,
    pub _groupVertexOffsets: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::Collections::Generic::List_1<i32>,
    >,
    pub _allCollisionVertexList: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _allCollisionFaceIndices: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _normalCosineThreshold: f32,
    pub _hasLODGroups: bool,
    pub _LODTransitionValues: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _isMeshReadWrite: bool,
    pub _colliderInfos: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_GenerateGeoCache_HEU_ColliderInfo,
    >,
    pub _materialCache: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_MaterialData,
    >,
    pub _materialIDToDataMap: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::HoudiniEngineUnity::HEU_MaterialData,
    >,
    pub _assetCacheFolderPath: *mut crate::System::String,
    pub _meshIndexFormat: *mut crate::HoudiniEngineUnity::HEU_MeshIndexFormat,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_GenerateGeoCache =>
    "HoudiniEngineUnity"."HEU_GenerateGeoCache"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_GenerateGeoCache {
    type Target = crate::System::Object;
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
    #[cfg(feature = "HoudiniEngineUnity+HEU_GenerateGeoCache+__c")]
    pub type __c = crate::HoudiniEngineUnity::HEU_GenerateGeoCache___c;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PopulateGeometryData(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        bUseLODGroups: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PopulateGeometryData", (session, bUseLODGroups))?;
        Ok(__cordl_ret)
    }
    pub fn PopulateUnityMaterialData(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateUnityMaterialData", (session))?;
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
    pub fn get_AssetID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AssetID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GeoID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_GeoID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PartID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PartID", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
