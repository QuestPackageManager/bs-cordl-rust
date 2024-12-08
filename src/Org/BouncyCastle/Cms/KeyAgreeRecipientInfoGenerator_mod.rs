#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInfoGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyAgreeRecipientInfoGenerator {
    __cordl_parent: crate::System::Object,
    pub keyAgreementOID: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub keyEncryptionOID: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub recipientCerts: *mut crate::System::Collections::IList,
    pub senderKeyPair: *mut crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInfoGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::KeyAgreeRecipientInfoGenerator => "Org.BouncyCastle.Cms"
    ."KeyAgreeRecipientInfoGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInfoGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::KeyAgreeRecipientInfoGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInfoGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::KeyAgreeRecipientInfoGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInfoGenerator")]
impl crate::Org::BouncyCastle::Cms::KeyAgreeRecipientInfoGenerator {
    pub fn set_SenderKeyPair(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SenderKeyPair", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_KeyEncryptionOID(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_KeyEncryptionOID", (value))?;
        Ok(__cordl_ret)
    }
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
    pub fn set_KeyAgreementOID(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_KeyAgreementOID", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_RecipientCerts(
        &mut self,
        value: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RecipientCerts", (value))?;
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
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInfoGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::KeyAgreeRecipientInfoGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
