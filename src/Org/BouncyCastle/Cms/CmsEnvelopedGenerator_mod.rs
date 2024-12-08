#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedGenerator {
    __cordl_parent: crate::System::Object,
    pub recipientInfoGenerators: *mut crate::System::Collections::IList,
    pub _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub unprotectedAttributeGenerator: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::CmsEnvelopedGenerator =>
    "Org.BouncyCastle.Cms"."CmsEnvelopedGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedGenerator")]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator {
    pub const Cast5Cbc: &'static str = "1.2.840.113533.7.66.10";
    pub const IdeaCbc: &'static str = "1.3.6.1.4.1.188.7.1.1.2";
    pub fn AddKekRecipient_Il2CppArray0(
        &mut self,
        keyAlgorithm: *mut crate::System::String,
        key: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        keyIdentifier: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddKekRecipient", (keyAlgorithm, key, keyIdentifier))?;
        Ok(__cordl_ret)
    }
    pub fn AddKekRecipient_KekIdentifier1(
        &mut self,
        keyAlgorithm: *mut crate::System::String,
        key: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        kekIdentifier: *mut crate::Org::BouncyCastle::Asn1::Cms::KekIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddKekRecipient", (keyAlgorithm, key, kekIdentifier))?;
        Ok(__cordl_ret)
    }
    pub fn AddKeyAgreementRecipient(
        &mut self,
        agreementAlgorithm: *mut crate::System::String,
        senderPrivateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        senderPublicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        recipientCert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        cekWrapAlgorithm: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddKeyAgreementRecipient",
                (
                    agreementAlgorithm,
                    senderPrivateKey,
                    senderPublicKey,
                    recipientCert,
                    cekWrapAlgorithm,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddKeyAgreementRecipients(
        &mut self,
        agreementAlgorithm: *mut crate::System::String,
        senderPrivateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        senderPublicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        recipientCerts: *mut crate::System::Collections::ICollection,
        cekWrapAlgorithm: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddKeyAgreementRecipients",
                (
                    agreementAlgorithm,
                    senderPrivateKey,
                    senderPublicKey,
                    recipientCerts,
                    cekWrapAlgorithm,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddKeyTransRecipient_AsymmetricKeyParameter_Il2CppArray1(
        &mut self,
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        subKeyId: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddKeyTransRecipient", (pubKey, subKeyId))?;
        Ok(__cordl_ret)
    }
    pub fn AddKeyTransRecipient_X509Certificate0(
        &mut self,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddKeyTransRecipient", (cert))?;
        Ok(__cordl_ret)
    }
    pub fn AddPasswordRecipient(
        &mut self,
        pbeKey: *mut crate::Org::BouncyCastle::Cms::CmsPbeKey,
        kekAlgorithmOid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPasswordRecipient", (pbeKey, kekAlgorithmOid))?;
        Ok(__cordl_ret)
    }
    pub fn AddRecipientInfoGenerator(
        &mut self,
        recipientInfoGenerator: *mut crate::Org::BouncyCastle::Cms::RecipientInfoGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddRecipientInfoGenerator", (recipientInfoGenerator))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateAsn1Parameters(
        &mut self,
        encryptionOid: *mut crate::System::String,
        encKeyBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable = __cordl_object
            .invoke("GenerateAsn1Parameters", (encryptionOid, encKeyBytes))?;
        Ok(__cordl_ret)
    }
    pub fn GetAlgorithmIdentifier(
        &mut self,
        encryptionOid: *mut crate::System::String,
        encKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        asn1Params: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
        cipherParameters: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke(
                "GetAlgorithmIdentifier",
                (encryptionOid, encKey, asn1Params, cipherParameters),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SecureRandom1(
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_rand))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SecureRandom1(
        &mut self,
        _cordl_rand: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_rand))?;
        Ok(__cordl_ret)
    }
    pub fn get_UnprotectedAttributeGenerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator = __cordl_object
            .invoke("get_UnprotectedAttributeGenerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_UnprotectedAttributeGenerator(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UnprotectedAttributeGenerator", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
