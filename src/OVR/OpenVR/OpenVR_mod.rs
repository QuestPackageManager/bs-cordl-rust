#[cfg(feature = "OVR+OpenVR+OpenVR")]
#[repr(C)]
#[derive(Debug)]
pub struct OpenVR {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVR+OpenVR+OpenVR")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::OpenVR {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "OpenVR";
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
#[cfg(feature = "OVR+OpenVR+OpenVR")]
impl std::ops::Deref for crate::OVR::OpenVR::OpenVR {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+OpenVR")]
impl std::ops::DerefMut for crate::OVR::OpenVR::OpenVR {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
    pub const k_nDriverNone: u32 = 4294967295u32;
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
    pub const k_ulInvalidSpatialAnchorHandle: u32 = 0u32;
    pub const k_ulOverlayHandleInvalid: u64 = 0u64;
    pub const k_unActionPropertyTag: u32 = 32u32;
    pub const k_unBoolPropertyTag: u32 = 4u32;
    pub const k_unControllerStateAxisCount: u32 = 5u32;
    pub const k_unFloatPropertyTag: u32 = 1u32;
    pub const k_unHapticVibrationPropertyTag: u32 = 35u32;
    pub const k_unHiddenAreaPropertyTag: u32 = 30u32;
    pub const k_unHmdMatrix34PropertyTag: u32 = 20u32;
    pub const k_unHmdMatrix44PropertyTag: u32 = 21u32;
    pub const k_unHmdVector3PropertyTag: u32 = 22u32;
    pub const k_unHmdVector4PropertyTag: u32 = 23u32;
    pub const k_unInputValuePropertyTag: u32 = 33u32;
    pub const k_unInt32PropertyTag: u32 = 2u32;
    pub const k_unInvalidPropertyTag: u32 = 0u32;
    pub const k_unMaxActionNameLength: u32 = 64u32;
    pub const k_unMaxActionOriginCount: u32 = 16u32;
    pub const k_unMaxActionSetNameLength: u32 = 64u32;
    pub const k_unMaxApplicationKeyLength: u32 = 128u32;
    pub const k_unMaxDriverDebugResponseSize: u32 = 32768u32;
    pub const k_unMaxOverlayCount: u32 = 64u32;
    pub const k_unMaxOverlayIntersectionMaskPrimitivesCount: u32 = 32u32;
    pub const k_unMaxPropertyStringSize: u32 = 32768u32;
    pub const k_unMaxSettingsKeyLength: u32 = 128u32;
    pub const k_unMaxTrackedDeviceCount: u32 = 64u32;
    pub const k_unNotificationTextMaxSize: u32 = 256u32;
    pub const k_unOpenVRInternalReserved_End: u32 = 10000u32;
    pub const k_unOpenVRInternalReserved_Start: u32 = 1000u32;
    pub const k_unPathHandleInfoTag: u32 = 31u32;
    pub const k_unScreenshotHandleInvalid: u32 = 0u32;
    pub const k_unSkeletonPropertyTag: u32 = 36u32;
    pub const k_unSpatialAnchorPosePropertyTag: u32 = 40u32;
    pub const k_unStringPropertyTag: u32 = 5u32;
    pub const k_unTrackedDeviceIndexInvalid: u32 = 4294967295u32;
    pub const k_unTrackedDeviceIndexOther: u32 = 4294967294u32;
    pub const k_unTrackedDeviceIndex_Hmd: u32 = 0u32;
    pub const k_unUint64PropertyTag: u32 = 3u32;
    pub const k_unVROverlayMaxKeyLength: u32 = 128u32;
    pub const k_unVROverlayMaxNameLength: u32 = 128u32;
    pub const k_unWildcardPropertyTag: u32 = 34u32;
    #[cfg(feature = "OVR+OpenVR+OpenVR+COpenVRContext")]
    pub type COpenVRContext = crate::OVR::OpenVR::OpenVR_COpenVRContext;
    pub fn GetGenericInterface(
        pchInterfaceVersion: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRInitError>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::EVRInitError,
                            >,
                        ),
                        crate::System::IntPtr,
                        2usize,
                    >("GetGenericInterface")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGenericInterface", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (pchInterfaceVersion, peError))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInitToken() -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), u32, 0usize>("GetInitToken")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetInitToken", 0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetStringForHmdError(
        error: crate::OVR::OpenVR::EVRInitError,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::OVR::OpenVR::EVRInitError),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("GetStringForHmdError")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetStringForHmdError", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (error))? };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRInitError>,
        eApplicationType: crate::OVR::OpenVR::EVRApplicationType,
        pchStartupInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::EVRInitError,
                            >,
                            crate::OVR::OpenVR::EVRApplicationType,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem>,
                        3usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Init",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem> = unsafe {
            method.invoke_unchecked((), (peError, eApplicationType, pchStartupInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitInternal(
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRInitError>,
        eApplicationType: crate::OVR::OpenVR::EVRApplicationType,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::EVRInitError,
                            >,
                            crate::OVR::OpenVR::EVRApplicationType,
                        ),
                        u32,
                        2usize,
                    >("InitInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitInternal", 2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (peError, eApplicationType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitInternal2(
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRInitError>,
        eApplicationType: crate::OVR::OpenVR::EVRApplicationType,
        pchStartupInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::EVRInitError,
                            >,
                            crate::OVR::OpenVR::EVRApplicationType,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        u32,
                        3usize,
                    >("InitInternal2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitInternal2", 3usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            method.invoke_unchecked((), (peError, eApplicationType, pchStartupInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsHmdPresent() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("IsHmdPresent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsHmdPresent", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsInterfaceVersionValid(
        pchInterfaceVersion: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        bool,
                        1usize,
                    >("IsInterfaceVersionValid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsInterfaceVersionValid", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (pchInterfaceVersion))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsRuntimeInstalled() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("IsRuntimeInstalled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsRuntimeInstalled", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ())? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("Shutdown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Shutdown", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShutdownInternal() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ShutdownInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShutdownInternal", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
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
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Applications() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRApplications>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRApplications>,
                        0usize,
                    >("get_Applications")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Applications", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRApplications,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Chaperone() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperone>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperone>,
                        0usize,
                    >("get_Chaperone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Chaperone", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperone> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ChaperoneSetup() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperoneSetup>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperoneSetup>,
                        0usize,
                    >("get_ChaperoneSetup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ChaperoneSetup", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRChaperoneSetup,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Compositor() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRCompositor>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRCompositor>,
                        0usize,
                    >("get_Compositor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Compositor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRCompositor> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ExtendedDisplay() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRExtendedDisplay>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::OVR::OpenVR::CVRExtendedDisplay,
                        >,
                        0usize,
                    >("get_ExtendedDisplay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ExtendedDisplay", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRExtendedDisplay,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Input() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRInput>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRInput>,
                        0usize,
                    >("get_Input")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Input", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRInput> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_OpenVRInternal_ModuleContext() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::OpenVR_COpenVRContext>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::OVR::OpenVR::OpenVR_COpenVRContext,
                        >,
                        0usize,
                    >("get_OpenVRInternal_ModuleContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_OpenVRInternal_ModuleContext", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::OpenVR_COpenVRContext,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Overlay() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVROverlay>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVROverlay>,
                        0usize,
                    >("get_Overlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Overlay", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVROverlay> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_RenderModels() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRRenderModels>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRRenderModels>,
                        0usize,
                    >("get_RenderModels")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_RenderModels", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRRenderModels,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Screenshots() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRScreenshots>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRScreenshots>,
                        0usize,
                    >("get_Screenshots")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Screenshots", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRScreenshots> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Settings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSettings>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSettings>,
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSettings> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_SpatialAnchors() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSpatialAnchors>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSpatialAnchors>,
                        0usize,
                    >("get_SpatialAnchors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SpatialAnchors", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRSpatialAnchors,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_System() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem>,
                        0usize,
                    >("get_System")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_System", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_TrackedCamera() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRTrackedCamera>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRTrackedCamera>,
                        0usize,
                    >("get_TrackedCamera")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_TrackedCamera", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRTrackedCamera,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_VRToken() -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), u32, 0usize>("get_VRToken")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_VRToken", 0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_VRToken(
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_VRToken")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_VRToken", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))?
        };
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::OpenVR_COpenVRContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "OpenVR/COpenVRContext";
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
#[cfg(feature = "OVR+OpenVR+OpenVR+COpenVRContext")]
impl std::ops::Deref for crate::OVR::OpenVR::OpenVR_COpenVRContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+OpenVR+COpenVRContext")]
impl std::ops::DerefMut for crate::OVR::OpenVR::OpenVR_COpenVRContext {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+OpenVR+COpenVRContext")]
impl crate::OVR::OpenVR::OpenVR_COpenVRContext {
    pub fn CheckClear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CheckClear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckClear", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
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
    pub fn VRApplications(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRApplications>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRApplications>,
                        0usize,
                    >("VRApplications")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VRApplications", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRApplications,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn VRChaperone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperone>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperone>,
                        0usize,
                    >("VRChaperone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VRChaperone", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperone> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VRChaperoneSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperoneSetup>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRChaperoneSetup>,
                        0usize,
                    >("VRChaperoneSetup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VRChaperoneSetup", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRChaperoneSetup,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn VRCompositor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRCompositor>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRCompositor>,
                        0usize,
                    >("VRCompositor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VRCompositor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRCompositor> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VRExtendedDisplay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRExtendedDisplay>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::OVR::OpenVR::CVRExtendedDisplay,
                        >,
                        0usize,
                    >("VRExtendedDisplay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VRExtendedDisplay", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRExtendedDisplay,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn VRInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRInput>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRInput>,
                        0usize,
                    >("VRInput")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "VRInput",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRInput> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VROverlay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVROverlay>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVROverlay>,
                        0usize,
                    >("VROverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VROverlay", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVROverlay> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VRRenderModels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRRenderModels>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRRenderModels>,
                        0usize,
                    >("VRRenderModels")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VRRenderModels", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRRenderModels,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn VRScreenshots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRScreenshots>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRScreenshots>,
                        0usize,
                    >("VRScreenshots")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VRScreenshots", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRScreenshots> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VRSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSettings>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSettings>,
                        0usize,
                    >("VRSettings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VRSettings", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSettings> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VRSpatialAnchors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSpatialAnchors>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSpatialAnchors>,
                        0usize,
                    >("VRSpatialAnchors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VRSpatialAnchors", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRSpatialAnchors,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn VRSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem>,
                        0usize,
                    >("VRSystem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VRSystem", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRSystem> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VRTrackedCamera(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRTrackedCamera>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::OVR::OpenVR::CVRTrackedCamera>,
                        0usize,
                    >("VRTrackedCamera")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "VRTrackedCamera", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::OVR::OpenVR::CVRTrackedCamera,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
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
            method.invoke_unchecked(self, ())?
        };
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
