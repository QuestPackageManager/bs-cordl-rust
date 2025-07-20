#[cfg(feature = "System+Security+Principal+WellKnownSidType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WellKnownSidType {
    #[default]
    AccountAdministratorSid = 38i32,
    AccountCertAdminsSid = 46i32,
    AccountComputersSid = 44i32,
    AccountControllersSid = 45i32,
    AccountDomainAdminsSid = 41i32,
    AccountDomainGuestsSid = 43i32,
    AccountDomainUsersSid = 42i32,
    AccountEnterpriseAdminsSid = 48i32,
    AccountGuestSid = 39i32,
    AccountKrbtgtSid = 40i32,
    AccountPolicyAdminsSid = 49i32,
    AccountRasAndIasServersSid = 50i32,
    AccountSchemaAdminsSid = 47i32,
    AnonymousSid = 13i32,
    AuthenticatedUserSid = 17i32,
    BatchSid = 10i32,
    BuiltinAccountOperatorsSid = 30i32,
    BuiltinAdministratorsSid = 26i32,
    BuiltinAuthorizationAccessSid = 59i32,
    BuiltinBackupOperatorsSid = 33i32,
    BuiltinDomainSid = 25i32,
    BuiltinGuestsSid = 28i32,
    BuiltinIncomingForestTrustBuildersSid = 56i32,
    BuiltinNetworkConfigurationOperatorsSid = 37i32,
    BuiltinPerformanceLoggingUsersSid = 58i32,
    BuiltinPerformanceMonitoringUsersSid = 57i32,
    BuiltinPowerUsersSid = 29i32,
    BuiltinPreWindows2000CompatibleAccessSid = 35i32,
    BuiltinPrintOperatorsSid = 32i32,
    BuiltinRemoteDesktopUsersSid = 36i32,
    BuiltinReplicatorSid = 34i32,
    BuiltinSystemOperatorsSid = 31i32,
    BuiltinUsersSid = 27i32,
    CreatorGroupServerSid = 6i32,
    CreatorGroupSid = 4i32,
    CreatorOwnerServerSid = 5i32,
    CreatorOwnerSid = 3i32,
    DialupSid = 8i32,
    DigestAuthenticationSid = 52i32,
    EnterpriseControllersSid = 15i32,
    InteractiveSid = 11i32,
    LocalServiceSid = 23i32,
    LocalSid = 2i32,
    LocalSystemSid = 22i32,
    LogonIdsSid = 21i32,
    MaxDefined = 60i32,
    NTAuthoritySid = 7i32,
    NetworkServiceSid = 24i32,
    NetworkSid = 9i32,
    NtlmAuthenticationSid = 51i32,
    NullSid = 0i32,
    OtherOrganizationSid = 55i32,
    ProxySid = 14i32,
    RemoteLogonIdSid = 20i32,
    RestrictedCodeSid = 18i32,
    SChannelAuthenticationSid = 53i32,
    SelfSid = 16i32,
    ServiceSid = 12i32,
    TerminalServerSid = 19i32,
    ThisOrganizationSid = 54i32,
    WinAccountReadonlyControllersSid = 75i32,
    WinApplicationPackageAuthoritySid = 83i32,
    WinBuiltinAnyPackageSid = 84i32,
    WinBuiltinCertSvcDComAccessGroup = 78i32,
    WinBuiltinCryptoOperatorsSid = 64i32,
    WinBuiltinDCOMUsersSid = 61i32,
    WinBuiltinEventLogReadersGroup = 76i32,
    WinBuiltinIUsersSid = 62i32,
    WinCacheablePrincipalsGroupSid = 72i32,
    WinCapabilityDocumentsLibrarySid = 91i32,
    WinCapabilityEnterpriseAuthenticationSid = 93i32,
    WinCapabilityInternetClientServerSid = 86i32,
    WinCapabilityInternetClientSid = 85i32,
    WinCapabilityMusicLibrarySid = 90i32,
    WinCapabilityPicturesLibrarySid = 88i32,
    WinCapabilityPrivateNetworkClientServerSid = 87i32,
    WinCapabilityRemovableStorageSid = 94i32,
    WinCapabilitySharedUserCertificatesSid = 92i32,
    WinCapabilityVideosLibrarySid = 89i32,
    WinConsoleLogonSid = 81i32,
    WinCreatorOwnerRightsSid = 71i32,
    WinEnterpriseReadonlyControllersSid = 74i32,
    WinHighLabelSid = 68i32,
    WinIUserSid = 63i32,
    WinLocalLogonSid = 80i32,
    WinLowLabelSid = 66i32,
    WinMediumLabelSid = 67i32,
    WinMediumPlusLabelSid = 79i32,
    WinNewEnterpriseReadonlyControllersSid = 77i32,
    WinNonCacheablePrincipalsGroupSid = 73i32,
    WinSystemLabelSid = 69i32,
    WinThisOrganizationCertificateSid = 82i32,
    WinUntrustedLabelSid = 65i32,
    WinWriteRestrictedCodeSid = 70i32,
    WorldSid = 1i32,
}
#[cfg(feature = "System+Security+Principal+WellKnownSidType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Principal::WellKnownSidType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Security.Principal";
    const CLASS_NAME: &'static str = "WellKnownSidType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Security+Principal+WellKnownSidType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Security::Principal::WellKnownSidType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Security+Principal+WellKnownSidType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Security::Principal::WellKnownSidType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "System+Security+Principal+WellKnownSidType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Security::Principal::WellKnownSidType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Security+Principal+WellKnownSidType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Security::Principal::WellKnownSidType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
