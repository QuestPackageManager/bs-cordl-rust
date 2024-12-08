#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RsaDigestSigner")]
#[repr(C)]
#[derive(Debug)]
pub struct RsaDigestSigner {
    __cordl_parent: crate::System::Object,
    pub rsaEngine: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
    pub algId: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub forSigning: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RsaDigestSigner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Signers::RsaDigestSigner =>
    "Org.BouncyCastle.Crypto.Signers"."RsaDigestSigner"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RsaDigestSigner")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Signers::RsaDigestSigner {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RsaDigestSigner")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Signers::RsaDigestSigner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RsaDigestSigner")]
impl crate::Org::BouncyCastle::Crypto::Signers::RsaDigestSigner {
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        forSigning: bool,
        parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (forSigning, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn get_AlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AlgorithmName", ())?;
        Ok(__cordl_ret)
    }
    pub fn VerifySignature(
        &mut self,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("VerifySignature", (signature))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDigest0(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digest))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDigest_DerObjectIdentifier1(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        digestOid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digest, digestOid))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDigest_AlgorithmIdentifier2(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        algId: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digest, algId))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IRsa_IDigest_DerObjectIdentifier3(
        &mut self,
        rsa: *mut crate::Org::BouncyCastle::Crypto::IRsa,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        digestOid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rsa, digest, digestOid))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IRsa_IDigest_AlgorithmIdentifier4(
        &mut self,
        rsa: *mut crate::Org::BouncyCastle::Crypto::IRsa,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        algId: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rsa, digest, algId))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IAsymmetricBlockCipher_IDigest_AlgorithmIdentifier5(
        &mut self,
        rsaEngine: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        algId: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rsaEngine, digest, algId))?;
        Ok(__cordl_ret)
    }
    pub fn BlockUpdate(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BlockUpdate", (input, inOff, length))?;
        Ok(__cordl_ret)
    }
    pub fn DerEncode(
        &mut self,
        hash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("DerEncode", (hash))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateSignature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateSignature", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        input: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (input))?;
        Ok(__cordl_ret)
    }
    pub fn New_IDigest0(
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digest))?;
        Ok(__cordl_object)
    }
    pub fn New_IDigest_DerObjectIdentifier1(
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        digestOid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digest, digestOid))?;
        Ok(__cordl_object)
    }
    pub fn New_IDigest_AlgorithmIdentifier2(
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        algId: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digest, algId))?;
        Ok(__cordl_object)
    }
    pub fn New_IRsa_IDigest_DerObjectIdentifier3(
        rsa: *mut crate::Org::BouncyCastle::Crypto::IRsa,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        digestOid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rsa, digest, digestOid))?;
        Ok(__cordl_object)
    }
    pub fn New_IRsa_IDigest_AlgorithmIdentifier4(
        rsa: *mut crate::Org::BouncyCastle::Crypto::IRsa,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        algId: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rsa, digest, algId))?;
        Ok(__cordl_object)
    }
    pub fn New_IAsymmetricBlockCipher_IDigest_AlgorithmIdentifier5(
        rsaEngine: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        algId: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rsaEngine, digest, algId))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RsaDigestSigner")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Signers::RsaDigestSigner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
