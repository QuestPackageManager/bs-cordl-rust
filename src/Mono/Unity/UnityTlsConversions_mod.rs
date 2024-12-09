#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTlsConversions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTlsConversions => "Mono.Unity"
    ."UnityTlsConversions"
);
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
impl std::ops::Deref for crate::Mono::Unity::UnityTlsConversions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
impl std::ops::DerefMut for crate::Mono::Unity::UnityTlsConversions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
impl crate::Mono::Unity::UnityTlsConversions {}
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Unity::UnityTlsConversions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
