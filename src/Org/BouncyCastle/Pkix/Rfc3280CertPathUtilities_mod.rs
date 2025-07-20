#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3280CertPathUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct Rfc3280CertPathUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3280CertPathUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Pkix::Rfc3280CertPathUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Pkix";
    const CLASS_NAME: &'static str = "Rfc3280CertPathUtilities";
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
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3280CertPathUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::Rfc3280CertPathUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3280CertPathUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::Rfc3280CertPathUtilities {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3280CertPathUtilities")]
impl crate::Org::BouncyCastle::Pkix::Rfc3280CertPathUtilities {
    pub fn CheckCrl(
        dp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
        >,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        validDate: crate::System::DateTime,
        defaultCRLSignCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        defaultCRLSignKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::CertStatus,
        >,
        reasonMask: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::ReasonsMask,
        >,
        certPathCerts: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixParameters,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                            crate::System::DateTime,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::CertStatus,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::ReasonsMask,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >("CheckCrl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckCrl", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        dp,
                        paramsPKIX,
                        cert,
                        validDate,
                        defaultCRLSignCert,
                        defaultCRLSignKey,
                        certStatus,
                        reasonMask,
                        certPathCerts,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckCrls(
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        validDate: crate::System::DateTime,
        sign: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        workingPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        certPathCerts: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixParameters,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                            crate::System::DateTime,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("CheckCrls")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckCrls", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (paramsPKIX, cert, validDate, sign, workingPublicKey, certPathCerts),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PrepareCertB(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        policyNodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
            >,
        >,
        validPolicyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
        policyMapping: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
                        >,
                        5usize,
                    >("PrepareCertB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareCertB", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (certPath, index, policyNodes, validPolicyTree, policyMapping),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertA(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("PrepareNextCertA")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareNextCertA", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (certPath, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertG(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        nameConstraintValidator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidator,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("PrepareNextCertG")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareNextCertG", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (certPath, index, nameConstraintValidator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertH1(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        explicitPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("PrepareNextCertH1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareNextCertH1", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (certPath, index, explicitPolicy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertH2(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        policyMapping: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("PrepareNextCertH2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareNextCertH2", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (certPath, index, policyMapping))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertH3(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        inhibitAnyPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("PrepareNextCertH3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareNextCertH3", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (certPath, index, inhibitAnyPolicy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertI1(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        explicitPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("PrepareNextCertI1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareNextCertI1", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (certPath, index, explicitPolicy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertI2(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        policyMapping: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("PrepareNextCertI2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareNextCertI2", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (certPath, index, policyMapping))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertJ(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        inhibitAnyPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("PrepareNextCertJ")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareNextCertJ", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (certPath, index, inhibitAnyPolicy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertK(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("PrepareNextCertK")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareNextCertK", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (certPath, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertL(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        maxPathLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("PrepareNextCertL")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareNextCertL", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (certPath, index, maxPathLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertM(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        maxPathLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("PrepareNextCertM")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareNextCertM", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (certPath, index, maxPathLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertN(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("PrepareNextCertN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareNextCertN", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (certPath, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNextCertO(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        criticalExtensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        pathCheckers: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Utilities::Collections::ISet,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("PrepareNextCertO")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareNextCertO", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (certPath, index, criticalExtensions, pathCheckers),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertA(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        index: i32,
        workingPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        workingIssuerName: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Name,
        >,
        sign: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixParameters,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::X509::X509Name,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("ProcessCertA")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCertA", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        certPath,
                        paramsPKIX,
                        index,
                        workingPublicKey,
                        workingIssuerName,
                        sign,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertBC(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        nameConstraintValidator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixNameConstraintValidator,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ProcessCertBC")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCertBC", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (certPath, index, nameConstraintValidator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertD(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        acceptablePolicies: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        validPolicyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
        policyNodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
            >,
        >,
        inhibitAnyPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Utilities::Collections::ISet,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
                        >,
                        6usize,
                    >("ProcessCertD")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCertD", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        certPath,
                        index,
                        acceptablePolicies,
                        validPolicyTree,
                        policyNodes,
                        inhibitAnyPolicy,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertE(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        validPolicyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
                        >,
                        3usize,
                    >("ProcessCertE")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCertE", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (certPath, index, validPolicyTree))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertF(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        validPolicyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
        explicitPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("ProcessCertF")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCertF", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (certPath, index, validPolicyTree, explicitPolicy),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlA1i(
        currentDate: crate::System::DateTime,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixParameters,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Crl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Utilities::Collections::ISet,
                        >,
                        4usize,
                    >("ProcessCrlA1i")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCrlA1i", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (currentDate, paramsPKIX, cert, crl))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlA1ii(
        currentDate: crate::System::DateTime,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Utilities::Collections::ISet,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixParameters,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Crl,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::Org::BouncyCastle::Utilities::Collections::ISet,
                                >,
                            >,
                        >,
                        4usize,
                    >("ProcessCrlA1ii")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCrlA1ii", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Utilities::Collections::ISet,
                >,
            >,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (currentDate, paramsPKIX, cert, crl))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlB1(
        dp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
        >,
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Crl,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ProcessCrlB1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCrlB1", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (dp, cert, crl))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlB2(
        dp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
        >,
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Crl,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ProcessCrlB2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCrlB2", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (dp, cert, crl))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlC(
        deltaCRL: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        completeCRL: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Crl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Crl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixParameters,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ProcessCrlC")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCrlC", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (deltaCRL, completeCRL, pkixParams))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlD(
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        dp: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::ReasonsMask>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Crl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::X509::DistributionPoint,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Pkix::ReasonsMask,
                        >,
                        2usize,
                    >("ProcessCrlD")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCrlD", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::ReasonsMask,
        > = unsafe { cordl_method_info.invoke_unchecked((), (crl, dp))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlF(
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        defaultCRLSignCert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
        defaultCRLSignKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        certPathCerts: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Crl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixParameters,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Utilities::Collections::ISet,
                        >,
                        6usize,
                    >("ProcessCrlF")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCrlF", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        crl,
                        cert,
                        defaultCRLSignCert,
                        defaultCRLSignKey,
                        paramsPKIX,
                        certPathCerts,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlG(
        crl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        keys: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Crl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Utilities::Collections::ISet,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                        2usize,
                    >("ProcessCrlG")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCrlG", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = unsafe { cordl_method_info.invoke_unchecked((), (crl, keys))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlH(
        deltaCrls: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Utilities::Collections::ISet,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::X509::X509Crl,
                        >,
                        2usize,
                    >("ProcessCrlH")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCrlH", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Crl,
        > = unsafe { cordl_method_info.invoke_unchecked((), (deltaCrls, key))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlI(
        validDate: crate::System::DateTime,
        deltacrl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::CertStatus,
        >,
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Crl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::CertStatus,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixParameters,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("ProcessCrlI")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCrlI", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (validDate, deltacrl, cert, certStatus, pkixParams),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlJ(
        validDate: crate::System::DateTime,
        completecrl: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Crl>,
        cert: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        certStatus: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::CertStatus>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Crl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::CertStatus,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("ProcessCrlJ")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCrlJ", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (validDate, completecrl, cert, certStatus))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WrapupCertA(
        explicitPolicy: i32,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::X509::X509Certificate,
                            >,
                        ),
                        i32,
                        2usize,
                    >("WrapupCertA")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WrapupCertA", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (explicitPolicy, cert))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WrapupCertB(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        explicitPolicy: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("WrapupCertB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WrapupCertB", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (certPath, index, explicitPolicy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WrapupCertF(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        index: i32,
        pathCheckers: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        criticalExtensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Utilities::Collections::ISet,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("WrapupCertF")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WrapupCertF", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (certPath, index, pathCheckers, criticalExtensions),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WrapupCertG(
        certPath: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixCertPath,
        >,
        paramsPKIX: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
        userInitialPolicySet: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        index: i32,
        policyNodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
            >,
        >,
        validPolicyTree: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
        acceptablePolicies: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixCertPath,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixParameters,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Utilities::Collections::ISet,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Utilities::Collections::ISet,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
                        >,
                        7usize,
                    >("WrapupCertG")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WrapupCertG", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        certPath,
                        paramsPKIX,
                        userInitialPolicySet,
                        index,
                        policyNodes,
                        validPolicyTree,
                        acceptablePolicies,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+Rfc3280CertPathUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::Rfc3280CertPathUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
