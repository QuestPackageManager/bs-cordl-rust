#[cfg(feature = "Mono+Security+ASN1Convert")]
#[repr(C)]
#[derive(Debug)]
pub struct ASN1Convert {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Mono+Security+ASN1Convert")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::ASN1Convert => "Mono.Security"
    ."ASN1Convert"
);
#[cfg(feature = "Mono+Security+ASN1Convert")]
impl std::ops::Deref for crate::Mono::Security::ASN1Convert {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+ASN1Convert")]
impl std::ops::DerefMut for crate::Mono::Security::ASN1Convert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+ASN1Convert")]
impl crate::Mono::Security::ASN1Convert {}
#[cfg(feature = "Mono+Security+ASN1Convert")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::ASN1Convert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}