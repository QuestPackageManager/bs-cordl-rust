#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferInstancer")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_LoadBufferInstancer {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_LoadBufferBase,
    pub _instanceTransforms: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::HoudiniEngineUnity::HAPI_Transform,
    >,
    pub _instancePrefixes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _instanceNodeIDs: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _assetPaths: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _collisionAssetPaths: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferInstancer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_LoadBufferInstancer =>
    "HoudiniEngineUnity"."HEU_LoadBufferInstancer"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferInstancer")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_LoadBufferInstancer {
    type Target = crate::HoudiniEngineUnity::HEU_LoadBufferBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferInstancer")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_LoadBufferInstancer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferInstancer")]
impl crate::HoudiniEngineUnity::HEU_LoadBufferInstancer {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferInstancer")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_LoadBufferInstancer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
