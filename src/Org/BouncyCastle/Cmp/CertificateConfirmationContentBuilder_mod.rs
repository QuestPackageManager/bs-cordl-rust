#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContentBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct CertificateConfirmationContentBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub digestAlgFinder: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::DefaultDigestAlgorithmIdentifierFinder,
    >,
    pub acceptedCerts: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub acceptedReqIds: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
}
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContentBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cmp::CertificateConfirmationContentBuilder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cmp";
    const CLASS_NAME: &'static str = "CertificateConfirmationContentBuilder";
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
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateConfirmationContentBuilder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cmp::CertificateConfirmationContentBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        certHolder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        certReqId: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::CertificateConfirmationContentBuilder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::CertificateConfirmationContentBuilder,
        > = __cordl_object.invoke("AddAcceptedCertificate", (certHolder, certReqId))?;
        Ok(__cordl_ret.into())
    }
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::CertificateConfirmationContent,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::CertificateConfirmationContent,
        > = __cordl_object.invoke("Build", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_DefaultDigestAlgorithmIdentifierFinder1(
        digestAlgFinder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::DefaultDigestAlgorithmIdentifierFinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digestAlgFinder))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DefaultDigestAlgorithmIdentifierFinder1(
        &mut self,
        digestAlgFinder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::DefaultDigestAlgorithmIdentifierFinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digestAlgFinder))?;
        Ok(__cordl_ret.into())
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
