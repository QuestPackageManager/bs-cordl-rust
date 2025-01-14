#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilderResult")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixCertPathBuilderResult {
    __cordl_parent: crate::Org::BouncyCastle::Pkix::PkixCertPathValidatorResult,
    pub certPath: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Pkix::PkixCertPath,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCertPathBuilderResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Pkix";
    const CLASS_NAME: &'static str = "PkixCertPathBuilderResult";
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
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        trustAnchor: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::TrustAnchor,
        >,
        policyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
        subjectPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (certPath, trustAnchor, policyTree, subjectPublicKey),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        trustAnchor: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::TrustAnchor,
        >,
        policyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
        subjectPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixCertPath,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::TrustAnchor,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (certPath, trustAnchor, policyTree, subjectPublicKey),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_CertPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixCertPath>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixCertPath>,
                0usize,
            >("get_CertPath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_CertPath", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
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
