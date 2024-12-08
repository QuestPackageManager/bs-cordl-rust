#[cfg(feature = "HoudiniEngineUnity+HEU_VertexEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_VertexEntry {
    __cordl_parent: crate::System::Object,
    pub _meshKey: i32,
    pub _vertexIndex: i32,
    pub _normalIndex: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VertexEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_VertexEntry =>
    "HoudiniEngineUnity"."HEU_VertexEntry"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_VertexEntry")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_VertexEntry {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VertexEntry")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_VertexEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VertexEntry")]
impl crate::HoudiniEngineUnity::HEU_VertexEntry {
    pub fn New(
        meshKey: i32,
        vertexIndex: i32,
        normalIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (meshKey, vertexIndex, normalIndex))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        meshKey: i32,
        vertexIndex: i32,
        normalIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (meshKey, vertexIndex, normalIndex))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VertexEntry")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_VertexEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
