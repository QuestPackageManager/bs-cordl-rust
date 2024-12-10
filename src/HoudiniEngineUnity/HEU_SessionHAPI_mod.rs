#[cfg(feature = "HoudiniEngineUnity+HEU_SessionHAPI")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_SessionHAPI {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_SessionBase,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionHAPI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_SessionHAPI =>
    "HoudiniEngineUnity"."HEU_SessionHAPI"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionHAPI")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_SessionHAPI {
    type Target = crate::HoudiniEngineUnity::HEU_SessionBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionHAPI")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_SessionHAPI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionHAPI")]
impl crate::HoudiniEngineUnity::HEU_SessionHAPI {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionHAPI")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_SessionHAPI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
