#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedData")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsSignedData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub signedContent: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::CmsProcessable,
    >,
    pub signedData: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::SignedData,
    >,
    pub contentInfo: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
    >,
    pub signerInfoStore: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::SignerInformationStore,
    >,
    pub attrCertStore: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::X509::Store::IX509Store,
    >,
    pub certificateStore: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::X509::Store::IX509Store,
    >,
    pub crlStore: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::X509::Store::IX509Store,
    >,
    pub hashes: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::CmsSignedData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "CmsSignedData";
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
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsSignedData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GetAttributeCertificates(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::Store::IX509Store>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        > = __cordl_object.invoke("GetAttributeCertificates", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCertificates(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::Store::IX509Store>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        > = __cordl_object.invoke("GetCertificates", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCrls(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::Store::IX509Store>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        > = __cordl_object.invoke("GetCrls", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEncoded_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetEncoded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEncoded_Il2CppString1(
        &mut self,
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetEncoded", (encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSignerInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::SignerInformationStore>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInformationStore,
        > = __cordl_object.invoke("GetSignerInfos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_CmsProcessable_ContentInfo6(
        signedContent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsProcessable,
        >,
        sigData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signedContent, sigData))?;
        Ok(__cordl_object.into())
    }
    pub fn New_CmsProcessable_Il2CppArray2(
        signedContent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsProcessable,
        >,
        sigBlock: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signedContent, sigBlock))?;
        Ok(__cordl_object.into())
    }
    pub fn New_CmsProcessable_Stream4(
        signedContent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsProcessable,
        >,
        sigData: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signedContent, sigData))?;
        Ok(__cordl_object.into())
    }
    pub fn New_CmsSignedData0(
        c: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsSignedData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (c))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ContentInfo8(
        sigData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigData))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IDictionary_ContentInfo7(
        hashes: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        sigData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hashes, sigData))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IDictionary_Il2CppArray3(
        hashes: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        sigBlock: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hashes, sigBlock))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray1(
        sigBlock: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigBlock))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Stream5(
        sigData: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigData))?;
        Ok(__cordl_object.into())
    }
    pub fn ReplaceCertificatesAndCrls(
        signedData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsSignedData,
        >,
        x509Certs: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
        x509Crls: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
        x509AttrCerts: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Store,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsSignedData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsSignedData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ReplaceCertificatesAndCrls",
                (signedData, x509Certs, x509Crls, x509AttrCerts),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceSigners(
        signedData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsSignedData,
        >,
        signerInformationStore: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::SignerInformationStore,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsSignedData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsSignedData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReplaceSigners", (signedData, signerInformationStore))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CmsProcessable_ContentInfo6(
        &mut self,
        signedContent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsProcessable,
        >,
        sigData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signedContent, sigData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CmsProcessable_Il2CppArray2(
        &mut self,
        signedContent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsProcessable,
        >,
        sigBlock: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signedContent, sigBlock))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CmsProcessable_Stream4(
        &mut self,
        signedContent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsProcessable,
        >,
        sigData: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signedContent, sigData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CmsSignedData0(
        &mut self,
        c: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsSignedData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ContentInfo8(
        &mut self,
        sigData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IDictionary_ContentInfo7(
        &mut self,
        hashes: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        sigData: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hashes, sigData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IDictionary_Il2CppArray3(
        &mut self,
        hashes: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        sigBlock: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hashes, sigBlock))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        sigBlock: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigBlock))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Stream5(
        &mut self,
        sigData: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigData))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::ContentInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::ContentInfo,
        > = __cordl_object.invoke("get_ContentInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SignedContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsProcessable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsProcessable,
        > = __cordl_object.invoke("get_SignedContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SignedContentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerObjectIdentifier>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = __cordl_object.invoke("get_SignedContentType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SignedContentTypeOid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_SignedContentTypeOid", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedData")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Cms::CmsSignedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
