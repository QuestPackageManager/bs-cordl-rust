#[cfg(feature = "HoudiniEngineUnity+HEU_VertexEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_VertexEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _meshKey: i32,
    pub _vertexIndex: i32,
    pub _normalIndex: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VertexEntry")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HEU_VertexEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_VertexEntry";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_VertexEntry")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_VertexEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (meshKey, vertexIndex, normalIndex))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
