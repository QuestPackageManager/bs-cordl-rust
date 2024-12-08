#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsSignedDataGenerator {
    __cordl_parent: crate::Org::BouncyCastle::Cms::CmsSignedGenerator,
    pub signerInfs: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::CmsSignedDataGenerator
    => "Org.BouncyCastle.Cms"."CmsSignedDataGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsSignedDataGenerator {
    type Target = crate::Org::BouncyCastle::Cms::CmsSignedGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsSignedDataGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataGenerator")]
impl crate::Org::BouncyCastle::Cms::CmsSignedDataGenerator {
    #[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataGenerator+SignerInf")]
    pub type SignerInf = crate::Org::BouncyCastle::Cms::CmsSignedDataGenerator_SignerInf;
    pub fn AddSignerInfoGenerator(
        &mut self,
        signerInfoGenerator: *mut crate::Org::BouncyCastle::Cms::SignerInfoGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSignerInfoGenerator", (signerInfoGenerator))?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_Il2CppArray2(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        subjectKeyID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        digestOID: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSigner", (privateKey, subjectKeyID, digestOID))?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_Il2CppArray_AttributeTable_AttributeTable6(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        subjectKeyID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        digestOID: *mut crate::System::String,
        signedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        unsignedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSigner",
                (privateKey, subjectKeyID, digestOID, signedAttr, unsignedAttr),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_Il2CppArray_CmsAttributeTableGenerator_CmsAttributeTableGenerator10(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        subjectKeyID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        digestOID: *mut crate::System::String,
        signedAttrGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsignedAttrGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSigner",
                (privateKey, subjectKeyID, digestOID, signedAttrGen, unsignedAttrGen),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_Il2CppArray_String3(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        subjectKeyID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        encryptionOID: *mut crate::System::String,
        digestOID: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSigner", (privateKey, subjectKeyID, encryptionOID, digestOID))?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_Il2CppArray_String_AttributeTable_AttributeTable7(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        subjectKeyID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        encryptionOID: *mut crate::System::String,
        digestOID: *mut crate::System::String,
        signedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        unsignedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSigner",
                (
                    privateKey,
                    subjectKeyID,
                    encryptionOID,
                    digestOID,
                    signedAttr,
                    unsignedAttr,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_Il2CppArray_String_CmsAttributeTableGenerator_CmsAttributeTableGenerator11(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        subjectKeyID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        encryptionOID: *mut crate::System::String,
        digestOID: *mut crate::System::String,
        signedAttrGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsignedAttrGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSigner",
                (
                    privateKey,
                    subjectKeyID,
                    encryptionOID,
                    digestOID,
                    signedAttrGen,
                    unsignedAttrGen,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_X509Certificate0(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        digestOID: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSigner", (privateKey, cert, digestOID))?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_X509Certificate_AttributeTable_AttributeTable4(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        digestOID: *mut crate::System::String,
        signedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        unsignedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSigner",
                (privateKey, cert, digestOID, signedAttr, unsignedAttr),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_X509Certificate_CmsAttributeTableGenerator_CmsAttributeTableGenerator8(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        digestOID: *mut crate::System::String,
        signedAttrGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsignedAttrGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSigner",
                (privateKey, cert, digestOID, signedAttrGen, unsignedAttrGen),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_X509Certificate_String1(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        encryptionOID: *mut crate::System::String,
        digestOID: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSigner", (privateKey, cert, encryptionOID, digestOID))?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_X509Certificate_String_AttributeTable_AttributeTable5(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        encryptionOID: *mut crate::System::String,
        digestOID: *mut crate::System::String,
        signedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        unsignedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSigner",
                (privateKey, cert, encryptionOID, digestOID, signedAttr, unsignedAttr),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_X509Certificate_String_CmsAttributeTableGenerator_CmsAttributeTableGenerator9(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        encryptionOID: *mut crate::System::String,
        digestOID: *mut crate::System::String,
        signedAttrGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsignedAttrGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSigner",
                (
                    privateKey,
                    cert,
                    encryptionOID,
                    digestOID,
                    signedAttrGen,
                    unsignedAttrGen,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GenerateCounterSigners(
        &mut self,
        signer: *mut crate::Org::BouncyCastle::Cms::SignerInformation,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::SignerInformationStore,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::SignerInformationStore = __cordl_object
            .invoke("GenerateCounterSigners", (signer))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_CmsProcessable0(
        &mut self,
        content: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::CmsSignedData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsSignedData = __cordl_object
            .invoke("Generate", (content))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_CmsProcessable__cordl_bool2(
        &mut self,
        content: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
        encapsulate: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::CmsSignedData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsSignedData = __cordl_object
            .invoke("Generate", (content, encapsulate))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_String_CmsProcessable__cordl_bool1(
        &mut self,
        signedContentType: *mut crate::System::String,
        content: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
        encapsulate: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::CmsSignedData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsSignedData = __cordl_object
            .invoke("Generate", (signedContentType, content, encapsulate))?;
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
    pub fn doAddSigner(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        signerIdentifier: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        encryptionOID: *mut crate::System::String,
        digestOID: *mut crate::System::String,
        signedAttrGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsignedAttrGen: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        baseSignedTable: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "doAddSigner",
                (
                    privateKey,
                    signerIdentifier,
                    encryptionOID,
                    digestOID,
                    signedAttrGen,
                    unsignedAttrGen,
                    baseSignedTable,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsSignedDataGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataGenerator+SignerInf")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsSignedDataGenerator_SignerInf {
    __cordl_parent: crate::System::Object,
    pub outer: *mut crate::Org::BouncyCastle::Cms::CmsSignedGenerator,
    pub sigCalc: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
    pub signerIdentifier: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
    pub digestOID: *mut crate::System::String,
    pub encOID: *mut crate::System::String,
    pub sAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    pub unsAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    pub baseSignedTable: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataGenerator+SignerInf")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsSignedDataGenerator_SignerInf => "Org.BouncyCastle.Cms"
    ."CmsSignedDataGenerator/SignerInf"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataGenerator+SignerInf")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsSignedDataGenerator_SignerInf {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataGenerator+SignerInf")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsSignedDataGenerator_SignerInf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataGenerator+SignerInf")]
impl crate::Org::BouncyCastle::Cms::CmsSignedDataGenerator_SignerInf {
    pub fn New_AsymmetricKeyParameter_String_String_CmsAttributeTableGenerator_CmsAttributeTableGenerator_AttributeTable0(
        outer: *mut crate::Org::BouncyCastle::Cms::CmsSignedGenerator,
        key: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        signerIdentifier: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        digestOID: *mut crate::System::String,
        encOID: *mut crate::System::String,
        sAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        baseSignedTable: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    outer,
                    key,
                    signerIdentifier,
                    digestOID,
                    encOID,
                    sAttr,
                    unsAttr,
                    baseSignedTable,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_ISignatureFactory_CmsAttributeTableGenerator_CmsAttributeTableGenerator_AttributeTable1(
        outer: *mut crate::Org::BouncyCastle::Cms::CmsSignedGenerator,
        sigCalc: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        signerIdentifier: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        sAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        baseSignedTable: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (outer, sigCalc, signerIdentifier, sAttr, unsAttr, baseSignedTable),
            )?;
        Ok(__cordl_object)
    }
    pub fn ToSignerInfo(
        &mut self,
        contentType: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        content: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::SignerInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerInfo = __cordl_object
            .invoke("ToSignerInfo", (contentType, content, random))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AsymmetricKeyParameter_String_String_CmsAttributeTableGenerator_CmsAttributeTableGenerator_AttributeTable0(
        &mut self,
        outer: *mut crate::Org::BouncyCastle::Cms::CmsSignedGenerator,
        key: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        signerIdentifier: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        digestOID: *mut crate::System::String,
        encOID: *mut crate::System::String,
        sAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        baseSignedTable: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    outer,
                    key,
                    signerIdentifier,
                    digestOID,
                    encOID,
                    sAttr,
                    unsAttr,
                    baseSignedTable,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ISignatureFactory_CmsAttributeTableGenerator_CmsAttributeTableGenerator_AttributeTable1(
        &mut self,
        outer: *mut crate::Org::BouncyCastle::Cms::CmsSignedGenerator,
        sigCalc: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        signerIdentifier: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        sAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        baseSignedTable: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (outer, sigCalc, signerIdentifier, sAttr, unsAttr, baseSignedTable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_DigestAlgorithmID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_DigestAlgorithmID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SignedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator = __cordl_object
            .invoke("get_SignedAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UnsignedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator = __cordl_object
            .invoke("get_UnsignedAttributes", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataGenerator+SignerInf")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsSignedDataGenerator_SignerInf {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
