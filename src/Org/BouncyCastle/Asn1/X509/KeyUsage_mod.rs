#[cfg(feature = "Org+BouncyCastle+Asn1+X509+KeyUsage")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyUsage {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::DerBitString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+KeyUsage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::KeyUsage =>
    "Org.BouncyCastle.Asn1.X509"."KeyUsage"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+KeyUsage")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::KeyUsage {
    type Target = crate::Org::BouncyCastle::Asn1::DerBitString;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+KeyUsage")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::KeyUsage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+KeyUsage")]
impl crate::Org::BouncyCastle::Asn1::X509::KeyUsage {
    pub const CrlSign: i32 = 2i32;
    pub const DataEncipherment: i32 = 16i32;
    pub const DecipherOnly: i32 = 32768i32;
    pub const DigitalSignature: i32 = 128i32;
    pub const EncipherOnly: i32 = 1i32;
    pub const KeyAgreement: i32 = 8i32;
    pub const KeyCertSign: i32 = 4i32;
    pub const KeyEncipherment: i32 = 32i32;
    pub const NonRepudiation: i32 = 64i32;
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_0(
        &mut self,
        usage: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (usage))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerBitString1(
        &mut self,
        usage: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (usage))?;
        Ok(__cordl_ret)
    }
    pub fn New_i32_0(usage: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (usage))?;
        Ok(__cordl_object)
    }
    pub fn New_DerBitString1(
        usage: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (usage))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+KeyUsage")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::KeyUsage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
