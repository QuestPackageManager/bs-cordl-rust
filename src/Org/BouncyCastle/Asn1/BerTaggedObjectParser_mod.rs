#[cfg(feature = "Org+BouncyCastle+Asn1+BerTaggedObjectParser")]
#[repr(C)]
#[derive(Debug)]
pub struct BerTaggedObjectParser {
    __cordl_parent: crate::System::Object,
    pub _constructed: bool,
    pub _tagNumber: i32,
    pub _parser: *mut crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerTaggedObjectParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::BerTaggedObjectParser
    => "Org.BouncyCastle.Asn1"."BerTaggedObjectParser"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+BerTaggedObjectParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::BerTaggedObjectParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerTaggedObjectParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::BerTaggedObjectParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerTaggedObjectParser")]
impl crate::Org::BouncyCastle::Asn1::BerTaggedObjectParser {
    pub fn _ctor_i32_Stream0(
        &mut self,
        baseTag: i32,
        tagNumber: i32,
        contentStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (baseTag, tagNumber, contentStream))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_Asn1StreamParser1(
        &mut self,
        constructed: bool,
        tagNumber: i32,
        parser: *mut crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (constructed, tagNumber, parser))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsConstructed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsConstructed", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectParser(
        &mut self,
        tag: i32,
        isExplicit: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::IAsn1Convertible,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::IAsn1Convertible = __cordl_object
            .invoke("GetObjectParser", (tag, isExplicit))?;
        Ok(__cordl_ret)
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
    pub fn get_TagNo(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TagNo", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_i32_Stream0(
        baseTag: i32,
        tagNumber: i32,
        contentStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (baseTag, tagNumber, contentStream))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool_Asn1StreamParser1(
        constructed: bool,
        tagNumber: i32,
        parser: *mut crate::Org::BouncyCastle::Asn1::Asn1StreamParser,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (constructed, tagNumber, parser))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerTaggedObjectParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::BerTaggedObjectParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
