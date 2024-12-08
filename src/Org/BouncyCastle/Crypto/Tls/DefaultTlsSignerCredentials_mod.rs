#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsSignerCredentials")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultTlsSignerCredentials {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSignerCredentials,
    pub mContext: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    pub mCertificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    pub mPrivateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    pub mSignatureAndHashAlgorithm: *mut crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
    pub mSigner: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSigner,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsSignerCredentials")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DefaultTlsSignerCredentials =>
    "Org.BouncyCastle.Crypto.Tls"."DefaultTlsSignerCredentials"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsSignerCredentials")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsSignerCredentials {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSignerCredentials;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsSignerCredentials")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsSignerCredentials {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsSignerCredentials")]
impl crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsSignerCredentials {
    pub fn GenerateCertificateSignature(
        &mut self,
        hash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateCertificateSignature", (hash))?;
        Ok(__cordl_ret)
    }
    pub fn New_SignatureAndHashAlgorithm1(
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        certificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        signatureAndHashAlgorithm: *mut crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (context, certificate, privateKey, signatureAndHashAlgorithm),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_TlsContext_Certificate_AsymmetricKeyParameter0(
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        certificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context, certificate, privateKey))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_SignatureAndHashAlgorithm1(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        certificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        signatureAndHashAlgorithm: *mut crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (context, certificate, privateKey, signatureAndHashAlgorithm),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TlsContext_Certificate_AsymmetricKeyParameter0(
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
    pub fn get_SignatureAndHashAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm = __cordl_object
            .invoke("get_SignatureAndHashAlgorithm", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DefaultTlsSignerCredentials")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DefaultTlsSignerCredentials {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
