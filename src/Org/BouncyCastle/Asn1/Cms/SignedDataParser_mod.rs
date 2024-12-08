#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+SignedDataParser")]
#[repr(C)]
#[derive(Debug)]
pub struct SignedDataParser {
    __cordl_parent: crate::System::Object,
    pub _seq: *mut crate::Org::BouncyCastle::Asn1::Asn1SequenceParser,
    pub _version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub _nextObject: *mut crate::System::Object,
    pub _certsCalled: bool,
    pub _crlsCalled: bool,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+SignedDataParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cms::SignedDataParser
    => "Org.BouncyCastle.Asn1.Cms"."SignedDataParser"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+SignedDataParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cms::SignedDataParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+SignedDataParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cms::SignedDataParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+SignedDataParser")]
impl crate::Org::BouncyCastle::Asn1::Cms::SignedDataParser {
    pub fn GetCertificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser = __cordl_object
            .invoke("GetCertificates", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCrls(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser = __cordl_object
            .invoke("GetCrls", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDigestAlgorithms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser = __cordl_object
            .invoke("GetDigestAlgorithms", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEncapContentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfoParser,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::ContentInfoParser = __cordl_object
            .invoke("GetEncapContentInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSignerInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser = __cordl_object
            .invoke("GetSignerInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1SequenceParser,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+SignedDataParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::SignedDataParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}