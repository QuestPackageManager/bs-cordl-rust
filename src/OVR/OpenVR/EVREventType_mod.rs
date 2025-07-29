#[cfg(feature = "cordl_class_OVR+OpenVR+EVREventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVREventType {
    #[default]
    VREvent_ActionBindingReloaded = 409i32,
    VREvent_ApplicationListUpdated = 1303i32,
    VREvent_ApplicationMimeTypeLoad = 1304i32,
    VREvent_ApplicationTransitionAborted = 1301i32,
    VREvent_ApplicationTransitionNewAppLaunchComplete = 1305i32,
    VREvent_ApplicationTransitionNewAppStarted = 1302i32,
    VREvent_ApplicationTransitionStarted = 1300i32,
    VREvent_AudioSettingsHaveChanged = 820i32,
    VREvent_BackgroundSettingHasChanged = 850i32,
    VREvent_ButtonPress = 200i32,
    VREvent_ButtonTouch = 202i32,
    VREvent_ButtonUnpress = 201i32,
    VREvent_ButtonUntouch = 203i32,
    VREvent_CameraSettingsHaveChanged = 851i32,
    VREvent_ChaperoneDataHasChanged = 800i32,
    VREvent_ChaperoneSettingsHaveChanged = 803i32,
    VREvent_ChaperoneTempDataHasChanged = 802i32,
    VREvent_ChaperoneUniverseHasChanged = 801i32,
    VREvent_Compositor_ChaperoneBoundsHidden = 1411i32,
    VREvent_Compositor_ChaperoneBoundsShown = 1410i32,
    VREvent_Compositor_MirrorWindowHidden = 1401i32,
    VREvent_Compositor_MirrorWindowShown = 1400i32,
    VREvent_ConsoleClosed = 421i32,
    VREvent_ConsoleOpened = 420i32,
    VREvent_DashboardActivated = 502i32,
    VREvent_DashboardDeactivated = 503i32,
    VREvent_DashboardOverlayCreated = 518i32,
    VREvent_DashboardRequested = 505i32,
    VREvent_DashboardSectionSettingChanged = 864i32,
    VREvent_DashboardThumbSelected = 504i32,
    VREvent_DriverRequestedQuit = 704i32,
    VREvent_DualAnalog_Cancel = 257i32,
    VREvent_DualAnalog_ModeSwitch1 = 255i32,
    VREvent_DualAnalog_ModeSwitch2 = 256i32,
    VREvent_DualAnalog_Move = 254i32,
    VREvent_DualAnalog_Press = 250i32,
    VREvent_DualAnalog_Touch = 252i32,
    VREvent_DualAnalog_Unpress = 251i32,
    VREvent_DualAnalog_Untouch = 253i32,
    VREvent_EnableHomeAppSettingsHaveChanged = 856i32,
    VREvent_EnterStandbyMode = 106i32,
    VREvent_EnvironmentSettingsHaveChanged = 854i32,
    VREvent_FirmwareUpdateFinished = 1101i32,
    VREvent_FirmwareUpdateStarted = 1100i32,
    VREvent_FocusEnter = 303i32,
    VREvent_FocusLeave = 304i32,
    VREvent_HideKeyboard = 510i32,
    VREvent_HideRenderModels = 410i32,
    VREvent_ImageFailed = 517i32,
    VREvent_ImageLoaded = 508i32,
    VREvent_InputFocusCaptured = 400i32,
    VREvent_InputFocusChanged = 406i32,
    VREvent_InputFocusReleased = 401i32,
    VREvent_Input_ActionManifestLoadFailed = 1704i32,
    VREvent_Input_ActionManifestReloaded = 1703i32,
    VREvent_Input_BindingLoadFailed = 1701i32,
    VREvent_Input_BindingLoadSuccessful = 1702i32,
    VREvent_Input_HapticVibration = 1700i32,
    VREvent_IpdChanged = 105i32,
    VREvent_KeyboardCharInput = 1201i32,
    VREvent_KeyboardClosed = 1200i32,
    VREvent_KeyboardDone = 1202i32,
    VREvent_KeyboardSectionSettingChanged = 862i32,
    VREvent_LeaveStandbyMode = 107i32,
    VREvent_LensDistortionChanged = 110i32,
    VREvent_LighthouseSectionSettingChanged = 858i32,
    VREvent_MCImageUpdated = 1000i32,
    VREvent_MessageOverlayCloseRequested = 1651i32,
    VREvent_MessageOverlay_Closed = 1650i32,
    VREvent_ModelSkinSettingsHaveChanged = 853i32,
    VREvent_MouseButtonDown = 301i32,
    VREvent_MouseButtonUp = 302i32,
    VREvent_MouseMove = 300i32,
    VREvent_None = 0i32,
    VREvent_Notification_BeginInteraction = 602i32,
    VREvent_Notification_Destroyed = 603i32,
    VREvent_Notification_Hidden = 601i32,
    VREvent_Notification_Shown = 600i32,
    VREvent_NotificationsSectionSettingChanged = 861i32,
    VREvent_NullSectionSettingChanged = 859i32,
    VREvent_OverlayFocusChanged = 307i32,
    VREvent_OverlayGamepadFocusGained = 511i32,
    VREvent_OverlayGamepadFocusLost = 512i32,
    VREvent_OverlayHidden = 501i32,
    VREvent_OverlaySharedTextureChanged = 513i32,
    VREvent_OverlayShown = 500i32,
    VREvent_PerfSectionSettingChanged = 863i32,
    VREvent_PerformanceTest_DisableCapture = 1601i32,
    VREvent_PerformanceTest_EnableCapture = 1600i32,
    VREvent_PerformanceTest_FidelityLevel = 1602i32,
    VREvent_PowerSettingsHaveChanged = 855i32,
    VREvent_PrimaryDashboardDeviceChanged = 525i32,
    VREvent_ProcessConnected = 1306i32,
    VREvent_ProcessDisconnected = 1307i32,
    VREvent_ProcessQuit = 701i32,
    VREvent_PropertyChanged = 111i32,
    VREvent_Quit = 700i32,
    VREvent_QuitAborted_UserPrompt = 702i32,
    VREvent_QuitAcknowledged = 703i32,
    VREvent_RenderToast = 507i32,
    VREvent_ReprojectionSettingHasChanged = 852i32,
    VREvent_RequestScreenshot = 520i32,
    VREvent_ResetDashboard = 506i32,
    VREvent_RoomViewHidden = 527i32,
    VREvent_RoomViewShown = 526i32,
    VREvent_SceneApplicationChanged = 404i32,
    VREvent_SceneApplicationSecondaryRenderingStarted = 407i32,
    VREvent_SceneApplicationUsingWrongGraphicsAdapter = 408i32,
    VREvent_SceneFocusChanged = 405i32,
    VREvent_SceneFocusGained = 403i32,
    VREvent_SceneFocusLost = 402i32,
    VREvent_ScreenshotFailed = 522i32,
    VREvent_ScreenshotProgressToDashboard = 524i32,
    VREvent_ScreenshotTaken = 521i32,
    VREvent_ScreenshotTriggered = 516i32,
    VREvent_Scroll = 305i32,
    VREvent_SeatedZeroPoseReset = 804i32,
    VREvent_ShowKeyboard = 509i32,
    VREvent_ShowRenderModels = 411i32,
    VREvent_SpatialAnchors_DescriptorUpdated = 1801i32,
    VREvent_SpatialAnchors_PoseUpdated = 1800i32,
    VREvent_SpatialAnchors_RequestDescriptorUpdate = 1803i32,
    VREvent_SpatialAnchors_RequestPoseUpdate = 1802i32,
    VREvent_StatusUpdate = 900i32,
    VREvent_SteamVRSectionSettingChanged = 857i32,
    VREvent_SubmitScreenshotToDashboard = 523i32,
    VREvent_SwitchGamepadFocus = 519i32,
    VREvent_TouchPadMove = 306i32,
    VREvent_TrackedCamera_EditingSurface = 1550i32,
    VREvent_TrackedCamera_PauseVideoStream = 1502i32,
    VREvent_TrackedCamera_ResumeVideoStream = 1503i32,
    VREvent_TrackedCamera_StartVideoStream = 1500i32,
    VREvent_TrackedCamera_StopVideoStream = 1501i32,
    VREvent_TrackedDeviceActivated = 100i32,
    VREvent_TrackedDeviceDeactivated = 101i32,
    VREvent_TrackedDeviceRoleChanged = 108i32,
    VREvent_TrackedDeviceUpdated = 102i32,
    VREvent_TrackedDeviceUserInteractionEnded = 104i32,
    VREvent_TrackedDeviceUserInteractionStarted = 103i32,
    VREvent_TrackersSectionSettingChanged = 866i32,
    VREvent_UserInterfaceSectionSettingChanged = 860i32,
    VREvent_VendorSpecific_Reserved_End = 19999i32,
    VREvent_VendorSpecific_Reserved_Start = 10000i32,
    VREvent_WatchdogWakeUpRequested = 109i32,
    VREvent_WebInterfaceSectionSettingChanged = 865i32,
    VREvent_WebInterface_InstallDriverCompleted = 950i32,
    VREvent_WirelessDisconnect = 112i32,
    VREvent_WirelessReconnect = 113i32,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVREventType")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::EVREventType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "EVREventType";
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
#[cfg(feature = "cordl_class_OVR+OpenVR+EVREventType")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::EVREventType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+EVREventType")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::EVREventType {
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
#[cfg(feature = "cordl_class_OVR+OpenVR+EVREventType")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::EVREventType {
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
#[cfg(feature = "cordl_class_OVR+OpenVR+EVREventType")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::EVREventType {
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
