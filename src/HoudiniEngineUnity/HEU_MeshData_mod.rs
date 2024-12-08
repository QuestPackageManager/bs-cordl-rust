#[cfg(feature = "HoudiniEngineUnity+HEU_MeshData")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_MeshData {
    __cordl_parent: crate::System::Object,
    pub _indices: *mut crate::System::Collections::Generic::List_1<i32>,
    pub _vertices: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector3,
    >,
    pub _colors: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Color32,
    >,
    pub _normals: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector3,
    >,
    pub _tangents: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector4,
    >,
    pub _uvs: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
    >,
    pub _triangleNormals: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::Vector3,
    >,
    pub _pointIndexToMeshIndexMap: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        i32,
    >,
    pub _meshTopology: crate::UnityEngine::MeshTopology,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MeshData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_MeshData =>
    "HoudiniEngineUnity"."HEU_MeshData"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_MeshData")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_MeshData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MeshData")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_MeshData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MeshData")]
impl crate::HoudiniEngineUnity::HEU_MeshData {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_MeshData")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_MeshData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
