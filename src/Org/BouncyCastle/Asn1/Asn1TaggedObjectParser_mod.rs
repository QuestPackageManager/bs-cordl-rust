#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObjectParser")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1TaggedObjectParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObjectParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Asn1TaggedObjectParser
    => "Org.BouncyCastle.Asn1"."Asn1TaggedObjectParser"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObjectParser")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Asn1TaggedObjectParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObjectParser")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Asn1TaggedObjectParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObjectParser")]
impl crate::Org::BouncyCastle::Asn1::Asn1TaggedObjectParser {
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
    pub fn get_TagNo(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TagNo", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Asn1TaggedObjectParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Asn1TaggedObjectParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
