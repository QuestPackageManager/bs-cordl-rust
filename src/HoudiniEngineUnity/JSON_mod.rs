#[cfg(feature = "HoudiniEngineUnity+JSON")]
#[repr(C)]
#[derive(Debug)]
pub struct JSON {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "HoudiniEngineUnity+JSON")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::JSON => "HoudiniEngineUnity"
    ."JSON"
);
#[cfg(feature = "HoudiniEngineUnity+JSON")]
impl std::ops::Deref for crate::HoudiniEngineUnity::JSON {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSON")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::JSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSON")]
impl crate::HoudiniEngineUnity::JSON {}
#[cfg(feature = "HoudiniEngineUnity+JSON")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::JSON {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
