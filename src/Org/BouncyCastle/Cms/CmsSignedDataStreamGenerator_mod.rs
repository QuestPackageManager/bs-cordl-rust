#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+CmsSignedDataOutputStream"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CmsSignedDataStreamGenerator_CmsSignedDataOutputStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream,
    pub outer: *mut crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator,
    pub _out: *mut crate::System::IO::Stream,
    pub _contentOID: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub _sGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    pub _sigGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    pub _eiGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+CmsSignedDataOutputStream"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_CmsSignedDataOutputStream =>
    "Org.BouncyCastle.Cms"."CmsSignedDataStreamGenerator/CmsSignedDataOutputStream"
);
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+CmsSignedDataOutputStream"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_CmsSignedDataOutputStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+CmsSignedDataOutputStream"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_CmsSignedDataOutputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+CmsSignedDataOutputStream"
)]
impl crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_CmsSignedDataOutputStream {
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn DoClose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoClose", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        outer: *mut crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator,
        outStream: *mut crate::System::IO::Stream,
        contentOID: *mut crate::System::String,
        sGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        sigGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        eiGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outer, outStream, contentOID, sGen, sigGen, eiGen))?;
        Ok(__cordl_object)
    }
    pub fn Write(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (bytes, off, len))?;
        Ok(__cordl_ret)
    }
    pub fn WriteByte(
        &mut self,
        b: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteByte", (b))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        outer: *mut crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator,
        outStream: *mut crate::System::IO::Stream,
        contentOID: *mut crate::System::String,
        sGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        sigGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
        eiGen: *mut crate::Org::BouncyCastle::Asn1::BerSequenceGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (outer, outStream, contentOID, sGen, sigGen, eiGen))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+CmsSignedDataOutputStream"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_CmsSignedDataOutputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsSignedDataStreamGenerator {
    __cordl_parent: crate::Org::BouncyCastle::Cms::CmsSignedGenerator,
    pub _signerInfs: *mut crate::System::Collections::IList,
    pub _messageDigestOids: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub _messageDigests: *mut crate::System::Collections::IDictionary,
    pub _messageHashes: *mut crate::System::Collections::IDictionary,
    pub _messageDigestsLocked: bool,
    pub _bufferSize: i32,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator => "Org.BouncyCastle.Cms"
    ."CmsSignedDataStreamGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator {
    type Target = crate::Org::BouncyCastle::Cms::CmsSignedGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator")]
