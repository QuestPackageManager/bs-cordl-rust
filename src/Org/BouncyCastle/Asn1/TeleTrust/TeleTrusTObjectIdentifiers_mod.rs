#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTObjectIdentifiers")]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTObjectIdentifiers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTObjectIdentifiers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTObjectIdentifiers =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTObjectIdentifiers"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTObjectIdentifiers")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTObjectIdentifiers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTObjectIdentifiers")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTObjectIdentifiers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTObjectIdentifiers")]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTObjectIdentifiers {
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
#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTObjectIdentifiers")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTObjectIdentifiers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
