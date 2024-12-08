#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PkcsObjectIdentifiers")]
#[repr(C)]
#[derive(Debug)]
pub struct PkcsObjectIdentifiers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PkcsObjectIdentifiers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Pkcs::PkcsObjectIdentifiers =>
    "Org.BouncyCastle.Asn1.Pkcs"."PkcsObjectIdentifiers"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PkcsObjectIdentifiers")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Pkcs::PkcsObjectIdentifiers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PkcsObjectIdentifiers")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Pkcs::PkcsObjectIdentifiers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PkcsObjectIdentifiers")]
impl crate::Org::BouncyCastle::Asn1::Pkcs::PkcsObjectIdentifiers {
    pub const BagTypes: &'static str = "1.2.840.113549.1.12.10.1";
    pub const CertTypes: &'static str = "1.2.840.113549.1.9.22";
    pub const CrlTypes: &'static str = "1.2.840.113549.1.9.23";
    pub const DigestAlgorithm: &'static str = "1.2.840.113549.2";
    pub const EncryptionAlgorithm: &'static str = "1.2.840.113549.3";
    pub const IdAA: &'static str = "1.2.840.113549.1.9.16.2";
    pub const IdCT: &'static str = "1.2.840.113549.1.9.16.1";
    pub const IdCti: &'static str = "1.2.840.113549.1.9.16.6";
    pub const IdSpq: &'static str = "1.2.840.113549.1.9.16.5";
    pub const Pkcs1: &'static str = "1.2.840.113549.1.1";
    pub const Pkcs12: &'static str = "1.2.840.113549.1.12";
    pub const Pkcs12PbeIds: &'static str = "1.2.840.113549.1.12.1";
    pub const Pkcs3: &'static str = "1.2.840.113549.1.3";
    pub const Pkcs5: &'static str = "1.2.840.113549.1.5";
    pub const Pkcs7: &'static str = "1.2.840.113549.1.7";
    pub const Pkcs9: &'static str = "1.2.840.113549.1.9";
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PkcsObjectIdentifiers")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Pkcs::PkcsObjectIdentifiers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
