#[cfg(feature = "Mono+Net+Security+MonoTlsProviderFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoTlsProviderFactory {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Mono+Net+Security+MonoTlsProviderFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::MonoTlsProviderFactory =>
    "Mono.Net.Security"."MonoTlsProviderFactory"
);
#[cfg(feature = "Mono+Net+Security+MonoTlsProviderFactory")]
impl std::ops::Deref for crate::Mono::Net::Security::MonoTlsProviderFactory {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
impl crate::Mono::Net::Security::MonoTlsProviderFactory {
    pub fn CreateDefaultProviderImpl() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MobileTlsProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDefaultProviderImpl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProvider() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MobileTlsProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetProvider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProviderInternal() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MobileTlsProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProviderInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeInternal() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeProviderRegistration() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeProviderRegistration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LookupProvider(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        throwOnError: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MobileTlsProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LookupProvider", (name, throwOnError))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateProviders() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PopulateProviders", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateUnityProviders() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PopulateUnityProviders", ())?;
        Ok(__cordl_ret.into())
    }
}
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
