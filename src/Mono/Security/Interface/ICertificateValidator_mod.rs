#[cfg(feature = "Mono+Security+Interface+ICertificateValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct ICertificateValidator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Security+Interface+ICertificateValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Interface::ICertificateValidator
    => "Mono.Security.Interface"."ICertificateValidator"
);
#[cfg(feature = "Mono+Security+Interface+ICertificateValidator")]
impl std::ops::Deref for crate::Mono::Security::Interface::ICertificateValidator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Interface+ICertificateValidator")]
impl std::ops::DerefMut for crate::Mono::Security::Interface::ICertificateValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Interface+ICertificateValidator")]
impl crate::Mono::Security::Interface::ICertificateValidator {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Mono+Security+Interface+ICertificateValidator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Interface::ICertificateValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
