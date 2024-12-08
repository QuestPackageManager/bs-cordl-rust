#[cfg(feature = "HoudiniEngineUnity+HEU_MeshIndexFormat")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_MeshIndexFormat {
    __cordl_parent: crate::System::Object,
    pub _indexFormat: crate::UnityEngine::Rendering::IndexFormat,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MeshIndexFormat")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_MeshIndexFormat =>
    "HoudiniEngineUnity"."HEU_MeshIndexFormat"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_MeshIndexFormat")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_MeshIndexFormat {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MeshIndexFormat")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_MeshIndexFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MeshIndexFormat")]
impl crate::HoudiniEngineUnity::HEU_MeshIndexFormat {
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
    pub fn SetFormatForMesh(
        &mut self,
        mesh: *mut crate::UnityEngine::Mesh,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFormatForMesh", (mesh))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateIndexFormat(
        &mut self,
        numVertices: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateIndexFormat", (numVertices))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_MeshIndexFormat")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_MeshIndexFormat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
