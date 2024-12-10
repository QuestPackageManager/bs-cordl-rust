#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeCapabilitiesAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct SmimeCapabilitiesAttribute {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X509::AttributeX509,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeCapabilitiesAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Smime::SmimeCapabilitiesAttribute =>
    "Org.BouncyCastle.Asn1.Smime"."SmimeCapabilitiesAttribute"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeCapabilitiesAttribute")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Smime::SmimeCapabilitiesAttribute {
    type Target = crate::Org::BouncyCastle::Asn1::X509::AttributeX509;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeCapabilitiesAttribute")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Smime::SmimeCapabilitiesAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeCapabilitiesAttribute")]
impl crate::Org::BouncyCastle::Asn1::Smime::SmimeCapabilitiesAttribute {
    pub fn New(
        capabilities: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Smime::SmimeCapabilityVector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (capabilities))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        capabilities: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Smime::SmimeCapabilityVector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (capabilities))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeCapabilitiesAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Smime::SmimeCapabilitiesAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
