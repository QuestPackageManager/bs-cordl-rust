#[cfg(feature = "Mono+Net+Security+MobileTlsProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct MobileTlsProvider {
    __cordl_parent: crate::Mono::Security::Interface::MonoTlsProvider,
}
#[cfg(feature = "Mono+Net+Security+MobileTlsProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Net::Security::MobileTlsProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Net.Security";
    const CLASS_NAME: &'static str = "MobileTlsProvider";
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
#[cfg(feature = "Mono+Net+Security+MobileTlsProvider")]
impl std::ops::Deref for crate::Mono::Net::Security::MobileTlsProvider {
    type Target = crate::Mono::Security::Interface::MonoTlsProvider;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+MobileTlsProvider")]
impl std::ops::DerefMut for crate::Mono::Net::Security::MobileTlsProvider {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+MobileTlsProvider")]
impl crate::Mono::Net::Security::MobileTlsProvider {
    pub fn CreateSslStream(
        &mut self,
        sslStream: quest_hook::libil2cpp::Gc<crate::System::Net::Security::SslStream>,
        innerStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        leaveInnerStreamOpen: bool,
        settings: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::Interface::MonoTlsSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MobileAuthenticatedStream>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::Security::SslStream,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Security::Interface::MonoTlsSettings,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Mono::Net::Security::MobileAuthenticatedStream,
                        >,
                        4usize,
                    >("CreateSslStream")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSslStream", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileAuthenticatedStream,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (sslStream, innerStream, leaveInnerStreamOpen, settings),
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
    pub fn ValidateCertificate(
        &mut self,
        validator: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::ChainValidationHelper,
        >,
        targetHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        serverMode: bool,
        certificates: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        >,
        wantsChain: bool,
        chain: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Security::Cryptography::X509Certificates::X509Chain,
            >,
        >,
        errors: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Security::SslPolicyErrors,
        >,
        status11: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Net::Security::ChainValidationHelper,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
                            >,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Security::Cryptography::X509Certificates::X509Chain,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Net::Security::SslPolicyErrors,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        bool,
                        8usize,
                    >("ValidateCertificate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateCertificate", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        validator,
                        targetHost,
                        serverMode,
                        certificates,
                        wantsChain,
                        chain,
                        errors,
                        status11,
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
#[cfg(feature = "Mono+Net+Security+MobileTlsProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::MobileTlsProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
