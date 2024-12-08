#[cfg(feature = "Org+BouncyCastle+Asn1+X509+CertPolicyID")]
#[repr(C)]
#[derive(Debug)]
pub struct CertPolicyID {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+CertPolicyID")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X509::CertPolicyID =>
    "Org.BouncyCastle.Asn1.X509"."CertPolicyID"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+CertPolicyID")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::CertPolicyID {
    type Target = crate::Org::BouncyCastle::Asn1::DerObjectIdentifier;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+CertPolicyID")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::CertPolicyID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+CertPolicyID")]
impl crate::Org::BouncyCastle::Asn1::X509::CertPolicyID {
    pub fn New(
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (id))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+CertPolicyID")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::CertPolicyID {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
