#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+TimeStampedDataParser")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeStampedDataParser {
    __cordl_parent: crate::System::Object,
    pub version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub dataUri: *mut crate::Org::BouncyCastle::Asn1::DerIA5String,
    pub metaData: *mut crate::Org::BouncyCastle::Asn1::Cms::MetaData,
    pub content: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetStringParser,
    pub temporalEvidence: *mut crate::Org::BouncyCastle::Asn1::Cms::Evidence,
    pub parser: *mut crate::Org::BouncyCastle::Asn1::Asn1SequenceParser,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+TimeStampedDataParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Cms::TimeStampedDataParser => "Org.BouncyCastle.Asn1.Cms"
    ."TimeStampedDataParser"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+TimeStampedDataParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cms::TimeStampedDataParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+TimeStampedDataParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cms::TimeStampedDataParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+TimeStampedDataParser")]
impl crate::Org::BouncyCastle::Asn1::Cms::TimeStampedDataParser {
    pub fn GetTemporalEvidence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::Evidence,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::Evidence = __cordl_object
            .invoke("GetTemporalEvidence", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        parser: *mut crate::Org::BouncyCastle::Asn1::Asn1SequenceParser,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parser))?;
        Ok(__cordl_ret)
    }
    pub fn get_DataUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerIA5String,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerIA5String = __cordl_object
            .invoke("get_DataUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MetaData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::MetaData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::MetaData = __cordl_object
            .invoke("get_MetaData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Content(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetStringParser,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetStringParser = __cordl_object
            .invoke("get_Content", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        parser: *mut crate::Org::BouncyCastle::Asn1::Asn1SequenceParser,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parser))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+TimeStampedDataParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::TimeStampedDataParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
