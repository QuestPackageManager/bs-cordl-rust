#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecific")]
#[repr(C)]
#[derive(Debug)]
pub struct BerApplicationSpecific {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::DerApplicationSpecific,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecific")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::BerApplicationSpecific
    => "Org.BouncyCastle.Asn1"."BerApplicationSpecific"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecific")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::BerApplicationSpecific {
    type Target = crate::Org::BouncyCastle::Asn1::DerApplicationSpecific;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecific")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::BerApplicationSpecific {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecific")]
impl crate::Org::BouncyCastle::Asn1::BerApplicationSpecific {
    pub fn New(
        tagNo: i32,
        vec: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagNo, vec))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        tagNo: i32,
        vec: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tagNo, vec))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerApplicationSpecific")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::BerApplicationSpecific {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
