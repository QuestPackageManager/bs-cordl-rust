#[cfg(feature = "Org+BouncyCastle+Asn1+Utilities+Asn1Dump")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1Dump {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Utilities+Asn1Dump")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Utilities::Asn1Dump =>
    "Org.BouncyCastle.Asn1.Utilities"."Asn1Dump"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Utilities+Asn1Dump")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Utilities::Asn1Dump {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Utilities+Asn1Dump")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Utilities::Asn1Dump {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Utilities+Asn1Dump")]
impl crate::Org::BouncyCastle::Asn1::Utilities::Asn1Dump {
    pub const SampleSize: i32 = 32i32;
    pub const Tab: &'static str = "    ";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Utilities+Asn1Dump")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Utilities::Asn1Dump {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
