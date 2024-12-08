#[cfg(feature = "Org+BouncyCastle+Asn1+X509+ReasonFlags")]
#[repr(C)]
#[derive(Debug)]
pub struct ReasonFlags {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::DerBitString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+ReasonFlags")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::ReasonFlags =>
    "Org.BouncyCastle.Asn1.X509"."ReasonFlags"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+ReasonFlags")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::ReasonFlags {
    type Target = crate::Org::BouncyCastle::Asn1::DerBitString;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+ReasonFlags")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::ReasonFlags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+ReasonFlags")]
impl crate::Org::BouncyCastle::Asn1::X509::ReasonFlags {
    pub const AACompromise: i32 = 32768i32;
    pub const AffiliationChanged: i32 = 16i32;
    pub const CACompromise: i32 = 32i32;
    pub const CertificateHold: i32 = 2i32;
    pub const CessationOfOperation: i32 = 4i32;
    pub const KeyCompromise: i32 = 64i32;
    pub const PrivilegeWithdrawn: i32 = 1i32;
    pub const Superseded: i32 = 8i32;
    pub const Unused: i32 = 128i32;
    pub fn _ctor_i32_0(
        &mut self,
        reasons: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reasons))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerBitString1(
        &mut self,
        reasons: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reasons))?;
        Ok(__cordl_ret)
    }
    pub fn New_i32_0(reasons: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reasons))?;
        Ok(__cordl_object)
    }
    pub fn New_DerBitString1(
        reasons: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reasons))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+ReasonFlags")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::ReasonFlags {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
