#[cfg(feature = "Org+BouncyCastle+Asn1+X509+CrlReason")]
#[repr(C)]
#[derive(Debug)]
pub struct CrlReason {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::DerEnumerated,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+CrlReason")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::CrlReason =>
    "Org.BouncyCastle.Asn1.X509"."CrlReason"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+CrlReason")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::CrlReason {
    type Target = crate::Org::BouncyCastle::Asn1::DerEnumerated;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+CrlReason")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::CrlReason {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+CrlReason")]
impl crate::Org::BouncyCastle::Asn1::X509::CrlReason {
    pub const AACompromise: i32 = 10i32;
    pub const AffiliationChanged: i32 = 3i32;
    pub const CACompromise: i32 = 2i32;
    pub const CertificateHold: i32 = 6i32;
    pub const CessationOfOperation: i32 = 5i32;
    pub const KeyCompromise: i32 = 1i32;
    pub const PrivilegeWithdrawn: i32 = 9i32;
    pub const RemoveFromCrl: i32 = 8i32;
    pub const Superseded: i32 = 4i32;
    pub const Unspecified: i32 = 0i32;
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
        reason: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reason))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerEnumerated1(
        &mut self,
        reason: *mut crate::Org::BouncyCastle::Asn1::DerEnumerated,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reason))?;
        Ok(__cordl_ret)
    }
    pub fn New_i32_0(reason: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reason))?;
        Ok(__cordl_object)
    }
    pub fn New_DerEnumerated1(
        reason: *mut crate::Org::BouncyCastle::Asn1::DerEnumerated,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reason))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+CrlReason")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::CrlReason {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
