#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PkcsObjectIdentifiers")]
#[repr(C)]
#[derive(Debug)]
pub struct PkcsObjectIdentifiers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PkcsObjectIdentifiers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Pkcs::PkcsObjectIdentifiers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Pkcs";
    const CLASS_NAME: &'static str = "PkcsObjectIdentifiers";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PkcsObjectIdentifiers")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Pkcs::PkcsObjectIdentifiers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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
