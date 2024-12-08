#[cfg(feature = "Org+BouncyCastle+X509+Extension+SubjectKeyIdentifierStructure")]
#[repr(C)]
#[derive(Debug)]
pub struct SubjectKeyIdentifierStructure {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X509::SubjectKeyIdentifier,
}
#[cfg(feature = "Org+BouncyCastle+X509+Extension+SubjectKeyIdentifierStructure")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::X509::Extension::SubjectKeyIdentifierStructure =>
    "Org.BouncyCastle.X509.Extension"."SubjectKeyIdentifierStructure"
);
#[cfg(feature = "Org+BouncyCastle+X509+Extension+SubjectKeyIdentifierStructure")]
impl std::ops::Deref
for crate::Org::BouncyCastle::X509::Extension::SubjectKeyIdentifierStructure {
    type Target = crate::Org::BouncyCastle::Asn1::X509::SubjectKeyIdentifier;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Extension+SubjectKeyIdentifierStructure")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::X509::Extension::SubjectKeyIdentifierStructure {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Extension+SubjectKeyIdentifierStructure")]
impl crate::Org::BouncyCastle::X509::Extension::SubjectKeyIdentifierStructure {
    pub fn New_Asn1OctetString0(
        encodedValue: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encodedValue))?;
        Ok(__cordl_object)
    }
    pub fn New_AsymmetricKeyParameter1(
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pubKey))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Asn1OctetString0(
        &mut self,
        encodedValue: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encodedValue))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AsymmetricKeyParameter1(
        &mut self,
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pubKey))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+Extension+SubjectKeyIdentifierStructure")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::Extension::SubjectKeyIdentifierStructure {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
