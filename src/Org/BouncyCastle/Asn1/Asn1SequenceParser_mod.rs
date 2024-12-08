#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1SequenceParser")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1SequenceParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1SequenceParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Asn1SequenceParser =>
    "Org.BouncyCastle.Asn1"."Asn1SequenceParser"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1SequenceParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Asn1SequenceParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1SequenceParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Asn1SequenceParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1SequenceParser")]
impl crate::Org::BouncyCastle::Asn1::Asn1SequenceParser {
    pub fn ReadObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::IAsn1Convertible,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::IAsn1Convertible = __cordl_object
            .invoke("ReadObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1SequenceParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Asn1SequenceParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}