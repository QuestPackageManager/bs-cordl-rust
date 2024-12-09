#[cfg(feature = "Mono+Net+Security+MonoTlsProviderFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoTlsProviderFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Net+Security+MonoTlsProviderFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::MonoTlsProviderFactory =>
    "Mono.Net.Security"."MonoTlsProviderFactory"
);
#[cfg(feature = "Mono+Net+Security+MonoTlsProviderFactory")]
impl std::ops::Deref for crate::Mono::Net::Security::MonoTlsProviderFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+MonoTlsProviderFactory")]
impl std::ops::DerefMut for crate::Mono::Net::Security::MonoTlsProviderFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+MonoTlsProviderFactory")]
impl crate::Mono::Net::Security::MonoTlsProviderFactory {}
#[cfg(feature = "Mono+Net+Security+MonoTlsProviderFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::MonoTlsProviderFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
