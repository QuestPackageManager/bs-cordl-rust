#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct RsaOaepWrapper {
    __cordl_parent: crate::System::Object,
    pub algId: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub engine: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper =>
    "Org.BouncyCastle.Crypto.Operators"."RsaOaepWrapper"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
impl crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper {
    pub fn New(
        forWrapping: bool,
        parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
        digestOid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (forWrapping, parameters, digestOid))?;
        Ok(__cordl_object)
    }
    pub fn Unwrap(
        &mut self,
        cipherText: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IBlockResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IBlockResult = __cordl_object
            .invoke("Unwrap", (cipherText, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn Wrap(
        &mut self,
        keyData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IBlockResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IBlockResult = __cordl_object
            .invoke("Wrap", (keyData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        forWrapping: bool,
        parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
        digestOid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (forWrapping, parameters, digestOid))?;
        Ok(__cordl_ret)
    }
    pub fn get_AlgorithmDetails(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_AlgorithmDetails", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
