#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+PskTlsServer")]
#[repr(C)]
#[derive(Debug)]
pub struct PskTlsServer {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsServer,
    pub mPskIdentityManager: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentityManager,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+PskTlsServer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::PskTlsServer =>
    "Org.BouncyCastle.Crypto.Tls"."PskTlsServer"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+PskTlsServer")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::PskTlsServer {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsServer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+PskTlsServer")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::PskTlsServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+PskTlsServer")]
impl crate::Org::BouncyCastle::Crypto::Tls::PskTlsServer {
    pub fn CreatePskKeyExchange(
        &mut self,
        keyExchange: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange = __cordl_object
            .invoke("CreatePskKeyExchange", (keyExchange))?;
        Ok(__cordl_ret)
    }
    pub fn GetCipherSuites(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("GetCipherSuites", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCredentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCredentials = __cordl_object
            .invoke("GetCredentials", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDHParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters = __cordl_object
            .invoke("GetDHParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetKeyExchange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange = __cordl_object
            .invoke("GetKeyExchange", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRsaEncryptionCredentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsEncryptionCredentials,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsEncryptionCredentials = __cordl_object
            .invoke("GetRsaEncryptionCredentials", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_TlsCipherFactory_TlsPskIdentityManager1(
        cipherFactory: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
        pskIdentityManager: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentityManager,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipherFactory, pskIdentityManager))?;
        Ok(__cordl_object)
    }
    pub fn New_TlsPskIdentityManager0(
        pskIdentityManager: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentityManager,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pskIdentityManager))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_TlsCipherFactory_TlsPskIdentityManager1(
        &mut self,
        cipherFactory: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
        pskIdentityManager: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentityManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipherFactory, pskIdentityManager))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TlsPskIdentityManager0(
        &mut self,
        pskIdentityManager: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentityManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pskIdentityManager))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+PskTlsServer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::PskTlsServer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
