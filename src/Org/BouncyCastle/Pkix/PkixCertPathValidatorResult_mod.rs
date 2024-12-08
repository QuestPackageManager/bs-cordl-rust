#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathValidatorResult")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixCertPathValidatorResult {
    __cordl_parent: crate::System::Object,
    pub trustAnchor: *mut crate::Org::BouncyCastle::Pkix::TrustAnchor,
    pub policyTree: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
    pub subjectPublicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathValidatorResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Pkix::PkixCertPathValidatorResult => "Org.BouncyCastle.Pkix"
    ."PkixCertPathValidatorResult"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathValidatorResult")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorResult {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathValidatorResult")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathValidatorResult")]
impl crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorResult {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        trustAnchor: *mut crate::Org::BouncyCastle::Pkix::TrustAnchor,
        policyTree: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        subjectPublicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (trustAnchor, policyTree, subjectPublicKey))?;
        Ok(__cordl_object)
    }
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
    pub fn _ctor(
        &mut self,
        trustAnchor: *mut crate::Org::BouncyCastle::Pkix::TrustAnchor,
        policyTree: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        subjectPublicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (trustAnchor, policyTree, subjectPublicKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_PolicyTree(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode = __cordl_object
            .invoke("get_PolicyTree", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SubjectPublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter = __cordl_object
            .invoke("get_SubjectPublicKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TrustAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkix::TrustAnchor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkix::TrustAnchor = __cordl_object
            .invoke("get_TrustAnchor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathValidatorResult")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
