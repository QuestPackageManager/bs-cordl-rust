#[cfg(feature = "Mono+Security+X509+X501")]
#[repr(C)]
#[derive(Debug)]
pub struct X501 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Mono+Security+X509+X501")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::X509::X501 =>
    "Mono.Security.X509"."X501"
);
#[cfg(feature = "Mono+Security+X509+X501")]
impl std::ops::Deref for crate::Mono::Security::X509::X501 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X501")]
impl std::ops::DerefMut for crate::Mono::Security::X509::X501 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X501")]
impl crate::Mono::Security::X509::X501 {}
#[cfg(feature = "Mono+Security+X509+X501")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::X509::X501 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
