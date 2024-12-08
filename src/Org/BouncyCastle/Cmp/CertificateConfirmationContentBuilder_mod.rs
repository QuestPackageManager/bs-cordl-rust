#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContentBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct CertificateConfirmationContentBuilder {
    __cordl_parent: crate::System::Object,
    pub digestAlgFinder: *mut crate::Org::BouncyCastle::Cms::DefaultDigestAlgorithmIdentifierFinder,
    pub acceptedCerts: *mut crate::System::Collections::IList,
    pub acceptedReqIds: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContentBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cmp::CertificateConfirmationContentBuilder =>
    "Org.BouncyCastle.Cmp"."CertificateConfirmationContentBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContentBuilder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cmp::CertificateConfirmationContentBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContentBuilder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cmp::CertificateConfirmationContentBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContentBuilder")]
impl crate::Org::BouncyCastle::Cmp::CertificateConfirmationContentBuilder {
    pub fn AddAcceptedCertificate(
        &mut self,
        certHolder: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        certReqId: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::CertificateConfirmationContentBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::CertificateConfirmationContentBuilder = __cordl_object
            .invoke("AddAcceptedCertificate", (certHolder, certReqId))?;
        Ok(__cordl_ret)
    }
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cmp::CertificateConfirmationContent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cmp::CertificateConfirmationContent = __cordl_object
            .invoke("Build", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_DefaultDigestAlgorithmIdentifierFinder1(
        digestAlgFinder: *mut crate::Org::BouncyCastle::Cms::DefaultDigestAlgorithmIdentifierFinder,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digestAlgFinder))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DefaultDigestAlgorithmIdentifierFinder1(
        &mut self,
        digestAlgFinder: *mut crate::Org::BouncyCastle::Cms::DefaultDigestAlgorithmIdentifierFinder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digestAlgFinder))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContentBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cmp::CertificateConfirmationContentBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
