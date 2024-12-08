#[cfg(feature = "Org+BouncyCastle+Asn1+BerSet")]
#[repr(C)]
#[derive(Debug)]
pub struct BerSet {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::DerSet,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerSet")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::BerSet =>
    "Org.BouncyCastle.Asn1"."BerSet"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+BerSet")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::BerSet {
    type Target = crate::Org::BouncyCastle::Asn1::DerSet;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerSet")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::BerSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerSet")]
impl crate::Org::BouncyCastle::Asn1::BerSet {
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
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Encodable1(
        &mut self,
        element: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (element))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1EncodableVector2(
        &mut self,
        elementVector: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (elementVector))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1EncodableVector__cordl_bool3(
        &mut self,
        elementVector: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
        needsSorting: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (elementVector, needsSorting))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1Encodable1(
        element: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (element))?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1EncodableVector2(
        elementVector: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (elementVector))?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1EncodableVector__cordl_bool3(
        elementVector: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
        needsSorting: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (elementVector, needsSorting))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerSet")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Asn1::BerSet {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
