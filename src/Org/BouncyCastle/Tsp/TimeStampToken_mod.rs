#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampToken+CertID")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeStampToken_CertID {
    __cordl_parent: crate::System::Object,
    pub certID: *mut crate::Org::BouncyCastle::Asn1::Ess::EssCertID,
    pub certIDv2: *mut crate::Org::BouncyCastle::Asn1::Ess::EssCertIDv2,
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampToken+CertID")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Tsp::TimeStampToken_CertID =>
    "Org.BouncyCastle.Tsp"."TimeStampToken/CertID"
);
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampToken+CertID")]
impl std::ops::Deref for crate::Org::BouncyCastle::Tsp::TimeStampToken_CertID {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampToken+CertID")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Tsp::TimeStampToken_CertID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampToken+CertID")]
impl crate::Org::BouncyCastle::Tsp::TimeStampToken_CertID {
    pub fn _ctor_EssCertID0(
        &mut self,
        certID: *mut crate::Org::BouncyCastle::Asn1::Ess::EssCertID,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certID))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_EssCertIDv2_1(
        &mut self,
        certID: *mut crate::Org::BouncyCastle::Asn1::Ess::EssCertIDv2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certID))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("GetHashAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCertHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetCertHash", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IssuerSerial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::IssuerSerial,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::IssuerSerial = __cordl_object
            .invoke("get_IssuerSerial", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashAlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetHashAlgorithmName", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_EssCertID0(
        certID: *mut crate::Org::BouncyCastle::Asn1::Ess::EssCertID,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certID))?;
        Ok(__cordl_object)
    }
    pub fn New_EssCertIDv2_1(
        certID: *mut crate::Org::BouncyCastle::Asn1::Ess::EssCertIDv2,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certID))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampToken+CertID")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Tsp::TimeStampToken_CertID {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampToken")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeStampToken {
    __cordl_parent: crate::System::Object,
    pub tsToken: *mut crate::Org::BouncyCastle::Cms::CmsSignedData,
    pub tsaSignerInfo: *mut crate::Org::BouncyCastle::Cms::SignerInformation,
    pub tstInfo: *mut crate::Org::BouncyCastle::Tsp::TimeStampTokenInfo,
    pub certID: *mut crate::Org::BouncyCastle::Tsp::TimeStampToken_CertID,
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampToken")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Tsp::TimeStampToken =>
    "Org.BouncyCastle.Tsp"."TimeStampToken"
);
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampToken")]
impl std::ops::Deref for crate::Org::BouncyCastle::Tsp::TimeStampToken {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampToken")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Tsp::TimeStampToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampToken")]
impl crate::Org::BouncyCastle::Tsp::TimeStampToken {
    #[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampToken+CertID")]
    pub type CertID = crate::Org::BouncyCastle::Tsp::TimeStampToken_CertID;
    pub fn Validate(
        &mut self,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Validate", (cert))?;
        Ok(__cordl_ret)
    }
    pub fn GetCertificates(
        &mut self,
        _cordl_type: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::Store::IX509Store = __cordl_object
            .invoke("GetCertificates", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn ToCmsSignedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::CmsSignedData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsSignedData = __cordl_object
            .invoke("ToCmsSignedData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEncoded_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetEncoded", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEncoded_String1(
        &mut self,
        encoding: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetEncoded", (encoding))?;
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
    pub fn GetAttributeCertificates(
        &mut self,
        _cordl_type: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::Store::IX509Store = __cordl_object
            .invoke("GetAttributeCertificates", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ContentInfo0(
        &mut self,
        contentInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contentInfo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CmsSignedData1(
        &mut self,
        signedData: *mut crate::Org::BouncyCastle::Cms::CmsSignedData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signedData))?;
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
    pub fn GetCrls(
        &mut self,
        _cordl_type: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::Store::IX509Store = __cordl_object
            .invoke("GetCrls", (_cordl_type))?;
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
    pub fn get_TimeStampInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Tsp::TimeStampTokenInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Tsp::TimeStampTokenInfo = __cordl_object
            .invoke("get_TimeStampInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_ContentInfo0(
        contentInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contentInfo))?;
        Ok(__cordl_object)
    }
    pub fn New_CmsSignedData1(
        signedData: *mut crate::Org::BouncyCastle::Cms::CmsSignedData,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signedData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampToken")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Tsp::TimeStampToken {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
