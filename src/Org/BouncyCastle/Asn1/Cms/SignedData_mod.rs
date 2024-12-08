#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+SignedData")]
#[repr(C)]
#[derive(Debug)]
pub struct SignedData {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub digestAlgorithms: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    pub contentInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    pub certificates: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    pub crls: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    pub signerInfos: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    pub certsBer: bool,
    pub crlsBer: bool,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+SignedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cms::SignedData =>
    "Org.BouncyCastle.Asn1.Cms"."SignedData"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+SignedData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cms::SignedData {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+SignedData")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cms::SignedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+SignedData")]
impl crate::Org::BouncyCastle::Asn1::Cms::SignedData {
    pub fn get_Certificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Set> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Set = __cordl_object
            .invoke("get_Certificates", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SignerInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Set> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Set = __cordl_object
            .invoke("get_SignerInfos", ())?;
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
    pub fn get_CRLs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Set> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Set = __cordl_object
            .invoke("get_CRLs", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckForVersion3(
        &mut self,
        signerInfs: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckForVersion3", (signerInfs))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateVersion(
        &mut self,
        contentOid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        certs: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        crls: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        signerInfs: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("CalculateVersion", (contentOid, certs, crls, signerInfs))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Set_ContentInfo_Asn1Set_Asn1Set_Asn1Set0(
        &mut self,
        digestAlgorithms: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        contentInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        certificates: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        crls: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        signerInfos: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (digestAlgorithms, contentInfo, certificates, crls, signerInfos),
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
    pub fn get_DigestAlgorithms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Set> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Set = __cordl_object
            .invoke("get_DigestAlgorithms", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EncapContentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo = __cordl_object
            .invoke("get_EncapContentInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Asn1Set_ContentInfo_Asn1Set_Asn1Set_Asn1Set0(
        digestAlgorithms: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        contentInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        certificates: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        crls: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        signerInfos: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (digestAlgorithms, contentInfo, certificates, crls, signerInfos),
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+SignedData")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::SignedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
