#[cfg(feature = "cordl_class_OVR+OpenVR+ETrackedDeviceProperty")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ETrackedDeviceProperty {
    #[default]
    Prop_AllWirelessDongleDescriptions_String = 1008i32,
    Prop_AttachedDeviceId_String = 3000i32,
    Prop_AudioBridgeFirmwareVersion_Uint64 = 2061i32,
    Prop_AudioFirmwareVersion_Uint64 = 2032i32,
    Prop_Axis0Type_Int32 = 3002i32,
    Prop_Axis1Type_Int32 = 3003i32,
    Prop_Axis2Type_Int32 = 3004i32,
    Prop_Axis3Type_Int32 = 3005i32,
    Prop_Axis4Type_Int32 = 3006i32,
    Prop_BlockServerShutdown_Bool = 1023i32,
    Prop_CameraCompatibilityMode_Int32 = 2033i32,
    Prop_CameraFirmwareDescription_String = 2028i32,
    Prop_CameraFirmwareVersion_Uint64 = 2027i32,
    Prop_CameraFrameLayout_Int32 = 1040i32,
    Prop_CameraToHeadTransform_Matrix34 = 2016i32,
    Prop_CameraToHeadTransforms_Matrix34_Array = 2055i32,
    Prop_CanUnifyCoordinateSystemWithHmd_Bool = 1024i32,
    Prop_ConfigurationIncludesLighthouse20Features_Bool = 2069i32,
    Prop_ConnectedWirelessDongle_String = 1009i32,
    Prop_ContainsProximitySensor_Bool = 1025i32,
    Prop_ControllerRoleHint_Int32 = 3007i32,
    Prop_ControllerType_String = 7000i32,
    Prop_CurrentUniverseId_Uint64 = 2004i32,
    Prop_DeviceBatteryPercentage_Float = 1012i32,
    Prop_DeviceCanPowerOff_Bool = 1027i32,
    Prop_DeviceClass_Int32 = 1029i32,
    Prop_DeviceIsCharging_Bool = 1011i32,
    Prop_DeviceIsWireless_Bool = 1010i32,
    Prop_DeviceProvidesBatteryStatus_Bool = 1026i32,
    Prop_DisplayAllowNightMode_Bool = 2037i32,
    Prop_DisplayBootloaderVersion_Uint64 = 2030i32,
    Prop_DisplayDebugMode_Bool = 2044i32,
    Prop_DisplayFPGAVersion_Uint64 = 2029i32,
    Prop_DisplayFirmwareVersion_Uint64 = 2006i32,
    Prop_DisplayFrequency_Float = 2002i32,
    Prop_DisplayGCBlackClamp_Float = 2014i32,
    Prop_DisplayGCImage_String = 2021i32,
    Prop_DisplayGCOffset_Float = 2018i32,
    Prop_DisplayGCPrescale_Float = 2020i32,
    Prop_DisplayGCScale_Float = 2019i32,
    Prop_DisplayGCType_Int32 = 2017i32,
    Prop_DisplayHardwareVersion_Uint64 = 2031i32,
    Prop_DisplayHiddenArea_Binary_End = 5150i32,
    Prop_DisplayHiddenArea_Binary_Start = 5100i32,
    Prop_DisplayMCImageData_Binary = 2041i32,
    Prop_DisplayMCImageHeight_Int32 = 2039i32,
    Prop_DisplayMCImageLeft_String = 2012i32,
    Prop_DisplayMCImageNumChannels_Int32 = 2040i32,
    Prop_DisplayMCImageRight_String = 2013i32,
    Prop_DisplayMCImageWidth_Int32 = 2038i32,
    Prop_DisplayMCOffset_Float = 2009i32,
    Prop_DisplayMCScale_Float = 2010i32,
    Prop_DisplayMCType_Int32 = 2008i32,
    Prop_DisplaySuppressed_Bool = 2036i32,
    Prop_DistortionMeshResolution_Int32 = 2056i32,
    Prop_DoNotApplyPrediction_Bool = 2054i32,
    Prop_DongleVersion_Uint64 = 1022i32,
    Prop_DriverDirectModeSendsVsyncEvents_Bool = 2043i32,
    Prop_DriverIsDrawingControllers_Bool = 2057i32,
    Prop_DriverProvidedChaperonePath_String = 2048i32,
    Prop_DriverRequestedMuraCorrectionMode_Int32 = 2200i32,
    Prop_DriverRequestedMuraFeather_InnerBottom_Int32 = 2204i32,
    Prop_DriverRequestedMuraFeather_InnerLeft_Int32 = 2201i32,
    Prop_DriverRequestedMuraFeather_InnerRight_Int32 = 2202i32,
    Prop_DriverRequestedMuraFeather_InnerTop_Int32 = 2203i32,
    Prop_DriverRequestedMuraFeather_OuterBottom_Int32 = 2208i32,
    Prop_DriverRequestedMuraFeather_OuterLeft_Int32 = 2205i32,
    Prop_DriverRequestedMuraFeather_OuterRight_Int32 = 2206i32,
    Prop_DriverRequestedMuraFeather_OuterTop_Int32 = 2207i32,
    Prop_DriverRequestsApplicationPause_Bool = 2058i32,
    Prop_DriverRequestsReducedRendering_Bool = 2059i32,
    Prop_DriverVersion_String = 1031i32,
    Prop_EdidProductID_Int32 = 2015i32,
    Prop_EdidVendorID_Int32 = 2011i32,
    Prop_ExpectedControllerCount_Int32 = 2050i32,
    Prop_ExpectedTrackingReferenceCount_Int32 = 2049i32,
    Prop_FPGAVersion_Uint64 = 1019i32,
    Prop_FieldOfViewBottomDegrees_Float = 4003i32,
    Prop_FieldOfViewLeftDegrees_Float = 4000i32,
    Prop_FieldOfViewRightDegrees_Float = 4001i32,
    Prop_FieldOfViewTopDegrees_Float = 4002i32,
    Prop_FirmwareVersion_Uint64 = 1018i32,
    Prop_Firmware_ForceUpdateRequired_Bool = 1032i32,
    Prop_Firmware_ManualUpdateURL_String = 1016i32,
    Prop_Firmware_ManualUpdate_Bool = 1015i32,
    Prop_Firmware_ProgrammingTarget_String = 1028i32,
    Prop_Firmware_UpdateAvailable_Bool = 1014i32,
    Prop_GraphicsAdapterLuid_Uint64 = 2045i32,
    Prop_HardwareRevision_String = 1007i32,
    Prop_HardwareRevision_Uint64 = 1017i32,
    Prop_HasCameraComponent_Bool = 6004i32,
    Prop_HasCamera_Bool = 1030i32,
    Prop_HasControllerComponent_Bool = 6003i32,
    Prop_HasDisplayComponent_Bool = 6002i32,
    Prop_HasDriverDirectModeComponent_Bool = 6005i32,
    Prop_HasSpatialAnchorsSupport_Bool = 6007i32,
    Prop_HasVirtualDisplayComponent_Bool = 6006i32,
    Prop_IconPathName_String = 5000i32,
    Prop_ImageBridgeFirmwareVersion_Uint64 = 2062i32,
    Prop_ImuFactoryAccelerometerBias_Vector3 = 2066i32,
    Prop_ImuFactoryAccelerometerScale_Vector3 = 2067i32,
    Prop_ImuFactoryGyroBias_Vector3 = 2064i32,
    Prop_ImuFactoryGyroScale_Vector3 = 2065i32,
    Prop_ImuToHeadTransform_Matrix34 = 2063i32,
    Prop_InputProfilePath_String = 1037i32,
    Prop_InstallPath_String = 6001i32,
    Prop_Invalid = 0i32,
    Prop_IsOnDesktop_Bool = 2007i32,
    Prop_LegacyInputProfile_String = 7001i32,
    Prop_LensCenterLeftU_Float = 2022i32,
    Prop_LensCenterLeftV_Float = 2023i32,
    Prop_LensCenterRightU_Float = 2024i32,
    Prop_LensCenterRightV_Float = 2025i32,
    Prop_ManufacturerName_String = 1005i32,
    Prop_MinimumIpdStepMeters_Float = 2060i32,
    Prop_ModeLabel_String = 4006i32,
    Prop_ModelNumber_String = 1001i32,
    Prop_NamedIconPathControllerLeftDeviceOff_String = 2051i32,
    Prop_NamedIconPathControllerRightDeviceOff_String = 2052i32,
    Prop_NamedIconPathDeviceAlertLow_String = 5008i32,
    Prop_NamedIconPathDeviceNotReady_String = 5006i32,
    Prop_NamedIconPathDeviceOff_String = 5001i32,
    Prop_NamedIconPathDeviceReadyAlert_String = 5005i32,
    Prop_NamedIconPathDeviceReady_String = 5004i32,
    Prop_NamedIconPathDeviceSearchingAlert_String = 5003i32,
    Prop_NamedIconPathDeviceSearching_String = 5002i32,
    Prop_NamedIconPathDeviceStandby_String = 5007i32,
    Prop_NamedIconPathTrackingReferenceDeviceOff_String = 2053i32,
    Prop_NeverTracked_Bool = 1038i32,
    Prop_NumCameras_Int32 = 1039i32,
    Prop_ParentContainer = 5151i32,
    Prop_ParentDriver_Uint64 = 1034i32,
    Prop_PreviousUniverseId_Uint64 = 2005i32,
    Prop_RadioVersion_Uint64 = 1021i32,
    Prop_RegisteredDeviceType_String = 1036i32,
    Prop_RenderModelName_String = 1003i32,
    Prop_ReportsTimeSinceVSync_Bool = 2000i32,
    Prop_ResourceRoot_String = 1035i32,
    Prop_ScreenshotHorizontalFieldOfViewDegrees_Float = 2034i32,
    Prop_ScreenshotVerticalFieldOfViewDegrees_Float = 2035i32,
    Prop_SecondsFromPhotonsToVblank_Float = 2042i32,
    Prop_SecondsFromVsyncToPhotons_Float = 2001i32,
    Prop_SerialNumber_String = 1002i32,
    Prop_StatusDisplayTransform_Matrix34 = 1013i32,
    Prop_SupportedButtons_Uint64 = 3001i32,
    Prop_TrackedDeviceProperty_Max = 1000000i32,
    Prop_TrackingFirmwareVersion_String = 1006i32,
    Prop_TrackingRangeMaximumMeters_Float = 4005i32,
    Prop_TrackingRangeMinimumMeters_Float = 4004i32,
    Prop_TrackingSystemName_String = 1000i32,
    Prop_UserConfigPath_String = 6000i32,
    Prop_UserHeadToEyeDepthMeters_Float = 2026i32,
    Prop_UserIpdMeters_Float = 2003i32,
    Prop_VRCVersion_Uint64 = 1020i32,
    Prop_VendorSpecific_Reserved_End = 10999i32,
    Prop_VendorSpecific_Reserved_Start = 10000i32,
    Prop_ViveSystemButtonFixRequired_Bool = 1033i32,
    Prop_WillDriftInYaw_Bool = 1004i32,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+ETrackedDeviceProperty")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::ETrackedDeviceProperty {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "ETrackedDeviceProperty";
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
#[cfg(feature = "cordl_class_OVR+OpenVR+ETrackedDeviceProperty")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::ETrackedDeviceProperty {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+ETrackedDeviceProperty")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::ETrackedDeviceProperty {
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
#[cfg(feature = "cordl_class_OVR+OpenVR+ETrackedDeviceProperty")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::ETrackedDeviceProperty {
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
#[cfg(feature = "cordl_class_OVR+OpenVR+ETrackedDeviceProperty")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::ETrackedDeviceProperty {
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
