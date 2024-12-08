#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+PskTlsClient")]
#[repr(C)]
#[derive(Debug)]
pub struct PskTlsClient {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsClient,
    pub mDHVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier,
    pub mPskIdentity: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentity,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+PskTlsClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::PskTlsClient =>
    "Org.BouncyCastle.Crypto.Tls"."PskTlsClient"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+PskTlsClient")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::PskTlsClient {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsClient;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+PskTlsClient")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::PskTlsClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+PskTlsClient")]
impl crate::Org::BouncyCastle::Crypto::Tls::PskTlsClient {
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
    pub fn GetAuthentication(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication = __cordl_object
            .invoke("GetAuthentication", ())?;
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
    pub fn New_TlsCipherFactory_TlsDHVerifier_TlsPskIdentity2(
        cipherFactory: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
        dhVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier,
        pskIdentity: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentity,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipherFactory, dhVerifier, pskIdentity))?;
        Ok(__cordl_object)
    }
    pub fn New_TlsCipherFactory_TlsPskIdentity1(
        cipherFactory: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
        pskIdentity: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentity,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipherFactory, pskIdentity))?;
        Ok(__cordl_object)
    }
    pub fn New_TlsPskIdentity0(
        pskIdentity: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentity,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pskIdentity))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_TlsCipherFactory_TlsDHVerifier_TlsPskIdentity2(
        &mut self,
        cipherFactory: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
        dhVerifier: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsDHVerifier,
        pskIdentity: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipherFactory, dhVerifier, pskIdentity))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TlsCipherFactory_TlsPskIdentity1(
        &mut self,
        cipherFactory: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
        pskIdentity: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipherFactory, pskIdentity))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TlsPskIdentity0(
        &mut self,
        pskIdentity: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPskIdentity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pskIdentity))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+PskTlsClient")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::PskTlsClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
