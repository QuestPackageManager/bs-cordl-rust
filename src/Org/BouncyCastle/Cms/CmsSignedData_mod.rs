#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedData")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsSignedData {
    __cordl_parent: crate::System::Object,
    pub signedContent: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
    pub signedData: *mut crate::Org::BouncyCastle::Asn1::Cms::SignedData,
    pub contentInfo: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    pub signerInfoStore: *mut crate::Org::BouncyCastle::Cms::SignerInformationStore,
    pub attrCertStore: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    pub certificateStore: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    pub crlStore: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    pub hashes: *mut crate::System::Collections::IDictionary,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::CmsSignedData =>
    "Org.BouncyCastle.Cms"."CmsSignedData"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsSignedData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedData")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsSignedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedData")]
impl crate::Org::BouncyCastle::Cms::CmsSignedData {
    pub fn _ctor_CmsSignedData0(
        &mut self,
        c: *mut crate::Org::BouncyCastle::Cms::CmsSignedData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (c))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        sigBlock: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigBlock))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CmsProcessable_Il2CppArray2(
        &mut self,
        signedContent: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
        sigBlock: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signedContent, sigBlock))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDictionary_Il2CppArray3(
        &mut self,
        hashes: *mut crate::System::Collections::IDictionary,
        sigBlock: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hashes, sigBlock))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CmsProcessable_Stream4(
        &mut self,
        signedContent: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
        sigData: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signedContent, sigData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream5(
        &mut self,
        sigData: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CmsProcessable_ContentInfo6(
        &mut self,
        signedContent: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
        sigData: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signedContent, sigData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDictionary_ContentInfo7(
        &mut self,
        hashes: *mut crate::System::Collections::IDictionary,
        sigData: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hashes, sigData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ContentInfo8(
        &mut self,
        sigData: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigData))?;
        Ok(__cordl_ret)
    }
    pub fn get_SignedContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsProcessable = __cordl_object
            .invoke("get_SignedContent", ())?;
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
    pub fn GetSignerInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::SignerInformationStore,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::SignerInformationStore = __cordl_object
            .invoke("GetSignerInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SignedContentTypeOid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_SignedContentTypeOid", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ContentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo = __cordl_object
            .invoke("get_ContentInfo", ())?;
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
    pub fn get_SignedContentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier = __cordl_object
            .invoke("get_SignedContentType", ())?;
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
    pub fn New_CmsSignedData0(
        c: *mut crate::Org::BouncyCastle::Cms::CmsSignedData,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (c))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray1(
        sigBlock: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigBlock))?;
        Ok(__cordl_object)
    }
    pub fn New_CmsProcessable_Il2CppArray2(
        signedContent: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
        sigBlock: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signedContent, sigBlock))?;
        Ok(__cordl_object)
    }
    pub fn New_IDictionary_Il2CppArray3(
        hashes: *mut crate::System::Collections::IDictionary,
        sigBlock: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hashes, sigBlock))?;
        Ok(__cordl_object)
    }
    pub fn New_CmsProcessable_Stream4(
        signedContent: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
        sigData: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signedContent, sigData))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream5(
        sigData: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigData))?;
        Ok(__cordl_object)
    }
    pub fn New_CmsProcessable_ContentInfo6(
        signedContent: *mut crate::Org::BouncyCastle::Cms::CmsProcessable,
        sigData: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signedContent, sigData))?;
        Ok(__cordl_object)
    }
    pub fn New_IDictionary_ContentInfo7(
        hashes: *mut crate::System::Collections::IDictionary,
        sigData: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hashes, sigData))?;
        Ok(__cordl_object)
    }
    pub fn New_ContentInfo8(
        sigData: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedData")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Cms::CmsSignedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
