#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+AuthEnvelopedDataParser")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthEnvelopedDataParser {
    __cordl_parent: crate::System::Object,
    pub seq: *mut crate::Org::BouncyCastle::Asn1::Asn1SequenceParser,
    pub version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub nextObject: *mut crate::Org::BouncyCastle::Asn1::IAsn1Convertible,
    pub originatorInfoCalled: bool,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+AuthEnvelopedDataParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Cms::AuthEnvelopedDataParser =>
    "Org.BouncyCastle.Asn1.Cms"."AuthEnvelopedDataParser"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+AuthEnvelopedDataParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cms::AuthEnvelopedDataParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+AuthEnvelopedDataParser")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Cms::AuthEnvelopedDataParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+AuthEnvelopedDataParser")]
impl crate::Org::BouncyCastle::Asn1::Cms::AuthEnvelopedDataParser {
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
    pub fn GetAuthEncryptedContentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::EncryptedContentInfoParser,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::EncryptedContentInfoParser = __cordl_object
            .invoke("GetAuthEncryptedContentInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAuthAttrs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser = __cordl_object
            .invoke("GetAuthAttrs", ())?;
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
    pub fn GetUnauthAttrs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1SetParser = __cordl_object
            .invoke("GetUnauthAttrs", ())?;
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
    pub fn GetMac(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("GetMac", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+AuthEnvelopedDataParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::AuthEnvelopedDataParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
