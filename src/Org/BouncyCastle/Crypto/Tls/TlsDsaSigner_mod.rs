#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDsaSigner")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsDsaSigner {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSigner,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDsaSigner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::TlsDsaSigner =>
    "Org.BouncyCastle.Crypto.Tls"."TlsDsaSigner"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDsaSigner")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsDsaSigner {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsSigner;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDsaSigner")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsDsaSigner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDsaSigner")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsDsaSigner {
    pub fn CreateDsaImpl(
        &mut self,
        hashAlgorithm: u8,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::IDsa> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IDsa = __cordl_object
            .invoke("CreateDsaImpl", (hashAlgorithm))?;
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
    pub fn CreateVerifyer(
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
    pub fn CreateSigner(
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
    pub fn get_SignatureAlgorithm(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_SignatureAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn MakeSigner(
        &mut self,
        algorithm: *mut crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        raw: bool,
        forSigning: bool,
        cp: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::ISigner> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ISigner = __cordl_object
            .invoke("MakeSigner", (algorithm, raw, forSigning, cp))?;
        Ok(__cordl_ret)
    }
    pub fn VerifyRawSignature(
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
    pub fn GenerateRawSignature(
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
    pub fn MakeInitParameters(
        &mut self,
        forSigning: bool,
        cp: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters = __cordl_object
            .invoke("MakeInitParameters", (forSigning, cp))?;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsDsaSigner")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsDsaSigner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
