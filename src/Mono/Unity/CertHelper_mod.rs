#[cfg(feature = "Mono+Unity+CertHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct CertHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Unity+CertHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::CertHelper => "Mono.Unity"
    ."CertHelper"
);
#[cfg(feature = "Mono+Unity+CertHelper")]
impl std::ops::Deref for crate::Mono::Unity::CertHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+CertHelper")]
impl std::ops::DerefMut for crate::Mono::Unity::CertHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+CertHelper")]
impl crate::Mono::Unity::CertHelper {}
#[cfg(feature = "Mono+Unity+CertHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Unity::CertHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
