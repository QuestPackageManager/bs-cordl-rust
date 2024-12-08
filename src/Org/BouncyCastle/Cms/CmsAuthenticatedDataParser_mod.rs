#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataParser")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsAuthenticatedDataParser {
    __cordl_parent: crate::Org::BouncyCastle::Cms::CmsContentInfoParser,
    pub _recipientInfoStore: *mut crate::Org::BouncyCastle::Cms::RecipientInformationStore,
    pub authData: *mut crate::Org::BouncyCastle::Asn1::Cms::AuthenticatedDataParser,
    pub macAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub mac: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub authAttrs: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    pub unauthAttrs: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    pub authAttrNotRead: bool,
    pub unauthAttrNotRead: bool,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsAuthenticatedDataParser => "Org.BouncyCastle.Cms"
    ."CmsAuthenticatedDataParser"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataParser {
    type Target = crate::Org::BouncyCastle::Cms::CmsContentInfoParser;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataParser")]
impl crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataParser {
    pub fn GetAuthAttrs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable = __cordl_object
            .invoke("GetAuthAttrs", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMac(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetMac", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRecipientInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::RecipientInformationStore,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::RecipientInformationStore = __cordl_object
            .invoke("GetRecipientInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUnauthAttrs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable = __cordl_object
            .invoke("GetUnauthAttrs", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray0(
        envelopedData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (envelopedData))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream1(
        envelopedData: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (envelopedData))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        envelopedData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (envelopedData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream1(
        &mut self,
        envelopedData: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (envelopedData))?;
        Ok(__cordl_ret)
    }
    pub fn get_MacAlgOid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_MacAlgOid", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MacAlgParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("get_MacAlgParams", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MacAlgorithmID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_MacAlgorithmID", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAuthenticatedDataParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsAuthenticatedDataParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}