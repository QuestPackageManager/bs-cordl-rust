#[cfg(feature = "Org+BouncyCastle+Cms+EnvelopedDataHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvelopedDataHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Cms+EnvelopedDataHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::EnvelopedDataHelper =>
    "Org.BouncyCastle.Cms"."EnvelopedDataHelper"
);
#[cfg(feature = "Org+BouncyCastle+Cms+EnvelopedDataHelper")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::EnvelopedDataHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+EnvelopedDataHelper")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::EnvelopedDataHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+EnvelopedDataHelper")]
impl crate::Org::BouncyCastle::Cms::EnvelopedDataHelper {
    pub fn CreateKeyGenerator(
        &mut self,
        algorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::CipherKeyGenerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::CipherKeyGenerator = __cordl_object
            .invoke("CreateKeyGenerator", (algorithm, random))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateEncryptionAlgID(
        &mut self,
        encryptionOID: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        encKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("GenerateEncryptionAlgID", (encryptionOID, encKey, random))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "Org+BouncyCastle+Cms+EnvelopedDataHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::EnvelopedDataHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
