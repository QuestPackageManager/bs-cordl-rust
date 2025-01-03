#[cfg(feature = "Mono+Security+Interface+MonoTlsProviderFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoTlsProviderFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Security+Interface+MonoTlsProviderFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Security::Interface::MonoTlsProviderFactory => "Mono.Security.Interface"
    ."MonoTlsProviderFactory"
);
#[cfg(feature = "Mono+Security+Interface+MonoTlsProviderFactory")]
impl std::ops::Deref for crate::Mono::Security::Interface::MonoTlsProviderFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Interface+MonoTlsProviderFactory")]
impl std::ops::DerefMut for crate::Mono::Security::Interface::MonoTlsProviderFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Interface+MonoTlsProviderFactory")]
impl crate::Mono::Security::Interface::MonoTlsProviderFactory {
    pub fn GetProvider() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::MonoTlsProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetProvider", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+Interface+MonoTlsProviderFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Interface::MonoTlsProviderFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
