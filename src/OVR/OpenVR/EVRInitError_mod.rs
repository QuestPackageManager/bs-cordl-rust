#[cfg(feature = "cordl_class_OVR+OpenVR+EVRInitError")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum EVRInitError {
    #[cfg_attr(feature = "derive_Default", default)]
    Compositor_D3D11HardwareRequired = 401i32,
    Compositor_Failed = 400i32,
    Compositor_FirmwareRequiresUpdate = 402i32,
    Compositor_OverlayInitFailed = 403i32,
    Compositor_ScreenshotsInitFailed = 404i32,
    Compositor_UnableToCreateDevice = 405i32,
    Driver_CalibrationInvalid = 207i32,
    Driver_Failed = 200i32,
    Driver_HmdDisplayMirrored = 212i32,
    Driver_HmdDisplayNotFound = 208i32,
    Driver_HmdDriverIdOutOfBounds = 211i32,
    Driver_HmdInUse = 205i32,
    Driver_HmdUnknown = 202i32,
    Driver_NotCalibrated = 206i32,
    Driver_NotLoaded = 203i32,
    Driver_RuntimeOutOfDate = 204i32,
    Driver_TrackedDeviceInterfaceUnknown = 209i32,
    Driver_Unknown = 201i32,
    IPC_CompositorConnectFailed = 306i32,
    IPC_CompositorInitFailed = 303i32,
    IPC_CompositorInvalidConnectResponse = 307i32,
    IPC_ConnectFailed = 301i32,
    IPC_ConnectFailedAfterMultipleAttempts = 308i32,
    IPC_Failed = 305i32,
    IPC_MutexInitFailed = 304i32,
    IPC_ServerInitFailed = 300i32,
    IPC_SharedStateInitFailed = 302i32,
    Init_AnotherAppLaunching = 117i32,
    Init_AppInfoInitFailed = 114i32,
    Init_FactoryNotFound = 104i32,
    Init_FileNotFound = 103i32,
    Init_FirmwareRecoveryBusy = 139i32,
    Init_FirmwareUpdateBusy = 138i32,
    Init_HmdDriverIdIsNone = 125i32,
    Init_HmdNotFound = 108i32,
    Init_HmdNotFoundPresenceFailed = 126i32,
    Init_InitCanceledByUser = 116i32,
    Init_InstallationCorrupt = 101i32,
    Init_InstallationNotFound = 100i32,
    Init_InterfaceNotFound = 105i32,
    Init_Internal = 124i32,
    Init_InvalidApplicationType = 130i32,
    Init_InvalidInterface = 106i32,
    Init_LowPowerWatchdogNotSupported = 129i32,
    Init_NoConfigPath = 111i32,
    Init_NoLogPath = 112i32,
    Init_NoServerForBackgroundApp = 121i32,
    Init_NotAvailableToUtilityApps = 123i32,
    Init_NotAvailableToWatchdogApps = 131i32,
    Init_NotInitialized = 109i32,
    Init_NotSupportedWithCompositor = 122i32,
    Init_PathRegistryNotFound = 110i32,
    Init_PathRegistryNotWritable = 113i32,
    Init_RebootingBusy = 137i32,
    Init_Retry = 115i32,
    Init_SettingsInitFailed = 118i32,
    Init_ShuttingDown = 119i32,
    Init_TooManyObjects = 120i32,
    Init_TrackerManagerInitFailed = 142i32,
    Init_USBServiceBusy = 140i32,
    Init_UserConfigDirectoryInvalid = 107i32,
    Init_VRClientDLLNotFound = 102i32,
    Init_VRDashboardNotFound = 133i32,
    Init_VRDashboardStartupFailed = 134i32,
    Init_VRHomeNotFound = 135i32,
    Init_VRHomeStartupFailed = 136i32,
    Init_VRMonitorNotFound = 127i32,
    Init_VRMonitorStartupFailed = 128i32,
    Init_VRWebHelperStartupFailed = 141i32,
    Init_WatchdogDisabledInSettings = 132i32,
    None = 0i32,
    Steam_SteamInstallationNotFound = 2000i32,
    Unknown = 1i32,
    VendorSpecific_HmdFound_CantOpenDevice = 1101i32,
    VendorSpecific_HmdFound_CantReadFirmwareVersion = 1107i32,
    VendorSpecific_HmdFound_ConfigFailedSanityCheck = 1113i32,
    VendorSpecific_HmdFound_ConfigTooBig = 1104i32,
    VendorSpecific_HmdFound_ConfigTooSmall = 1105i32,
    VendorSpecific_HmdFound_NoStoredConfig = 1103i32,
    VendorSpecific_HmdFound_UnableToGetUserDataNext = 1110i32,
    VendorSpecific_HmdFound_UnableToGetUserDataStart = 1109i32,
    VendorSpecific_HmdFound_UnableToInitZLib = 1106i32,
    VendorSpecific_HmdFound_UnableToRequestConfigStart = 1102i32,
    VendorSpecific_HmdFound_UnableToSendUserDataStart = 1108i32,
    VendorSpecific_HmdFound_UserDataAddressRange = 1111i32,
    VendorSpecific_HmdFound_UserDataError = 1112i32,
    VendorSpecific_UnableToConnectToOculusRuntime = 1000i32,
    VendorSpecific_WindowsNotInDevMode = 1001i32,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRInitError")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::EVRInitError {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "EVRInitError";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRInitError")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::EVRInitError {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRInitError")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::EVRInitError {
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
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRInitError")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::EVRInitError {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVRInitError")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::EVRInitError {
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
