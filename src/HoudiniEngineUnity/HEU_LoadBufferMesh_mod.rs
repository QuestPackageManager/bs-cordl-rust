#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferMesh")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_LoadBufferMesh {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_LoadBufferBase,
    pub _geoCache: *mut crate::HoudiniEngineUnity::HEU_GenerateGeoCache,
    pub _LODGroupMeshes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_GeoGroup,
    >,
    pub _defaultMaterialKey: i32,
    pub _bGenerateUVs: bool,
    pub _bGenerateTangents: bool,
    pub _bGenerateNormals: bool,
    pub _bPartInstanced: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferMesh")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_LoadBufferMesh =>
    "HoudiniEngineUnity"."HEU_LoadBufferMesh"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferMesh")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_LoadBufferMesh {
    type Target = crate::HoudiniEngineUnity::HEU_LoadBufferBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferMesh")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_LoadBufferMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferMesh")]
impl crate::HoudiniEngineUnity::HEU_LoadBufferMesh {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferMesh")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_LoadBufferMesh {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
