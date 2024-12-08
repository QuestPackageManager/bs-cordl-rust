#[cfg(feature = "Org+BouncyCastle+Asn1+BerTaggedObject")]
#[repr(C)]
#[derive(Debug)]
pub struct BerTaggedObject {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::DerTaggedObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerTaggedObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::BerTaggedObject =>
    "Org.BouncyCastle.Asn1"."BerTaggedObject"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+BerTaggedObject")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::BerTaggedObject {
    type Target = crate::Org::BouncyCastle::Asn1::DerTaggedObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerTaggedObject")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::BerTaggedObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerTaggedObject")]
impl crate::Org::BouncyCastle::Asn1::BerTaggedObject {
    pub fn Encode(
        &mut self,
        derOut: *mut crate::Org::BouncyCastle::Asn1::DerOutputStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (derOut))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_Asn1Encodable0(
        &mut self,
        tagNo: i32,
        obj: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tagNo, obj))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_i32_Asn1Encodable1(
        &mut self,
        explicitly: bool,
        tagNo: i32,
        obj: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (explicitly, tagNo, obj))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_2(
        &mut self,
        tagNo: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tagNo))?;
        Ok(__cordl_ret)
    }
    pub fn New_i32_Asn1Encodable0(
        tagNo: i32,
        obj: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagNo, obj))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool_i32_Asn1Encodable1(
        explicitly: bool,
        tagNo: i32,
        obj: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (explicitly, tagNo, obj))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_2(tagNo: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagNo))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerTaggedObject")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::BerTaggedObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
