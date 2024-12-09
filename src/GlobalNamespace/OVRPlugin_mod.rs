#[cfg(feature = "OVRPlugin+Media+InputVideoBufferType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Media_OVRPlugin_InputVideoBufferType {
    EnumSize = 2147483647i32,
    Memory = 0i32,
    TextureHandle = 1i32,
}
#[cfg(feature = "OVRPlugin+Media+InputVideoBufferType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::Media_OVRPlugin_InputVideoBufferType => ""
    ."OVRPlugin/Media/InputVideoBufferType"
);
#[cfg(feature = "OVRPlugin+Media+MrcActivationMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Media_OVRPlugin_MrcActivationMode {
    Automatic = 0i32,
    Disabled = 1i32,
    EnumSize = 2147483647i32,
}
#[cfg(feature = "OVRPlugin+Media+MrcActivationMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::Media_OVRPlugin_MrcActivationMode => ""
    ."OVRPlugin/Media/MrcActivationMode"
);
#[cfg(feature = "OVRPlugin+Media+PlatformCameraMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Media_OVRPlugin_PlatformCameraMode {
    Disabled = -1i32,
    EnumSize = 2147483647i32,
    Initialized = 0i32,
    MobileMRC = 7i32,
    RemoteDroneControlled = 4i32,
    RemoteSpatialMapped = 5i32,
    SmartNavigated = 2i32,
    SpectatorMode = 6i32,
    StabilizedPoV = 3i32,
    UserControlled = 1i32,
}
#[cfg(feature = "OVRPlugin+Media+PlatformCameraMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::Media_OVRPlugin_PlatformCameraMode => ""
    ."OVRPlugin/Media/PlatformCameraMode"
);
#[cfg(feature = "OVRPlugin")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin => ""."OVRPlugin"
);
#[cfg(feature = "OVRPlugin")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin")]
impl crate::GlobalNamespace::OVRPlugin {
    pub const AppPerfFrameStatsMaxCount: i32 = 5i32;
    pub const EventDataBufferSize: i32 = 4000i32;
    pub const OverlayShapeFlagShift: i32 = 4i32;
    pub const RENDER_MODEL_NULL_KEY: i32 = 0i32;
    pub const SpaceFilterInfoComponentsMaxSize: i32 = 16i32;
    pub const SpaceFilterInfoIdsMaxSize: i32 = 1024i32;
    pub const SpatialEntityMaxQueryResultsPerEvent: i32 = 128i32;
    pub const isSupportedPlatform: bool = true;
    pub const pluginName: &'static str = "OVRPlugin";
    #[cfg(feature = "OVRPlugin+AppPerfFrameStats")]
    pub type AppPerfFrameStats = crate::GlobalNamespace::OVRPlugin_AppPerfFrameStats;
    #[cfg(feature = "OVRPlugin+AppPerfStats")]
    pub type AppPerfStats = crate::GlobalNamespace::OVRPlugin_AppPerfStats;
    #[cfg(feature = "OVRPlugin+BatteryStatus")]
    pub type BatteryStatus = crate::GlobalNamespace::OVRPlugin_BatteryStatus;
    #[cfg(feature = "OVRPlugin+BlendFactor")]
    pub type BlendFactor = crate::GlobalNamespace::OVRPlugin_BlendFactor;
    #[cfg(feature = "OVRPlugin+BodyJointLocation")]
    pub type BodyJointLocation = crate::GlobalNamespace::OVRPlugin_BodyJointLocation;
    #[cfg(feature = "OVRPlugin+BodyState")]
    pub type BodyState = crate::GlobalNamespace::OVRPlugin_BodyState;
    #[cfg(feature = "OVRPlugin+BodyStateInternal")]
    pub type BodyStateInternal = crate::GlobalNamespace::OVRPlugin_BodyStateInternal;
    #[cfg(feature = "OVRPlugin+Bone")]
    pub type Bone = crate::GlobalNamespace::OVRPlugin_Bone;
    #[cfg(feature = "OVRPlugin+BoneCapsule")]
    pub type BoneCapsule = crate::GlobalNamespace::OVRPlugin_BoneCapsule;
    #[cfg(feature = "OVRPlugin+BoneId")]
    pub type BoneId = crate::GlobalNamespace::OVRPlugin_BoneId;
    #[cfg(feature = "OVRPlugin+Bool")]
    pub type Bool = crate::GlobalNamespace::OVRPlugin_Bool;
    #[cfg(feature = "OVRPlugin+BoundaryGeometry")]
    pub type BoundaryGeometry = crate::GlobalNamespace::OVRPlugin_BoundaryGeometry;
    #[cfg(feature = "OVRPlugin+BoundaryTestResult")]
    pub type BoundaryTestResult = crate::GlobalNamespace::OVRPlugin_BoundaryTestResult;
    #[cfg(feature = "OVRPlugin+BoundaryType")]
    pub type BoundaryType = crate::GlobalNamespace::OVRPlugin_BoundaryType;
    #[cfg(feature = "OVRPlugin+Boundsf")]
    pub type Boundsf = crate::GlobalNamespace::OVRPlugin_Boundsf;
    #[cfg(feature = "OVRPlugin+CameraAnchorType")]
    pub type CameraAnchorType = crate::GlobalNamespace::OVRPlugin_CameraAnchorType;
    #[cfg(feature = "OVRPlugin+CameraDevice")]
    pub type CameraDevice = crate::GlobalNamespace::OVRPlugin_CameraDevice;
    #[cfg(feature = "OVRPlugin+CameraDeviceDepthQuality")]
    pub type CameraDeviceDepthQuality = crate::GlobalNamespace::OVRPlugin_CameraDeviceDepthQuality;
    #[cfg(feature = "OVRPlugin+CameraDeviceDepthSensingMode")]
    pub type CameraDeviceDepthSensingMode = crate::GlobalNamespace::OVRPlugin_CameraDeviceDepthSensingMode;
    #[cfg(feature = "OVRPlugin+CameraDeviceIntrinsicsParameters")]
    pub type CameraDeviceIntrinsicsParameters = crate::GlobalNamespace::OVRPlugin_CameraDeviceIntrinsicsParameters;
    #[cfg(feature = "OVRPlugin+CameraExtrinsics")]
    pub type CameraExtrinsics = crate::GlobalNamespace::OVRPlugin_CameraExtrinsics;
    #[cfg(feature = "OVRPlugin+CameraIntrinsics")]
    pub type CameraIntrinsics = crate::GlobalNamespace::OVRPlugin_CameraIntrinsics;
    #[cfg(feature = "OVRPlugin+CameraStatus")]
    pub type CameraStatus = crate::GlobalNamespace::OVRPlugin_CameraStatus;
    #[cfg(feature = "OVRPlugin+ColorSpace")]
    pub type ColorSpace = crate::GlobalNamespace::OVRPlugin_ColorSpace;
    #[cfg(feature = "OVRPlugin+Colorf")]
    pub type Colorf = crate::GlobalNamespace::OVRPlugin_Colorf;
    #[cfg(feature = "OVRPlugin+Controller")]
    pub type Controller = crate::GlobalNamespace::OVRPlugin_Controller;
    #[cfg(feature = "OVRPlugin+ControllerState")]
    pub type ControllerState = crate::GlobalNamespace::OVRPlugin_ControllerState;
    #[cfg(feature = "OVRPlugin+ControllerState2")]
    pub type ControllerState2 = crate::GlobalNamespace::OVRPlugin_ControllerState2;
    #[cfg(feature = "OVRPlugin+ControllerState4")]
    pub type ControllerState4 = crate::GlobalNamespace::OVRPlugin_ControllerState4;
    #[cfg(feature = "OVRPlugin+ControllerState5")]
    pub type ControllerState5 = crate::GlobalNamespace::OVRPlugin_ControllerState5;
    #[cfg(feature = "OVRPlugin+ControllerState6")]
    pub type ControllerState6 = crate::GlobalNamespace::OVRPlugin_ControllerState6;
    #[cfg(feature = "OVRPlugin+EventDataBuffer")]
    pub type EventDataBuffer = crate::GlobalNamespace::OVRPlugin_EventDataBuffer;
    #[cfg(feature = "OVRPlugin+EventType")]
    pub type EventType = crate::GlobalNamespace::OVRPlugin_EventType;
    #[cfg(feature = "OVRPlugin+Eye")]
    pub type Eye = crate::GlobalNamespace::OVRPlugin_Eye;
    #[cfg(feature = "OVRPlugin+EyeGazeState")]
    pub type EyeGazeState = crate::GlobalNamespace::OVRPlugin_EyeGazeState;
    #[cfg(feature = "OVRPlugin+EyeGazesState")]
    pub type EyeGazesState = crate::GlobalNamespace::OVRPlugin_EyeGazesState;
    #[cfg(feature = "OVRPlugin+EyeGazesStateInternal")]
    pub type EyeGazesStateInternal = crate::GlobalNamespace::OVRPlugin_EyeGazesStateInternal;
    #[cfg(feature = "OVRPlugin+EyeTextureFormat")]
    pub type EyeTextureFormat = crate::GlobalNamespace::OVRPlugin_EyeTextureFormat;
    #[cfg(feature = "OVRPlugin+FaceConstants")]
    pub type FaceConstants = crate::GlobalNamespace::OVRPlugin_FaceConstants;
    #[cfg(feature = "OVRPlugin+FaceExpression")]
    pub type FaceExpression = crate::GlobalNamespace::OVRPlugin_FaceExpression;
    #[cfg(feature = "OVRPlugin+FaceExpressionStatus")]
    pub type FaceExpressionStatus = crate::GlobalNamespace::OVRPlugin_FaceExpressionStatus;
    #[cfg(feature = "OVRPlugin+FaceExpressionStatusInternal")]
    pub type FaceExpressionStatusInternal = crate::GlobalNamespace::OVRPlugin_FaceExpressionStatusInternal;
    #[cfg(feature = "OVRPlugin+FaceRegionConfidence")]
    pub type FaceRegionConfidence = crate::GlobalNamespace::OVRPlugin_FaceRegionConfidence;
    #[cfg(feature = "OVRPlugin+FaceState")]
    pub type FaceState = crate::GlobalNamespace::OVRPlugin_FaceState;
    #[cfg(feature = "OVRPlugin+FaceState2Internal")]
    pub type FaceState2Internal = crate::GlobalNamespace::OVRPlugin_FaceState2Internal;
    #[cfg(feature = "OVRPlugin+FaceStateInternal")]
    pub type FaceStateInternal = crate::GlobalNamespace::OVRPlugin_FaceStateInternal;
    #[cfg(feature = "OVRPlugin+FeatureType")]
    pub type FeatureType = crate::GlobalNamespace::OVRPlugin_FeatureType;
    #[cfg(feature = "OVRPlugin+FixedFoveatedRenderingLevel")]
    pub type FixedFoveatedRenderingLevel = crate::GlobalNamespace::OVRPlugin_FixedFoveatedRenderingLevel;
    #[cfg(feature = "OVRPlugin+FoveatedRenderingLevel")]
    pub type FoveatedRenderingLevel = crate::GlobalNamespace::OVRPlugin_FoveatedRenderingLevel;
    #[cfg(feature = "OVRPlugin+Fovf")]
    pub type Fovf = crate::GlobalNamespace::OVRPlugin_Fovf;
    #[cfg(feature = "OVRPlugin+Frustumf")]
    pub type Frustumf = crate::GlobalNamespace::OVRPlugin_Frustumf;
    #[cfg(feature = "OVRPlugin+Frustumf2")]
    pub type Frustumf2 = crate::GlobalNamespace::OVRPlugin_Frustumf2;
    #[cfg(feature = "OVRPlugin+GUID")]
    pub type GUID = crate::GlobalNamespace::OVRPlugin_GUID;
    #[cfg(feature = "OVRPlugin+Hand")]
    pub type Hand = crate::GlobalNamespace::OVRPlugin_Hand;
    #[cfg(feature = "OVRPlugin+HandFinger")]
    pub type HandFinger = crate::GlobalNamespace::OVRPlugin_HandFinger;
    #[cfg(feature = "OVRPlugin+HandFingerPinch")]
    pub type HandFingerPinch = crate::GlobalNamespace::OVRPlugin_HandFingerPinch;
    #[cfg(feature = "OVRPlugin+HandState")]
    pub type HandState = crate::GlobalNamespace::OVRPlugin_HandState;
    #[cfg(feature = "OVRPlugin+HandStateInternal")]
    pub type HandStateInternal = crate::GlobalNamespace::OVRPlugin_HandStateInternal;
    #[cfg(feature = "OVRPlugin+HandStatus")]
    pub type HandStatus = crate::GlobalNamespace::OVRPlugin_HandStatus;
    #[cfg(feature = "OVRPlugin+Handedness")]
    pub type Handedness = crate::GlobalNamespace::OVRPlugin_Handedness;
    #[cfg(feature = "OVRPlugin+HapticsAmplitudeEnvelopeVibration")]
    pub type HapticsAmplitudeEnvelopeVibration = crate::GlobalNamespace::OVRPlugin_HapticsAmplitudeEnvelopeVibration;
    #[cfg(feature = "OVRPlugin+HapticsBuffer")]
    pub type HapticsBuffer = crate::GlobalNamespace::OVRPlugin_HapticsBuffer;
    #[cfg(feature = "OVRPlugin+HapticsConstants")]
    pub type HapticsConstants = crate::GlobalNamespace::OVRPlugin_HapticsConstants;
    #[cfg(feature = "OVRPlugin+HapticsDesc")]
    pub type HapticsDesc = crate::GlobalNamespace::OVRPlugin_HapticsDesc;
    #[cfg(feature = "OVRPlugin+HapticsLocation")]
    pub type HapticsLocation = crate::GlobalNamespace::OVRPlugin_HapticsLocation;
    #[cfg(feature = "OVRPlugin+HapticsPcmVibration")]
    pub type HapticsPcmVibration = crate::GlobalNamespace::OVRPlugin_HapticsPcmVibration;
    #[cfg(feature = "OVRPlugin+HapticsState")]
    pub type HapticsState = crate::GlobalNamespace::OVRPlugin_HapticsState;
    #[cfg(feature = "OVRPlugin+InsightPassthroughColorMapType")]
    pub type InsightPassthroughColorMapType = crate::GlobalNamespace::OVRPlugin_InsightPassthroughColorMapType;
    #[cfg(feature = "OVRPlugin+InsightPassthroughKeyboardHandsIntensity")]
    pub type InsightPassthroughKeyboardHandsIntensity = crate::GlobalNamespace::OVRPlugin_InsightPassthroughKeyboardHandsIntensity;
    #[cfg(feature = "OVRPlugin+InsightPassthroughStyle")]
    pub type InsightPassthroughStyle = crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle;
    #[cfg(feature = "OVRPlugin+InsightPassthroughStyle2")]
    pub type InsightPassthroughStyle2 = crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle2;
    #[cfg(feature = "OVRPlugin+InsightPassthroughStyleFlags")]
    pub type InsightPassthroughStyleFlags = crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyleFlags;
    #[cfg(feature = "OVRPlugin+InteractionProfile")]
    pub type InteractionProfile = crate::GlobalNamespace::OVRPlugin_InteractionProfile;
    #[cfg(feature = "OVRPlugin+KeyboardDescription")]
    pub type KeyboardDescription = crate::GlobalNamespace::OVRPlugin_KeyboardDescription;
    #[cfg(feature = "OVRPlugin+KeyboardDescriptionConstants")]
    pub type KeyboardDescriptionConstants = crate::GlobalNamespace::OVRPlugin_KeyboardDescriptionConstants;
    #[cfg(feature = "OVRPlugin+KeyboardState")]
    pub type KeyboardState = crate::GlobalNamespace::OVRPlugin_KeyboardState;
    #[cfg(feature = "OVRPlugin+Ktx")]
    pub type Ktx = crate::GlobalNamespace::OVRPlugin_Ktx;
    #[cfg(feature = "OVRPlugin+LayerDesc")]
    pub type LayerDesc = crate::GlobalNamespace::OVRPlugin_LayerDesc;
    #[cfg(feature = "OVRPlugin+LayerDescInternal")]
    pub type LayerDescInternal = crate::GlobalNamespace::OVRPlugin_LayerDescInternal;
    #[cfg(feature = "OVRPlugin+LayerFlags")]
    pub type LayerFlags = crate::GlobalNamespace::OVRPlugin_LayerFlags;
    #[cfg(feature = "OVRPlugin+LayerLayout")]
    pub type LayerLayout = crate::GlobalNamespace::OVRPlugin_LayerLayout;
    #[cfg(feature = "OVRPlugin+LayerSharpenType")]
    pub type LayerSharpenType = crate::GlobalNamespace::OVRPlugin_LayerSharpenType;
    #[cfg(feature = "OVRPlugin+LayerSubmit")]
    pub type LayerSubmit = crate::GlobalNamespace::OVRPlugin_LayerSubmit;
    #[cfg(feature = "OVRPlugin+LayerSuperSamplingType")]
    pub type LayerSuperSamplingType = crate::GlobalNamespace::OVRPlugin_LayerSuperSamplingType;
    #[cfg(feature = "OVRPlugin+LogCallback2DelegateType")]
    pub type LogCallback2DelegateType = crate::GlobalNamespace::OVRPlugin_LogCallback2DelegateType;
    #[cfg(feature = "OVRPlugin+LogLevel")]
    pub type LogLevel = crate::GlobalNamespace::OVRPlugin_LogLevel;
    #[cfg(feature = "OVRPlugin+Media")]
    pub type Media = crate::GlobalNamespace::OVRPlugin_Media;
    #[cfg(feature = "OVRPlugin+Mesh")]
    pub type Mesh = crate::GlobalNamespace::OVRPlugin_Mesh;
    #[cfg(feature = "OVRPlugin+MeshConstants")]
    pub type MeshConstants = crate::GlobalNamespace::OVRPlugin_MeshConstants;
    #[cfg(feature = "OVRPlugin+MeshType")]
    pub type MeshType = crate::GlobalNamespace::OVRPlugin_MeshType;
    #[cfg(feature = "OVRPlugin+Node")]
    pub type Node = crate::GlobalNamespace::OVRPlugin_Node;
    #[cfg(feature = "OVRPlugin+OVRP_0_1_0")]
    pub type OVRP_0_1_0 = crate::GlobalNamespace::OVRPlugin_OVRP_0_1_0;
    #[cfg(feature = "OVRPlugin+OVRP_0_1_1")]
    pub type OVRP_0_1_1 = crate::GlobalNamespace::OVRPlugin_OVRP_0_1_1;
    #[cfg(feature = "OVRPlugin+OVRP_0_1_2")]
    pub type OVRP_0_1_2 = crate::GlobalNamespace::OVRPlugin_OVRP_0_1_2;
    #[cfg(feature = "OVRPlugin+OVRP_0_1_3")]
    pub type OVRP_0_1_3 = crate::GlobalNamespace::OVRPlugin_OVRP_0_1_3;
    #[cfg(feature = "OVRPlugin+OVRP_0_5_0")]
    pub type OVRP_0_5_0 = crate::GlobalNamespace::OVRPlugin_OVRP_0_5_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_0_0")]
    pub type OVRP_1_0_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_0_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_10_0")]
    pub type OVRP_1_10_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_10_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_11_0")]
    pub type OVRP_1_11_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_11_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_12_0")]
    pub type OVRP_1_12_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_12_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_15_0")]
    pub type OVRP_1_15_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_15_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_16_0")]
    pub type OVRP_1_16_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_16_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_17_0")]
    pub type OVRP_1_17_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_17_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_18_0")]
    pub type OVRP_1_18_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_18_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_19_0")]
    pub type OVRP_1_19_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_19_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_1_0")]
    pub type OVRP_1_1_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_1_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_21_0")]
    pub type OVRP_1_21_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_21_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_28_0")]
    pub type OVRP_1_28_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_28_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_29_0")]
    pub type OVRP_1_29_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_29_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_2_0")]
    pub type OVRP_1_2_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_2_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_30_0")]
    pub type OVRP_1_30_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_30_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_31_0")]
    pub type OVRP_1_31_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_31_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_32_0")]
    pub type OVRP_1_32_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_32_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_34_0")]
    pub type OVRP_1_34_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_34_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_35_0")]
    pub type OVRP_1_35_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_35_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_36_0")]
    pub type OVRP_1_36_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_36_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_37_0")]
    pub type OVRP_1_37_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_37_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_38_0")]
    pub type OVRP_1_38_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_38_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_39_0")]
    pub type OVRP_1_39_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_39_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_3_0")]
    pub type OVRP_1_3_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_3_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_40_0")]
    pub type OVRP_1_40_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_40_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_41_0")]
    pub type OVRP_1_41_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_41_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_42_0")]
    pub type OVRP_1_42_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_42_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_43_0")]
    pub type OVRP_1_43_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_43_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_44_0")]
    pub type OVRP_1_44_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_44_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_45_0")]
    pub type OVRP_1_45_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_45_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_46_0")]
    pub type OVRP_1_46_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_46_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_47_0")]
    pub type OVRP_1_47_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_47_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_48_0")]
    pub type OVRP_1_48_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_48_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_49_0")]
    pub type OVRP_1_49_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_49_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_50_0")]
    pub type OVRP_1_50_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_50_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_51_0")]
    pub type OVRP_1_51_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_51_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_52_0")]
    pub type OVRP_1_52_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_52_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_53_0")]
    pub type OVRP_1_53_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_53_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_54_0")]
    pub type OVRP_1_54_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_54_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_55_0")]
    pub type OVRP_1_55_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_55_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_55_1")]
    pub type OVRP_1_55_1 = crate::GlobalNamespace::OVRPlugin_OVRP_1_55_1;
    #[cfg(feature = "OVRPlugin+OVRP_1_56_0")]
    pub type OVRP_1_56_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_56_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_57_0")]
    pub type OVRP_1_57_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_57_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_58_0")]
    pub type OVRP_1_58_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_58_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_59_0")]
    pub type OVRP_1_59_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_59_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_5_0")]
    pub type OVRP_1_5_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_5_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_60_0")]
    pub type OVRP_1_60_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_60_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_61_0")]
    pub type OVRP_1_61_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_61_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_62_0")]
    pub type OVRP_1_62_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_62_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_63_0")]
    pub type OVRP_1_63_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_63_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_64_0")]
    pub type OVRP_1_64_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_64_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_65_0")]
    pub type OVRP_1_65_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_65_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_66_0")]
    pub type OVRP_1_66_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_66_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_67_0")]
    pub type OVRP_1_67_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_67_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_68_0")]
    pub type OVRP_1_68_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_68_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_69_0")]
    pub type OVRP_1_69_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_69_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_6_0")]
    pub type OVRP_1_6_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_6_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_70_0")]
    pub type OVRP_1_70_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_70_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_71_0")]
    pub type OVRP_1_71_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_71_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_72_0")]
    pub type OVRP_1_72_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_72_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_73_0")]
    pub type OVRP_1_73_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_73_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_74_0")]
    pub type OVRP_1_74_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_74_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_75_0")]
    pub type OVRP_1_75_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_75_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_76_0")]
    pub type OVRP_1_76_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_76_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_78_0")]
    pub type OVRP_1_78_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_78_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_79_0")]
    pub type OVRP_1_79_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_79_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_7_0")]
    pub type OVRP_1_7_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_7_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_81_0")]
    pub type OVRP_1_81_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_81_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_82_0")]
    pub type OVRP_1_82_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_82_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_83_0")]
    pub type OVRP_1_83_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_83_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_84_0")]
    pub type OVRP_1_84_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_84_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_85_0")]
    pub type OVRP_1_85_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_85_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_86_0")]
    pub type OVRP_1_86_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_86_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_87_0")]
    pub type OVRP_1_87_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_87_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_88_0")]
    pub type OVRP_1_88_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_88_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_8_0")]
    pub type OVRP_1_8_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_8_0;
    #[cfg(feature = "OVRPlugin+OVRP_1_9_0")]
    pub type OVRP_1_9_0 = crate::GlobalNamespace::OVRPlugin_OVRP_1_9_0;
    #[cfg(feature = "OVRPlugin+OverlayFlag")]
    pub type OverlayFlag = crate::GlobalNamespace::OVRPlugin_OverlayFlag;
    #[cfg(feature = "OVRPlugin+OverlayShape")]
    pub type OverlayShape = crate::GlobalNamespace::OVRPlugin_OverlayShape;
    #[cfg(feature = "OVRPlugin+PassthroughCapabilities")]
    pub type PassthroughCapabilities = crate::GlobalNamespace::OVRPlugin_PassthroughCapabilities;
    #[cfg(feature = "OVRPlugin+PassthroughCapabilityFields")]
    pub type PassthroughCapabilityFields = crate::GlobalNamespace::OVRPlugin_PassthroughCapabilityFields;
    #[cfg(feature = "OVRPlugin+PassthroughCapabilityFlags")]
    pub type PassthroughCapabilityFlags = crate::GlobalNamespace::OVRPlugin_PassthroughCapabilityFlags;
    #[cfg(feature = "OVRPlugin+PassthroughColorLutChannels")]
    pub type PassthroughColorLutChannels = crate::GlobalNamespace::OVRPlugin_PassthroughColorLutChannels;
    #[cfg(feature = "OVRPlugin+PassthroughColorLutData")]
    pub type PassthroughColorLutData = crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData;
    #[cfg(feature = "OVRPlugin+PassthroughPreferenceFields")]
    pub type PassthroughPreferenceFields = crate::GlobalNamespace::OVRPlugin_PassthroughPreferenceFields;
    #[cfg(feature = "OVRPlugin+PassthroughPreferenceFlags")]
    pub type PassthroughPreferenceFlags = crate::GlobalNamespace::OVRPlugin_PassthroughPreferenceFlags;
    #[cfg(feature = "OVRPlugin+PassthroughPreferences")]
    pub type PassthroughPreferences = crate::GlobalNamespace::OVRPlugin_PassthroughPreferences;
    #[cfg(feature = "OVRPlugin+PerfMetrics")]
    pub type PerfMetrics = crate::GlobalNamespace::OVRPlugin_PerfMetrics;
    #[cfg(feature = "OVRPlugin+PinnedArray_1")]
    pub type PinnedArray_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::OVRPlugin_PinnedArray_1<
        T,
    >;
    #[cfg(feature = "OVRPlugin+PlatformUI")]
    pub type PlatformUI = crate::GlobalNamespace::OVRPlugin_PlatformUI;
    #[cfg(feature = "OVRPlugin+PolygonalBoundary2DInternal")]
    pub type PolygonalBoundary2DInternal = crate::GlobalNamespace::OVRPlugin_PolygonalBoundary2DInternal;
    #[cfg(feature = "OVRPlugin+PoseStatef")]
    pub type PoseStatef = crate::GlobalNamespace::OVRPlugin_PoseStatef;
    #[cfg(feature = "OVRPlugin+Posef")]
    pub type Posef = crate::GlobalNamespace::OVRPlugin_Posef;
    #[cfg(feature = "OVRPlugin+ProcessorPerformanceLevel")]
    pub type ProcessorPerformanceLevel = crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel;
    #[cfg(feature = "OVRPlugin+Qpl")]
    pub type Qpl = crate::GlobalNamespace::OVRPlugin_Qpl;
    #[cfg(feature = "OVRPlugin+Quatf")]
    pub type Quatf = crate::GlobalNamespace::OVRPlugin_Quatf;
    #[cfg(feature = "OVRPlugin+RecenterFlags")]
    pub type RecenterFlags = crate::GlobalNamespace::OVRPlugin_RecenterFlags;
    #[cfg(feature = "OVRPlugin+Rectf")]
    pub type Rectf = crate::GlobalNamespace::OVRPlugin_Rectf;
    #[cfg(feature = "OVRPlugin+Recti")]
    pub type Recti = crate::GlobalNamespace::OVRPlugin_Recti;
    #[cfg(feature = "OVRPlugin+RenderModelFlags")]
    pub type RenderModelFlags = crate::GlobalNamespace::OVRPlugin_RenderModelFlags;
    #[cfg(feature = "OVRPlugin+RenderModelProperties")]
    pub type RenderModelProperties = crate::GlobalNamespace::OVRPlugin_RenderModelProperties;
    #[cfg(feature = "OVRPlugin+RenderModelPropertiesInternal")]
    pub type RenderModelPropertiesInternal = crate::GlobalNamespace::OVRPlugin_RenderModelPropertiesInternal;
    #[cfg(feature = "OVRPlugin+Result")]
    pub type Result = crate::GlobalNamespace::OVRPlugin_Result;
    #[cfg(feature = "OVRPlugin+RoomLayout")]
    pub type RoomLayout = crate::GlobalNamespace::OVRPlugin_RoomLayout;
    #[cfg(feature = "OVRPlugin+RoomLayoutInternal")]
    pub type RoomLayoutInternal = crate::GlobalNamespace::OVRPlugin_RoomLayoutInternal;
    #[cfg(feature = "OVRPlugin+SceneCaptureRequestInternal")]
    pub type SceneCaptureRequestInternal = crate::GlobalNamespace::OVRPlugin_SceneCaptureRequestInternal;
    #[cfg(feature = "OVRPlugin+Size3f")]
    pub type Size3f = crate::GlobalNamespace::OVRPlugin_Size3f;
    #[cfg(feature = "OVRPlugin+Sizef")]
    pub type Sizef = crate::GlobalNamespace::OVRPlugin_Sizef;
    #[cfg(feature = "OVRPlugin+Sizei")]
    pub type Sizei = crate::GlobalNamespace::OVRPlugin_Sizei;
    #[cfg(feature = "OVRPlugin+Skeleton")]
    pub type Skeleton = crate::GlobalNamespace::OVRPlugin_Skeleton;
    #[cfg(feature = "OVRPlugin+Skeleton2")]
    pub type Skeleton2 = crate::GlobalNamespace::OVRPlugin_Skeleton2;
    #[cfg(feature = "OVRPlugin+Skeleton2Internal")]
    pub type Skeleton2Internal = crate::GlobalNamespace::OVRPlugin_Skeleton2Internal;
    #[cfg(feature = "OVRPlugin+SkeletonConstants")]
    pub type SkeletonConstants = crate::GlobalNamespace::OVRPlugin_SkeletonConstants;
    #[cfg(feature = "OVRPlugin+SkeletonType")]
    pub type SkeletonType = crate::GlobalNamespace::OVRPlugin_SkeletonType;
    #[cfg(feature = "OVRPlugin+SpaceComponentType")]
    pub type SpaceComponentType = crate::GlobalNamespace::OVRPlugin_SpaceComponentType;
    #[cfg(feature = "OVRPlugin+SpaceContainerInternal")]
    pub type SpaceContainerInternal = crate::GlobalNamespace::OVRPlugin_SpaceContainerInternal;
    #[cfg(feature = "OVRPlugin+SpaceFilterInfoComponents")]
    pub type SpaceFilterInfoComponents = crate::GlobalNamespace::OVRPlugin_SpaceFilterInfoComponents;
    #[cfg(feature = "OVRPlugin+SpaceFilterInfoIds")]
    pub type SpaceFilterInfoIds = crate::GlobalNamespace::OVRPlugin_SpaceFilterInfoIds;
    #[cfg(feature = "OVRPlugin+SpaceLocationFlags")]
    pub type SpaceLocationFlags = crate::GlobalNamespace::OVRPlugin_SpaceLocationFlags;
    #[cfg(feature = "OVRPlugin+SpaceLocationf")]
    pub type SpaceLocationf = crate::GlobalNamespace::OVRPlugin_SpaceLocationf;
    #[cfg(feature = "OVRPlugin+SpaceQueryActionType")]
    pub type SpaceQueryActionType = crate::GlobalNamespace::OVRPlugin_SpaceQueryActionType;
    #[cfg(feature = "OVRPlugin+SpaceQueryFilterType")]
    pub type SpaceQueryFilterType = crate::GlobalNamespace::OVRPlugin_SpaceQueryFilterType;
    #[cfg(feature = "OVRPlugin+SpaceQueryInfo")]
    pub type SpaceQueryInfo = crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo;
    #[cfg(feature = "OVRPlugin+SpaceQueryResult")]
    pub type SpaceQueryResult = crate::GlobalNamespace::OVRPlugin_SpaceQueryResult;
    #[cfg(feature = "OVRPlugin+SpaceQueryType")]
    pub type SpaceQueryType = crate::GlobalNamespace::OVRPlugin_SpaceQueryType;
    #[cfg(feature = "OVRPlugin+SpaceSemanticLabelInternal")]
    pub type SpaceSemanticLabelInternal = crate::GlobalNamespace::OVRPlugin_SpaceSemanticLabelInternal;
    #[cfg(feature = "OVRPlugin+SpaceStorageLocation")]
    pub type SpaceStorageLocation = crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation;
    #[cfg(feature = "OVRPlugin+SpaceStoragePersistenceMode")]
    pub type SpaceStoragePersistenceMode = crate::GlobalNamespace::OVRPlugin_SpaceStoragePersistenceMode;
    #[cfg(feature = "OVRPlugin+SpatialAnchorCreateInfo")]
    pub type SpatialAnchorCreateInfo = crate::GlobalNamespace::OVRPlugin_SpatialAnchorCreateInfo;
    #[cfg(feature = "OVRPlugin+Step")]
    pub type Step = crate::GlobalNamespace::OVRPlugin_Step;
    #[cfg(feature = "OVRPlugin+SystemHeadset")]
    pub type SystemHeadset = crate::GlobalNamespace::OVRPlugin_SystemHeadset;
    #[cfg(feature = "OVRPlugin+SystemRegion")]
    pub type SystemRegion = crate::GlobalNamespace::OVRPlugin_SystemRegion;
    #[cfg(feature = "OVRPlugin+TextureRectMatrixf")]
    pub type TextureRectMatrixf = crate::GlobalNamespace::OVRPlugin_TextureRectMatrixf;
    #[cfg(feature = "OVRPlugin+TiledMultiResLevel")]
    pub type TiledMultiResLevel = crate::GlobalNamespace::OVRPlugin_TiledMultiResLevel;
    #[cfg(feature = "OVRPlugin+TrackedKeyboardFlags")]
    pub type TrackedKeyboardFlags = crate::GlobalNamespace::OVRPlugin_TrackedKeyboardFlags;
    #[cfg(feature = "OVRPlugin+TrackedKeyboardPresentationStyles")]
    pub type TrackedKeyboardPresentationStyles = crate::GlobalNamespace::OVRPlugin_TrackedKeyboardPresentationStyles;
    #[cfg(feature = "OVRPlugin+TrackedKeyboardQueryFlags")]
    pub type TrackedKeyboardQueryFlags = crate::GlobalNamespace::OVRPlugin_TrackedKeyboardQueryFlags;
    #[cfg(feature = "OVRPlugin+Tracker")]
    pub type Tracker = crate::GlobalNamespace::OVRPlugin_Tracker;
    #[cfg(feature = "OVRPlugin+TrackingConfidence")]
    pub type TrackingConfidence = crate::GlobalNamespace::OVRPlugin_TrackingConfidence;
    #[cfg(feature = "OVRPlugin+TrackingOrigin")]
    pub type TrackingOrigin = crate::GlobalNamespace::OVRPlugin_TrackingOrigin;
    #[cfg(feature = "OVRPlugin+TriangleMeshInternal")]
    pub type TriangleMeshInternal = crate::GlobalNamespace::OVRPlugin_TriangleMeshInternal;
    #[cfg(feature = "OVRPlugin+UnityOpenXR")]
    pub type UnityOpenXR = crate::GlobalNamespace::OVRPlugin_UnityOpenXR;
    #[cfg(feature = "OVRPlugin+Vector2f")]
    pub type Vector2f = crate::GlobalNamespace::OVRPlugin_Vector2f;
    #[cfg(feature = "OVRPlugin+Vector2i")]
    pub type Vector2i = crate::GlobalNamespace::OVRPlugin_Vector2i;
    #[cfg(feature = "OVRPlugin+Vector3f")]
    pub type Vector3f = crate::GlobalNamespace::OVRPlugin_Vector3f;
    #[cfg(feature = "OVRPlugin+Vector4f")]
    pub type Vector4f = crate::GlobalNamespace::OVRPlugin_Vector4f;
    #[cfg(feature = "OVRPlugin+Vector4s")]
    pub type Vector4s = crate::GlobalNamespace::OVRPlugin_Vector4s;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardCreateInfo")]
    pub type VirtualKeyboardCreateInfo = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardCreateInfo;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardInputInfo")]
    pub type VirtualKeyboardInputInfo = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardInputInfo;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardInputSource")]
    pub type VirtualKeyboardInputSource = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardInputSource;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardInputStateFlags")]
    pub type VirtualKeyboardInputStateFlags = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardInputStateFlags;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardLocationInfo")]
    pub type VirtualKeyboardLocationInfo = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationInfo;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardLocationType")]
    pub type VirtualKeyboardLocationType = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationType;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationState")]
    pub type VirtualKeyboardModelAnimationState = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationState;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationStates")]
    pub type VirtualKeyboardModelAnimationStates = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationStates;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationStatesInternal")]
    pub type VirtualKeyboardModelAnimationStatesInternal = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationStatesInternal;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardModelVisibility")]
    pub type VirtualKeyboardModelVisibility = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelVisibility;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardSpaceCreateInfo")]
    pub type VirtualKeyboardSpaceCreateInfo = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardSpaceCreateInfo;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardTextureData")]
    pub type VirtualKeyboardTextureData = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureData;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardTextureIds")]
    pub type VirtualKeyboardTextureIds = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureIds;
    #[cfg(feature = "OVRPlugin+VirtualKeyboardTextureIdsInternal")]
    pub type VirtualKeyboardTextureIdsInternal = crate::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureIdsInternal;
    #[cfg(feature = "OVRPlugin+XrApi")]
    pub type XrApi = crate::GlobalNamespace::OVRPlugin_XrApi;
}
#[cfg(feature = "OVRPlugin")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+AppPerfFrameStats")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_AppPerfFrameStats {
    pub HmdVsyncIndex: i32,
    pub AppFrameIndex: i32,
    pub AppDroppedFrameCount: i32,
    pub AppMotionToPhotonLatency: f32,
    pub AppQueueAheadTime: f32,
    pub AppCpuElapsedTime: f32,
    pub AppGpuElapsedTime: f32,
    pub CompositorFrameIndex: i32,
    pub CompositorDroppedFrameCount: i32,
    pub CompositorLatency: f32,
    pub CompositorCpuElapsedTime: f32,
    pub CompositorGpuElapsedTime: f32,
    pub CompositorCpuStartToGpuEndElapsedTime: f32,
    pub CompositorGpuEndToVsyncElapsedTime: f32,
}
#[cfg(feature = "OVRPlugin+AppPerfFrameStats")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_AppPerfFrameStats =>
    ""."OVRPlugin/AppPerfFrameStats"
);
#[cfg(feature = "OVRPlugin+AppPerfFrameStats")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_AppPerfFrameStats {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+AppPerfFrameStats")]
impl crate::GlobalNamespace::OVRPlugin_AppPerfFrameStats {}
#[cfg(feature = "OVRPlugin+AppPerfStats")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_AppPerfStats {
    pub FrameStats: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_AppPerfFrameStats,
    >,
    pub FrameStatsCount: i32,
    pub AnyFrameStatsDropped: crate::GlobalNamespace::OVRPlugin_Bool,
    pub AdaptiveGpuPerformanceScale: f32,
}
#[cfg(feature = "OVRPlugin+AppPerfStats")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_AppPerfStats => ""
    ."OVRPlugin/AppPerfStats"
);
#[cfg(feature = "OVRPlugin+AppPerfStats")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_AppPerfStats {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+AppPerfStats")]
impl crate::GlobalNamespace::OVRPlugin_AppPerfStats {}
#[cfg(feature = "OVRPlugin+BatteryStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_BatteryStatus {
    Charging = 0i32,
    Discharging = 1i32,
    Full = 2i32,
    NotCharging = 3i32,
    Unknown = 4i32,
}
#[cfg(feature = "OVRPlugin+BatteryStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_BatteryStatus => ""
    ."OVRPlugin/BatteryStatus"
);
#[cfg(feature = "OVRPlugin+BlendFactor")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_BlendFactor {
    DstAlpha = 4i32,
    One = 1i32,
    OneMinusDstAlpha = 5i32,
    OneMinusSrcAlpha = 3i32,
    SrcAlpha = 2i32,
    Zero = 0i32,
}
#[cfg(feature = "OVRPlugin+BlendFactor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_BlendFactor => ""
    ."OVRPlugin/BlendFactor"
);
#[cfg(feature = "OVRPlugin+BodyJointLocation")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_BodyJointLocation {
    pub LocationFlags: crate::GlobalNamespace::OVRPlugin_SpaceLocationFlags,
    pub Pose: crate::GlobalNamespace::OVRPlugin_Posef,
}
#[cfg(feature = "OVRPlugin+BodyJointLocation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_BodyJointLocation =>
    ""."OVRPlugin/BodyJointLocation"
);
#[cfg(feature = "OVRPlugin+BodyJointLocation")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_BodyJointLocation {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+BodyJointLocation")]
impl crate::GlobalNamespace::OVRPlugin_BodyJointLocation {
    pub fn get_OrientationTracked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_OrientationTracked",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_OrientationValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_OrientationValid",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_PositionTracked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_PositionTracked",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_PositionValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_PositionValid",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+BodyState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_BodyState {
    pub JointLocations: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    >,
    pub Confidence: f32,
    pub SkeletonChangedCount: u32,
    pub Time: f64,
}
#[cfg(feature = "OVRPlugin+BodyState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_BodyState => ""
    ."OVRPlugin/BodyState"
);
#[cfg(feature = "OVRPlugin+BodyState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_BodyState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+BodyState")]
impl crate::GlobalNamespace::OVRPlugin_BodyState {}
#[cfg(feature = "OVRPlugin+BodyStateInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_BodyStateInternal {
    pub IsActive: crate::GlobalNamespace::OVRPlugin_Bool,
    pub Confidence: f32,
    pub SkeletonChangedCount: u32,
    pub Time: f64,
    pub JointLocation_0: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_1: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_2: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_3: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_4: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_5: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_6: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_7: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_8: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_9: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_10: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_11: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_12: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_13: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_14: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_15: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_16: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_17: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_18: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_19: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_20: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_21: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_22: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_23: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_24: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_25: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_26: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_27: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_28: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_29: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_30: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_31: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_32: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_33: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_34: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_35: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_36: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_37: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_38: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_39: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_40: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_41: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_42: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_43: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_44: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_45: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_46: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_47: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_48: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_49: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_50: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_51: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_52: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_53: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_54: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_55: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_56: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_57: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_58: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_59: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_60: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_61: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_62: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_63: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_64: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_65: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_66: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_67: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_68: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
    pub JointLocation_69: crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
}
#[cfg(feature = "OVRPlugin+BodyStateInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_BodyStateInternal =>
    ""."OVRPlugin/BodyStateInternal"
);
#[cfg(feature = "OVRPlugin+BodyStateInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_BodyStateInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+BodyStateInternal")]
impl crate::GlobalNamespace::OVRPlugin_BodyStateInternal {}
#[cfg(feature = "OVRPlugin+Bone")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Bone {
    pub Id: crate::GlobalNamespace::OVRPlugin_BoneId,
    pub ParentBoneIndex: i16,
    pub Pose: crate::GlobalNamespace::OVRPlugin_Posef,
}
#[cfg(feature = "OVRPlugin+Bone")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Bone => ""
    ."OVRPlugin/Bone"
);
#[cfg(feature = "OVRPlugin+Bone")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Bone {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Bone")]
impl crate::GlobalNamespace::OVRPlugin_Bone {}
#[cfg(feature = "OVRPlugin+BoneCapsule")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_BoneCapsule {
    pub BoneIndex: i16,
    pub StartPoint: crate::GlobalNamespace::OVRPlugin_Vector3f,
    pub EndPoint: crate::GlobalNamespace::OVRPlugin_Vector3f,
    pub Radius: f32,
}
#[cfg(feature = "OVRPlugin+BoneCapsule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_BoneCapsule => ""
    ."OVRPlugin/BoneCapsule"
);
#[cfg(feature = "OVRPlugin+BoneCapsule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_BoneCapsule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+BoneCapsule")]
impl crate::GlobalNamespace::OVRPlugin_BoneCapsule {}
#[cfg(feature = "OVRPlugin+BoneId")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_BoneId {
    Body_Chest = 5i32,
    Body_End = 70i32,
    Body_Head = 7i32,
    Body_Hips = 1i32,
    Body_LeftArmLower = 11i32,
    Body_LeftArmUpper = 10i32,
    Body_LeftHandIndexDistal = 27i32,
    Body_LeftHandIndexIntermediate = 26i32,
    Body_LeftHandIndexMetacarpal = 24i32,
    Body_LeftHandIndexProximal = 25i32,
    Body_LeftHandIndexTip = 28i32,
    Body_LeftHandLittleDistal = 42i32,
    Body_LeftHandLittleIntermediate = 41i32,
    Body_LeftHandLittleMetacarpal = 39i32,
    Body_LeftHandLittleProximal = 40i32,
    Body_LeftHandLittleTip = 43i32,
    Body_LeftHandMiddleDistal = 32i32,
    Body_LeftHandMiddleIntermediate = 31i32,
    Body_LeftHandMiddleMetacarpal = 29i32,
    Body_LeftHandMiddleProximal = 30i32,
    Body_LeftHandMiddleTip = 33i32,
    Body_LeftHandPalm = 18i32,
    Body_LeftHandRingDistal = 37i32,
    Body_LeftHandRingIntermediate = 36i32,
    Body_LeftHandRingMetacarpal = 34i32,
    Body_LeftHandRingProximal = 35i32,
    Body_LeftHandRingTip = 38i32,
    Body_LeftHandThumbDistal = 22i32,
    Body_LeftHandThumbMetacarpal = 20i32,
    Body_LeftHandThumbProximal = 21i32,
    Body_LeftHandThumbTip = 23i32,
    Body_LeftHandWrist = 19i32,
    Body_LeftHandWristTwist = 12i32,
    Body_LeftScapula = 9i32,
    Body_LeftShoulder = 8i32,
    Body_Neck = 6i32,
    Body_RightArmLower = 16i32,
    Body_RightArmUpper = 15i32,
    Body_RightHandIndexDistal = 53i32,
    Body_RightHandIndexIntermediate = 52i32,
    Body_RightHandIndexMetacarpal = 50i32,
    Body_RightHandIndexProximal = 51i32,
    Body_RightHandIndexTip = 54i32,
    Body_RightHandLittleDistal = 68i32,
    Body_RightHandLittleIntermediate = 67i32,
    Body_RightHandLittleMetacarpal = 65i32,
    Body_RightHandLittleProximal = 66i32,
    Body_RightHandLittleTip = 69i32,
    Body_RightHandMiddleDistal = 58i32,
    Body_RightHandMiddleIntermediate = 57i32,
    Body_RightHandMiddleMetacarpal = 55i32,
    Body_RightHandMiddleProximal = 56i32,
    Body_RightHandMiddleTip = 59i32,
    Body_RightHandPalm = 44i32,
    Body_RightHandRingDistal = 63i32,
    Body_RightHandRingIntermediate = 62i32,
    Body_RightHandRingMetacarpal = 60i32,
    Body_RightHandRingProximal = 61i32,
    Body_RightHandRingTip = 64i32,
    Body_RightHandThumbDistal = 48i32,
    Body_RightHandThumbMetacarpal = 46i32,
    Body_RightHandThumbProximal = 47i32,
    Body_RightHandThumbTip = 49i32,
    Body_RightHandWrist = 45i32,
    Body_RightHandWristTwist = 17i32,
    Body_RightScapula = 14i32,
    Body_RightShoulder = 13i32,
    Body_Root = 0i32,
    Body_SpineLower = 2i32,
    Body_SpineMiddle = 3i32,
    Body_SpineUpper = 4i32,
    Invalid = -1i32,
}
#[cfg(feature = "OVRPlugin+BoneId")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_BoneId => ""
    ."OVRPlugin/BoneId"
);
#[cfg(feature = "OVRPlugin+Bool")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_Bool {
    False = 0i32,
    True = 1i32,
}
#[cfg(feature = "OVRPlugin+Bool")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Bool => ""
    ."OVRPlugin/Bool"
);
#[cfg(feature = "OVRPlugin+BoundaryGeometry")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_BoundaryGeometry {
    pub BoundaryType: crate::GlobalNamespace::OVRPlugin_BoundaryType,
    pub Points: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Vector3f,
    >,
    pub PointsCount: i32,
}
#[cfg(feature = "OVRPlugin+BoundaryGeometry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_BoundaryGeometry =>
    ""."OVRPlugin/BoundaryGeometry"
);
#[cfg(feature = "OVRPlugin+BoundaryGeometry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_BoundaryGeometry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+BoundaryGeometry")]
impl crate::GlobalNamespace::OVRPlugin_BoundaryGeometry {}
#[cfg(feature = "OVRPlugin+BoundaryTestResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_BoundaryTestResult {
    pub IsTriggering: crate::GlobalNamespace::OVRPlugin_Bool,
    pub ClosestDistance: f32,
    pub ClosestPoint: crate::GlobalNamespace::OVRPlugin_Vector3f,
    pub ClosestPointNormal: crate::GlobalNamespace::OVRPlugin_Vector3f,
}
#[cfg(feature = "OVRPlugin+BoundaryTestResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_BoundaryTestResult =>
    ""."OVRPlugin/BoundaryTestResult"
);
#[cfg(feature = "OVRPlugin+BoundaryTestResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_BoundaryTestResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+BoundaryTestResult")]
impl crate::GlobalNamespace::OVRPlugin_BoundaryTestResult {}
#[cfg(feature = "OVRPlugin+BoundaryType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_BoundaryType {
    OuterBoundary = 1i32,
    PlayArea = 256i32,
}
#[cfg(feature = "OVRPlugin+BoundaryType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_BoundaryType => ""
    ."OVRPlugin/BoundaryType"
);
#[cfg(feature = "OVRPlugin+Boundsf")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Boundsf {
    pub Pos: crate::GlobalNamespace::OVRPlugin_Vector3f,
    pub Size: crate::GlobalNamespace::OVRPlugin_Size3f,
}
#[cfg(feature = "OVRPlugin+Boundsf")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Boundsf => ""
    ."OVRPlugin/Boundsf"
);
#[cfg(feature = "OVRPlugin+Boundsf")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Boundsf {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Boundsf")]
impl crate::GlobalNamespace::OVRPlugin_Boundsf {}
#[cfg(feature = "OVRPlugin+CameraAnchorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_CameraAnchorType {
    CameraAnchorType_Count = 2i32,
    CameraAnchorType_Custom = 1i32,
    CameraAnchorType_EnumSize = 2147483647i32,
    CameraAnchorType_PreDefined = 0i32,
}
#[cfg(feature = "OVRPlugin+CameraAnchorType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_CameraAnchorType =>
    ""."OVRPlugin/CameraAnchorType"
);
#[cfg(feature = "OVRPlugin+CameraDevice")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_CameraDevice {
    None = 0i32,
    WebCamera0 = 100i32,
    WebCamera1 = 101i32,
    ZEDCamera = 300i32,
}
#[cfg(feature = "OVRPlugin+CameraDevice")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_CameraDevice => ""
    ."OVRPlugin/CameraDevice"
);
#[cfg(feature = "OVRPlugin+CameraDeviceDepthQuality")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_CameraDeviceDepthQuality {
    High = 2i32,
    Low = 0i32,
    Medium = 1i32,
}
#[cfg(feature = "OVRPlugin+CameraDeviceDepthQuality")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_CameraDeviceDepthQuality => ""
    ."OVRPlugin/CameraDeviceDepthQuality"
);
#[cfg(feature = "OVRPlugin+CameraDeviceDepthSensingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_CameraDeviceDepthSensingMode {
    Fill = 1i32,
    Standard = 0i32,
}
#[cfg(feature = "OVRPlugin+CameraDeviceDepthSensingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_CameraDeviceDepthSensingMode => ""
    ."OVRPlugin/CameraDeviceDepthSensingMode"
);
#[cfg(feature = "OVRPlugin+CameraDeviceIntrinsicsParameters")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_CameraDeviceIntrinsicsParameters {
    pub fx: f32,
    pub fy: f32,
    pub cx: f32,
    pub cy: f32,
    pub disto0: f64,
    pub disto1: f64,
    pub disto2: f64,
    pub disto3: f64,
    pub disto4: f64,
    pub v_fov: f32,
    pub h_fov: f32,
    pub d_fov: f32,
    pub w: i32,
    pub h: i32,
}
#[cfg(feature = "OVRPlugin+CameraDeviceIntrinsicsParameters")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_CameraDeviceIntrinsicsParameters => ""
    ."OVRPlugin/CameraDeviceIntrinsicsParameters"
);
#[cfg(feature = "OVRPlugin+CameraDeviceIntrinsicsParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_CameraDeviceIntrinsicsParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+CameraDeviceIntrinsicsParameters")]
impl crate::GlobalNamespace::OVRPlugin_CameraDeviceIntrinsicsParameters {}
#[cfg(feature = "OVRPlugin+CameraExtrinsics")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_CameraExtrinsics {
    pub IsValid: crate::GlobalNamespace::OVRPlugin_Bool,
    pub LastChangedTimeSeconds: f64,
    pub CameraStatusData: crate::GlobalNamespace::OVRPlugin_CameraStatus,
    pub AttachedToNode: crate::GlobalNamespace::OVRPlugin_Node,
    pub RelativePose: crate::GlobalNamespace::OVRPlugin_Posef,
}
#[cfg(feature = "OVRPlugin+CameraExtrinsics")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_CameraExtrinsics =>
    ""."OVRPlugin/CameraExtrinsics"
);
#[cfg(feature = "OVRPlugin+CameraExtrinsics")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_CameraExtrinsics {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+CameraExtrinsics")]
impl crate::GlobalNamespace::OVRPlugin_CameraExtrinsics {}
#[cfg(feature = "OVRPlugin+CameraIntrinsics")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_CameraIntrinsics {
    pub IsValid: crate::GlobalNamespace::OVRPlugin_Bool,
    pub LastChangedTimeSeconds: f64,
    pub FOVPort: crate::GlobalNamespace::OVRPlugin_Fovf,
    pub VirtualNearPlaneDistanceMeters: f32,
    pub VirtualFarPlaneDistanceMeters: f32,
    pub ImageSensorPixelResolution: crate::GlobalNamespace::OVRPlugin_Sizei,
}
#[cfg(feature = "OVRPlugin+CameraIntrinsics")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_CameraIntrinsics =>
    ""."OVRPlugin/CameraIntrinsics"
);
#[cfg(feature = "OVRPlugin+CameraIntrinsics")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_CameraIntrinsics {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+CameraIntrinsics")]
impl crate::GlobalNamespace::OVRPlugin_CameraIntrinsics {}
#[cfg(feature = "OVRPlugin+CameraStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_CameraStatus {
    CameraStatus_Calibrated = 4i32,
    CameraStatus_Calibrating = 2i32,
    CameraStatus_CalibrationFailed = 3i32,
    CameraStatus_Connected = 1i32,
    CameraStatus_EnumSize = 2147483647i32,
    CameraStatus_None = 0i32,
    CameraStatus_ThirdPerson = 5i32,
}
#[cfg(feature = "OVRPlugin+CameraStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_CameraStatus => ""
    ."OVRPlugin/CameraStatus"
);
#[cfg(feature = "OVRPlugin+ColorSpace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_ColorSpace {
    Adobe_RGB = 8i32,
    P3 = 7i32,
    Quest = 6i32,
    Rec_2020 = 2i32,
    Rec_709 = 3i32,
    Rift_CV1 = 4i32,
    Rift_S = 5i32,
    Unknown = 0i32,
    Unmanaged = 1i32,
}
#[cfg(feature = "OVRPlugin+ColorSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_ColorSpace => ""
    ."OVRPlugin/ColorSpace"
);
#[cfg(feature = "OVRPlugin+Colorf")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Colorf {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[cfg(feature = "OVRPlugin+Colorf")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Colorf => ""
    ."OVRPlugin/Colorf"
);
#[cfg(feature = "OVRPlugin+Colorf")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Colorf {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Colorf")]
impl crate::GlobalNamespace::OVRPlugin_Colorf {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+Controller")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_Controller {
    Active = -2147483648i32,
    All = -1i32,
    Gamepad = 16i32,
    Hands = 96i32,
    LHand = 32i32,
    LTouch = 1i32,
    None = 0i32,
    RHand = 64i32,
    RTouch = 2i32,
    Remote = 4i32,
    Touch = 3i32,
}
#[cfg(feature = "OVRPlugin+Controller")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Controller => ""
    ."OVRPlugin/Controller"
);
#[cfg(feature = "OVRPlugin+ControllerState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_ControllerState {
    pub ConnectedControllers: u32,
    pub Buttons: u32,
    pub Touches: u32,
    pub NearTouches: u32,
    pub LIndexTrigger: f32,
    pub RIndexTrigger: f32,
    pub LHandTrigger: f32,
    pub RHandTrigger: f32,
    pub LThumbstick: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub RThumbstick: crate::GlobalNamespace::OVRPlugin_Vector2f,
}
#[cfg(feature = "OVRPlugin+ControllerState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_ControllerState => ""
    ."OVRPlugin/ControllerState"
);
#[cfg(feature = "OVRPlugin+ControllerState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_ControllerState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+ControllerState")]
impl crate::GlobalNamespace::OVRPlugin_ControllerState {}
#[cfg(feature = "OVRPlugin+ControllerState2")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_ControllerState2 {
    pub ConnectedControllers: u32,
    pub Buttons: u32,
    pub Touches: u32,
    pub NearTouches: u32,
    pub LIndexTrigger: f32,
    pub RIndexTrigger: f32,
    pub LHandTrigger: f32,
    pub RHandTrigger: f32,
    pub LThumbstick: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub RThumbstick: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub LTouchpad: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub RTouchpad: crate::GlobalNamespace::OVRPlugin_Vector2f,
}
#[cfg(feature = "OVRPlugin+ControllerState2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_ControllerState2 =>
    ""."OVRPlugin/ControllerState2"
);
#[cfg(feature = "OVRPlugin+ControllerState2")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_ControllerState2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+ControllerState2")]
impl crate::GlobalNamespace::OVRPlugin_ControllerState2 {
    pub fn _ctor(
        &mut self,
        cs: crate::GlobalNamespace::OVRPlugin_ControllerState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (cs),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+ControllerState4")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_ControllerState4 {
    pub ConnectedControllers: u32,
    pub Buttons: u32,
    pub Touches: u32,
    pub NearTouches: u32,
    pub LIndexTrigger: f32,
    pub RIndexTrigger: f32,
    pub LHandTrigger: f32,
    pub RHandTrigger: f32,
    pub LThumbstick: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub RThumbstick: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub LTouchpad: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub RTouchpad: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub LBatteryPercentRemaining: u8,
    pub RBatteryPercentRemaining: u8,
    pub LRecenterCount: u8,
    pub RRecenterCount: u8,
    pub Reserved_27: u8,
    pub Reserved_26: u8,
    pub Reserved_25: u8,
    pub Reserved_24: u8,
    pub Reserved_23: u8,
    pub Reserved_22: u8,
    pub Reserved_21: u8,
    pub Reserved_20: u8,
    pub Reserved_19: u8,
    pub Reserved_18: u8,
    pub Reserved_17: u8,
    pub Reserved_16: u8,
    pub Reserved_15: u8,
    pub Reserved_14: u8,
    pub Reserved_13: u8,
    pub Reserved_12: u8,
    pub Reserved_11: u8,
    pub Reserved_10: u8,
    pub Reserved_09: u8,
    pub Reserved_08: u8,
    pub Reserved_07: u8,
    pub Reserved_06: u8,
    pub Reserved_05: u8,
    pub Reserved_04: u8,
    pub Reserved_03: u8,
    pub Reserved_02: u8,
    pub Reserved_01: u8,
    pub Reserved_00: u8,
}
#[cfg(feature = "OVRPlugin+ControllerState4")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_ControllerState4 =>
    ""."OVRPlugin/ControllerState4"
);
#[cfg(feature = "OVRPlugin+ControllerState4")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_ControllerState4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+ControllerState4")]
impl crate::GlobalNamespace::OVRPlugin_ControllerState4 {
    pub fn _ctor(
        &mut self,
        cs: crate::GlobalNamespace::OVRPlugin_ControllerState2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (cs),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+ControllerState5")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_ControllerState5 {
    pub ConnectedControllers: u32,
    pub Buttons: u32,
    pub Touches: u32,
    pub NearTouches: u32,
    pub LIndexTrigger: f32,
    pub RIndexTrigger: f32,
    pub LHandTrigger: f32,
    pub RHandTrigger: f32,
    pub LThumbstick: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub RThumbstick: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub LTouchpad: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub RTouchpad: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub LBatteryPercentRemaining: u8,
    pub RBatteryPercentRemaining: u8,
    pub LRecenterCount: u8,
    pub RRecenterCount: u8,
    pub LThumbRestForce: f32,
    pub RThumbRestForce: f32,
    pub LStylusForce: f32,
    pub RStylusForce: f32,
    pub LIndexTriggerCurl: f32,
    pub RIndexTriggerCurl: f32,
    pub LIndexTriggerSlide: f32,
    pub RIndexTriggerSlide: f32,
}
#[cfg(feature = "OVRPlugin+ControllerState5")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_ControllerState5 =>
    ""."OVRPlugin/ControllerState5"
);
#[cfg(feature = "OVRPlugin+ControllerState5")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_ControllerState5 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+ControllerState5")]
impl crate::GlobalNamespace::OVRPlugin_ControllerState5 {
    pub fn _ctor(
        &mut self,
        cs: crate::GlobalNamespace::OVRPlugin_ControllerState4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (cs),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+ControllerState6")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_ControllerState6 {
    pub ConnectedControllers: u32,
    pub Buttons: u32,
    pub Touches: u32,
    pub NearTouches: u32,
    pub LIndexTrigger: f32,
    pub RIndexTrigger: f32,
    pub LHandTrigger: f32,
    pub RHandTrigger: f32,
    pub LThumbstick: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub RThumbstick: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub LTouchpad: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub RTouchpad: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub LBatteryPercentRemaining: u8,
    pub RBatteryPercentRemaining: u8,
    pub LRecenterCount: u8,
    pub RRecenterCount: u8,
    pub LThumbRestForce: f32,
    pub RThumbRestForce: f32,
    pub LStylusForce: f32,
    pub RStylusForce: f32,
    pub LIndexTriggerCurl: f32,
    pub RIndexTriggerCurl: f32,
    pub LIndexTriggerSlide: f32,
    pub RIndexTriggerSlide: f32,
    pub LIndexTriggerForce: f32,
    pub RIndexTriggerForce: f32,
}
#[cfg(feature = "OVRPlugin+ControllerState6")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_ControllerState6 =>
    ""."OVRPlugin/ControllerState6"
);
#[cfg(feature = "OVRPlugin+ControllerState6")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_ControllerState6 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+ControllerState6")]
impl crate::GlobalNamespace::OVRPlugin_ControllerState6 {
    pub fn _ctor(
        &mut self,
        cs: crate::GlobalNamespace::OVRPlugin_ControllerState5,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (cs),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+EventDataBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_EventDataBuffer {
    pub EventType: crate::GlobalNamespace::OVRPlugin_EventType,
    pub EventData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "OVRPlugin+EventDataBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_EventDataBuffer => ""
    ."OVRPlugin/EventDataBuffer"
);
#[cfg(feature = "OVRPlugin+EventDataBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_EventDataBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+EventDataBuffer")]
impl crate::GlobalNamespace::OVRPlugin_EventDataBuffer {}
#[cfg(feature = "OVRPlugin+EventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_EventType {
    DisplayRefreshRateChanged = 1i32,
    SceneCaptureComplete = 100i32,
    SpaceEraseComplete = 54i32,
    SpaceListSaveResult = 57i32,
    SpaceQueryComplete = 52i32,
    SpaceQueryResults = 51i32,
    SpaceSaveComplete = 53i32,
    SpaceSetComponentStatusComplete = 50i32,
    SpaceShareResult = 56i32,
    SpatialAnchorCreateComplete = 49i32,
    Unknown = 0i32,
    VirtualKeyboardBackspace = 202i32,
    VirtualKeyboardCommitText = 201i32,
    VirtualKeyboardEnter = 203i32,
    VirtualKeyboardHidden = 205i32,
    VirtualKeyboardShown = 204i32,
}
#[cfg(feature = "OVRPlugin+EventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_EventType => ""
    ."OVRPlugin/EventType"
);
#[cfg(feature = "OVRPlugin+Eye")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_Eye {
    Count = 2i32,
    Left = 0i32,
    None = -1i32,
    Right = 1i32,
}
#[cfg(feature = "OVRPlugin+Eye")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Eye => ""
    ."OVRPlugin/Eye"
);
#[cfg(feature = "OVRPlugin+EyeGazeState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_EyeGazeState {
    pub Pose: crate::GlobalNamespace::OVRPlugin_Posef,
    pub Confidence: f32,
    pub _isValid: crate::GlobalNamespace::OVRPlugin_Bool,
}
#[cfg(feature = "OVRPlugin+EyeGazeState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_EyeGazeState => ""
    ."OVRPlugin/EyeGazeState"
);
#[cfg(feature = "OVRPlugin+EyeGazeState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_EyeGazeState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+EyeGazeState")]
impl crate::GlobalNamespace::OVRPlugin_EyeGazeState {
    pub fn get_IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsValid",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+EyeGazesState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_EyeGazesState {
    pub EyeGazes: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_EyeGazeState,
    >,
    pub Time: f64,
}
#[cfg(feature = "OVRPlugin+EyeGazesState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_EyeGazesState => ""
    ."OVRPlugin/EyeGazesState"
);
#[cfg(feature = "OVRPlugin+EyeGazesState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_EyeGazesState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+EyeGazesState")]
impl crate::GlobalNamespace::OVRPlugin_EyeGazesState {}
#[cfg(feature = "OVRPlugin+EyeGazesStateInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_EyeGazesStateInternal {
    pub EyeGazes_0: crate::GlobalNamespace::OVRPlugin_EyeGazeState,
    pub EyeGazes_1: crate::GlobalNamespace::OVRPlugin_EyeGazeState,
    pub Time: f64,
}
#[cfg(feature = "OVRPlugin+EyeGazesStateInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_EyeGazesStateInternal
    => ""."OVRPlugin/EyeGazesStateInternal"
);
#[cfg(feature = "OVRPlugin+EyeGazesStateInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_EyeGazesStateInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+EyeGazesStateInternal")]
impl crate::GlobalNamespace::OVRPlugin_EyeGazesStateInternal {}
#[cfg(feature = "OVRPlugin+EyeTextureFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_EyeTextureFormat {
    B8G8R8A8 = 5i32,
    B8G8R8A8_sRGB = 4i32,
    Default = 0i32,
    EnumSize = 2147483647i32,
    R11G11B10_FP = 3i32,
    R16G16B16A16_FP = 2i32,
    R5G6B5 = 11i32,
    R8G8B8A8 = 1i32,
}
#[cfg(feature = "OVRPlugin+EyeTextureFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_EyeTextureFormat =>
    ""."OVRPlugin/EyeTextureFormat"
);
#[cfg(feature = "OVRPlugin+FaceConstants")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_FaceConstants {
    MaxFaceExpressions = 63i32,
    MaxFaceRegionConfidences = 2i32,
}
#[cfg(feature = "OVRPlugin+FaceConstants")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_FaceConstants => ""
    ."OVRPlugin/FaceConstants"
);
#[cfg(feature = "OVRPlugin+FaceExpression")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_FaceExpression {
    Brow_Lowerer_L = 0i32,
    Brow_Lowerer_R = 1i32,
    Cheek_Puff_L = 2i32,
    Cheek_Puff_R = 3i32,
    Cheek_Raiser_L = 4i32,
    Cheek_Raiser_R = 5i32,
    Cheek_Suck_L = 6i32,
    Cheek_Suck_R = 7i32,
    Chin_Raiser_B = 8i32,
    Chin_Raiser_T = 9i32,
    Dimpler_L = 10i32,
    Dimpler_R = 11i32,
    Eyes_Closed_L = 12i32,
    Eyes_Closed_R = 13i32,
    Eyes_Look_Down_L = 14i32,
    Eyes_Look_Down_R = 15i32,
    Eyes_Look_Left_L = 16i32,
    Eyes_Look_Left_R = 17i32,
    Eyes_Look_Right_L = 18i32,
    Eyes_Look_Right_R = 19i32,
    Eyes_Look_Up_L = 20i32,
    Eyes_Look_Up_R = 21i32,
    Inner_Brow_Raiser_L = 22i32,
    Inner_Brow_Raiser_R = 23i32,
    Invalid = -1i32,
    Jaw_Drop = 24i32,
    Jaw_Sideways_Left = 25i32,
    Jaw_Sideways_Right = 26i32,
    Jaw_Thrust = 27i32,
    Lid_Tightener_L = 28i32,
    Lid_Tightener_R = 29i32,
    Lip_Corner_Depressor_L = 30i32,
    Lip_Corner_Depressor_R = 31i32,
    Lip_Corner_Puller_L = 32i32,
    Lip_Corner_Puller_R = 33i32,
    Lip_Funneler_LB = 34i32,
    Lip_Funneler_LT = 35i32,
    Lip_Funneler_RB = 36i32,
    Lip_Funneler_RT = 37i32,
    Lip_Pressor_L = 38i32,
    Lip_Pressor_R = 39i32,
    Lip_Pucker_L = 40i32,
    Lip_Pucker_R = 41i32,
    Lip_Stretcher_L = 42i32,
    Lip_Stretcher_R = 43i32,
    Lip_Suck_LB = 44i32,
    Lip_Suck_LT = 45i32,
    Lip_Suck_RB = 46i32,
    Lip_Suck_RT = 47i32,
    Lip_Tightener_L = 48i32,
    Lip_Tightener_R = 49i32,
    Lips_Toward = 50i32,
    Lower_Lip_Depressor_L = 51i32,
    Lower_Lip_Depressor_R = 52i32,
    Max = 63i32,
    Mouth_Left = 53i32,
    Mouth_Right = 54i32,
    Nose_Wrinkler_L = 55i32,
    Nose_Wrinkler_R = 56i32,
    Outer_Brow_Raiser_L = 57i32,
    Outer_Brow_Raiser_R = 58i32,
    Upper_Lid_Raiser_L = 59i32,
    Upper_Lid_Raiser_R = 60i32,
    Upper_Lip_Raiser_L = 61i32,
    Upper_Lip_Raiser_R = 62i32,
}
#[cfg(feature = "OVRPlugin+FaceExpression")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_FaceExpression => ""
    ."OVRPlugin/FaceExpression"
);
#[cfg(feature = "OVRPlugin+FaceExpressionStatus")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_FaceExpressionStatus {
    pub IsValid: bool,
    pub IsEyeFollowingBlendshapesValid: bool,
}
#[cfg(feature = "OVRPlugin+FaceExpressionStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_FaceExpressionStatus
    => ""."OVRPlugin/FaceExpressionStatus"
);
#[cfg(feature = "OVRPlugin+FaceExpressionStatus")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_FaceExpressionStatus {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+FaceExpressionStatus")]
impl crate::GlobalNamespace::OVRPlugin_FaceExpressionStatus {}
#[cfg(feature = "OVRPlugin+FaceExpressionStatusInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_FaceExpressionStatusInternal {
    pub IsValid: crate::GlobalNamespace::OVRPlugin_Bool,
    pub IsEyeFollowingBlendshapesValid: crate::GlobalNamespace::OVRPlugin_Bool,
}
#[cfg(feature = "OVRPlugin+FaceExpressionStatusInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_FaceExpressionStatusInternal => ""
    ."OVRPlugin/FaceExpressionStatusInternal"
);
#[cfg(feature = "OVRPlugin+FaceExpressionStatusInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_FaceExpressionStatusInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+FaceExpressionStatusInternal")]
impl crate::GlobalNamespace::OVRPlugin_FaceExpressionStatusInternal {
    pub fn ToFaceExpressionStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_FaceExpressionStatus,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_FaceExpressionStatus = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToFaceExpressionStatus",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+FaceRegionConfidence")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_FaceRegionConfidence {
    Lower = 0i32,
    Max = 2i32,
    Upper = 1i32,
}
#[cfg(feature = "OVRPlugin+FaceRegionConfidence")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_FaceRegionConfidence
    => ""."OVRPlugin/FaceRegionConfidence"
);
#[cfg(feature = "OVRPlugin+FaceState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_FaceState {
    pub ExpressionWeights: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub ExpressionWeightConfidences: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub Status: crate::GlobalNamespace::OVRPlugin_FaceExpressionStatus,
    pub Time: f64,
}
#[cfg(feature = "OVRPlugin+FaceState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_FaceState => ""
    ."OVRPlugin/FaceState"
);
#[cfg(feature = "OVRPlugin+FaceState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_FaceState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+FaceState")]
impl crate::GlobalNamespace::OVRPlugin_FaceState {}
#[cfg(feature = "OVRPlugin+FaceState2Internal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_FaceState2Internal {
    pub ExpressionWeights_0: f32,
    pub ExpressionWeights_1: f32,
    pub ExpressionWeights_2: f32,
    pub ExpressionWeights_3: f32,
    pub ExpressionWeights_4: f32,
    pub ExpressionWeights_5: f32,
    pub ExpressionWeights_6: f32,
    pub ExpressionWeights_7: f32,
    pub ExpressionWeights_8: f32,
    pub ExpressionWeights_9: f32,
    pub ExpressionWeights_10: f32,
    pub ExpressionWeights_11: f32,
    pub ExpressionWeights_12: f32,
    pub ExpressionWeights_13: f32,
    pub ExpressionWeights_14: f32,
    pub ExpressionWeights_15: f32,
    pub ExpressionWeights_16: f32,
    pub ExpressionWeights_17: f32,
    pub ExpressionWeights_18: f32,
    pub ExpressionWeights_19: f32,
    pub ExpressionWeights_20: f32,
    pub ExpressionWeights_21: f32,
    pub ExpressionWeights_22: f32,
    pub ExpressionWeights_23: f32,
    pub ExpressionWeights_24: f32,
    pub ExpressionWeights_25: f32,
    pub ExpressionWeights_26: f32,
    pub ExpressionWeights_27: f32,
    pub ExpressionWeights_28: f32,
    pub ExpressionWeights_29: f32,
    pub ExpressionWeights_30: f32,
    pub ExpressionWeights_31: f32,
    pub ExpressionWeights_32: f32,
    pub ExpressionWeights_33: f32,
    pub ExpressionWeights_34: f32,
    pub ExpressionWeights_35: f32,
    pub ExpressionWeights_36: f32,
    pub ExpressionWeights_37: f32,
    pub ExpressionWeights_38: f32,
    pub ExpressionWeights_39: f32,
    pub ExpressionWeights_40: f32,
    pub ExpressionWeights_41: f32,
    pub ExpressionWeights_42: f32,
    pub ExpressionWeights_43: f32,
    pub ExpressionWeights_44: f32,
    pub ExpressionWeights_45: f32,
    pub ExpressionWeights_46: f32,
    pub ExpressionWeights_47: f32,
    pub ExpressionWeights_48: f32,
    pub ExpressionWeights_49: f32,
    pub ExpressionWeights_50: f32,
    pub ExpressionWeights_51: f32,
    pub ExpressionWeights_52: f32,
    pub ExpressionWeights_53: f32,
    pub ExpressionWeights_54: f32,
    pub ExpressionWeights_55: f32,
    pub ExpressionWeights_56: f32,
    pub ExpressionWeights_57: f32,
    pub ExpressionWeights_58: f32,
    pub ExpressionWeights_59: f32,
    pub ExpressionWeights_60: f32,
    pub ExpressionWeights_61: f32,
    pub ExpressionWeights_62: f32,
    pub ExpressionWeightConfidences_0: f32,
    pub ExpressionWeightConfidences_1: f32,
    pub Status: crate::GlobalNamespace::OVRPlugin_FaceExpressionStatusInternal,
    pub Time: f64,
}
#[cfg(feature = "OVRPlugin+FaceState2Internal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_FaceState2Internal =>
    ""."OVRPlugin/FaceState2Internal"
);
#[cfg(feature = "OVRPlugin+FaceState2Internal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_FaceState2Internal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+FaceState2Internal")]
impl crate::GlobalNamespace::OVRPlugin_FaceState2Internal {}
#[cfg(feature = "OVRPlugin+FaceStateInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_FaceStateInternal {
    pub ExpressionWeights_0: f32,
    pub ExpressionWeights_1: f32,
    pub ExpressionWeights_2: f32,
    pub ExpressionWeights_3: f32,
    pub ExpressionWeights_4: f32,
    pub ExpressionWeights_5: f32,
    pub ExpressionWeights_6: f32,
    pub ExpressionWeights_7: f32,
    pub ExpressionWeights_8: f32,
    pub ExpressionWeights_9: f32,
    pub ExpressionWeights_10: f32,
    pub ExpressionWeights_11: f32,
    pub ExpressionWeights_12: f32,
    pub ExpressionWeights_13: f32,
    pub ExpressionWeights_14: f32,
    pub ExpressionWeights_15: f32,
    pub ExpressionWeights_16: f32,
    pub ExpressionWeights_17: f32,
    pub ExpressionWeights_18: f32,
    pub ExpressionWeights_19: f32,
    pub ExpressionWeights_20: f32,
    pub ExpressionWeights_21: f32,
    pub ExpressionWeights_22: f32,
    pub ExpressionWeights_23: f32,
    pub ExpressionWeights_24: f32,
    pub ExpressionWeights_25: f32,
    pub ExpressionWeights_26: f32,
    pub ExpressionWeights_27: f32,
    pub ExpressionWeights_28: f32,
    pub ExpressionWeights_29: f32,
    pub ExpressionWeights_30: f32,
    pub ExpressionWeights_31: f32,
    pub ExpressionWeights_32: f32,
    pub ExpressionWeights_33: f32,
    pub ExpressionWeights_34: f32,
    pub ExpressionWeights_35: f32,
    pub ExpressionWeights_36: f32,
    pub ExpressionWeights_37: f32,
    pub ExpressionWeights_38: f32,
    pub ExpressionWeights_39: f32,
    pub ExpressionWeights_40: f32,
    pub ExpressionWeights_41: f32,
    pub ExpressionWeights_42: f32,
    pub ExpressionWeights_43: f32,
    pub ExpressionWeights_44: f32,
    pub ExpressionWeights_45: f32,
    pub ExpressionWeights_46: f32,
    pub ExpressionWeights_47: f32,
    pub ExpressionWeights_48: f32,
    pub ExpressionWeights_49: f32,
    pub ExpressionWeights_50: f32,
    pub ExpressionWeights_51: f32,
    pub ExpressionWeights_52: f32,
    pub ExpressionWeights_53: f32,
    pub ExpressionWeights_54: f32,
    pub ExpressionWeights_55: f32,
    pub ExpressionWeights_56: f32,
    pub ExpressionWeights_57: f32,
    pub ExpressionWeights_58: f32,
    pub ExpressionWeights_59: f32,
    pub ExpressionWeights_60: f32,
    pub ExpressionWeights_61: f32,
    pub ExpressionWeights_62: f32,
    pub ExpressionWeightConfidences_0: f32,
    pub ExpressionWeightConfidences_1: f32,
    pub Status: crate::GlobalNamespace::OVRPlugin_FaceExpressionStatusInternal,
    pub Time: f64,
}
#[cfg(feature = "OVRPlugin+FaceStateInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_FaceStateInternal =>
    ""."OVRPlugin/FaceStateInternal"
);
#[cfg(feature = "OVRPlugin+FaceStateInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_FaceStateInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+FaceStateInternal")]
impl crate::GlobalNamespace::OVRPlugin_FaceStateInternal {}
#[cfg(feature = "OVRPlugin+FeatureType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_FeatureType {
    BodyTracking = 4i32,
    Count = 7i32,
    EnumSize = 2147483647i32,
    EyeTracking = 2i32,
    FaceTracking = 3i32,
    GazeBasedFoveatedRendering = 6i32,
    HandTracking = 0i32,
    KeyboardTracking = 1i32,
    Passthrough = 5i32,
}
#[cfg(feature = "OVRPlugin+FeatureType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_FeatureType => ""
    ."OVRPlugin/FeatureType"
);
#[cfg(feature = "OVRPlugin+FixedFoveatedRenderingLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_FixedFoveatedRenderingLevel {
    EnumSize = 2147483647i32,
    High = 3i32,
    HighTop = 4i32,
    Low = 1i32,
    Medium = 2i32,
    Off = 0i32,
}
#[cfg(feature = "OVRPlugin+FixedFoveatedRenderingLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_FixedFoveatedRenderingLevel => ""
    ."OVRPlugin/FixedFoveatedRenderingLevel"
);
#[cfg(feature = "OVRPlugin+FoveatedRenderingLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_FoveatedRenderingLevel {
    EnumSize = 2147483647i32,
    High = 3i32,
    HighTop = 4i32,
    Low = 1i32,
    Medium = 2i32,
    Off = 0i32,
}
#[cfg(feature = "OVRPlugin+FoveatedRenderingLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_FoveatedRenderingLevel => ""
    ."OVRPlugin/FoveatedRenderingLevel"
);
#[cfg(feature = "OVRPlugin+Fovf")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Fovf {
    pub UpTan: f32,
    pub DownTan: f32,
    pub LeftTan: f32,
    pub RightTan: f32,
}
#[cfg(feature = "OVRPlugin+Fovf")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Fovf => ""
    ."OVRPlugin/Fovf"
);
#[cfg(feature = "OVRPlugin+Fovf")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Fovf {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Fovf")]
impl crate::GlobalNamespace::OVRPlugin_Fovf {}
#[cfg(feature = "OVRPlugin+Frustumf")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Frustumf {
    pub zNear: f32,
    pub zFar: f32,
    pub fovX: f32,
    pub fovY: f32,
}
#[cfg(feature = "OVRPlugin+Frustumf")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Frustumf => ""
    ."OVRPlugin/Frustumf"
);
#[cfg(feature = "OVRPlugin+Frustumf")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Frustumf {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Frustumf")]
impl crate::GlobalNamespace::OVRPlugin_Frustumf {}
#[cfg(feature = "OVRPlugin+Frustumf2")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Frustumf2 {
    pub zNear: f32,
    pub zFar: f32,
    pub Fov: crate::GlobalNamespace::OVRPlugin_Fovf,
}
#[cfg(feature = "OVRPlugin+Frustumf2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Frustumf2 => ""
    ."OVRPlugin/Frustumf2"
);
#[cfg(feature = "OVRPlugin+Frustumf2")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Frustumf2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Frustumf2")]
impl crate::GlobalNamespace::OVRPlugin_Frustumf2 {}
#[cfg(feature = "OVRPlugin+GUID")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_GUID {
    __cordl_parent: crate::System::Object,
    pub a: i32,
    pub b: i16,
    pub c: i16,
    pub d0: u8,
    pub d1: u8,
    pub d2: u8,
    pub d3: u8,
    pub d4: u8,
    pub d5: u8,
    pub d6: u8,
    pub d7: u8,
}
#[cfg(feature = "OVRPlugin+GUID")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_GUID => ""
    ."OVRPlugin/GUID"
);
#[cfg(feature = "OVRPlugin+GUID")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_GUID {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+GUID")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_GUID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+GUID")]
impl crate::GlobalNamespace::OVRPlugin_GUID {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+GUID")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_GUID {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+Hand")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_Hand {
    HandLeft = 0i32,
    HandRight = 1i32,
    None = -1i32,
}
#[cfg(feature = "OVRPlugin+Hand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Hand => ""
    ."OVRPlugin/Hand"
);
#[cfg(feature = "OVRPlugin+HandFinger")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_HandFinger {
    Index = 1i32,
    Max = 5i32,
    Middle = 2i32,
    Pinky = 4i32,
    Ring = 3i32,
    Thumb = 0i32,
}
#[cfg(feature = "OVRPlugin+HandFinger")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_HandFinger => ""
    ."OVRPlugin/HandFinger"
);
#[cfg(feature = "OVRPlugin+HandFingerPinch")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_HandFingerPinch {
    Index = 2i32,
    Middle = 4i32,
    Pinky = 16i32,
    Ring = 8i32,
    Thumb = 1i32,
}
#[cfg(feature = "OVRPlugin+HandFingerPinch")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_HandFingerPinch => ""
    ."OVRPlugin/HandFingerPinch"
);
#[cfg(feature = "OVRPlugin+HandState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_HandState {
    pub Status: crate::GlobalNamespace::OVRPlugin_HandStatus,
    pub RootPose: crate::GlobalNamespace::OVRPlugin_Posef,
    pub BoneRotations: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Quatf,
    >,
    pub Pinches: crate::GlobalNamespace::OVRPlugin_HandFingerPinch,
    pub PinchStrength: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub PointerPose: crate::GlobalNamespace::OVRPlugin_Posef,
    pub HandScale: f32,
    pub HandConfidence: crate::GlobalNamespace::OVRPlugin_TrackingConfidence,
    pub FingerConfidences: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_TrackingConfidence,
    >,
    pub RequestedTimeStamp: f64,
    pub SampleTimeStamp: f64,
}
#[cfg(feature = "OVRPlugin+HandState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_HandState => ""
    ."OVRPlugin/HandState"
);
#[cfg(feature = "OVRPlugin+HandState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_HandState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+HandState")]
impl crate::GlobalNamespace::OVRPlugin_HandState {}
#[cfg(feature = "OVRPlugin+HandStateInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_HandStateInternal {
    pub Status: crate::GlobalNamespace::OVRPlugin_HandStatus,
    pub RootPose: crate::GlobalNamespace::OVRPlugin_Posef,
    pub BoneRotations_0: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_1: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_2: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_3: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_4: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_5: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_6: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_7: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_8: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_9: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_10: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_11: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_12: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_13: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_14: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_15: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_16: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_17: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_18: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_19: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_20: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_21: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_22: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub BoneRotations_23: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub Pinches: crate::GlobalNamespace::OVRPlugin_HandFingerPinch,
    pub PinchStrength_0: f32,
    pub PinchStrength_1: f32,
    pub PinchStrength_2: f32,
    pub PinchStrength_3: f32,
    pub PinchStrength_4: f32,
    pub PointerPose: crate::GlobalNamespace::OVRPlugin_Posef,
    pub HandScale: f32,
    pub HandConfidence: crate::GlobalNamespace::OVRPlugin_TrackingConfidence,
    pub FingerConfidences_0: crate::GlobalNamespace::OVRPlugin_TrackingConfidence,
    pub FingerConfidences_1: crate::GlobalNamespace::OVRPlugin_TrackingConfidence,
    pub FingerConfidences_2: crate::GlobalNamespace::OVRPlugin_TrackingConfidence,
    pub FingerConfidences_3: crate::GlobalNamespace::OVRPlugin_TrackingConfidence,
    pub FingerConfidences_4: crate::GlobalNamespace::OVRPlugin_TrackingConfidence,
    pub RequestedTimeStamp: f64,
    pub SampleTimeStamp: f64,
}
#[cfg(feature = "OVRPlugin+HandStateInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_HandStateInternal =>
    ""."OVRPlugin/HandStateInternal"
);
#[cfg(feature = "OVRPlugin+HandStateInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_HandStateInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+HandStateInternal")]
impl crate::GlobalNamespace::OVRPlugin_HandStateInternal {}
#[cfg(feature = "OVRPlugin+HandStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_HandStatus {
    DominantHand = 128i32,
    HandTracked = 1i32,
    InputStateValid = 2i32,
    MenuPressed = 256i32,
    SystemGestureInProgress = 64i32,
}
#[cfg(feature = "OVRPlugin+HandStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_HandStatus => ""
    ."OVRPlugin/HandStatus"
);
#[cfg(feature = "OVRPlugin+Handedness")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_Handedness {
    LeftHanded = 1i32,
    RightHanded = 2i32,
    Unsupported = 0i32,
}
#[cfg(feature = "OVRPlugin+Handedness")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Handedness => ""
    ."OVRPlugin/Handedness"
);
#[cfg(feature = "OVRPlugin+HapticsAmplitudeEnvelopeVibration")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_HapticsAmplitudeEnvelopeVibration {
    pub Duration: f32,
    pub AmplitudeCount: u32,
    pub Amplitudes: crate::System::IntPtr,
}
#[cfg(feature = "OVRPlugin+HapticsAmplitudeEnvelopeVibration")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_HapticsAmplitudeEnvelopeVibration => ""
    ."OVRPlugin/HapticsAmplitudeEnvelopeVibration"
);
#[cfg(feature = "OVRPlugin+HapticsAmplitudeEnvelopeVibration")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_HapticsAmplitudeEnvelopeVibration {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+HapticsAmplitudeEnvelopeVibration")]
impl crate::GlobalNamespace::OVRPlugin_HapticsAmplitudeEnvelopeVibration {}
#[cfg(feature = "OVRPlugin+HapticsBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_HapticsBuffer {
    pub Samples: crate::System::IntPtr,
    pub SamplesCount: i32,
}
#[cfg(feature = "OVRPlugin+HapticsBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_HapticsBuffer => ""
    ."OVRPlugin/HapticsBuffer"
);
#[cfg(feature = "OVRPlugin+HapticsBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_HapticsBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+HapticsBuffer")]
impl crate::GlobalNamespace::OVRPlugin_HapticsBuffer {}
#[cfg(feature = "OVRPlugin+HapticsConstants")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_HapticsConstants {
    MaxSamples = 4000i32,
}
#[cfg(feature = "OVRPlugin+HapticsConstants")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_HapticsConstants =>
    ""."OVRPlugin/HapticsConstants"
);
#[cfg(feature = "OVRPlugin+HapticsDesc")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_HapticsDesc {
    pub SampleRateHz: i32,
    pub SampleSizeInBytes: i32,
    pub MinimumSafeSamplesQueued: i32,
    pub MinimumBufferSamplesCount: i32,
    pub OptimalBufferSamplesCount: i32,
    pub MaximumBufferSamplesCount: i32,
}
#[cfg(feature = "OVRPlugin+HapticsDesc")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_HapticsDesc => ""
    ."OVRPlugin/HapticsDesc"
);
#[cfg(feature = "OVRPlugin+HapticsDesc")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_HapticsDesc {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+HapticsDesc")]
impl crate::GlobalNamespace::OVRPlugin_HapticsDesc {}
#[cfg(feature = "OVRPlugin+HapticsLocation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_HapticsLocation {
    Hand = 1i32,
    Index = 4i32,
    None = 0i32,
    Thumb = 2i32,
}
#[cfg(feature = "OVRPlugin+HapticsLocation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_HapticsLocation => ""
    ."OVRPlugin/HapticsLocation"
);
#[cfg(feature = "OVRPlugin+HapticsPcmVibration")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_HapticsPcmVibration {
    pub BufferSize: u32,
    pub Buffer: crate::System::IntPtr,
    pub SampleRateHz: f32,
    pub Append: crate::GlobalNamespace::OVRPlugin_Bool,
    pub SamplesConsumed: crate::System::IntPtr,
}
#[cfg(feature = "OVRPlugin+HapticsPcmVibration")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_HapticsPcmVibration
    => ""."OVRPlugin/HapticsPcmVibration"
);
#[cfg(feature = "OVRPlugin+HapticsPcmVibration")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_HapticsPcmVibration {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+HapticsPcmVibration")]
impl crate::GlobalNamespace::OVRPlugin_HapticsPcmVibration {}
#[cfg(feature = "OVRPlugin+HapticsState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_HapticsState {
    pub SamplesAvailable: i32,
    pub SamplesQueued: i32,
}
#[cfg(feature = "OVRPlugin+HapticsState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_HapticsState => ""
    ."OVRPlugin/HapticsState"
);
#[cfg(feature = "OVRPlugin+HapticsState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_HapticsState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+HapticsState")]
impl crate::GlobalNamespace::OVRPlugin_HapticsState {}
#[cfg(feature = "OVRPlugin+InsightPassthroughColorMapType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_InsightPassthroughColorMapType {
    BrightnessContrastSaturation = 4i32,
    ColorLut = 6i32,
    InterpolatedColorLut = 7i32,
    MonoToMono = 2i32,
    MonoToRgba = 1i32,
    None = 0i32,
}
#[cfg(feature = "OVRPlugin+InsightPassthroughColorMapType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_InsightPassthroughColorMapType => ""
    ."OVRPlugin/InsightPassthroughColorMapType"
);
#[cfg(feature = "OVRPlugin+InsightPassthroughKeyboardHandsIntensity")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_InsightPassthroughKeyboardHandsIntensity {
    pub LeftHandIntensity: f32,
    pub RightHandIntensity: f32,
}
#[cfg(feature = "OVRPlugin+InsightPassthroughKeyboardHandsIntensity")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_InsightPassthroughKeyboardHandsIntensity => ""
    ."OVRPlugin/InsightPassthroughKeyboardHandsIntensity"
);
#[cfg(feature = "OVRPlugin+InsightPassthroughKeyboardHandsIntensity")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_InsightPassthroughKeyboardHandsIntensity {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+InsightPassthroughKeyboardHandsIntensity")]
impl crate::GlobalNamespace::OVRPlugin_InsightPassthroughKeyboardHandsIntensity {}
#[cfg(feature = "OVRPlugin+InsightPassthroughStyle")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_InsightPassthroughStyle {
    pub Flags: crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyleFlags,
    pub TextureOpacityFactor: f32,
    pub EdgeColor: crate::GlobalNamespace::OVRPlugin_Colorf,
    pub TextureColorMapType: crate::GlobalNamespace::OVRPlugin_InsightPassthroughColorMapType,
    pub TextureColorMapDataSize: u32,
    pub TextureColorMapData: crate::System::IntPtr,
}
#[cfg(feature = "OVRPlugin+InsightPassthroughStyle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_InsightPassthroughStyle => ""
    ."OVRPlugin/InsightPassthroughStyle"
);
#[cfg(feature = "OVRPlugin+InsightPassthroughStyle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+InsightPassthroughStyle")]
impl crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle {}
#[cfg(feature = "OVRPlugin+InsightPassthroughStyle2")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_InsightPassthroughStyle2 {
    pub Flags: crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyleFlags,
    pub TextureOpacityFactor: f32,
    pub EdgeColor: crate::GlobalNamespace::OVRPlugin_Colorf,
    pub TextureColorMapType: crate::GlobalNamespace::OVRPlugin_InsightPassthroughColorMapType,
    pub TextureColorMapDataSize: u32,
    pub TextureColorMapData: crate::System::IntPtr,
    pub LutSource: u64,
    pub LutTarget: u64,
    pub LutWeight: f32,
}
#[cfg(feature = "OVRPlugin+InsightPassthroughStyle2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_InsightPassthroughStyle2 => ""
    ."OVRPlugin/InsightPassthroughStyle2"
);
#[cfg(feature = "OVRPlugin+InsightPassthroughStyle2")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+InsightPassthroughStyle2")]
impl crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle2 {
    pub fn CopyTo(
        &mut self,
        target: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyTo",
            (target),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+InsightPassthroughStyleFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_InsightPassthroughStyleFlags {
    HasEdgeColor = 2i32,
    HasTextureColorMap = 4i32,
    HasTextureOpacityFactor = 1i32,
}
#[cfg(feature = "OVRPlugin+InsightPassthroughStyleFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_InsightPassthroughStyleFlags => ""
    ."OVRPlugin/InsightPassthroughStyleFlags"
);
#[cfg(feature = "OVRPlugin+InteractionProfile")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_InteractionProfile {
    None = 0i32,
    Touch = 1i32,
    TouchPlus = 4i32,
    TouchPro = 2i32,
}
#[cfg(feature = "OVRPlugin+InteractionProfile")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_InteractionProfile =>
    ""."OVRPlugin/InteractionProfile"
);
#[cfg(feature = "OVRPlugin+KeyboardDescription")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_KeyboardDescription {
    pub Name: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub TrackedKeyboardId: u64,
    pub Dimensions: crate::GlobalNamespace::OVRPlugin_Vector3f,
    pub KeyboardFlags: crate::GlobalNamespace::OVRPlugin_TrackedKeyboardFlags,
    pub SupportedPresentationStyles: crate::GlobalNamespace::OVRPlugin_TrackedKeyboardPresentationStyles,
}
#[cfg(feature = "OVRPlugin+KeyboardDescription")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_KeyboardDescription
    => ""."OVRPlugin/KeyboardDescription"
);
#[cfg(feature = "OVRPlugin+KeyboardDescription")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_KeyboardDescription {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+KeyboardDescription")]
impl crate::GlobalNamespace::OVRPlugin_KeyboardDescription {}
#[cfg(feature = "OVRPlugin+KeyboardDescriptionConstants")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_KeyboardDescriptionConstants {
    NameMaxLength = 128i32,
}
#[cfg(feature = "OVRPlugin+KeyboardDescriptionConstants")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_KeyboardDescriptionConstants => ""
    ."OVRPlugin/KeyboardDescriptionConstants"
);
#[cfg(feature = "OVRPlugin+KeyboardState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_KeyboardState {
    pub IsActive: crate::GlobalNamespace::OVRPlugin_Bool,
    pub OrientationValid: crate::GlobalNamespace::OVRPlugin_Bool,
    pub PositionValid: crate::GlobalNamespace::OVRPlugin_Bool,
    pub OrientationTracked: crate::GlobalNamespace::OVRPlugin_Bool,
    pub PositionTracked: crate::GlobalNamespace::OVRPlugin_Bool,
    pub PoseState: crate::GlobalNamespace::OVRPlugin_PoseStatef,
    pub ContrastParameters: crate::GlobalNamespace::OVRPlugin_Vector4f,
}
#[cfg(feature = "OVRPlugin+KeyboardState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_KeyboardState => ""
    ."OVRPlugin/KeyboardState"
);
#[cfg(feature = "OVRPlugin+KeyboardState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_KeyboardState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+KeyboardState")]
impl crate::GlobalNamespace::OVRPlugin_KeyboardState {}
#[cfg(feature = "OVRPlugin+Ktx")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_Ktx {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+Ktx")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Ktx => ""
    ."OVRPlugin/Ktx"
);
#[cfg(feature = "OVRPlugin+Ktx")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_Ktx {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+Ktx")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_Ktx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+Ktx")]
impl crate::GlobalNamespace::OVRPlugin_Ktx {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+Ktx")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_Ktx {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+LayerDesc")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_LayerDesc {
    pub Shape: crate::GlobalNamespace::OVRPlugin_OverlayShape,
    pub Layout: crate::GlobalNamespace::OVRPlugin_LayerLayout,
    pub TextureSize: crate::GlobalNamespace::OVRPlugin_Sizei,
    pub MipLevels: i32,
    pub SampleCount: i32,
    pub Format: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    pub LayerFlags: i32,
    pub Fov: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Fovf,
    >,
    pub VisibleRect: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Rectf,
    >,
    pub MaxViewportSize: crate::GlobalNamespace::OVRPlugin_Sizei,
    pub DepthFormat: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    pub MotionVectorFormat: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    pub MotionVectorDepthFormat: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    pub MotionVectorTextureSize: crate::GlobalNamespace::OVRPlugin_Sizei,
}
#[cfg(feature = "OVRPlugin+LayerDesc")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_LayerDesc => ""
    ."OVRPlugin/LayerDesc"
);
#[cfg(feature = "OVRPlugin+LayerDesc")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_LayerDesc {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+LayerDesc")]
impl crate::GlobalNamespace::OVRPlugin_LayerDesc {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+LayerDescInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_LayerDescInternal {
    pub Shape: crate::GlobalNamespace::OVRPlugin_OverlayShape,
    pub Layout: crate::GlobalNamespace::OVRPlugin_LayerLayout,
    pub TextureSize: crate::GlobalNamespace::OVRPlugin_Sizei,
    pub MipLevels: i32,
    pub SampleCount: i32,
    pub Format: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    pub LayerFlags: i32,
    pub Fov0: crate::GlobalNamespace::OVRPlugin_Fovf,
    pub Fov1: crate::GlobalNamespace::OVRPlugin_Fovf,
    pub VisibleRect0: crate::GlobalNamespace::OVRPlugin_Rectf,
    pub VisibleRect1: crate::GlobalNamespace::OVRPlugin_Rectf,
    pub MaxViewportSize: crate::GlobalNamespace::OVRPlugin_Sizei,
    pub DepthFormat: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    pub MotionVectorFormat: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    pub MotionVectorDepthFormat: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    pub MotionVectorTextureSize: crate::GlobalNamespace::OVRPlugin_Sizei,
}
#[cfg(feature = "OVRPlugin+LayerDescInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_LayerDescInternal =>
    ""."OVRPlugin/LayerDescInternal"
);
#[cfg(feature = "OVRPlugin+LayerDescInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_LayerDescInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+LayerDescInternal")]
impl crate::GlobalNamespace::OVRPlugin_LayerDescInternal {
    pub fn ToLayerDesc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_LayerDesc> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_LayerDesc = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToLayerDesc",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        layerDesc: crate::GlobalNamespace::OVRPlugin_LayerDesc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (layerDesc),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+LayerFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_LayerFlags {
    AndroidSurfaceSwapChain = 128i32,
    BicubicFiltering = 16384i32,
    ChromaticAberrationCorrection = 16i32,
    LoadingScreen = 2i32,
    NoAllocation = 32i32,
    ProtectedContent = 64i32,
    Static = 1i32,
    SymmetricFov = 4i32,
    TextureOriginAtBottomLeft = 8i32,
}
#[cfg(feature = "OVRPlugin+LayerFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_LayerFlags => ""
    ."OVRPlugin/LayerFlags"
);
#[cfg(feature = "OVRPlugin+LayerLayout")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_LayerLayout {
    Array = 3i32,
    DoubleWide = 2i32,
    EnumSize = 15i32,
    Mono = 1i32,
    Stereo = 0i32,
}
#[cfg(feature = "OVRPlugin+LayerLayout")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_LayerLayout => ""
    ."OVRPlugin/LayerLayout"
);
#[cfg(feature = "OVRPlugin+LayerSharpenType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_LayerSharpenType {
    None = 0i32,
    Normal = 8192i32,
    Quality = 65536i32,
}
#[cfg(feature = "OVRPlugin+LayerSharpenType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_LayerSharpenType =>
    ""."OVRPlugin/LayerSharpenType"
);
#[cfg(feature = "OVRPlugin+LayerSubmit")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_LayerSubmit {
    pub LayerId: i32,
    pub TextureStage: i32,
    pub ViewportRect: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Recti,
    >,
    pub Pose: crate::GlobalNamespace::OVRPlugin_Posef,
    pub LayerSubmitFlags: i32,
}
#[cfg(feature = "OVRPlugin+LayerSubmit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_LayerSubmit => ""
    ."OVRPlugin/LayerSubmit"
);
#[cfg(feature = "OVRPlugin+LayerSubmit")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_LayerSubmit {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+LayerSubmit")]
impl crate::GlobalNamespace::OVRPlugin_LayerSubmit {}
#[cfg(feature = "OVRPlugin+LayerSuperSamplingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_LayerSuperSamplingType {
    None = 0i32,
    Normal = 4096i32,
    Quality = 256i32,
}
#[cfg(feature = "OVRPlugin+LayerSuperSamplingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_LayerSuperSamplingType => ""
    ."OVRPlugin/LayerSuperSamplingType"
);
#[cfg(feature = "OVRPlugin+LogCallback2DelegateType")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_LogCallback2DelegateType {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVRPlugin+LogCallback2DelegateType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_LogCallback2DelegateType => ""
    ."OVRPlugin/LogCallback2DelegateType"
);
#[cfg(feature = "OVRPlugin+LogCallback2DelegateType")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_LogCallback2DelegateType {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+LogCallback2DelegateType")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_LogCallback2DelegateType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+LogCallback2DelegateType")]
impl crate::GlobalNamespace::OVRPlugin_LogCallback2DelegateType {
    pub fn BeginInvoke(
        &mut self,
        logLevel: crate::GlobalNamespace::OVRPlugin_LogLevel,
        message: crate::System::IntPtr,
        _cordl_size: i32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (logLevel, message, _cordl_size, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        logLevel: crate::GlobalNamespace::OVRPlugin_LogLevel,
        message: crate::System::IntPtr,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (logLevel, message, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+LogCallback2DelegateType")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_LogCallback2DelegateType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+LogLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_LogLevel {
    Debug = 0i32,
    Error = 2i32,
    Info = 1i32,
}
#[cfg(feature = "OVRPlugin+LogLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_LogLevel => ""
    ."OVRPlugin/LogLevel"
);
#[cfg(feature = "OVRPlugin+Media")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_Media {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+Media")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Media => ""
    ."OVRPlugin/Media"
);
#[cfg(feature = "OVRPlugin+Media")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_Media {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+Media")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_Media {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+Media")]
impl crate::GlobalNamespace::OVRPlugin_Media {
    #[cfg(feature = "OVRPlugin+Media+InputVideoBufferType")]
    pub type InputVideoBufferType = crate::GlobalNamespace::Media_OVRPlugin_InputVideoBufferType;
    #[cfg(feature = "OVRPlugin+Media+MrcActivationMode")]
    pub type MrcActivationMode = crate::GlobalNamespace::Media_OVRPlugin_MrcActivationMode;
    #[cfg(feature = "OVRPlugin+Media+PlatformCameraMode")]
    pub type PlatformCameraMode = crate::GlobalNamespace::Media_OVRPlugin_PlatformCameraMode;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+Media")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_Media {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+Mesh")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_Mesh {
    __cordl_parent: crate::System::Object,
    pub Type: crate::GlobalNamespace::OVRPlugin_MeshType,
    pub NumVertices: u32,
    pub NumIndices: u32,
    pub VertexPositions: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Vector3f,
    >,
    pub Indices: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
    pub VertexNormals: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Vector3f,
    >,
    pub VertexUV0: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Vector2f,
    >,
    pub BlendIndices: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Vector4s,
    >,
    pub BlendWeights: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Vector4f,
    >,
}
#[cfg(feature = "OVRPlugin+Mesh")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Mesh => ""
    ."OVRPlugin/Mesh"
);
#[cfg(feature = "OVRPlugin+Mesh")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_Mesh {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+Mesh")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_Mesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+Mesh")]
impl crate::GlobalNamespace::OVRPlugin_Mesh {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+Mesh")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_Mesh {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+MeshConstants")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_MeshConstants {
    MaxIndices = 18000i32,
    MaxVertices = 3000i32,
}
#[cfg(feature = "OVRPlugin+MeshConstants")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_MeshConstants => ""
    ."OVRPlugin/MeshConstants"
);
#[cfg(feature = "OVRPlugin+MeshType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_MeshType {
    HandLeft = 0i32,
    HandRight = 1i32,
    None = -1i32,
}
#[cfg(feature = "OVRPlugin+MeshType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_MeshType => ""
    ."OVRPlugin/MeshType"
);
#[cfg(feature = "OVRPlugin+Node")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_Node {
    ControllerLeft = 12i32,
    ControllerRight = 13i32,
    Count = 14i32,
    DeviceObjectZero = 10i32,
    EyeCenter = 2i32,
    EyeLeft = 0i32,
    EyeRight = 1i32,
    HandLeft = 3i32,
    HandRight = 4i32,
    Head = 9i32,
    None = -1i32,
    TrackedKeyboard = 11i32,
    TrackerOne = 6i32,
    TrackerThree = 8i32,
    TrackerTwo = 7i32,
    TrackerZero = 5i32,
}
#[cfg(feature = "OVRPlugin+Node")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Node => ""
    ."OVRPlugin/Node"
);
#[cfg(feature = "OVRPlugin+OVRP_0_1_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_0_1_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_0_1_0 => ""
    ."OVRPlugin/OVRP_0_1_0"
);
#[cfg(feature = "OVRPlugin+OVRP_0_1_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_0_1_0 {}
#[cfg(feature = "OVRPlugin+OVRP_0_1_0")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_1")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_0_1_1 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_0_1_1 => ""
    ."OVRPlugin/OVRP_0_1_1"
);
#[cfg(feature = "OVRPlugin+OVRP_0_1_1")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_1 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_1")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_1")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_0_1_1 {}
#[cfg(feature = "OVRPlugin+OVRP_0_1_1")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_2")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_0_1_2 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_0_1_2 => ""
    ."OVRPlugin/OVRP_0_1_2"
);
#[cfg(feature = "OVRPlugin+OVRP_0_1_2")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_2 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_2")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_2")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_0_1_2 {}
#[cfg(feature = "OVRPlugin+OVRP_0_1_2")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_3")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_0_1_3 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_0_1_3 => ""
    ."OVRPlugin/OVRP_0_1_3"
);
#[cfg(feature = "OVRPlugin+OVRP_0_1_3")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_3 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_3")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_3")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_0_1_3 {}
#[cfg(feature = "OVRPlugin+OVRP_0_1_3")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_3 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_5_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_0_5_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_0_5_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_0_5_0 => ""
    ."OVRPlugin/OVRP_0_5_0"
);
#[cfg(feature = "OVRPlugin+OVRP_0_5_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_0_5_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_5_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_0_5_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_0_5_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_0_5_0 {}
#[cfg(feature = "OVRPlugin+OVRP_0_5_0")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_0_5_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_0_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_0_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_0_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_0_0 => ""
    ."OVRPlugin/OVRP_1_0_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_0_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_0_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_0_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_0_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_0_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_0_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_0_0")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_1_0_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_10_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_10_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_10_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_10_0 => ""
    ."OVRPlugin/OVRP_1_10_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_10_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_10_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_10_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_10_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_10_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_10_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_10_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_10_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_11_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_11_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_11_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_11_0 => ""
    ."OVRPlugin/OVRP_1_11_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_11_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_11_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_11_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_11_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_11_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_11_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_11_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_11_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_12_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_12_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_12_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_12_0 => ""
    ."OVRPlugin/OVRP_1_12_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_12_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_12_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_12_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_12_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_12_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_12_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_12_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_12_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_15_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_15_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_15_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_15_0 => ""
    ."OVRPlugin/OVRP_1_15_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_15_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_15_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_15_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_15_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_15_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_15_0 {
    pub const OVRP_EXTERNAL_CAMERA_NAME_SIZE: i32 = 32i32;
}
#[cfg(feature = "OVRPlugin+OVRP_1_15_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_15_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_16_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_16_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_16_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_16_0 => ""
    ."OVRPlugin/OVRP_1_16_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_16_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_16_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_16_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_16_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_16_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_16_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_16_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_16_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_17_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_17_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_17_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_17_0 => ""
    ."OVRPlugin/OVRP_1_17_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_17_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_17_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_17_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_17_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_17_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_17_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_17_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_17_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_18_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_18_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_18_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_18_0 => ""
    ."OVRPlugin/OVRP_1_18_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_18_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_18_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_18_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_18_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_18_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_18_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_18_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_18_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_19_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_19_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_19_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_19_0 => ""
    ."OVRPlugin/OVRP_1_19_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_19_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_19_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_19_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_19_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_19_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_19_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_19_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_19_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_1_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_1_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_1_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_1_0 => ""
    ."OVRPlugin/OVRP_1_1_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_1_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_1_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_1_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_1_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_1_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_1_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_1_0")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_1_1_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_21_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_21_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_21_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_21_0 => ""
    ."OVRPlugin/OVRP_1_21_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_21_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_21_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_21_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_21_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_21_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_21_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_21_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_21_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_28_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_28_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_28_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_28_0 => ""
    ."OVRPlugin/OVRP_1_28_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_28_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_28_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_28_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_28_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_28_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_28_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_28_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_28_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_29_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_29_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_29_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_29_0 => ""
    ."OVRPlugin/OVRP_1_29_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_29_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_29_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_29_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_29_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_29_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_29_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_29_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_29_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_2_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_2_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_2_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_2_0 => ""
    ."OVRPlugin/OVRP_1_2_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_2_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_2_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_2_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_2_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_2_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_2_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_2_0")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_1_2_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_30_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_30_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_30_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_30_0 => ""
    ."OVRPlugin/OVRP_1_30_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_30_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_30_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_30_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_30_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_30_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_30_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_30_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_30_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_31_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_31_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_31_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_31_0 => ""
    ."OVRPlugin/OVRP_1_31_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_31_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_31_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_31_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_31_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_31_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_31_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_31_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_31_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_32_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_32_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_32_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_32_0 => ""
    ."OVRPlugin/OVRP_1_32_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_32_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_32_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_32_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_32_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_32_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_32_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_32_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_32_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_34_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_34_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_34_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_34_0 => ""
    ."OVRPlugin/OVRP_1_34_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_34_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_34_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_34_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_34_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_34_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_34_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_34_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_34_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_35_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_35_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_35_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_35_0 => ""
    ."OVRPlugin/OVRP_1_35_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_35_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_35_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_35_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_35_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_35_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_35_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_35_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_35_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_36_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_36_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_36_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_36_0 => ""
    ."OVRPlugin/OVRP_1_36_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_36_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_36_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_36_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_36_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_36_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_36_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_36_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_36_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_37_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_37_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_37_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_37_0 => ""
    ."OVRPlugin/OVRP_1_37_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_37_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_37_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_37_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_37_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_37_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_37_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_37_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_37_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_38_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_38_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_38_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_38_0 => ""
    ."OVRPlugin/OVRP_1_38_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_38_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_38_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_38_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_38_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_38_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_38_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_38_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_38_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_39_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_39_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_39_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_39_0 => ""
    ."OVRPlugin/OVRP_1_39_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_39_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_39_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_39_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_39_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_39_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_39_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_39_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_39_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_3_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_3_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_3_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_3_0 => ""
    ."OVRPlugin/OVRP_1_3_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_3_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_3_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_3_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_3_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_3_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_3_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_3_0")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_1_3_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_40_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_40_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_40_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_40_0 => ""
    ."OVRPlugin/OVRP_1_40_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_40_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_40_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_40_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_40_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_40_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_40_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_40_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_40_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_41_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_41_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_41_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_41_0 => ""
    ."OVRPlugin/OVRP_1_41_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_41_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_41_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_41_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_41_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_41_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_41_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_41_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_41_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_42_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_42_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_42_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_42_0 => ""
    ."OVRPlugin/OVRP_1_42_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_42_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_42_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_42_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_42_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_42_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_42_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_42_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_42_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_43_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_43_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_43_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_43_0 => ""
    ."OVRPlugin/OVRP_1_43_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_43_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_43_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_43_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_43_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_43_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_43_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_43_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_43_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_44_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_44_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_44_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_44_0 => ""
    ."OVRPlugin/OVRP_1_44_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_44_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_44_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_44_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_44_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_44_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_44_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_44_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_44_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_45_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_45_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_45_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_45_0 => ""
    ."OVRPlugin/OVRP_1_45_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_45_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_45_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_45_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_45_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_45_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_45_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_45_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_45_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_46_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_46_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_46_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_46_0 => ""
    ."OVRPlugin/OVRP_1_46_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_46_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_46_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_46_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_46_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_46_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_46_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_46_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_46_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_47_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_47_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_47_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_47_0 => ""
    ."OVRPlugin/OVRP_1_47_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_47_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_47_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_47_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_47_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_47_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_47_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_47_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_47_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_48_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_48_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_48_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_48_0 => ""
    ."OVRPlugin/OVRP_1_48_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_48_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_48_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_48_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_48_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_48_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_48_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_48_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_48_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_49_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_49_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_49_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_49_0 => ""
    ."OVRPlugin/OVRP_1_49_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_49_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_49_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_49_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_49_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_49_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_49_0 {
    pub const OVRP_ANCHOR_NAME_SIZE: i32 = 32i32;
}
#[cfg(feature = "OVRPlugin+OVRP_1_49_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_49_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_50_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_50_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_50_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_50_0 => ""
    ."OVRPlugin/OVRP_1_50_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_50_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_50_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_50_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_50_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_50_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_50_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_50_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_50_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_51_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_51_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_51_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_51_0 => ""
    ."OVRPlugin/OVRP_1_51_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_51_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_51_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_51_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_51_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_51_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_51_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_51_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_51_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_52_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_52_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_52_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_52_0 => ""
    ."OVRPlugin/OVRP_1_52_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_52_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_52_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_52_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_52_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_52_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_52_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_52_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_52_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_53_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_53_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_53_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_53_0 => ""
    ."OVRPlugin/OVRP_1_53_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_53_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_53_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_53_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_53_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_53_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_53_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_53_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_53_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_54_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_54_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_54_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_54_0 => ""
    ."OVRPlugin/OVRP_1_54_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_54_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_54_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_54_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_54_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_54_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_54_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_54_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_54_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_55_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_55_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_55_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_55_0 => ""
    ."OVRPlugin/OVRP_1_55_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_55_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_55_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_55_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_55_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_55_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_55_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_55_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_55_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_55_1")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_55_1 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_55_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_55_1 => ""
    ."OVRPlugin/OVRP_1_55_1"
);
#[cfg(feature = "OVRPlugin+OVRP_1_55_1")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_55_1 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_55_1")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_55_1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_55_1")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_55_1 {}
#[cfg(feature = "OVRPlugin+OVRP_1_55_1")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_55_1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_56_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_56_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_56_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_56_0 => ""
    ."OVRPlugin/OVRP_1_56_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_56_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_56_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_56_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_56_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_56_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_56_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_56_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_56_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_57_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_57_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_57_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_57_0 => ""
    ."OVRPlugin/OVRP_1_57_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_57_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_57_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_57_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_57_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_57_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_57_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_57_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_57_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_58_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_58_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_58_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_58_0 => ""
    ."OVRPlugin/OVRP_1_58_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_58_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_58_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_58_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_58_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_58_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_58_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_58_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_58_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_59_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_59_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_59_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_59_0 => ""
    ."OVRPlugin/OVRP_1_59_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_59_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_59_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_59_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_59_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_59_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_59_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_59_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_59_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_5_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_5_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_5_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_5_0 => ""
    ."OVRPlugin/OVRP_1_5_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_5_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_5_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_5_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_5_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_5_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_5_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_5_0")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_1_5_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_60_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_60_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_60_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_60_0 => ""
    ."OVRPlugin/OVRP_1_60_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_60_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_60_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_60_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_60_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_60_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_60_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_60_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_60_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_61_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_61_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_61_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_61_0 => ""
    ."OVRPlugin/OVRP_1_61_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_61_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_61_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_61_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_61_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_61_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_61_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_61_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_61_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_62_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_62_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_62_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_62_0 => ""
    ."OVRPlugin/OVRP_1_62_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_62_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_62_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_62_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_62_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_62_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_62_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_62_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_62_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_63_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_63_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_63_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_63_0 => ""
    ."OVRPlugin/OVRP_1_63_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_63_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_63_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_63_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_63_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_63_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_63_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_63_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_63_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_64_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_64_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_64_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_64_0 => ""
    ."OVRPlugin/OVRP_1_64_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_64_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_64_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_64_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_64_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_64_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_64_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_64_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_64_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_65_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_65_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_65_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_65_0 => ""
    ."OVRPlugin/OVRP_1_65_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_65_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_65_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_65_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_65_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_65_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_65_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_65_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_65_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_66_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_66_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_66_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_66_0 => ""
    ."OVRPlugin/OVRP_1_66_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_66_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_66_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_66_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_66_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_66_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_66_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_66_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_66_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_67_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_67_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_67_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_67_0 => ""
    ."OVRPlugin/OVRP_1_67_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_67_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_67_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_67_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_67_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_67_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_67_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_67_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_67_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_68_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_68_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_68_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_68_0 => ""
    ."OVRPlugin/OVRP_1_68_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_68_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_68_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_68_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_68_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_68_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_68_0 {
    pub const OVRP_RENDER_MODEL_MAX_NAME_LENGTH: i32 = 64i32;
    pub const OVRP_RENDER_MODEL_MAX_PATH_LENGTH: i32 = 256i32;
}
#[cfg(feature = "OVRPlugin+OVRP_1_68_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_68_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_69_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_69_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_69_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_69_0 => ""
    ."OVRPlugin/OVRP_1_69_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_69_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_69_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_69_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_69_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_69_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_69_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_69_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_69_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_6_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_6_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_6_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_6_0 => ""
    ."OVRPlugin/OVRP_1_6_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_6_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_6_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_6_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_6_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_6_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_6_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_6_0")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_1_6_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_70_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_70_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_70_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_70_0 => ""
    ."OVRPlugin/OVRP_1_70_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_70_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_70_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_70_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_70_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_70_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_70_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_70_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_70_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_71_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_71_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_71_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_71_0 => ""
    ."OVRPlugin/OVRP_1_71_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_71_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_71_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_71_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_71_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_71_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_71_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_71_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_71_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_72_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_72_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_72_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_72_0 => ""
    ."OVRPlugin/OVRP_1_72_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_72_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_72_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_72_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_72_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_72_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_72_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_72_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_72_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_73_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_73_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_73_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_73_0 => ""
    ."OVRPlugin/OVRP_1_73_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_73_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_73_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_73_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_73_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_73_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_73_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_73_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_73_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_74_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_74_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_74_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_74_0 => ""
    ."OVRPlugin/OVRP_1_74_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_74_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_74_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_74_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_74_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_74_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_74_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_74_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_74_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_75_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_75_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_75_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_75_0 => ""
    ."OVRPlugin/OVRP_1_75_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_75_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_75_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_75_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_75_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_75_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_75_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_75_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_75_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_76_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_76_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_76_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_76_0 => ""
    ."OVRPlugin/OVRP_1_76_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_76_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_76_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_76_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_76_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_76_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_76_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_76_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_76_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_78_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_78_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_78_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_78_0 => ""
    ."OVRPlugin/OVRP_1_78_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_78_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_78_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_78_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_78_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_78_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_78_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_78_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_78_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_79_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_79_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_79_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_79_0 => ""
    ."OVRPlugin/OVRP_1_79_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_79_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_79_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_79_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_79_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_79_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_79_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_79_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_79_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_7_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_7_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_7_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_7_0 => ""
    ."OVRPlugin/OVRP_1_7_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_7_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_7_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_7_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_7_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_7_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_7_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_7_0")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_1_7_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_81_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_81_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_81_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_81_0 => ""
    ."OVRPlugin/OVRP_1_81_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_81_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_81_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_81_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_81_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_81_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_81_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_81_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_81_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_82_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_82_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_82_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_82_0 => ""
    ."OVRPlugin/OVRP_1_82_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_82_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_82_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_82_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_82_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_82_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_82_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_82_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_82_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_83_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_83_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_83_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_83_0 => ""
    ."OVRPlugin/OVRP_1_83_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_83_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_83_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_83_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_83_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_83_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_83_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_83_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_83_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_84_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_84_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_84_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_84_0 => ""
    ."OVRPlugin/OVRP_1_84_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_84_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_84_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_84_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_84_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_84_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_84_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_84_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_84_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_85_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_85_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_85_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_85_0 => ""
    ."OVRPlugin/OVRP_1_85_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_85_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_85_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_85_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_85_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_85_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_85_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_85_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_85_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_86_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_86_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_86_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_86_0 => ""
    ."OVRPlugin/OVRP_1_86_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_86_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_86_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_86_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_86_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_86_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_86_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_86_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_86_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_87_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_87_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_87_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_87_0 => ""
    ."OVRPlugin/OVRP_1_87_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_87_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_87_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_87_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_87_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_87_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_87_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_87_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_87_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_88_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_88_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_88_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_88_0 => ""
    ."OVRPlugin/OVRP_1_88_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_88_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_88_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_88_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_88_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_88_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_88_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_88_0")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_OVRP_1_88_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_8_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_8_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_8_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_8_0 => ""
    ."OVRPlugin/OVRP_1_8_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_8_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_8_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_8_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_8_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_8_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_8_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_8_0")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_1_8_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_9_0")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_OVRP_1_9_0 {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+OVRP_1_9_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_9_0 => ""
    ."OVRPlugin/OVRP_1_9_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_9_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_9_0 {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_9_0")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_OVRP_1_9_0 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+OVRP_1_9_0")]
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_9_0 {}
#[cfg(feature = "OVRPlugin+OVRP_1_9_0")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_OVRP_1_9_0 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+OverlayFlag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_OverlayFlag {
    AutoFiltering = 1024i32,
    BicubicFiltering = 64i32,
    EfficientSharpen = 32i32,
    EfficientSuperSample = 16i32,
    ExpensiveSharpen = 128i32,
    ExpensiveSuperSample = 8i32,
    HeadLocked = 2i32,
    Hidden = 512i32,
    NoDepth = 4i32,
    None = 0i32,
    OnTop = 1i32,
    SecureContent = 256i32,
    ShapeFlagRangeMask = 240i32,
}
#[cfg(feature = "OVRPlugin+OverlayFlag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OverlayFlag => ""
    ."OVRPlugin/OverlayFlag"
);
#[cfg(feature = "OVRPlugin+OverlayShape")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_OverlayShape {
    Cubemap = 2i32,
    Cylinder = 1i32,
    Equirect = 5i32,
    Fisheye = 9i32,
    KeyboardHandsPassthrough = 10i32,
    KeyboardMaskedHandsPassthrough = 11i32,
    OffcenterCubemap = 4i32,
    Quad = 0i32,
    ReconstructionPassthrough = 7i32,
    SurfaceProjectedPassthrough = 8i32,
}
#[cfg(feature = "OVRPlugin+OverlayShape")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OverlayShape => ""
    ."OVRPlugin/OverlayShape"
);
#[cfg(feature = "OVRPlugin+PassthroughCapabilities")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_PassthroughCapabilities {
    pub Fields: crate::GlobalNamespace::OVRPlugin_PassthroughCapabilityFields,
    pub Flags: crate::GlobalNamespace::OVRPlugin_PassthroughCapabilityFlags,
    pub MaxColorLutResolution: u32,
}
#[cfg(feature = "OVRPlugin+PassthroughCapabilities")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_PassthroughCapabilities => ""
    ."OVRPlugin/PassthroughCapabilities"
);
#[cfg(feature = "OVRPlugin+PassthroughCapabilities")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_PassthroughCapabilities {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+PassthroughCapabilities")]
impl crate::GlobalNamespace::OVRPlugin_PassthroughCapabilities {}
#[cfg(feature = "OVRPlugin+PassthroughCapabilityFields")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_PassthroughCapabilityFields {
    Flags = 1i32,
    MaxColorLutResolution = 2i32,
}
#[cfg(feature = "OVRPlugin+PassthroughCapabilityFields")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_PassthroughCapabilityFields => ""
    ."OVRPlugin/PassthroughCapabilityFields"
);
#[cfg(feature = "OVRPlugin+PassthroughCapabilityFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_PassthroughCapabilityFlags {
    Color = 2i32,
    Depth = 4i32,
    Passthrough = 1i32,
}
#[cfg(feature = "OVRPlugin+PassthroughCapabilityFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_PassthroughCapabilityFlags => ""
    ."OVRPlugin/PassthroughCapabilityFlags"
);
#[cfg(feature = "OVRPlugin+PassthroughColorLutChannels")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_PassthroughColorLutChannels {
    Rgb = 1i32,
    Rgba = 2i32,
}
#[cfg(feature = "OVRPlugin+PassthroughColorLutChannels")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_PassthroughColorLutChannels => ""
    ."OVRPlugin/PassthroughColorLutChannels"
);
#[cfg(feature = "OVRPlugin+PassthroughColorLutData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_PassthroughColorLutData {
    pub BufferSize: u32,
    pub Buffer: crate::System::IntPtr,
}
#[cfg(feature = "OVRPlugin+PassthroughColorLutData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_PassthroughColorLutData => ""
    ."OVRPlugin/PassthroughColorLutData"
);
#[cfg(feature = "OVRPlugin+PassthroughColorLutData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+PassthroughColorLutData")]
impl crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData {}
#[cfg(feature = "OVRPlugin+PassthroughPreferenceFields")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_PassthroughPreferenceFields {
    Flags = 1i32,
}
#[cfg(feature = "OVRPlugin+PassthroughPreferenceFields")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_PassthroughPreferenceFields => ""
    ."OVRPlugin/PassthroughPreferenceFields"
);
#[cfg(feature = "OVRPlugin+PassthroughPreferenceFlags")]
#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_PassthroughPreferenceFlags {
    DefaultToActive = 1i64,
}
#[cfg(feature = "OVRPlugin+PassthroughPreferenceFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_PassthroughPreferenceFlags => ""
    ."OVRPlugin/PassthroughPreferenceFlags"
);
#[cfg(feature = "OVRPlugin+PassthroughPreferences")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_PassthroughPreferences {
    pub Fields: crate::GlobalNamespace::OVRPlugin_PassthroughPreferenceFields,
    pub Flags: crate::GlobalNamespace::OVRPlugin_PassthroughPreferenceFlags,
}
#[cfg(feature = "OVRPlugin+PassthroughPreferences")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_PassthroughPreferences => ""
    ."OVRPlugin/PassthroughPreferences"
);
#[cfg(feature = "OVRPlugin+PassthroughPreferences")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_PassthroughPreferences {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+PassthroughPreferences")]
impl crate::GlobalNamespace::OVRPlugin_PassthroughPreferences {}
#[cfg(feature = "OVRPlugin+PerfMetrics")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_PerfMetrics {
    App_CpuTime_Float = 0i32,
    App_GpuTime_Float = 1i32,
    Compositor_CpuTime_Float = 3i32,
    Compositor_DroppedFrameCount_Int = 5i32,
    Compositor_GpuTime_Float = 4i32,
    Compositor_SpaceWarp_Mode_Int = 14i32,
    Count = 40i32,
    Device_CpuClockFrequencyInMHz_Float = 10i32,
    Device_CpuClockLevel_Int = 12i32,
    Device_CpuCore0UtilPercentage_Float = 32i32,
    Device_CpuCore1UtilPercentage_Float = 33i32,
    Device_CpuCore2UtilPercentage_Float = 34i32,
    Device_CpuCore3UtilPercentage_Float = 35i32,
    Device_CpuCore4UtilPercentage_Float = 36i32,
    Device_CpuCore5UtilPercentage_Float = 37i32,
    Device_CpuCore6UtilPercentage_Float = 38i32,
    Device_CpuCore7UtilPercentage_Float = 39i32,
    Device_GpuClockFrequencyInMHz_Float = 11i32,
    Device_GpuClockLevel_Int = 13i32,
    EnumSize = 2147483647i32,
    System_CpuUtilAveragePercentage_Float = 8i32,
    System_CpuUtilWorstPercentage_Float = 9i32,
    System_GpuUtilPercentage_Float = 7i32,
}
#[cfg(feature = "OVRPlugin+PerfMetrics")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_PerfMetrics => ""
    ."OVRPlugin/PerfMetrics"
);
#[cfg(feature = "OVRPlugin+PinnedArray_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_PinnedArray_1<T: quest_hook::libil2cpp::Type> {
    pub _handle: crate::System::Runtime::InteropServices::GCHandle,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "OVRPlugin+PinnedArray_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_PinnedArray_1 < T >
    => ""."OVRPlugin/PinnedArray`1<T>" < T >
);
#[cfg(feature = "OVRPlugin+PinnedArray_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_PinnedArray_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+PinnedArray_1")]
impl<T: quest_hook::libil2cpp::Type> crate::GlobalNamespace::OVRPlugin_PinnedArray_1<T> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (array),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+PlatformUI")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_PlatformUI {
    ConfirmQuit = 1i32,
    GlobalMenuTutorial = 2i32,
    None = -1i32,
}
#[cfg(feature = "OVRPlugin+PlatformUI")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_PlatformUI => ""
    ."OVRPlugin/PlatformUI"
);
#[cfg(feature = "OVRPlugin+PolygonalBoundary2DInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_PolygonalBoundary2DInternal {
    pub vertexCapacityInput: i32,
    pub vertexCountOutput: i32,
    pub vertices: crate::System::IntPtr,
}
#[cfg(feature = "OVRPlugin+PolygonalBoundary2DInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_PolygonalBoundary2DInternal => ""
    ."OVRPlugin/PolygonalBoundary2DInternal"
);
#[cfg(feature = "OVRPlugin+PolygonalBoundary2DInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_PolygonalBoundary2DInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+PolygonalBoundary2DInternal")]
impl crate::GlobalNamespace::OVRPlugin_PolygonalBoundary2DInternal {}
#[cfg(feature = "OVRPlugin+PoseStatef")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_PoseStatef {
    pub Pose: crate::GlobalNamespace::OVRPlugin_Posef,
    pub Velocity: crate::GlobalNamespace::OVRPlugin_Vector3f,
    pub Acceleration: crate::GlobalNamespace::OVRPlugin_Vector3f,
    pub AngularVelocity: crate::GlobalNamespace::OVRPlugin_Vector3f,
    pub AngularAcceleration: crate::GlobalNamespace::OVRPlugin_Vector3f,
    pub Time: f64,
}
#[cfg(feature = "OVRPlugin+PoseStatef")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_PoseStatef => ""
    ."OVRPlugin/PoseStatef"
);
#[cfg(feature = "OVRPlugin+PoseStatef")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_PoseStatef {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+PoseStatef")]
impl crate::GlobalNamespace::OVRPlugin_PoseStatef {}
#[cfg(feature = "OVRPlugin+Posef")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Posef {
    pub Orientation: crate::GlobalNamespace::OVRPlugin_Quatf,
    pub Position: crate::GlobalNamespace::OVRPlugin_Vector3f,
}
#[cfg(feature = "OVRPlugin+Posef")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Posef => ""
    ."OVRPlugin/Posef"
);
#[cfg(feature = "OVRPlugin+Posef")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Posef {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Posef")]
impl crate::GlobalNamespace::OVRPlugin_Posef {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+ProcessorPerformanceLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_ProcessorPerformanceLevel {
    Boost = 3i32,
    EnumSize = 2147483647i32,
    PowerSavings = 0i32,
    SustainedHigh = 2i32,
    SustainedLow = 1i32,
}
#[cfg(feature = "OVRPlugin+ProcessorPerformanceLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel => ""
    ."OVRPlugin/ProcessorPerformanceLevel"
);
#[cfg(feature = "OVRPlugin+Qpl")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_Qpl {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+Qpl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Qpl => ""
    ."OVRPlugin/Qpl"
);
#[cfg(feature = "OVRPlugin+Qpl")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_Qpl {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+Qpl")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_Qpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+Qpl")]
impl crate::GlobalNamespace::OVRPlugin_Qpl {
    pub const AutoSetTimeoutMs: i32 = 0i32;
    pub const AutoSetTimestampMs: i64 = -1i64;
    pub const DefaultInstanceKey: i32 = 0i32;
    #[cfg(feature = "OVRPlugin+Qpl+ResultType")]
    pub type ResultType = crate::GlobalNamespace::Qpl_OVRPlugin_ResultType;
}
#[cfg(feature = "OVRPlugin+Qpl")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlugin_Qpl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+Quatf")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Quatf {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[cfg(feature = "OVRPlugin+Quatf")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Quatf => ""
    ."OVRPlugin/Quatf"
);
#[cfg(feature = "OVRPlugin+Quatf")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Quatf {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Quatf")]
impl crate::GlobalNamespace::OVRPlugin_Quatf {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+RecenterFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_RecenterFlags {
    Count = -2147483647i32,
    Default = 0i32,
    IgnoreAll = -2147483648i32,
}
#[cfg(feature = "OVRPlugin+RecenterFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_RecenterFlags => ""
    ."OVRPlugin/RecenterFlags"
);
#[cfg(feature = "OVRPlugin+Rectf")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Rectf {
    pub Pos: crate::GlobalNamespace::OVRPlugin_Vector2f,
    pub Size: crate::GlobalNamespace::OVRPlugin_Sizef,
}
#[cfg(feature = "OVRPlugin+Rectf")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Rectf => ""
    ."OVRPlugin/Rectf"
);
#[cfg(feature = "OVRPlugin+Rectf")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Rectf {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Rectf")]
impl crate::GlobalNamespace::OVRPlugin_Rectf {}
#[cfg(feature = "OVRPlugin+Recti")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Recti {
    pub Pos: crate::GlobalNamespace::OVRPlugin_Vector2i,
    pub Size: crate::GlobalNamespace::OVRPlugin_Sizei,
}
#[cfg(feature = "OVRPlugin+Recti")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Recti => ""
    ."OVRPlugin/Recti"
);
#[cfg(feature = "OVRPlugin+Recti")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Recti {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Recti")]
impl crate::GlobalNamespace::OVRPlugin_Recti {}
#[cfg(feature = "OVRPlugin+RenderModelFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_RenderModelFlags {
    SupportsGltf20Subset1 = 1i32,
    SupportsGltf20Subset2 = 2i32,
}
#[cfg(feature = "OVRPlugin+RenderModelFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_RenderModelFlags =>
    ""."OVRPlugin/RenderModelFlags"
);
#[cfg(feature = "OVRPlugin+RenderModelProperties")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_RenderModelProperties {
    pub ModelName: *mut crate::System::String,
    pub ModelKey: u64,
    pub VendorId: u32,
    pub ModelVersion: u32,
}
#[cfg(feature = "OVRPlugin+RenderModelProperties")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_RenderModelProperties
    => ""."OVRPlugin/RenderModelProperties"
);
#[cfg(feature = "OVRPlugin+RenderModelProperties")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_RenderModelProperties {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+RenderModelProperties")]
impl crate::GlobalNamespace::OVRPlugin_RenderModelProperties {}
#[cfg(feature = "OVRPlugin+RenderModelPropertiesInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_RenderModelPropertiesInternal {
    pub ModelName: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub ModelKey: u64,
    pub VendorId: u32,
    pub ModelVersion: u32,
}
#[cfg(feature = "OVRPlugin+RenderModelPropertiesInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_RenderModelPropertiesInternal => ""
    ."OVRPlugin/RenderModelPropertiesInternal"
);
#[cfg(feature = "OVRPlugin+RenderModelPropertiesInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_RenderModelPropertiesInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+RenderModelPropertiesInternal")]
impl crate::GlobalNamespace::OVRPlugin_RenderModelPropertiesInternal {}
#[cfg(feature = "OVRPlugin+Result")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_Result {
    Failure = -1000i32,
    Failure_DataIsInvalid = -1008i32,
    Failure_DeprecatedOperation = -1009i32,
    Failure_ErrorInitializationFailed = -1011i32,
    Failure_ErrorLimitReached = -1010i32,
    Failure_InsufficientSize = -1007i32,
    Failure_InvalidOperation = -1003i32,
    Failure_InvalidParameter = -1001i32,
    Failure_NotInitialized = -1002i32,
    Failure_NotYetImplemented = -1005i32,
    Failure_OperationFailed = -1006i32,
    Failure_SpaceCloudStorageDisabled = -2000i32,
    Failure_SpaceLocalizationFailed = -2002i32,
    Failure_SpaceMappingInsufficient = -2001i32,
    Failure_SpaceNetworkRequestFailed = -2004i32,
    Failure_SpaceNetworkTimeout = -2003i32,
    Failure_Unsupported = -1004i32,
    Success = 0i32,
    Success_EventUnavailable = 1i32,
    Success_Pending = 2i32,
}
#[cfg(feature = "OVRPlugin+Result")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Result => ""
    ."OVRPlugin/Result"
);
#[cfg(feature = "OVRPlugin+RoomLayout")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_RoomLayout {
    pub floorUuid: crate::System::Guid,
    pub ceilingUuid: crate::System::Guid,
    pub wallUuids: *mut quest_hook::libil2cpp::Il2CppArray<crate::System::Guid>,
}
#[cfg(feature = "OVRPlugin+RoomLayout")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_RoomLayout => ""
    ."OVRPlugin/RoomLayout"
);
#[cfg(feature = "OVRPlugin+RoomLayout")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_RoomLayout {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+RoomLayout")]
impl crate::GlobalNamespace::OVRPlugin_RoomLayout {}
#[cfg(feature = "OVRPlugin+RoomLayoutInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_RoomLayoutInternal {
    pub floorUuid: crate::System::Guid,
    pub ceilingUuid: crate::System::Guid,
    pub wallUuidCapacityInput: i32,
    pub wallUuidCountOutput: i32,
    pub wallUuids: crate::System::IntPtr,
}
#[cfg(feature = "OVRPlugin+RoomLayoutInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_RoomLayoutInternal =>
    ""."OVRPlugin/RoomLayoutInternal"
);
#[cfg(feature = "OVRPlugin+RoomLayoutInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_RoomLayoutInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+RoomLayoutInternal")]
impl crate::GlobalNamespace::OVRPlugin_RoomLayoutInternal {}
#[cfg(feature = "OVRPlugin+SceneCaptureRequestInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_SceneCaptureRequestInternal {
    pub requestByteCount: i32,
    pub request: *mut crate::System::String,
}
#[cfg(feature = "OVRPlugin+SceneCaptureRequestInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_SceneCaptureRequestInternal => ""
    ."OVRPlugin/SceneCaptureRequestInternal"
);
#[cfg(feature = "OVRPlugin+SceneCaptureRequestInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_SceneCaptureRequestInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+SceneCaptureRequestInternal")]
impl crate::GlobalNamespace::OVRPlugin_SceneCaptureRequestInternal {}
#[cfg(feature = "OVRPlugin+Size3f")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Size3f {
    pub w: f32,
    pub h: f32,
    pub d: f32,
}
#[cfg(feature = "OVRPlugin+Size3f")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Size3f => ""
    ."OVRPlugin/Size3f"
);
#[cfg(feature = "OVRPlugin+Size3f")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Size3f {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Size3f")]
impl crate::GlobalNamespace::OVRPlugin_Size3f {}
#[cfg(feature = "OVRPlugin+Sizef")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Sizef {
    pub w: f32,
    pub h: f32,
}
#[cfg(feature = "OVRPlugin+Sizef")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Sizef => ""
    ."OVRPlugin/Sizef"
);
#[cfg(feature = "OVRPlugin+Sizef")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Sizef {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Sizef")]
impl crate::GlobalNamespace::OVRPlugin_Sizef {}
#[cfg(feature = "OVRPlugin+Sizei")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Sizei {
    pub w: i32,
    pub h: i32,
}
#[cfg(feature = "OVRPlugin+Sizei")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Sizei => ""
    ."OVRPlugin/Sizei"
);
#[cfg(feature = "OVRPlugin+Sizei")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Sizei {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Sizei")]
impl crate::GlobalNamespace::OVRPlugin_Sizei {
    pub fn Equals_OVRPlugin_Sizei0(
        &mut self,
        other: crate::GlobalNamespace::OVRPlugin_Sizei,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+Skeleton")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Skeleton {
    pub Type: crate::GlobalNamespace::OVRPlugin_SkeletonType,
    pub NumBones: u32,
    pub NumBoneCapsules: u32,
    pub Bones: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Bone,
    >,
    pub BoneCapsules: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    >,
}
#[cfg(feature = "OVRPlugin+Skeleton")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Skeleton => ""
    ."OVRPlugin/Skeleton"
);
#[cfg(feature = "OVRPlugin+Skeleton")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Skeleton {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Skeleton")]
impl crate::GlobalNamespace::OVRPlugin_Skeleton {}
#[cfg(feature = "OVRPlugin+Skeleton2")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Skeleton2 {
    pub Type: crate::GlobalNamespace::OVRPlugin_SkeletonType,
    pub NumBones: u32,
    pub NumBoneCapsules: u32,
    pub Bones: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_Bone,
    >,
    pub BoneCapsules: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    >,
}
#[cfg(feature = "OVRPlugin+Skeleton2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Skeleton2 => ""
    ."OVRPlugin/Skeleton2"
);
#[cfg(feature = "OVRPlugin+Skeleton2")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Skeleton2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Skeleton2")]
impl crate::GlobalNamespace::OVRPlugin_Skeleton2 {}
#[cfg(feature = "OVRPlugin+Skeleton2Internal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Skeleton2Internal {
    pub Type: crate::GlobalNamespace::OVRPlugin_SkeletonType,
    pub NumBones: u32,
    pub NumBoneCapsules: u32,
    pub Bones_0: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_1: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_2: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_3: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_4: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_5: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_6: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_7: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_8: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_9: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_10: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_11: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_12: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_13: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_14: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_15: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_16: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_17: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_18: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_19: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_20: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_21: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_22: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_23: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_24: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_25: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_26: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_27: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_28: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_29: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_30: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_31: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_32: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_33: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_34: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_35: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_36: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_37: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_38: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_39: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_40: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_41: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_42: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_43: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_44: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_45: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_46: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_47: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_48: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_49: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_50: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_51: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_52: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_53: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_54: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_55: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_56: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_57: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_58: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_59: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_60: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_61: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_62: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_63: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_64: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_65: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_66: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_67: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_68: crate::GlobalNamespace::OVRPlugin_Bone,
    pub Bones_69: crate::GlobalNamespace::OVRPlugin_Bone,
    pub BoneCapsules_0: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_1: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_2: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_3: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_4: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_5: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_6: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_7: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_8: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_9: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_10: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_11: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_12: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_13: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_14: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_15: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_16: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_17: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
    pub BoneCapsules_18: crate::GlobalNamespace::OVRPlugin_BoneCapsule,
}
#[cfg(feature = "OVRPlugin+Skeleton2Internal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Skeleton2Internal =>
    ""."OVRPlugin/Skeleton2Internal"
);
#[cfg(feature = "OVRPlugin+Skeleton2Internal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Skeleton2Internal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Skeleton2Internal")]
impl crate::GlobalNamespace::OVRPlugin_Skeleton2Internal {}
#[cfg(feature = "OVRPlugin+SkeletonConstants")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_SkeletonConstants {
    MaxBodyBones = 70i32,
    MaxBoneCapsules = 19i32,
    MaxHandBones = 24i32,
    MaxNumMicrogestures = 5i32,
}
#[cfg(feature = "OVRPlugin+SkeletonConstants")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SkeletonConstants =>
    ""."OVRPlugin/SkeletonConstants"
);
#[cfg(feature = "OVRPlugin+SkeletonType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_SkeletonType {
    Body = 2i32,
    HandLeft = 0i32,
    HandRight = 1i32,
    None = -1i32,
}
#[cfg(feature = "OVRPlugin+SkeletonType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SkeletonType => ""
    ."OVRPlugin/SkeletonType"
);
#[cfg(feature = "OVRPlugin+SpaceComponentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_SpaceComponentType {
    Bounded2D = 3i32,
    Bounded3D = 4i32,
    Locatable = 0i32,
    RoomLayout = 6i32,
    SemanticLabels = 5i32,
    Sharable = 2i32,
    SpaceContainer = 7i32,
    Storable = 1i32,
    TriangleMesh = 1000269000i32,
}
#[cfg(feature = "OVRPlugin+SpaceComponentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SpaceComponentType =>
    ""."OVRPlugin/SpaceComponentType"
);
#[cfg(feature = "OVRPlugin+SpaceContainerInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_SpaceContainerInternal {
    pub uuidCapacityInput: i32,
    pub uuidCountOutput: i32,
    pub uuids: crate::System::IntPtr,
}
#[cfg(feature = "OVRPlugin+SpaceContainerInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_SpaceContainerInternal => ""
    ."OVRPlugin/SpaceContainerInternal"
);
#[cfg(feature = "OVRPlugin+SpaceContainerInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_SpaceContainerInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+SpaceContainerInternal")]
impl crate::GlobalNamespace::OVRPlugin_SpaceContainerInternal {}
#[cfg(feature = "OVRPlugin+SpaceFilterInfoComponents")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_SpaceFilterInfoComponents {
    pub Components: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    >,
    pub NumComponents: i32,
}
#[cfg(feature = "OVRPlugin+SpaceFilterInfoComponents")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_SpaceFilterInfoComponents => ""
    ."OVRPlugin/SpaceFilterInfoComponents"
);
#[cfg(feature = "OVRPlugin+SpaceFilterInfoComponents")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_SpaceFilterInfoComponents {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+SpaceFilterInfoComponents")]
impl crate::GlobalNamespace::OVRPlugin_SpaceFilterInfoComponents {}
#[cfg(feature = "OVRPlugin+SpaceFilterInfoIds")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_SpaceFilterInfoIds {
    pub Ids: *mut quest_hook::libil2cpp::Il2CppArray<crate::System::Guid>,
    pub NumIds: i32,
}
#[cfg(feature = "OVRPlugin+SpaceFilterInfoIds")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SpaceFilterInfoIds =>
    ""."OVRPlugin/SpaceFilterInfoIds"
);
#[cfg(feature = "OVRPlugin+SpaceFilterInfoIds")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_SpaceFilterInfoIds {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+SpaceFilterInfoIds")]
impl crate::GlobalNamespace::OVRPlugin_SpaceFilterInfoIds {}
#[cfg(feature = "OVRPlugin+SpaceLocationFlags")]
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_SpaceLocationFlags {
    OrientationTracked = 4u64,
    OrientationValid = 1u64,
    PositionTracked = 8u64,
    PositionValid = 2u64,
}
#[cfg(feature = "OVRPlugin+SpaceLocationFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SpaceLocationFlags =>
    ""."OVRPlugin/SpaceLocationFlags"
);
#[cfg(feature = "OVRPlugin+SpaceLocationf")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_SpaceLocationf {
    pub locationFlags: crate::GlobalNamespace::OVRPlugin_SpaceLocationFlags,
    pub pose: crate::GlobalNamespace::OVRPlugin_Posef,
}
#[cfg(feature = "OVRPlugin+SpaceLocationf")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SpaceLocationf => ""
    ."OVRPlugin/SpaceLocationf"
);
#[cfg(feature = "OVRPlugin+SpaceLocationf")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_SpaceLocationf {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+SpaceLocationf")]
impl crate::GlobalNamespace::OVRPlugin_SpaceLocationf {}
#[cfg(feature = "OVRPlugin+SpaceQueryActionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_SpaceQueryActionType {
    Load = 0i32,
}
#[cfg(feature = "OVRPlugin+SpaceQueryActionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SpaceQueryActionType
    => ""."OVRPlugin/SpaceQueryActionType"
);
#[cfg(feature = "OVRPlugin+SpaceQueryFilterType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_SpaceQueryFilterType {
    Components = 2i32,
    Ids = 1i32,
    None = 0i32,
}
#[cfg(feature = "OVRPlugin+SpaceQueryFilterType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SpaceQueryFilterType
    => ""."OVRPlugin/SpaceQueryFilterType"
);
#[cfg(feature = "OVRPlugin+SpaceQueryInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_SpaceQueryInfo {
    pub QueryType: crate::GlobalNamespace::OVRPlugin_SpaceQueryType,
    pub MaxQuerySpaces: i32,
    pub Timeout: f64,
    pub Location: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
    pub ActionType: crate::GlobalNamespace::OVRPlugin_SpaceQueryActionType,
    pub FilterType: crate::GlobalNamespace::OVRPlugin_SpaceQueryFilterType,
    pub IdInfo: crate::GlobalNamespace::OVRPlugin_SpaceFilterInfoIds,
    pub ComponentsInfo: crate::GlobalNamespace::OVRPlugin_SpaceFilterInfoComponents,
}
#[cfg(feature = "OVRPlugin+SpaceQueryInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SpaceQueryInfo => ""
    ."OVRPlugin/SpaceQueryInfo"
);
#[cfg(feature = "OVRPlugin+SpaceQueryInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+SpaceQueryInfo")]
impl crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo {}
#[cfg(feature = "OVRPlugin+SpaceQueryResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_SpaceQueryResult {
    pub space: u64,
    pub uuid: crate::System::Guid,
}
#[cfg(feature = "OVRPlugin+SpaceQueryResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SpaceQueryResult =>
    ""."OVRPlugin/SpaceQueryResult"
);
#[cfg(feature = "OVRPlugin+SpaceQueryResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_SpaceQueryResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+SpaceQueryResult")]
impl crate::GlobalNamespace::OVRPlugin_SpaceQueryResult {}
#[cfg(feature = "OVRPlugin+SpaceQueryType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_SpaceQueryType {
    Action = 0i32,
}
#[cfg(feature = "OVRPlugin+SpaceQueryType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SpaceQueryType => ""
    ."OVRPlugin/SpaceQueryType"
);
#[cfg(feature = "OVRPlugin+SpaceSemanticLabelInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_SpaceSemanticLabelInternal {
    pub byteCapacityInput: i32,
    pub byteCountOutput: i32,
    pub labels: crate::System::IntPtr,
}
#[cfg(feature = "OVRPlugin+SpaceSemanticLabelInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_SpaceSemanticLabelInternal => ""
    ."OVRPlugin/SpaceSemanticLabelInternal"
);
#[cfg(feature = "OVRPlugin+SpaceSemanticLabelInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_SpaceSemanticLabelInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+SpaceSemanticLabelInternal")]
impl crate::GlobalNamespace::OVRPlugin_SpaceSemanticLabelInternal {}
#[cfg(feature = "OVRPlugin+SpaceStorageLocation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_SpaceStorageLocation {
    Cloud = 2i32,
    Invalid = 0i32,
    Local = 1i32,
}
#[cfg(feature = "OVRPlugin+SpaceStorageLocation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SpaceStorageLocation
    => ""."OVRPlugin/SpaceStorageLocation"
);
#[cfg(feature = "OVRPlugin+SpaceStoragePersistenceMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_SpaceStoragePersistenceMode {
    Indefinite = 1i32,
    Invalid = 0i32,
}
#[cfg(feature = "OVRPlugin+SpaceStoragePersistenceMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_SpaceStoragePersistenceMode => ""
    ."OVRPlugin/SpaceStoragePersistenceMode"
);
#[cfg(feature = "OVRPlugin+SpatialAnchorCreateInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_SpatialAnchorCreateInfo {
    pub BaseTracking: crate::GlobalNamespace::OVRPlugin_TrackingOrigin,
    pub PoseInSpace: crate::GlobalNamespace::OVRPlugin_Posef,
    pub Time: f64,
}
#[cfg(feature = "OVRPlugin+SpatialAnchorCreateInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_SpatialAnchorCreateInfo => ""
    ."OVRPlugin/SpatialAnchorCreateInfo"
);
#[cfg(feature = "OVRPlugin+SpatialAnchorCreateInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_SpatialAnchorCreateInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+SpatialAnchorCreateInfo")]
impl crate::GlobalNamespace::OVRPlugin_SpatialAnchorCreateInfo {}
#[cfg(feature = "OVRPlugin+Step")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_Step {
    Physics = 0i32,
    Render = -1i32,
}
#[cfg(feature = "OVRPlugin+Step")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Step => ""
    ."OVRPlugin/Step"
);
#[cfg(feature = "OVRPlugin+SystemHeadset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_SystemHeadset {
    Meta_Link_Quest_3 = 4104i32,
    Meta_Link_Quest_Pro = 4103i32,
    Meta_Quest_3 = 11i32,
    Meta_Quest_Pro = 10i32,
    None = 0i32,
    Oculus_Link_Quest = 4101i32,
    Oculus_Link_Quest_2 = 4102i32,
    Oculus_Quest = 8i32,
    Oculus_Quest_2 = 9i32,
    PC_Placeholder_4105 = 4105i32,
    PC_Placeholder_4106 = 4106i32,
    PC_Placeholder_4107 = 4107i32,
    Placeholder_12 = 12i32,
    Placeholder_13 = 13i32,
    Placeholder_14 = 14i32,
    Rift_CB = 4099i32,
    Rift_CV1 = 4098i32,
    Rift_DK1 = 4096i32,
    Rift_DK2 = 4097i32,
    Rift_S = 4100i32,
}
#[cfg(feature = "OVRPlugin+SystemHeadset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SystemHeadset => ""
    ."OVRPlugin/SystemHeadset"
);
#[cfg(feature = "OVRPlugin+SystemRegion")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_SystemRegion {
    China = 2i32,
    Japan = 1i32,
    Unspecified = 0i32,
}
#[cfg(feature = "OVRPlugin+SystemRegion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SystemRegion => ""
    ."OVRPlugin/SystemRegion"
);
#[cfg(feature = "OVRPlugin+TextureRectMatrixf")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_TextureRectMatrixf {
    pub leftRect: crate::UnityEngine::Rect,
    pub rightRect: crate::UnityEngine::Rect,
    pub leftScaleBias: crate::UnityEngine::Vector4,
    pub rightScaleBias: crate::UnityEngine::Vector4,
}
#[cfg(feature = "OVRPlugin+TextureRectMatrixf")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_TextureRectMatrixf =>
    ""."OVRPlugin/TextureRectMatrixf"
);
#[cfg(feature = "OVRPlugin+TextureRectMatrixf")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_TextureRectMatrixf {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+TextureRectMatrixf")]
impl crate::GlobalNamespace::OVRPlugin_TextureRectMatrixf {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+TiledMultiResLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_TiledMultiResLevel {
    EnumSize = 2147483647i32,
    LMSHigh = 3i32,
    LMSHighTop = 4i32,
    LMSLow = 1i32,
    LMSMedium = 2i32,
    Off = 0i32,
}
#[cfg(feature = "OVRPlugin+TiledMultiResLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_TiledMultiResLevel =>
    ""."OVRPlugin/TiledMultiResLevel"
);
#[cfg(feature = "OVRPlugin+TrackedKeyboardFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_TrackedKeyboardFlags {
    Connected = 8i32,
    Exists = 1i32,
    Local = 2i32,
    Remote = 4i32,
}
#[cfg(feature = "OVRPlugin+TrackedKeyboardFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_TrackedKeyboardFlags
    => ""."OVRPlugin/TrackedKeyboardFlags"
);
#[cfg(feature = "OVRPlugin+TrackedKeyboardPresentationStyles")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_TrackedKeyboardPresentationStyles {
    KeyLabel = 2i32,
    Opaque = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "OVRPlugin+TrackedKeyboardPresentationStyles")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_TrackedKeyboardPresentationStyles => ""
    ."OVRPlugin/TrackedKeyboardPresentationStyles"
);
#[cfg(feature = "OVRPlugin+TrackedKeyboardQueryFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_TrackedKeyboardQueryFlags {
    Local = 2i32,
    Remote = 4i32,
}
#[cfg(feature = "OVRPlugin+TrackedKeyboardQueryFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_TrackedKeyboardQueryFlags => ""
    ."OVRPlugin/TrackedKeyboardQueryFlags"
);
#[cfg(feature = "OVRPlugin+Tracker")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_Tracker {
    Count = 4i32,
    None = -1i32,
    One = 1i32,
    Three = 3i32,
    Two = 2i32,
    Zero = 0i32,
}
#[cfg(feature = "OVRPlugin+Tracker")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Tracker => ""
    ."OVRPlugin/Tracker"
);
#[cfg(feature = "OVRPlugin+TrackingConfidence")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_TrackingConfidence {
    High = 1065353216i32,
    Low = 0i32,
}
#[cfg(feature = "OVRPlugin+TrackingConfidence")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_TrackingConfidence =>
    ""."OVRPlugin/TrackingConfidence"
);
#[cfg(feature = "OVRPlugin+TrackingOrigin")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_TrackingOrigin {
    Count = 5i32,
    EyeLevel = 0i32,
    FloorLevel = 1i32,
    Stage = 2i32,
    View = 4i32,
}
#[cfg(feature = "OVRPlugin+TrackingOrigin")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_TrackingOrigin => ""
    ."OVRPlugin/TrackingOrigin"
);
#[cfg(feature = "OVRPlugin+TriangleMeshInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_TriangleMeshInternal {
    pub vertexCapacityInput: i32,
    pub vertexCountOutput: i32,
    pub vertices: crate::System::IntPtr,
    pub indexCapacityInput: i32,
    pub indexCountOutput: i32,
    pub indices: crate::System::IntPtr,
}
#[cfg(feature = "OVRPlugin+TriangleMeshInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_TriangleMeshInternal
    => ""."OVRPlugin/TriangleMeshInternal"
);
#[cfg(feature = "OVRPlugin+TriangleMeshInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_TriangleMeshInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+TriangleMeshInternal")]
impl crate::GlobalNamespace::OVRPlugin_TriangleMeshInternal {}
#[cfg(feature = "OVRPlugin+UnityOpenXR")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlugin_UnityOpenXR {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPlugin+UnityOpenXR")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_UnityOpenXR => ""
    ."OVRPlugin/UnityOpenXR"
);
#[cfg(feature = "OVRPlugin+UnityOpenXR")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_UnityOpenXR {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+UnityOpenXR")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlugin_UnityOpenXR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlugin+UnityOpenXR")]
impl crate::GlobalNamespace::OVRPlugin_UnityOpenXR {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+UnityOpenXR")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPlugin_UnityOpenXR {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlugin+Vector2f")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Vector2f {
    pub x: f32,
    pub y: f32,
}
#[cfg(feature = "OVRPlugin+Vector2f")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Vector2f => ""
    ."OVRPlugin/Vector2f"
);
#[cfg(feature = "OVRPlugin+Vector2f")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Vector2f {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Vector2f")]
impl crate::GlobalNamespace::OVRPlugin_Vector2f {}
#[cfg(feature = "OVRPlugin+Vector2i")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Vector2i {
    pub x: i32,
    pub y: i32,
}
#[cfg(feature = "OVRPlugin+Vector2i")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Vector2i => ""
    ."OVRPlugin/Vector2i"
);
#[cfg(feature = "OVRPlugin+Vector2i")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Vector2i {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Vector2i")]
impl crate::GlobalNamespace::OVRPlugin_Vector2i {}
#[cfg(feature = "OVRPlugin+Vector3f")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[cfg(feature = "OVRPlugin+Vector3f")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Vector3f => ""
    ."OVRPlugin/Vector3f"
);
#[cfg(feature = "OVRPlugin+Vector3f")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Vector3f {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Vector3f")]
impl crate::GlobalNamespace::OVRPlugin_Vector3f {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+Vector4f")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[cfg(feature = "OVRPlugin+Vector4f")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Vector4f => ""
    ."OVRPlugin/Vector4f"
);
#[cfg(feature = "OVRPlugin+Vector4f")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Vector4f {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Vector4f")]
impl crate::GlobalNamespace::OVRPlugin_Vector4f {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+Vector4s")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_Vector4s {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub w: i16,
}
#[cfg(feature = "OVRPlugin+Vector4s")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Vector4s => ""
    ."OVRPlugin/Vector4s"
);
#[cfg(feature = "OVRPlugin+Vector4s")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_Vector4s {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+Vector4s")]
impl crate::GlobalNamespace::OVRPlugin_Vector4s {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardCreateInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_VirtualKeyboardCreateInfo {}
#[cfg(feature = "OVRPlugin+VirtualKeyboardCreateInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardCreateInfo => ""
    ."OVRPlugin/VirtualKeyboardCreateInfo"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardCreateInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_VirtualKeyboardCreateInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardCreateInfo")]
impl crate::GlobalNamespace::OVRPlugin_VirtualKeyboardCreateInfo {}
#[cfg(feature = "OVRPlugin+VirtualKeyboardInputInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_VirtualKeyboardInputInfo {
    pub inputSource: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardInputSource,
    pub inputPose: crate::GlobalNamespace::OVRPlugin_Posef,
    pub inputState: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardInputStateFlags,
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardInputInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardInputInfo => ""
    ."OVRPlugin/VirtualKeyboardInputInfo"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardInputInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_VirtualKeyboardInputInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardInputInfo")]
impl crate::GlobalNamespace::OVRPlugin_VirtualKeyboardInputInfo {}
#[cfg(feature = "OVRPlugin+VirtualKeyboardInputSource")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_VirtualKeyboardInputSource {
    ControllerDirectLeft = 5i32,
    ControllerDirectRight = 6i32,
    ControllerRayLeft = 1i32,
    ControllerRayRight = 2i32,
    EnumSize = 2147483647i32,
    HandDirectIndexTipLeft = 7i32,
    HandDirectIndexTipRight = 8i32,
    HandRayLeft = 3i32,
    HandRayRight = 4i32,
    Invalid = 0i32,
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardInputSource")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardInputSource => ""
    ."OVRPlugin/VirtualKeyboardInputSource"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardInputStateFlags")]
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_VirtualKeyboardInputStateFlags {
    IsPressed = 1u64,
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardInputStateFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardInputStateFlags => ""
    ."OVRPlugin/VirtualKeyboardInputStateFlags"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardLocationInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_VirtualKeyboardLocationInfo {
    pub locationType: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationType,
    pub pose: crate::GlobalNamespace::OVRPlugin_Posef,
    pub scale: f32,
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardLocationInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationInfo => ""
    ."OVRPlugin/VirtualKeyboardLocationInfo"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardLocationInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardLocationInfo")]
impl crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationInfo {}
#[cfg(feature = "OVRPlugin+VirtualKeyboardLocationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_VirtualKeyboardLocationType {
    Custom = 0i32,
    Direct = 2i32,
    Far = 1i32,
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardLocationType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationType => ""
    ."OVRPlugin/VirtualKeyboardLocationType"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_VirtualKeyboardModelAnimationState {
    pub AnimationIndex: i32,
    pub Fraction: f32,
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationState => ""
    ."OVRPlugin/VirtualKeyboardModelAnimationState"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationState")]
impl crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationState {}
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationStates")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_VirtualKeyboardModelAnimationStates {
    pub States: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationState,
    >,
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationStates")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationStates => ""
    ."OVRPlugin/VirtualKeyboardModelAnimationStates"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationStates")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationStates {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationStates")]
impl crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationStates {}
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationStatesInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_VirtualKeyboardModelAnimationStatesInternal {
    pub StateCapacityInput: u32,
    pub StateCountOutput: u32,
    pub StatesBuffer: crate::System::IntPtr,
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationStatesInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationStatesInternal => ""
    ."OVRPlugin/VirtualKeyboardModelAnimationStatesInternal"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationStatesInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationStatesInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelAnimationStatesInternal")]
impl crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationStatesInternal {}
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelVisibility")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_VirtualKeyboardModelVisibility {
    pub _visible: crate::GlobalNamespace::OVRPlugin_Bool,
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelVisibility")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardModelVisibility => ""
    ."OVRPlugin/VirtualKeyboardModelVisibility"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelVisibility")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelVisibility {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardModelVisibility")]
impl crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelVisibility {
    pub fn get_Visible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Visible",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Visible(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Visible",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardSpaceCreateInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_VirtualKeyboardSpaceCreateInfo {
    pub locationType: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationType,
    pub pose: crate::GlobalNamespace::OVRPlugin_Posef,
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardSpaceCreateInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardSpaceCreateInfo => ""
    ."OVRPlugin/VirtualKeyboardSpaceCreateInfo"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardSpaceCreateInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_VirtualKeyboardSpaceCreateInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardSpaceCreateInfo")]
impl crate::GlobalNamespace::OVRPlugin_VirtualKeyboardSpaceCreateInfo {}
#[cfg(feature = "OVRPlugin+VirtualKeyboardTextureData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_VirtualKeyboardTextureData {
    pub TextureWidth: u32,
    pub TextureHeight: u32,
    pub BufferCapacityInput: u32,
    pub BufferCountOutput: u32,
    pub Buffer: crate::System::IntPtr,
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardTextureData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureData => ""
    ."OVRPlugin/VirtualKeyboardTextureData"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardTextureData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardTextureData")]
impl crate::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureData {}
#[cfg(feature = "OVRPlugin+VirtualKeyboardTextureIds")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_VirtualKeyboardTextureIds {
    pub TextureIds: *mut quest_hook::libil2cpp::Il2CppArray<u64>,
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardTextureIds")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureIds => ""
    ."OVRPlugin/VirtualKeyboardTextureIds"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardTextureIds")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureIds {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardTextureIds")]
impl crate::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureIds {}
#[cfg(feature = "OVRPlugin+VirtualKeyboardTextureIdsInternal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPlugin_VirtualKeyboardTextureIdsInternal {
    pub TextureIdCapacityInput: u32,
    pub TextureIdCountOutput: u32,
    pub TextureIdsBuffer: crate::System::IntPtr,
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardTextureIdsInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureIdsInternal => ""
    ."OVRPlugin/VirtualKeyboardTextureIdsInternal"
);
#[cfg(feature = "OVRPlugin+VirtualKeyboardTextureIdsInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureIdsInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardTextureIdsInternal")]
impl crate::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureIdsInternal {}
#[cfg(feature = "OVRPlugin+XrApi")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPlugin_XrApi {
    CAPI = 1i32,
    EnumSize = 2147483647i32,
    OpenXR = 3i32,
    Unknown = 0i32,
    VRAPI = 2i32,
}
#[cfg(feature = "OVRPlugin+XrApi")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_XrApi => ""
    ."OVRPlugin/XrApi"
);
#[cfg(feature = "OVRPlugin+Qpl+ResultType")]
#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Qpl_OVRPlugin_ResultType {
    Cancel = 4i16,
    Fail = 3i16,
    Success = 2i16,
}
#[cfg(feature = "OVRPlugin+Qpl+ResultType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Qpl_OVRPlugin_ResultType => ""
    ."OVRPlugin/Qpl/ResultType"
);
