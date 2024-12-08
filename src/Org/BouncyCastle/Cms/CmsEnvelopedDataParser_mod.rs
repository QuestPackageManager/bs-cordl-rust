#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataParser")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedDataParser {
    __cordl_parent: crate::Org::BouncyCastle::Cms::CmsContentInfoParser,
    pub recipientInfoStore: *mut crate::Org::BouncyCastle::Cms::RecipientInformationStore,
    pub envelopedData: *mut crate::Org::BouncyCastle::Asn1::Cms::EnvelopedDataParser,
    pub _encAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub _unprotectedAttributes: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    pub _attrNotRead: bool,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::CmsEnvelopedDataParser
    => "Org.BouncyCastle.Cms"."CmsEnvelopedDataParser"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataParser {
    type Target = crate::Org::BouncyCastle::Cms::CmsContentInfoParser;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataParser")]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedDataParser {
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
    pub fn GetUnprotectedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable = __cordl_object
            .invoke("GetUnprotectedAttributes", ())?;
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
    pub fn New_Il2CppArray0(
        envelopedData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (envelopedData))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream1(
        envelopedData: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (envelopedData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedDataParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsEnvelopedDataParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
