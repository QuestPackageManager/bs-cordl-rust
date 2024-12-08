#[cfg(feature = "OculusXRPlugin")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusXRPlugin {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OculusXRPlugin")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OculusXRPlugin => ""."OculusXRPlugin"
);
#[cfg(feature = "OculusXRPlugin")]
impl std::ops::Deref for OculusXRPlugin {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusXRPlugin")]
impl std::ops::DerefMut for OculusXRPlugin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusXRPlugin")]
impl OculusXRPlugin {}
#[cfg(feature = "OculusXRPlugin")]
impl quest_hook::libil2cpp::ObjectType for OculusXRPlugin {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
