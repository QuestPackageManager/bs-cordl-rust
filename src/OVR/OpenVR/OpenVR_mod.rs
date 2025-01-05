#[cfg(feature = "OVR+OpenVR+OpenVR")]
#[repr(C)]
#[derive(Debug)]
pub struct OpenVR {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "OVR+OpenVR+OpenVR")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::OpenVR => "OVR.OpenVR"."OpenVR"
);
#[cfg(feature = "OVR+OpenVR+OpenVR")]
impl std::ops::Deref for crate::OVR::OpenVR::OpenVR {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+OpenVR")]
impl std::ops::DerefMut for crate::OVR::OpenVR::OpenVR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+OpenVR")]
impl crate::OVR::OpenVR::OpenVR {
    pub const FnTable_Prefix: &'static str = "FnTable:";
    pub const IVRApplications_Version: &'static str = "IVRApplications_006";
    pub const IVRChaperoneSetup_Version: &'static str = "IVRChaperoneSetup_005";
    pub const IVRChaperone_Version: &'static str = "IVRChaperone_003";
    pub const IVRCompositor_Version: &'static str = "IVRCompositor_022";
    pub const IVRDriverManager_Version: &'static str = "IVRDriverManager_001";
    pub const IVRExtendedDisplay_Version: &'static str = "IVRExtendedDisplay_001";
    pub const IVRIOBuffer_Version: &'static str = "IVRIOBuffer_001";
    pub const IVRInput_Version: &'static str = "IVRInput_004";
    pub const IVRNotifications_Version: &'static str = "IVRNotifications_002";
    pub const IVROverlay_Version: &'static str = "IVROverlay_018";
    pub const IVRRenderModels_Version: &'static str = "IVRRenderModels_006";
    pub const IVRResources_Version: &'static str = "IVRResources_001";
    pub const IVRScreenshots_Version: &'static str = "IVRScreenshots_001";
    pub const IVRSettings_Version: &'static str = "IVRSettings_002";
    pub const IVRSpatialAnchors_Version: &'static str = "IVRSpatialAnchors_001";
    pub const IVRSystem_Version: &'static str = "IVRSystem_019";
    pub const IVRTrackedCamera_Version: &'static str = "IVRTrackedCamera_003";
    pub const k_nDriverNone: u32 = 2147533055u32;
    pub const k_pch_App_ActionManifestURL_String: &'static str = "ActionManifestURL";
    pub const k_pch_App_BindingAutosaveURLSuffix_String: &'static str = "AutosaveURL";
    pub const k_pch_App_BindingCurrentURLSuffix_String: &'static str = "CurrentURL";
    pub const k_pch_App_NeedToUpdateAutosaveSuffix_Bool: &'static str = "NeedToUpdateAutosave";
    pub const k_pch_Camera_BoundsColorGammaA_Int32: &'static str = "cameraBoundsColorGammaA";
    pub const k_pch_Camera_BoundsColorGammaB_Int32: &'static str = "cameraBoundsColorGammaB";
    pub const k_pch_Camera_BoundsColorGammaG_Int32: &'static str = "cameraBoundsColorGammaG";
    pub const k_pch_Camera_BoundsColorGammaR_Int32: &'static str = "cameraBoundsColorGammaR";
    pub const k_pch_Camera_BoundsStrength_Int32: &'static str = "cameraBoundsStrength";
    pub const k_pch_Camera_EnableCameraForCollisionBounds_Bool: &'static str = "enableCameraForCollisionBounds";
    pub const k_pch_Camera_EnableCameraForRoomView_Bool: &'static str = "enableCameraForRoomView";
    pub const k_pch_Camera_EnableCameraInDashboard_Bool: &'static str = "enableCameraInDashboard";
    pub const k_pch_Camera_EnableCamera_Bool: &'static str = "enableCamera";
    pub const k_pch_Camera_RoomViewMode_Int32: &'static str = "cameraRoomViewMode";
    pub const k_pch_Camera_Section: &'static str = "camera";
    pub const k_pch_CollisionBounds_CenterMarkerOn_Bool: &'static str = "CollisionBoundsCenterMarkerOn";
    pub const k_pch_CollisionBounds_ColorGammaA_Int32: &'static str = "CollisionBoundsColorGammaA";
    pub const k_pch_CollisionBounds_ColorGammaB_Int32: &'static str = "CollisionBoundsColorGammaB";
    pub const k_pch_CollisionBounds_ColorGammaG_Int32: &'static str = "CollisionBoundsColorGammaG";
    pub const k_pch_CollisionBounds_ColorGammaR_Int32: &'static str = "CollisionBoundsColorGammaR";
    pub const k_pch_CollisionBounds_FadeDistance_Float: &'static str = "CollisionBoundsFadeDistance";
    pub const k_pch_CollisionBounds_GroundPerimeterOn_Bool: &'static str = "CollisionBoundsGroundPerimeterOn";
    pub const k_pch_CollisionBounds_PlaySpaceOn_Bool: &'static str = "CollisionBoundsPlaySpaceOn";
    pub const k_pch_CollisionBounds_Section: &'static str = "collisionBounds";
    pub const k_pch_CollisionBounds_Style_Int32: &'static str = "CollisionBoundsStyle";
    pub const k_pch_Controller_Component_Base: &'static str = "base";
    pub const k_pch_Controller_Component_GDC2015: &'static str = "gdc2015";
    pub const k_pch_Controller_Component_HandGrip: &'static str = "handgrip";
    pub const k_pch_Controller_Component_Status: &'static str = "status";
    pub const k_pch_Controller_Component_Tip: &'static str = "tip";
    pub const k_pch_Dashboard_ArcadeMode_Bool: &'static str = "arcadeMode";
    pub const k_pch_Dashboard_EnableDashboard_Bool: &'static str = "enableDashboard";
    pub const k_pch_Dashboard_EnableWebUI: &'static str = "webUI";
    pub const k_pch_Dashboard_EnableWebUIDashboardReplacement: &'static str = "webUIDashboard";
    pub const k_pch_Dashboard_EnableWebUIDevTools: &'static str = "webUIDevTools";
    pub const k_pch_Dashboard_Section: &'static str = "dashboard";
    pub const k_pch_Driver_Enable_Bool: &'static str = "enable";
    pub const k_pch_Keyboard_OffsetLeftX: &'static str = "OffsetLeftX";
    pub const k_pch_Keyboard_OffsetRightX: &'static str = "OffsetRightX";
    pub const k_pch_Keyboard_OffsetY: &'static str = "OffsetY";
    pub const k_pch_Keyboard_ScaleX: &'static str = "ScaleX";
    pub const k_pch_Keyboard_ScaleY: &'static str = "ScaleY";
    pub const k_pch_Keyboard_Section: &'static str = "keyboard";
    pub const k_pch_Keyboard_Smoothing: &'static str = "Smoothing";
    pub const k_pch_Keyboard_TutorialCompletions: &'static str = "TutorialCompletions";
    pub const k_pch_Lighthouse_DBHistory_Bool: &'static str = "dbhistory";
    pub const k_pch_Lighthouse_DisableIMUExceptHMD_Bool: &'static str = "disableimuexcepthmd";
    pub const k_pch_Lighthouse_DisableIMU_Bool: &'static str = "disableimu";
    pub const k_pch_Lighthouse_DisambiguationDebug_Int32: &'static str = "disambiguationdebug";
    pub const k_pch_Lighthouse_EnableBluetooth_Bool: &'static str = "enableBluetooth";
    pub const k_pch_Lighthouse_PowerManagedBaseStations_String: &'static str = "PowerManagedBaseStations";
    pub const k_pch_Lighthouse_PrimaryBasestation_Int32: &'static str = "primarybasestation";
    pub const k_pch_Lighthouse_Section: &'static str = "driver_lighthouse";
    pub const k_pch_Lighthouse_UseDisambiguation_String: &'static str = "usedisambiguation";
    pub const k_pch_MimeType_GameTheater: &'static str = "vr/game_theater";
    pub const k_pch_MimeType_HomeApp: &'static str = "vr/home";
    pub const k_pch_Notifications_DoNotDisturb_Bool: &'static str = "DoNotDisturb";
    pub const k_pch_Notifications_Section: &'static str = "notifications";
    pub const k_pch_Null_DisplayFrequency_Float: &'static str = "displayFrequency";
    pub const k_pch_Null_ModelNumber_String: &'static str = "modelNumber";
    pub const k_pch_Null_RenderHeight_Int32: &'static str = "renderHeight";
    pub const k_pch_Null_RenderWidth_Int32: &'static str = "renderWidth";
    pub const k_pch_Null_SecondsFromVsyncToPhotons_Float: &'static str = "secondsFromVsyncToPhotons";
    pub const k_pch_Null_Section: &'static str = "driver_null";
    pub const k_pch_Null_SerialNumber_String: &'static str = "serialNumber";
    pub const k_pch_Null_WindowHeight_Int32: &'static str = "windowHeight";
    pub const k_pch_Null_WindowWidth_Int32: &'static str = "windowWidth";
    pub const k_pch_Null_WindowX_Int32: &'static str = "windowX";
    pub const k_pch_Null_WindowY_Int32: &'static str = "windowY";
    pub const k_pch_Perf_AllowTimingStore_Bool: &'static str = "allowTimingStore";
    pub const k_pch_Perf_HeuristicActive_Bool: &'static str = "heuristicActive";
    pub const k_pch_Perf_LinuxGPUProfiling_Bool: &'static str = "linuxGPUProfiling";
    pub const k_pch_Perf_NotifyInHMD_Bool: &'static str = "warnInHMD";
    pub const k_pch_Perf_NotifyOnlyOnce_Bool: &'static str = "warnOnlyOnce";
    pub const k_pch_Perf_SaveTimingsOnExit_Bool: &'static str = "saveTimingsOnExit";
    pub const k_pch_Perf_Section: &'static str = "perfcheck";
    pub const k_pch_Perf_TestData_Float: &'static str = "perfTestData";
    pub const k_pch_Power_AutoLaunchSteamVROnButtonPress: &'static str = "autoLaunchSteamVROnButtonPress";
    pub const k_pch_Power_PauseCompositorOnStandby_Bool: &'static str = "pauseCompositorOnStandby";
    pub const k_pch_Power_PowerOffOnExit_Bool: &'static str = "powerOffOnExit";
    pub const k_pch_Power_ReturnToWatchdogTimeout_Float: &'static str = "returnToWatchdogTimeout";
    pub const k_pch_Power_Section: &'static str = "power";
    pub const k_pch_Power_TurnOffControllersTimeout_Float: &'static str = "turnOffControllersTimeout";
    pub const k_pch_Power_TurnOffScreensTimeout_Float: &'static str = "turnOffScreensTimeout";
    pub const k_pch_SteamVR_ActivateMultipleDrivers_Bool: &'static str = "activateMultipleDrivers";
    pub const k_pch_SteamVR_AllowAsyncReprojection_Bool: &'static str = "allowAsyncReprojection";
    pub const k_pch_SteamVR_AllowDisplayLockedMode_Bool: &'static str = "allowDisplayLockedMode";
    pub const k_pch_SteamVR_AllowReprojection_Bool: &'static str = "allowInterleavedReprojection";
    pub const k_pch_SteamVR_AllowSupersampleFiltering_Bool: &'static str = "allowSupersampleFiltering";
    pub const k_pch_SteamVR_BackgroundCameraHeight_Float: &'static str = "backgroundCameraHeight";
    pub const k_pch_SteamVR_BackgroundDomeRadius_Float: &'static str = "backgroundDomeRadius";
    pub const k_pch_SteamVR_BackgroundUseDomeProjection_Bool: &'static str = "backgroundUseDomeProjection";
    pub const k_pch_SteamVR_Background_String: &'static str = "background";
    pub const k_pch_SteamVR_BaseStationPowerManagement_Bool: &'static str = "basestationPowerManagement";
    pub const k_pch_SteamVR_CycleBackgroundImageTimeSec_Int32: &'static str = "CycleBackgroundImageTimeSec";
    pub const k_pch_SteamVR_DebugInput: &'static str = "debugInput";
    pub const k_pch_SteamVR_DebugInputBinding: &'static str = "debugInputBinding";
    pub const k_pch_SteamVR_DebugProcessPipe_String: &'static str = "debugProcessPipe";
    pub const k_pch_SteamVR_DefaultMirrorView_Int32: &'static str = "defaultMirrorView";
    pub const k_pch_SteamVR_DirectModeEdidPid_Int32: &'static str = "directModeEdidPid";
    pub const k_pch_SteamVR_DirectModeEdidVid_Int32: &'static str = "directModeEdidVid";
    pub const k_pch_SteamVR_DirectMode_Bool: &'static str = "directMode";
    pub const k_pch_SteamVR_DisplayDebugX_Int32: &'static str = "displayDebugX";
    pub const k_pch_SteamVR_DisplayDebugY_Int32: &'static str = "displayDebugY";
    pub const k_pch_SteamVR_DisplayDebug_Bool: &'static str = "displayDebug";
    pub const k_pch_SteamVR_EnableHomeApp: &'static str = "enableHomeApp";
    pub const k_pch_SteamVR_EnableLinuxVulkanAsync_Bool: &'static str = "enableLinuxVulkanAsync";
    pub const k_pch_SteamVR_ForceFadeOnBadTracking_Bool: &'static str = "forceFadeOnBadTracking";
    pub const k_pch_SteamVR_ForceReprojection_Bool: &'static str = "forceReprojection";
    pub const k_pch_SteamVR_ForceWindows32bitVRMonitor: &'static str = "forceWindows32BitVRMonitor";
    pub const k_pch_SteamVR_ForcedDriverKey_String: &'static str = "forcedDriver";
    pub const k_pch_SteamVR_ForcedHmdKey_String: &'static str = "forcedHmd";
    pub const k_pch_SteamVR_GridColor_String: &'static str = "gridColor";
    pub const k_pch_SteamVR_HaveStartedTutorialForNativeChaperoneDriver_Bool: &'static str = "haveStartedTutorialForNativeChaperoneDriver";
    pub const k_pch_SteamVR_IPD_Float: &'static str = "ipd";
    pub const k_pch_SteamVR_InputBindingUIBlock: &'static str = "inputBindingUI";
    pub const k_pch_SteamVR_IpdOffset_Float: &'static str = "ipdOffset";
    pub const k_pch_SteamVR_LegacyInputRebinding: &'static str = "legacyInputRebinding";
    pub const k_pch_SteamVR_LogLevel_Int32: &'static str = "loglevel";
    pub const k_pch_SteamVR_MirrorViewGeometry_String: &'static str = "mirrorViewGeometry";
    pub const k_pch_SteamVR_NeverKillProcesses_Bool: &'static str = "neverKillProcesses";
    pub const k_pch_SteamVR_PlayAreaColor_String: &'static str = "playAreaColor";
    pub const k_pch_SteamVR_RenderCameraMode: &'static str = "renderCameraMode";
    pub const k_pch_SteamVR_RequireHmd_String: &'static str = "requireHmd";
    pub const k_pch_SteamVR_RetailDemo_Bool: &'static str = "retailDemo";
    pub const k_pch_SteamVR_Section: &'static str = "steamvr";
    pub const k_pch_SteamVR_SendSystemButtonToAllApps_Bool: &'static str = "sendSystemButtonToAllApps";
    pub const k_pch_SteamVR_ShowMirrorView_Bool: &'static str = "showMirrorView";
    pub const k_pch_SteamVR_ShowStage_Bool: &'static str = "showStage";
    pub const k_pch_SteamVR_SpeakersForwardYawOffsetDegrees_Float: &'static str = "speakersForwardYawOffsetDegrees";
    pub const k_pch_SteamVR_StartCompositorFromAppLaunch_Bool: &'static str = "startCompositorFromAppLaunch";
    pub const k_pch_SteamVR_StartDashboardFromAppLaunch_Bool: &'static str = "startDashboardFromAppLaunch";
    pub const k_pch_SteamVR_StartMonitorFromAppLaunch: &'static str = "startMonitorFromAppLaunch";
    pub const k_pch_SteamVR_StartOverlayAppsFromDashboard_Bool: &'static str = "startOverlayAppsFromDashboard";
    pub const k_pch_SteamVR_SupersampleManualOverride_Bool: &'static str = "supersampleManualOverride";
    pub const k_pch_SteamVR_SupersampleScale_Float: &'static str = "supersampleScale";
    pub const k_pch_SteamVR_UsingSpeakers_Bool: &'static str = "usingSpeakers";
    pub const k_pch_Trackers_Section: &'static str = "trackers";
    pub const k_pch_TrackingOverride_Section: &'static str = "TrackingOverrides";
    pub const k_pch_UserInterface_MinimizeToTray_Bool: &'static str = "MinimizeToTray";
    pub const k_pch_UserInterface_ScreenshotType_Int: &'static str = "screenshotType";
    pub const k_pch_UserInterface_Screenshots_Bool: &'static str = "screenshots";
    pub const k_pch_UserInterface_Section: &'static str = "userinterface";
    pub const k_pch_UserInterface_StatusAlwaysOnTop_Bool: &'static str = "StatusAlwaysOnTop";
    pub const k_pch_WebInterface_Section: &'static str = "WebInterface";
    pub const k_pch_WebInterface_WebEnable_Bool: &'static str = "WebEnable";
    pub const k_pch_WebInterface_WebPort_String: &'static str = "WebPort";
    pub const k_pch_audio_OffPlaybackDevice_String: &'static str = "offPlaybackDevice";
    pub const k_pch_audio_OffRecordDevice_String: &'static str = "offRecordDevice";
    pub const k_pch_audio_OnPlaybackDevice_String: &'static str = "onPlaybackDevice";
    pub const k_pch_audio_OnPlaybackMirrorDevice_String: &'static str = "onPlaybackMirrorDevice";
    pub const k_pch_audio_OnRecordDevice_String: &'static str = "onRecordDevice";
    pub const k_pch_audio_Section: &'static str = "audio";
    pub const k_pch_audio_VIVEHDMIGain: &'static str = "viveHDMIGain";
    pub const k_pch_modelskin_Section: &'static str = "modelskins";
    pub const k_ulInvalidActionHandle: u64 = 0u64;
    pub const k_ulInvalidActionSetHandle: u64 = 0u64;
    pub const k_ulInvalidDriverHandle: u64 = 0u64;
    pub const k_ulInvalidIOBufferHandle: u64 = 0u64;
    pub const k_ulInvalidInputValueHandle: u64 = 0u64;
    pub const k_ulInvalidPropertyContainer: u64 = 0u64;
    pub const k_ulInvalidSpatialAnchorHandle: u32 = 1447635456u32;
    pub const k_ulOverlayHandleInvalid: u64 = 0u64;
    pub const k_unActionPropertyTag: u32 = 589439264u32;
    pub const k_unBoolPropertyTag: u32 = 353633540u32;
    pub const k_unControllerStateAxisCount: u32 = 5u32;
    pub const k_unFloatPropertyTag: u32 = 67305985u32;
    pub const k_unHapticVibrationPropertyTag: u32 = 2200445987u32;
    pub const k_unHiddenAreaPropertyTag: u32 = 555753246u32;
    pub const k_unHmdMatrix34PropertyTag: u32 = 387323156u32;
    pub const k_unHmdMatrix44PropertyTag: u32 = 504829461u32;
    pub const k_unHmdVector3PropertyTag: u32 = 522065686u32;
    pub const k_unHmdVector4PropertyTag: u32 = 538910231u32;
    pub const k_unInputValuePropertyTag: u32 = 606282273u32;
    pub const k_unInt32PropertyTag: u32 = 84148994u32;
    pub const k_unInvalidPropertyTag: u32 = 0u32;
    pub const k_unMaxActionNameLength: u32 = 403718208u32;
    pub const k_unMaxActionOriginCount: u32 = 1447630864u32;
    pub const k_unMaxActionSetNameLength: u32 = 1226313792u32;
    pub const k_unMaxApplicationKeyLength: u32 = 1980661888u32;
    pub const k_unMaxDriverDebugResponseSize: u32 = 8388800u32;
    pub const k_unMaxOverlayCount: u32 = 1226580032u32;
    pub const k_unMaxOverlayIntersectionMaskPrimitivesCount: u32 = 1447631904u32;
    pub const k_unMaxPropertyStringSize: u32 = 8388800u32;
    pub const k_unMaxSettingsKeyLength: u32 = 1226735744u32;
    pub const k_unMaxTrackedDeviceCount: u32 = 16776768u32;
    pub const k_unNotificationTextMaxSize: u32 = 1227358337u32;
    pub const k_unOpenVRInternalReserved_End: u32 = 12587175u32;
    pub const k_unOpenVRInternalReserved_Start: u32 = 279439491u32;
    pub const k_unPathHandleInfoTag: u32 = 572596255u32;
    pub const k_unScreenshotHandleInvalid: u32 = 1447631360u32;
    pub const k_unSkeletonPropertyTag: u32 = 3900909604u32;
    pub const k_unSpatialAnchorPosePropertyTag: u32 = 2817033000u32;
    pub const k_unStringPropertyTag: u32 = 370480133u32;
    pub const k_unTrackedDeviceIndexInvalid: u32 = 255u32;
    pub const k_unTrackedDeviceIndexOther: u32 = 65534u32;
    pub const k_unTrackedDeviceIndex_Hmd: u32 = 4294852608u32;
    pub const k_unUint64PropertyTag: u32 = 335873027u32;
    pub const k_unVROverlayMaxKeyLength: u32 = 2155905152u32;
    pub const k_unVROverlayMaxNameLength: u32 = 541098112u32;
    pub const k_unWildcardPropertyTag: u32 = 673456930u32;
    #[cfg(feature = "OVR+OpenVR+OpenVR+COpenVRContext")]
    pub type COpenVRContext = crate::OVR::OpenVR::OpenVR_COpenVRContext;
    pub fn GetGenericInterface(
        pchInterfaceVersion: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRInitError>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGenericInterface", (pchInterfaceVersion, peError))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInitToken() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInitToken", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringForHmdError(
        error: crate::OVR::OpenVR::EVRInitError,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStringForHmdError", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRInitError>,
        eApplicationType: crate::OVR::OpenVR::EVRApplicationType,
        pchStartupInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Init", (peError, eApplicationType, pchStartupInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitInternal(
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRInitError>,
        eApplicationType: crate::OVR::OpenVR::EVRApplicationType,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitInternal", (peError, eApplicationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitInternal2(
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRInitError>,
        eApplicationType: crate::OVR::OpenVR::EVRApplicationType,
        pchStartupInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitInternal2", (peError, eApplicationType, pchStartupInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHmdPresent() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHmdPresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInterfaceVersionValid(
        pchInterfaceVersion: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInterfaceVersionValid", (pchInterfaceVersion))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRuntimeInstalled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsRuntimeInstalled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Shutdown() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Shutdown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShutdownInternal() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShutdownInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Applications() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRApplications>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRApplications,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Applications", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Chaperone() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperone>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperone> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Chaperone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChaperoneSetup() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperoneSetup>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRChaperoneSetup,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ChaperoneSetup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Compositor() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRCompositor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRCompositor> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Compositor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExtendedDisplay() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRExtendedDisplay>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRExtendedDisplay,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ExtendedDisplay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Input() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRInput>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRInput> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Input", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OpenVRInternal_ModuleContext() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::OpenVR_COpenVRContext>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::OpenVR_COpenVRContext,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_OpenVRInternal_ModuleContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Overlay() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVROverlay>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVROverlay> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Overlay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RenderModels() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRRenderModels>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRRenderModels,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_RenderModels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Screenshots() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRScreenshots>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRScreenshots> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Screenshots", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Settings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSettings>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSettings> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Settings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SpatialAnchors() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSpatialAnchors>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRSpatialAnchors,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_SpatialAnchors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_System() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_System", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TrackedCamera() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRTrackedCamera>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRTrackedCamera,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_TrackedCamera", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_VRToken() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_VRToken", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_VRToken(
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_VRToken", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+OpenVR")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::OpenVR {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+OpenVR+COpenVRContext")]
#[repr(C)]
#[derive(Debug)]
pub struct OpenVR_COpenVRContext {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_pVRSystem: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem>,
    pub m_pVRChaperone: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperone>,
    pub m_pVRChaperoneSetup: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::CVRChaperoneSetup,
    >,
    pub m_pVRCompositor: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRCompositor>,
    pub m_pVROverlay: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVROverlay>,
    pub m_pVRRenderModels: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::CVRRenderModels,
    >,
    pub m_pVRExtendedDisplay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::CVRExtendedDisplay,
    >,
    pub m_pVRSettings: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSettings>,
    pub m_pVRApplications: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::CVRApplications,
    >,
    pub m_pVRScreenshots: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRScreenshots>,
    pub m_pVRTrackedCamera: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::CVRTrackedCamera,
    >,
    pub m_pVRInput: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRInput>,
    pub m_pVRSpatialAnchors: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::CVRSpatialAnchors,
    >,
}
#[cfg(feature = "OVR+OpenVR+OpenVR+COpenVRContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::OpenVR_COpenVRContext =>
    "OVR.OpenVR"."OpenVR/COpenVRContext"
);
#[cfg(feature = "OVR+OpenVR+OpenVR+COpenVRContext")]
impl std::ops::Deref for crate::OVR::OpenVR::OpenVR_COpenVRContext {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+OpenVR+COpenVRContext")]
impl std::ops::DerefMut for crate::OVR::OpenVR::OpenVR_COpenVRContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+OpenVR+COpenVRContext")]
impl crate::OVR::OpenVR::OpenVR_COpenVRContext {
    pub fn CheckClear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckClear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn VRApplications(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRApplications>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRApplications,
        > = __cordl_object.invoke("VRApplications", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VRChaperone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperone>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperone> = __cordl_object
            .invoke("VRChaperone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VRChaperoneSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperoneSetup>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRChaperoneSetup,
        > = __cordl_object.invoke("VRChaperoneSetup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VRCompositor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRCompositor>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRCompositor> = __cordl_object
            .invoke("VRCompositor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VRExtendedDisplay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRExtendedDisplay>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRExtendedDisplay,
        > = __cordl_object.invoke("VRExtendedDisplay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VRInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRInput>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRInput> = __cordl_object
            .invoke("VRInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VROverlay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVROverlay>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVROverlay> = __cordl_object
            .invoke("VROverlay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VRRenderModels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRRenderModels>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRRenderModels,
        > = __cordl_object.invoke("VRRenderModels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VRScreenshots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRScreenshots>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRScreenshots> = __cordl_object
            .invoke("VRScreenshots", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VRSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSettings> = __cordl_object
            .invoke("VRSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VRSpatialAnchors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSpatialAnchors>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRSpatialAnchors,
        > = __cordl_object.invoke("VRSpatialAnchors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VRSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem> = __cordl_object
            .invoke("VRSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VRTrackedCamera(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRTrackedCamera>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRTrackedCamera,
        > = __cordl_object.invoke("VRTrackedCamera", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+OpenVR+COpenVRContext")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::OpenVR_COpenVRContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
