#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1Tags")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1Tags {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1Tags")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Asn1Tags =>
    "Org.BouncyCastle.Asn1"."Asn1Tags"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1Tags")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Asn1Tags {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1Tags")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Asn1Tags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1Tags")]
impl crate::Org::BouncyCastle::Asn1::Asn1Tags {
    pub const Application: i32 = 64i32;
    pub const BitString: i32 = 3i32;
    pub const BmpString: i32 = 30i32;
    pub const Boolean: i32 = 1i32;
    pub const Constructed: i32 = 32i32;
    pub const Enumerated: i32 = 10i32;
    pub const External: i32 = 8i32;
    pub const GeneralString: i32 = 27i32;
    pub const GeneralizedTime: i32 = 24i32;
    pub const GraphicString: i32 = 25i32;
    pub const IA5String: i32 = 22i32;
    pub const Integer: i32 = 2i32;
    pub const Null: i32 = 5i32;
    pub const NumericString: i32 = 18i32;
    pub const ObjectIdentifier: i32 = 6i32;
    pub const OctetString: i32 = 4i32;
    pub const PrintableString: i32 = 19i32;
    pub const Sequence: i32 = 16i32;
    pub const SequenceOf: i32 = 16i32;
    pub const Set: i32 = 17i32;
    pub const SetOf: i32 = 17i32;
    pub const T61String: i32 = 20i32;
    pub const Tagged: i32 = 128i32;
    pub const UniversalString: i32 = 28i32;
    pub const UtcTime: i32 = 23i32;
    pub const Utf8String: i32 = 12i32;
    pub const VideotexString: i32 = 21i32;
    pub const VisibleString: i32 = 26i32;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1Tags")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Asn1::Asn1Tags {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
