#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTlsConversions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Unity::UnityTlsConversions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTlsConversions";
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
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
impl std::ops::Deref for crate::Mono::Unity::UnityTlsConversions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
impl std::ops::DerefMut for crate::Mono::Unity::UnityTlsConversions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
impl crate::Mono::Unity::UnityTlsConversions {
    pub fn ConvertProtocolVersion(
        protocol: crate::Mono::Unity::UnityTls_unitytls_protocol,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Security::Interface::TlsProtocols> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Mono::Unity::UnityTls_unitytls_protocol),
                        crate::Mono::Security::Interface::TlsProtocols,
                        1usize,
                    >("ConvertProtocolVersion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertProtocolVersion", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Security::Interface::TlsProtocols = unsafe {
            method.invoke_unchecked((), (protocol))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxProtocol(
        protocols: crate::System::Security::Authentication::SslProtocols,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Unity::UnityTls_unitytls_protocol> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Security::Authentication::SslProtocols),
                        crate::Mono::Unity::UnityTls_unitytls_protocol,
                        1usize,
                    >("GetMaxProtocol")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetMaxProtocol", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_protocol = unsafe {
            method.invoke_unchecked((), (protocols))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMinProtocol(
        protocols: crate::System::Security::Authentication::SslProtocols,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Unity::UnityTls_unitytls_protocol> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Security::Authentication::SslProtocols),
                        crate::Mono::Unity::UnityTls_unitytls_protocol,
                        1usize,
                    >("GetMinProtocol")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetMinProtocol", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_protocol = unsafe {
            method.invoke_unchecked((), (protocols))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VerifyResultToAlertDescription(
        verifyResult: crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
        defaultAlert: crate::Mono::Security::Interface::AlertDescription,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Security::Interface::AlertDescription,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
                            crate::Mono::Security::Interface::AlertDescription,
                        ),
                        crate::Mono::Security::Interface::AlertDescription,
                        2usize,
                    >("VerifyResultToAlertDescription")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "VerifyResultToAlertDescription", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Security::Interface::AlertDescription = unsafe {
            method.invoke_unchecked((), (verifyResult, defaultAlert))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VerifyResultToChainStatus(
        verifyResult: crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Mono::Unity::UnityTls_unitytls_x509verify_result),
                        crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
                        1usize,
                    >("VerifyResultToChainStatus")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "VerifyResultToChainStatus", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags = unsafe {
            method.invoke_unchecked((), (verifyResult))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VerifyResultToPolicyErrror(
        verifyResult: crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Security::SslPolicyErrors> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Mono::Unity::UnityTls_unitytls_x509verify_result),
                        crate::System::Net::Security::SslPolicyErrors,
                        1usize,
                    >("VerifyResultToPolicyErrror")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "VerifyResultToPolicyErrror", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Net::Security::SslPolicyErrors = unsafe {
            method.invoke_unchecked((), (verifyResult))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Unity::UnityTlsConversions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
