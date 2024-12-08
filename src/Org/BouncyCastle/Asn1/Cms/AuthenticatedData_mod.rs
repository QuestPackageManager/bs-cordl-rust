#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+AuthenticatedData")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthenticatedData {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub originatorInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
    pub recipientInfos: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    pub macAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub digestAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub encapsulatedContentInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    pub authAttrs: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    pub mac: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub unauthAttrs: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+AuthenticatedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cms::AuthenticatedData
    => "Org.BouncyCastle.Asn1.Cms"."AuthenticatedData"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+AuthenticatedData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cms::AuthenticatedData {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+AuthenticatedData")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cms::AuthenticatedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+AuthenticatedData")]
impl crate::Org::BouncyCastle::Asn1::Cms::AuthenticatedData {
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RecipientInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Set> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Set = __cordl_object
            .invoke("get_RecipientInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MacAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_MacAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Mac(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_Mac", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OriginatorInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo = __cordl_object
            .invoke("get_OriginatorInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UnauthAttrs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Set> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Set = __cordl_object
            .invoke("get_UnauthAttrs", ())?;
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
    pub fn get_EncapsulatedContentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo = __cordl_object
            .invoke("get_EncapsulatedContentInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AuthAttrs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Set> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Set = __cordl_object
            .invoke("get_AuthAttrs", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_OriginatorInfo_Asn1Set_AlgorithmIdentifier_AlgorithmIdentifier_ContentInfo_Asn1Set_Asn1OctetString_Asn1Set0(
        &mut self,
        originatorInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
        recipientInfos: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        macAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        digestAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        encapsulatedContent: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        authAttrs: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        mac: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        unauthAttrs: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    originatorInfo,
                    recipientInfos,
                    macAlgorithm,
                    digestAlgorithm,
                    encapsulatedContent,
                    authAttrs,
                    mac,
                    unauthAttrs,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Sequence1(
        &mut self,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret)
    }
    pub fn New_OriginatorInfo_Asn1Set_AlgorithmIdentifier_AlgorithmIdentifier_ContentInfo_Asn1Set_Asn1OctetString_Asn1Set0(
        originatorInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
        recipientInfos: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        macAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        digestAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        encapsulatedContent: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        authAttrs: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        mac: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        unauthAttrs: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    originatorInfo,
                    recipientInfos,
                    macAlgorithm,
                    digestAlgorithm,
                    encapsulatedContent,
                    authAttrs,
                    mac,
                    unauthAttrs,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1Sequence1(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+AuthenticatedData")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::AuthenticatedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
