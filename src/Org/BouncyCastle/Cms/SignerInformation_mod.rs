#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct SignerInformation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub sid: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::SignerID>,
    pub info: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::SignerInfo>,
    pub digestAlgorithm: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    >,
    pub encryptionAlgorithm: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    >,
    pub signedAttributeSet: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1Set,
    >,
    pub unsignedAttributeSet: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1Set,
    >,
    pub content: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::CmsProcessable,
    >,
    pub signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub contentType: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    >,
    pub digestCalculator: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::IDigestCalculator,
    >,
    pub resultDigest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub signedAttributeTable: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    >,
    pub unsignedAttributeTable: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    >,
    pub isCounterSignature: bool,
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::SignerInformation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "SignerInformation";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformation")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::SignerInformation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn AddCounterSigners(
        signerInformation: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInformation,
        >,
        counterSigners: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInformationStore,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::SignerInformation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInformation,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddCounterSigners", (signerInformation, counterSigners))?;
        Ok(__cordl_ret.into())
    }
    pub fn DerDecode(
        &mut self,
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::DigestInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DigestInfo,
        > = __cordl_object.invoke("DerDecode", (encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoVerify(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DoVerify", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContentDigest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetContentDigest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCounterSignatures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::SignerInformationStore>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInformationStore,
        > = __cordl_object.invoke("GetCounterSignatures", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEncodedSignedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetEncodedSignedAttributes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSignature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetSignature", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSigningTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::Time>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::Time,
        > = __cordl_object.invoke("GetSigningTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSingleValuedSignedAttribute(
        &mut self,
        attrOID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        printableName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = __cordl_object
            .invoke("GetSingleValuedSignedAttribute", (attrOID, printableName))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNull(
        &mut self,
        o: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsNull", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_SignerInfo_DerObjectIdentifier_CmsProcessable_IDigestCalculator0(
        info: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::SignerInfo>,
        contentType: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        content: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsProcessable,
        >,
        digestCalculator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::IDigestCalculator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, contentType, content, digestCalculator))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SignerInformation1(
        baseInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInformation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (baseInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn ReplaceUnsignedAttributes(
        signerInformation: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInformation,
        >,
        unsignedAttributes: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::SignerInformation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInformation,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ReplaceUnsignedAttributes",
                (signerInformation, unsignedAttributes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSignerInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::SignerInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::SignerInfo,
        > = __cordl_object.invoke("ToSignerInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyDigest(
        &mut self,
        digest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("VerifyDigest", (digest, key, signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn Verify_AsymmetricKeyParameter0(
        &mut self,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", (pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn Verify_X509Certificate1(
        &mut self,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", (cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SignerInfo_DerObjectIdentifier_CmsProcessable_IDigestCalculator0(
        &mut self,
        info: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::SignerInfo>,
        contentType: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        content: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsProcessable,
        >,
        digestCalculator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::IDigestCalculator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, contentType, content, digestCalculator))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SignerInformation1(
        &mut self,
        baseInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInformation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (baseInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerObjectIdentifier>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = __cordl_object.invoke("get_ContentType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DigestAlgOid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DigestAlgOid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DigestAlgParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = __cordl_object.invoke("get_DigestAlgParams", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DigestAlgorithmID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        > = __cordl_object.invoke("get_DigestAlgorithmID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EncryptionAlgOid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_EncryptionAlgOid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EncryptionAlgParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = __cordl_object.invoke("get_EncryptionAlgParams", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EncryptionAlgorithmID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        > = __cordl_object.invoke("get_EncryptionAlgorithmID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCounterSignature(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCounterSignature", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SignedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::AttributeTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        > = __cordl_object.invoke("get_SignedAttributes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SignerID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::SignerID>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerID,
        > = __cordl_object.invoke("get_SignerID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnsignedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::AttributeTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        > = __cordl_object.invoke("get_UnsignedAttributes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret.into())
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
