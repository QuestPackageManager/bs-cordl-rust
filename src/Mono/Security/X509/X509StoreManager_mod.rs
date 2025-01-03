#[cfg(feature = "Mono+Security+X509+X509StoreManager")]
#[repr(C)]
#[derive(Debug)]
pub struct X509StoreManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Security+X509+X509StoreManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::X509::X509StoreManager =>
    "Mono.Security.X509"."X509StoreManager"
);
#[cfg(feature = "Mono+Security+X509+X509StoreManager")]
impl std::ops::Deref for crate::Mono::Security::X509::X509StoreManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509StoreManager")]
impl std::ops::DerefMut for crate::Mono::Security::X509::X509StoreManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509StoreManager")]
impl crate::Mono::Security::X509::X509StoreManager {
    pub fn get_CurrentUser() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Stores>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Stores,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentUser", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentUserPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentUserPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalMachine() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Stores>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Stores,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_LocalMachine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalMachinePath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_LocalMachinePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TrustedRootCertificates() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509CertificateCollection>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509CertificateCollection,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_TrustedRootCertificates", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+X509+X509StoreManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::X509::X509StoreManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
