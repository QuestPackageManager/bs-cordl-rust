#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ChainValidationHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub owner: quest_hook::libil2cpp::Gc<
        crate::System::WeakReference_1<
            quest_hook::libil2cpp::Gc<crate::System::Net::Security::SslStream>,
        >,
    >,
    pub settings: quest_hook::libil2cpp::Gc<
        crate::Mono::Security::Interface::MonoTlsSettings,
    >,
    pub provider: quest_hook::libil2cpp::Gc<
        crate::Mono::Net::Security::MobileTlsProvider,
    >,
    pub certValidationCallback: quest_hook::libil2cpp::Gc<
        crate::System::Net::ServerCertValidationCallback,
    >,
    pub certSelectionCallback: quest_hook::libil2cpp::Gc<
        crate::System::Net::Security::LocalCertSelectionCallback,
    >,
    pub tlsStream: quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MonoTlsStream>,
    pub request: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
}
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Net::Security::ChainValidationHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Net.Security";
    const CLASS_NAME: &'static str = "ChainValidationHelper";
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
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
impl std::ops::Deref for crate::Mono::Net::Security::ChainValidationHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
impl std::ops::DerefMut for crate::Mono::Net::Security::ChainValidationHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
impl crate::Mono::Net::Security::ChainValidationHelper {
    pub fn Create(
        provider: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        >,
        settings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::MonoTlsSettings>,
        >,
        stream: quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MonoTlsStream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::ChainValidationHelper>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Net::Security::MobileTlsProvider,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Mono::Security::Interface::MonoTlsSettings,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Net::Security::MonoTlsStream,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Mono::Net::Security::ChainValidationHelper,
                        >,
                        3usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::ChainValidationHelper,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (provider, settings, stream))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DefaultSelectionCallback(
        targetHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localCertificates: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
        remoteCertificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        acceptableIssuers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509Certificate,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
                        >,
                        4usize,
                    >("DefaultSelectionCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DefaultSelectionCallback", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (targetHost, localCertificates, remoteCertificate, acceptableIssuers),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInternalValidator(
        owner: quest_hook::libil2cpp::Gc<crate::System::Net::Security::SslStream>,
        provider: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        >,
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::ChainValidationHelper>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::Security::SslStream,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Net::Security::MobileTlsProvider,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Security::Interface::MonoTlsSettings,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Mono::Net::Security::ChainValidationHelper,
                        >,
                        3usize,
                    >("GetInternalValidator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetInternalValidator", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::ChainValidationHelper,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (owner, provider, settings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetValidationCallback(
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ServerCertValidationCallback>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Mono::Security::Interface::MonoTlsSettings,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Net::ServerCertValidationCallback,
                        >,
                        1usize,
                    >("GetValidationCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetValidationCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::ServerCertValidationCallback,
        > = unsafe { cordl_method_info.invoke_unchecked((), (settings))? };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallback(
        &mut self,
        leaf: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        chain: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Chain,
        >,
        errors: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509Certificate,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509Chain,
                            >,
                            crate::System::Net::Security::SslPolicyErrors,
                        ),
                        bool,
                        3usize,
                    >("InvokeCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeCallback", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (leaf, chain, errors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        owner: quest_hook::libil2cpp::Gc<crate::System::Net::Security::SslStream>,
        provider: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        >,
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
        cloneSettings: bool,
        stream: quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MonoTlsStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (owner, provider, settings, cloneSettings, stream))?;
        Ok(__cordl_object.into())
    }
    pub fn SelectClientCertificate(
        &mut self,
        targetHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localCertificates: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
        remoteCertificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        acceptableIssuers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        clientCertificate: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Security::Cryptography::X509Certificates::X509Certificate,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509Certificate,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Security::Cryptography::X509Certificates::X509Certificate,
                                >,
                            >,
                        ),
                        bool,
                        5usize,
                    >("SelectClientCertificate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectClientCertificate", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        targetHost,
                        localCertificates,
                        remoteCertificate,
                        acceptableIssuers,
                        clientCertificate,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateCertificate(
        &mut self,
        host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        serverMode: bool,
        leaf: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        chain: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Chain,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::ValidationResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509Certificate,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509Chain,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Mono::Security::Interface::ValidationResult,
                        >,
                        4usize,
                    >("ValidateCertificate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateCertificate", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::ValidationResult,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (host, serverMode, leaf, chain))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateChain_ByRefMut1(
        &mut self,
        host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        server: bool,
        leaf: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        chain: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Security::Cryptography::X509Certificates::X509Chain,
            >,
        >,
        certs: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
        errors: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::ValidationResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509Certificate,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Security::Cryptography::X509Certificates::X509Chain,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
                            >,
                            crate::System::Net::Security::SslPolicyErrors,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Mono::Security::Interface::ValidationResult,
                        >,
                        6usize,
                    >("ValidateChain")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateChain", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::ValidationResult,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (host, server, leaf, chain, certs, errors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateChain_X509Chain0(
        &mut self,
        host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        server: bool,
        leaf: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        chain: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Chain,
        >,
        certs: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
        errors: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::ValidationResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509Certificate,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509Chain,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
                            >,
                            crate::System::Net::Security::SslPolicyErrors,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Mono::Security::Interface::ValidationResult,
                        >,
                        6usize,
                    >("ValidateChain")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateChain", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::ValidationResult,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (host, server, leaf, chain, certs, errors))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::System::Net::Security::SslStream>,
        provider: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileTlsProvider,
        >,
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
        cloneSettings: bool,
        stream: quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MonoTlsStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::Security::SslStream,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Net::Security::MobileTlsProvider,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Security::Interface::MonoTlsSettings,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Net::Security::MonoTlsStream,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (owner, provider, settings, cloneSettings, stream),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Settings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::Interface::MonoTlsSettings>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Mono::Security::Interface::MonoTlsSettings,
                        >,
                        0usize,
                    >("get_Settings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Settings", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::ChainValidationHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
impl AsRef<crate::Mono::Security::Interface::ICertificateValidator>
for crate::Mono::Net::Security::ChainValidationHelper {
    fn as_ref(&self) -> &crate::Mono::Security::Interface::ICertificateValidator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Mono+Net+Security+ChainValidationHelper")]
impl AsMut<crate::Mono::Security::Interface::ICertificateValidator>
for crate::Mono::Net::Security::ChainValidationHelper {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Mono::Security::Interface::ICertificateValidator {
        unsafe { std::mem::transmute(self) }
    }
}
