#[cfg(feature = "Org+BouncyCastle+Asn1+BerOctetStringParser")]
#[repr(C)]
#[derive(Debug)]
pub struct BerOctetStringParser {
    __cordl_parent: crate::System::Object,
    pub _parser: *mut crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerOctetStringParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::BerOctetStringParser =>
    "Org.BouncyCastle.Asn1"."BerOctetStringParser"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+BerOctetStringParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::BerOctetStringParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerOctetStringParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::BerOctetStringParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerOctetStringParser")]
impl crate::Org::BouncyCastle::Asn1::BerOctetStringParser {
    pub fn GetOctetStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("GetOctetStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        parser: *mut crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parser))?;
        Ok(__cordl_object)
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
    pub fn _ctor(
        &mut self,
        parser: *mut crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parser))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerOctetStringParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::BerOctetStringParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}