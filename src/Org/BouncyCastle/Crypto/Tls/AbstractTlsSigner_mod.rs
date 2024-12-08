#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSigner")]
#[repr(C)]
#[derive(Debug)]
pub struct AbstractTlsSigner {
    __cordl_parent: crate::System::Object,
    pub mContext: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSigner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::AbstractTlsSigner => "Org.BouncyCastle.Crypto.Tls"
    ."AbstractTlsSigner"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSigner")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSigner {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSigner")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSigner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSigner")]
impl crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSigner {
    pub fn Init(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (context))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSigner_AsymmetricKeyParameter0(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::ISigner> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ISigner = __cordl_object
            .invoke("CreateSigner", (privateKey))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSigner_SignatureAndHashAlgorithm_AsymmetricKeyParameter1(
        &mut self,
        algorithm: *mut crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::ISigner> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ISigner = __cordl_object
            .invoke("CreateSigner", (algorithm, privateKey))?;
        Ok(__cordl_ret)
    }
    pub fn VerifyRawSignature_Il2CppArray_AsymmetricKeyParameter_Il2CppArray0(
        &mut self,
        sigBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        md5AndSha1: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("VerifyRawSignature", (sigBytes, publicKey, md5AndSha1))?;
        Ok(__cordl_ret)
    }
    pub fn VerifyRawSignature_SignatureAndHashAlgorithm_Il2CppArray_AsymmetricKeyParameter_Il2CppArray1(
        &mut self,
        algorithm: *mut crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        sigBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        hash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("VerifyRawSignature", (algorithm, sigBytes, publicKey, hash))?;
        Ok(__cordl_ret)
    }
    pub fn IsValidPublicKey(
        &mut self,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValidPublicKey", (publicKey))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateRawSignature_AsymmetricKeyParameter_Il2CppArray0(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        md5AndSha1: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateRawSignature", (privateKey, md5AndSha1))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateRawSignature_SignatureAndHashAlgorithm_AsymmetricKeyParameter_Il2CppArray1(
        &mut self,
        algorithm: *mut crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        hash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateRawSignature", (algorithm, privateKey, hash))?;
        Ok(__cordl_ret)
    }
    pub fn CreateVerifyer_AsymmetricKeyParameter0(
        &mut self,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::ISigner> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ISigner = __cordl_object
            .invoke("CreateVerifyer", (publicKey))?;
        Ok(__cordl_ret)
    }
    pub fn CreateVerifyer_SignatureAndHashAlgorithm_AsymmetricKeyParameter1(
        &mut self,
        algorithm: *mut crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::ISigner> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ISigner = __cordl_object
            .invoke("CreateVerifyer", (algorithm, publicKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsSigner")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSigner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