impl crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator {
    #[cfg(
        feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+CmsSignedDataOutputStream"
    )]
    pub type CmsSignedDataOutputStream = crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_CmsSignedDataOutputStream;
    #[cfg(
        feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+DigestAndSignerInfoGeneratorHolder"
    )]
    pub type DigestAndSignerInfoGeneratorHolder = crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_DigestAndSignerInfoGeneratorHolder;
    #[cfg(
        feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+SignerInfoGeneratorImpl"
    )]
    pub type SignerInfoGeneratorImpl = crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_SignerInfoGeneratorImpl;
    pub fn AddDigests_IEnumerable1(
        &mut self,
        digestOids: *mut crate::System::Collections::IEnumerable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDigests", (digestOids))?;
        Ok(__cordl_ret)
    }
    pub fn AddDigests_Il2CppArray0(
        &mut self,
        digestOids: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDigests", (digestOids))?;
        Ok(__cordl_ret)
    }
    pub fn AddSignerCallback(
        &mut self,
        si: *mut crate::Org::BouncyCastle::Cms::SignerInformation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSignerCallback", (si))?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_Il2CppArray6(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        subjectKeyID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        digestOid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSigner", (privateKey, subjectKeyID, digestOid))?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_Il2CppArray_AttributeTable_AttributeTable8(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        subjectKeyID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        digestOid: *mut crate::System::String,
        signedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        unsignedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSigner",
                (privateKey, subjectKeyID, digestOid, signedAttr, unsignedAttr),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_Il2CppArray_CmsAttributeTableGenerator_CmsAttributeTableGenerator9(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        subjectKeyID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        digestOid: *mut crate::System::String,
        signedAttrGenerator: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsignedAttrGenerator: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
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
                    digestOid,
                    signedAttrGenerator,
                    unsignedAttrGenerator,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_Il2CppArray_String7(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        subjectKeyID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        encryptionOid: *mut crate::System::String,
        digestOid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSigner", (privateKey, subjectKeyID, encryptionOid, digestOid))?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_Il2CppArray_String_CmsAttributeTableGenerator_CmsAttributeTableGenerator10(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        subjectKeyID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        encryptionOid: *mut crate::System::String,
        digestOid: *mut crate::System::String,
        signedAttrGenerator: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsignedAttrGenerator: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
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
                    encryptionOid,
                    digestOid,
                    signedAttrGenerator,
                    unsignedAttrGenerator,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_X509Certificate0(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        digestOid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSigner", (privateKey, cert, digestOid))?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_X509Certificate_AttributeTable_AttributeTable2(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        digestOid: *mut crate::System::String,
        signedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        unsignedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSigner",
                (privateKey, cert, digestOid, signedAttr, unsignedAttr),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_X509Certificate_CmsAttributeTableGenerator_CmsAttributeTableGenerator4(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        digestOid: *mut crate::System::String,
        signedAttrGenerator: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsignedAttrGenerator: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSigner",
                (privateKey, cert, digestOid, signedAttrGenerator, unsignedAttrGenerator),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_X509Certificate_String1(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        encryptionOid: *mut crate::System::String,
        digestOid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSigner", (privateKey, cert, encryptionOid, digestOid))?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_X509Certificate_String_AttributeTable_AttributeTable3(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        encryptionOid: *mut crate::System::String,
        digestOid: *mut crate::System::String,
        signedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        unsignedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSigner",
                (privateKey, cert, encryptionOid, digestOid, signedAttr, unsignedAttr),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddSigner_X509Certificate_String_CmsAttributeTableGenerator_CmsAttributeTableGenerator5(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        encryptionOid: *mut crate::System::String,
        digestOid: *mut crate::System::String,
        signedAttrGenerator: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsignedAttrGenerator: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
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
                    encryptionOid,
                    digestOid,
                    signedAttrGenerator,
                    unsignedAttrGenerator,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CalculateVersion(
        &mut self,
        contentOid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("CalculateVersion", (contentOid))?;
        Ok(__cordl_ret)
    }
    pub fn CheckForVersion3(
        &mut self,
        signerInfos: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckForVersion3", (signerInfos))?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureDigest(
        &mut self,
        digestOid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureDigest", (digestOid))?;
        Ok(__cordl_ret)
    }
    pub fn DoAddSigner(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        signerIdentifier: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        encryptionOid: *mut crate::System::String,
        digestOid: *mut crate::System::String,
        signedAttrGenerator: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsignedAttrGenerator: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DoAddSigner",
                (
                    privateKey,
                    signerIdentifier,
                    encryptionOid,
                    digestOid,
                    signedAttrGenerator,
                    unsignedAttrGenerator,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Generate(
        &mut self,
        outStream: *mut crate::System::IO::Stream,
        eContentType: *mut crate::System::String,
        encapsulate: bool,
        dataOutputStream: *mut crate::System::IO::Stream,
        content: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Generate",
                (outStream, eContentType, encapsulate, dataOutputStream, content),
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
    pub fn Open_Stream0(
        &mut self,
        outStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStream))?;
        Ok(__cordl_ret)
    }
    pub fn Open_String__cordl_bool3(
        &mut self,
        outStream: *mut crate::System::IO::Stream,
        signedContentType: *mut crate::System::String,
        encapsulate: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStream, signedContentType, encapsulate))?;
        Ok(__cordl_ret)
    }
    pub fn Open_String__cordl_bool_Stream4(
        &mut self,
        outStream: *mut crate::System::IO::Stream,
        signedContentType: *mut crate::System::String,
        encapsulate: bool,
        dataOutputStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke(
                "Open",
                (outStream, signedContentType, encapsulate, dataOutputStream),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Open__cordl_bool1(
        &mut self,
        outStream: *mut crate::System::IO::Stream,
        encapsulate: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStream, encapsulate))?;
        Ok(__cordl_ret)
    }
    pub fn Open__cordl_bool_Stream2(
        &mut self,
        outStream: *mut crate::System::IO::Stream,
        encapsulate: bool,
        dataOutputStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("Open", (outStream, encapsulate, dataOutputStream))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterDigestOid(
        &mut self,
        digestOid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDigestOid", (digestOid))?;
        Ok(__cordl_ret)
    }
    pub fn SetBufferSize(
        &mut self,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBufferSize", (bufferSize))?;
        Ok(__cordl_ret)
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
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+DigestAndSignerInfoGeneratorHolder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CmsSignedDataStreamGenerator_DigestAndSignerInfoGeneratorHolder {
    __cordl_parent: crate::System::Object,
    pub signerInf: *mut crate::Org::BouncyCastle::Cms::ISignerInfoGenerator,
    pub digestOID: *mut crate::System::String,
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+DigestAndSignerInfoGeneratorHolder"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_DigestAndSignerInfoGeneratorHolder
    => "Org.BouncyCastle.Cms"
    ."CmsSignedDataStreamGenerator/DigestAndSignerInfoGeneratorHolder"
);
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+DigestAndSignerInfoGeneratorHolder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_DigestAndSignerInfoGeneratorHolder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+DigestAndSignerInfoGeneratorHolder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_DigestAndSignerInfoGeneratorHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+DigestAndSignerInfoGeneratorHolder"
)]
impl crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_DigestAndSignerInfoGeneratorHolder {
    pub fn New(
        signerInf: *mut crate::Org::BouncyCastle::Cms::ISignerInfoGenerator,
        digestOID: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signerInf, digestOID))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        signerInf: *mut crate::Org::BouncyCastle::Cms::ISignerInfoGenerator,
        digestOID: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signerInf, digestOID))?;
        Ok(__cordl_ret)
    }
    pub fn get_DigestAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_DigestAlgorithm", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+DigestAndSignerInfoGeneratorHolder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_DigestAndSignerInfoGeneratorHolder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+SignerInfoGeneratorImpl"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CmsSignedDataStreamGenerator_SignerInfoGeneratorImpl {
    __cordl_parent: crate::System::Object,
    pub outer: *mut crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator,
    pub _signerIdentifier: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
    pub _digestOID: *mut crate::System::String,
    pub _encOID: *mut crate::System::String,
    pub _sAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    pub _unsAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    pub _encName: *mut crate::System::String,
    pub _sig: *mut crate::Org::BouncyCastle::Crypto::ISigner,
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+SignerInfoGeneratorImpl"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_SignerInfoGeneratorImpl =>
    "Org.BouncyCastle.Cms"."CmsSignedDataStreamGenerator/SignerInfoGeneratorImpl"
);
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+SignerInfoGeneratorImpl"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_SignerInfoGeneratorImpl {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+SignerInfoGeneratorImpl"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_SignerInfoGeneratorImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+SignerInfoGeneratorImpl"
)]
impl crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_SignerInfoGeneratorImpl {
    pub fn Generate(
        &mut self,
        contentType: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        digestAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        calculatedDigest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::SignerInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerInfo = __cordl_object
            .invoke("Generate", (contentType, digestAlgorithm, calculatedDigest))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        outer: *mut crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator,
        key: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        signerIdentifier: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        digestOID: *mut crate::System::String,
        encOID: *mut crate::System::String,
        sAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (outer, key, signerIdentifier, digestOID, encOID, sAttr, unsAttr),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        outer: *mut crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator,
        key: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        signerIdentifier: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerIdentifier,
        digestOID: *mut crate::System::String,
        encOID: *mut crate::System::String,
        sAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        unsAttr: *mut crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (outer, key, signerIdentifier, digestOID, encOID, sAttr, unsAttr),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Cms+CmsSignedDataStreamGenerator+SignerInfoGeneratorImpl"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsSignedDataStreamGenerator_SignerInfoGeneratorImpl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
