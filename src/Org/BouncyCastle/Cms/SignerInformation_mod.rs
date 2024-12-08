#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct SignerInformation {
    __cordl_parent: crate::System::Object,
    pub sid: *mut crate::Org::BouncyCastle::Cms::SignerID,
    pub info: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerInfo,
    pub digestAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub encryptionAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub signedAttributeSet: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    pub unsignedAttributeSet: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    pub content: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
    pub signature: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub contentType: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub digestCalculator: *mut crate::Org::BouncyCastle::Cms::IDigestCalculator,
    pub resultDigest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub signedAttributeTable: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    pub unsignedAttributeTable: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    pub isCounterSignature: bool,
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::SignerInformation =>
    "Org.BouncyCastle.Cms"."SignerInformation"
);
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformation")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::SignerInformation {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformation")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::SignerInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformation")]
impl crate::Org::BouncyCastle::Cms::SignerInformation {
    pub fn VerifyDigest(
        &mut self,
        digest: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        key: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("VerifyDigest", (digest, key, signature))?;
        Ok(__cordl_ret)
    }
    pub fn get_ContentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier = __cordl_object
            .invoke("get_ContentType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetContentDigest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetContentDigest", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCounterSignature(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCounterSignature", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SignedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable = __cordl_object
            .invoke("get_SignedAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SignerInfo_DerObjectIdentifier_CmsProcessable_IDigestCalculator0(
        &mut self,
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerInfo,
        contentType: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        content: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
        digestCalculator: *mut crate::Org::BouncyCastle::Cms::IDigestCalculator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, contentType, content, digestCalculator))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SignerInformation1(
        &mut self,
        baseInfo: *mut crate::Org::BouncyCastle::Cms::SignerInformation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (baseInfo))?;
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
    pub fn DoVerify(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DoVerify", (key))?;
        Ok(__cordl_ret)
    }
    pub fn get_UnsignedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable = __cordl_object
            .invoke("get_UnsignedAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToSignerInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::SignerInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerInfo = __cordl_object
            .invoke("ToSignerInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn DerDecode(
        &mut self,
        encoding: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::DigestInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::DigestInfo = __cordl_object
            .invoke("DerDecode", (encoding))?;
        Ok(__cordl_ret)
    }
    pub fn get_DigestAlgOid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DigestAlgOid", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SignerID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Cms::SignerID> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::SignerID = __cordl_object
            .invoke("get_SignerID", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSigningTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Cms::Time> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::Time = __cordl_object
            .invoke("GetSigningTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EncryptionAlgParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("get_EncryptionAlgParams", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSingleValuedSignedAttribute(
        &mut self,
        attrOID: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        printableName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("GetSingleValuedSignedAttribute", (attrOID, printableName))?;
        Ok(__cordl_ret)
    }
    pub fn get_DigestAlgParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("get_DigestAlgParams", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCounterSignatures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::SignerInformationStore,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::SignerInformationStore = __cordl_object
            .invoke("GetCounterSignatures", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSignature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetSignature", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EncryptionAlgOid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_EncryptionAlgOid", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEncodedSignedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetEncodedSignedAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsNull(
        &mut self,
        o: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsNull", (o))?;
        Ok(__cordl_ret)
    }
    pub fn Verify_AsymmetricKeyParameter0(
        &mut self,
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", (pubKey))?;
        Ok(__cordl_ret)
    }
    pub fn Verify_X509Certificate1(
        &mut self,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", (cert))?;
        Ok(__cordl_ret)
    }
    pub fn get_EncryptionAlgorithmID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_EncryptionAlgorithmID", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_SignerInfo_DerObjectIdentifier_CmsProcessable_IDigestCalculator0(
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::SignerInfo,
        contentType: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        content: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
        digestCalculator: *mut crate::Org::BouncyCastle::Cms::IDigestCalculator,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, contentType, content, digestCalculator))?;
        Ok(__cordl_object)
    }
    pub fn New_SignerInformation1(
        baseInfo: *mut crate::Org::BouncyCastle::Cms::SignerInformation,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (baseInfo))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::SignerInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
