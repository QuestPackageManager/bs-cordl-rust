#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsEncryptionCredentials")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultTlsEncryptionCredentials {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsEncryptionCredentials,
    pub mContext: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    pub mCertificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    pub mPrivateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsEncryptionCredentials")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DefaultTlsEncryptionCredentials =>
    "Org.BouncyCastle.Crypto.Tls"."DefaultTlsEncryptionCredentials"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsEncryptionCredentials")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsEncryptionCredentials {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsEncryptionCredentials;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsEncryptionCredentials")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsEncryptionCredentials {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsEncryptionCredentials")]
impl crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsEncryptionCredentials {
    pub fn DecryptPreMasterSecret(
        &mut self,
        encryptedPreMasterSecret: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("DecryptPreMasterSecret", (encryptedPreMasterSecret))?;
        Ok(__cordl_ret)
    }
    pub fn get_Certificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate = __cordl_object
            .invoke("get_Certificate", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        certificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (context, certificate, privateKey))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        certificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context, certificate, privateKey))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsEncryptionCredentials")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsEncryptionCredentials {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
