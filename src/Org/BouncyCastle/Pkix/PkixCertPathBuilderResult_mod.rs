#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilderResult")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixCertPathBuilderResult {
    __cordl_parent: crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorResult,
    pub certPath: *mut crate::Org::BouncyCastle::Pkix::PkixCertPath,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilderResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult => "Org.BouncyCastle.Pkix"
    ."PkixCertPathBuilderResult"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilderResult")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult {
    type Target = crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorResult;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilderResult")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilderResult")]
impl crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult {
    pub fn New(
        certPath: *mut crate::Org::BouncyCastle::Pkix::PkixCertPath,
        trustAnchor: *mut crate::Org::BouncyCastle::Pkix::TrustAnchor,
        policyTree: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        subjectPublicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (certPath, trustAnchor, policyTree, subjectPublicKey),
            )?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        certPath: *mut crate::Org::BouncyCastle::Pkix::PkixCertPath,
        trustAnchor: *mut crate::Org::BouncyCastle::Pkix::TrustAnchor,
        policyTree: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        subjectPublicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certPath, trustAnchor, policyTree, subjectPublicKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_CertPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkix::PkixCertPath,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkix::PkixCertPath = __cordl_object
            .invoke("get_CertPath", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilderResult")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
