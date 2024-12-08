#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeCapabilityVector")]
#[repr(C)]
#[derive(Debug)]
pub struct SmimeCapabilityVector {
    __cordl_parent: crate::System::Object,
    pub capabilities: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeCapabilityVector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Smime::SmimeCapabilityVector =>
    "Org.BouncyCastle.Asn1.Smime"."SmimeCapabilityVector"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeCapabilityVector")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Smime::SmimeCapabilityVector {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeCapabilityVector")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Smime::SmimeCapabilityVector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeCapabilityVector")]
impl crate::Org::BouncyCastle::Asn1::Smime::SmimeCapabilityVector {
    pub fn AddCapability_Asn1Encodable2(
        &mut self,
        capability: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        parameters: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCapability", (capability, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn AddCapability_DerObjectIdentifier0(
        &mut self,
        capability: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCapability", (capability))?;
        Ok(__cordl_ret)
    }
    pub fn AddCapability_i32_1(
        &mut self,
        capability: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCapability", (capability, value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToAsn1EncodableVector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector = __cordl_object
            .invoke("ToAsn1EncodableVector", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeCapabilityVector")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Smime::SmimeCapabilityVector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}