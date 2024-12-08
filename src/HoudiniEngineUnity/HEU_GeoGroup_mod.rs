#[cfg(feature = "HoudiniEngineUnity+HEU_GeoGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_GeoGroup {
    __cordl_parent: crate::System::Object,
    pub _groupName: *mut crate::System::String,
    pub _subMeshesMap: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::HoudiniEngineUnity::HEU_MeshData,
    >,
    pub _sharedNormalIndices: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::HoudiniEngineUnity::HEU_VertexEntry,
        >,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GeoGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_GeoGroup =>
    "HoudiniEngineUnity"."HEU_GeoGroup"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_GeoGroup")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_GeoGroup {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GeoGroup")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_GeoGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_GeoGroup")]
impl crate::HoudiniEngineUnity::HEU_GeoGroup {
    pub fn SetupNormalIndices(
        &mut self,
        indicesCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupNormalIndices", (indicesCount))?;
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
    pub fn CompareTo(
        &mut self,
        other: *mut crate::HoudiniEngineUnity::HEU_GeoGroup,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (other))?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_GeoGroup")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_GeoGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
