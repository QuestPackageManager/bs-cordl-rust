#[cfg(feature = "Org+BouncyCastle+Cms+PasswordRecipientInfoGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct PasswordRecipientInfoGenerator {
    __cordl_parent: crate::System::Object,
    pub keyDerivationAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub keyEncryptionKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    pub keyEncryptionKeyOID: *mut crate::System::String,
}
#[cfg(feature = "Org+BouncyCastle+Cms+PasswordRecipientInfoGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::PasswordRecipientInfoGenerator => "Org.BouncyCastle.Cms"
    ."PasswordRecipientInfoGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+PasswordRecipientInfoGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::PasswordRecipientInfoGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+PasswordRecipientInfoGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::PasswordRecipientInfoGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+PasswordRecipientInfoGenerator")]
impl crate::Org::BouncyCastle::Cms::PasswordRecipientInfoGenerator {
    pub fn Generate(
        &mut self,
        contentEncryptionKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::RecipientInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::RecipientInfo = __cordl_object
            .invoke("Generate", (contentEncryptionKey, random))?;
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
    pub fn set_KeyDerivationAlgorithm(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_KeyDerivationAlgorithm", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_KeyEncryptionKey(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_KeyEncryptionKey", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_KeyEncryptionKeyOID(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_KeyEncryptionKeyOID", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+PasswordRecipientInfoGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::PasswordRecipientInfoGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
