#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+EnvelopedDataParser")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvelopedDataParser {
    __cordl_parent: crate::System::Object,
    pub _seq: *mut crate::Org::BouncyCastle::Asn1::Asn1SequenceParser,
    pub _version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub _nextObject: *mut crate::Org::BouncyCastle::Asn1::IAsn1Convertible,
    pub _originatorInfoCalled: bool,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+EnvelopedDataParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Cms::EnvelopedDataParser => "Org.BouncyCastle.Asn1.Cms"
    ."EnvelopedDataParser"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+EnvelopedDataParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cms::EnvelopedDataParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+EnvelopedDataParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cms::EnvelopedDataParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+EnvelopedDataParser")]
impl crate::Org::BouncyCastle::Asn1::Cms::EnvelopedDataParser {
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
    pub fn GetEncryptedContentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::EncryptedContentInfoParser,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::EncryptedContentInfoParser = __cordl_object
            .invoke("GetEncryptedContentInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRecipientInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser = __cordl_object
            .invoke("GetRecipientInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUnprotectedAttrs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser = __cordl_object
            .invoke("GetUnprotectedAttrs", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1SequenceParser,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret)
    }
    pub fn GetOriginatorInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorInfo = __cordl_object
            .invoke("GetOriginatorInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1SequenceParser,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+EnvelopedDataParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::EnvelopedDataParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
