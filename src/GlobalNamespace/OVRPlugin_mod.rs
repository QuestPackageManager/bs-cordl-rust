#[cfg(feature = "OVRPlugin+Media+InputVideoBufferType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Media_OVRPlugin_InputVideoBufferType {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Media_OVRPlugin_MrcActivationMode {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Media_OVRPlugin_PlatformCameraMode {
    #[default]
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin => ""."OVRPlugin"
);
#[cfg(feature = "OVRPlugin")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn AddCustomMetadata(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        param: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddCustomMetadata", (name, param))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddInsightPassthroughSurfaceGeometry(
        layerId: i32,
        meshHandle: u64,
        T_world_model: crate::UnityEngine::Matrix4x4,
        geometryInstanceHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddInsightPassthroughSurfaceGeometry",
                (layerId, meshHandle, T_world_model, geometryInstanceHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AreControllerDrivenHandPosesNatural() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreControllerDrivenHandPosesNatural", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AreHandPosesGeneratedByControllerData(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreHandPosesGeneratedByControllerData", (stepId, nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateLayerDesc(
        shape: crate::GlobalNamespace::OVRPlugin_OverlayShape,
        layout: crate::GlobalNamespace::OVRPlugin_LayerLayout,
        textureSize: crate::GlobalNamespace::OVRPlugin_Sizei,
        mipLevels: i32,
        sampleCount: i32,
        format: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
        layerFlags: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_LayerDesc> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_LayerDesc = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CalculateLayerDesc",
                (shape, layout, textureSize, mipLevels, sampleCount, format, layerFlags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeVirtualKeyboardTextContext(
        textContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ChangeVirtualKeyboardTextContext", (textContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInsightTriangleMesh(
        layerId: i32,
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        triangles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        meshHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateInsightTriangleMesh",
                (layerId, vertices, triangles, meshHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePassthroughColorLut(
        channels: crate::GlobalNamespace::OVRPlugin_PassthroughColorLutChannels,
        resolution: u32,
        data: crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData,
        colorLut: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreatePassthroughColorLut",
                (channels, resolution, data, colorLut),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpaceUser(
        spaceUserId: u64,
        spaceUserHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSpaceUser", (spaceUserId, spaceUserHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSpatialAnchor(
        createInfo: crate::GlobalNamespace::OVRPlugin_SpatialAnchorCreateInfo,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSpatialAnchor", (createInfo, requestId))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateVirtualKeyboard(
        createInfo: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardCreateInfo,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateVirtualKeyboard", (createInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateVirtualKeyboardSpace(
        createInfo: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardSpaceCreateInfo,
        keyboardSpace: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateVirtualKeyboardSpace", (createInfo, keyboardSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyInsightPassthroughGeometryInstance(
        geometryInstanceHandle: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DestroyInsightPassthroughGeometryInstance",
                (geometryInstanceHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyInsightTriangleMesh(
        meshHandle: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyInsightTriangleMesh", (meshHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyPassthroughColorLut(
        colorLut: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyPassthroughColorLut", (colorLut))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroySpace(space: u64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroySpace", (space))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroySpaceUser(
        spaceUserHandle: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroySpaceUser", (spaceUserHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyVirtualKeyboard() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyVirtualKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnqueueDestroyLayer(
        layerID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnqueueDestroyLayer", (layerID))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnqueueSetupLayer(
        desc: crate::GlobalNamespace::OVRPlugin_LayerDesc,
        compositionDepth: i32,
        layerID: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnqueueSetupLayer", (desc, compositionDepth, layerID))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnqueueSubmitLayer(
        onTop: bool,
        headLocked: bool,
        noDepthBufferTesting: bool,
        leftTexture: crate::System::IntPtr,
        rightTexture: crate::System::IntPtr,
        layerId: i32,
        frameIndex: i32,
        pose: crate::GlobalNamespace::OVRPlugin_Posef,
        scale: crate::GlobalNamespace::OVRPlugin_Vector3f,
        layerIndex: i32,
        shape: crate::GlobalNamespace::OVRPlugin_OverlayShape,
        overrideTextureRectMatrix: bool,
        textureRectMatrix: crate::GlobalNamespace::OVRPlugin_TextureRectMatrixf,
        overridePerLayerColorScaleAndOffset: bool,
        colorScale: crate::UnityEngine::Vector4,
        colorOffset: crate::UnityEngine::Vector4,
        expensiveSuperSample: bool,
        bicubic: bool,
        efficientSuperSample: bool,
        efficientSharpen: bool,
        expensiveSharpen: bool,
        hidden: bool,
        secureContent: bool,
        automaticFiltering: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EnqueueSubmitLayer",
                (
                    onTop,
                    headLocked,
                    noDepthBufferTesting,
                    leftTexture,
                    rightTexture,
                    layerId,
                    frameIndex,
                    pose,
                    scale,
                    layerIndex,
                    shape,
                    overrideTextureRectMatrix,
                    textureRectMatrix,
                    overridePerLayerColorScaleAndOffset,
                    colorScale,
                    colorOffset,
                    expensiveSuperSample,
                    bicubic,
                    efficientSuperSample,
                    efficientSharpen,
                    expensiveSharpen,
                    hidden,
                    secureContent,
                    automaticFiltering,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EnumerateSpaceSupportedComponents(
        space: u64,
        numSupportedComponents: quest_hook::libil2cpp::ByRefMut<u32>,
        supportedComponents: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EnumerateSpaceSupportedComponents",
                (space, numSupportedComponents, supportedComponents),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EraseSpace(
        space: u64,
        location: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EraseSpace", (space, location, requestId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActiveController() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Controller,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Controller = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetActiveController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAdaptiveGPUPerformanceScale() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAdaptiveGPUPerformanceScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAppCpuStartToGpuEndTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAppCpuStartToGpuEndTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAppFramerate() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAppFramerate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAppPerfStats() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_AppPerfStats,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_AppPerfStats = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAppPerfStats", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBodyState(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        bodyState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_BodyState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBodyState", (stepId, bodyState))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryConfigured() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryConfigured", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryDimensions(
        boundaryType: crate::GlobalNamespace::OVRPlugin_BoundaryType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector3f> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector3f = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryDimensions", (boundaryType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryGeometry(
        boundaryType: crate::GlobalNamespace::OVRPlugin_BoundaryType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_BoundaryGeometry,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_BoundaryGeometry = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryGeometry", (boundaryType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryGeometry2(
        boundaryType: crate::GlobalNamespace::OVRPlugin_BoundaryType,
        points: crate::System::IntPtr,
        pointsCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryGeometry2", (boundaryType, points, pointsCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryVisible() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryVisible", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConnectedControllers() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Controller,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Controller = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetConnectedControllers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerHapticsDesc(
        controllerMask: u32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_HapticsDesc> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_HapticsDesc = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetControllerHapticsDesc", (controllerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerHapticsState(
        controllerMask: u32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_HapticsState> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_HapticsState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetControllerHapticsState", (controllerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerIsInHand(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetControllerIsInHand", (stepId, nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerSampleRateHz(
        controllerMask: crate::GlobalNamespace::OVRPlugin_Controller,
        sampleRateHz: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetControllerSampleRateHz", (controllerMask, sampleRateHz))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerState(
        controllerMask: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_ControllerState,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_ControllerState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetControllerState", (controllerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerState2(
        controllerMask: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_ControllerState2,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_ControllerState2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetControllerState2", (controllerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerState4(
        controllerMask: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_ControllerState4,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_ControllerState4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetControllerState4", (controllerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerState5(
        controllerMask: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_ControllerState5,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_ControllerState5 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetControllerState5", (controllerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControllerState6(
        controllerMask: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_ControllerState6,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_ControllerState6 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetControllerState6", (controllerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentDetachedInteractionProfile(
        hand: crate::GlobalNamespace::OVRPlugin_Hand,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_InteractionProfile,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_InteractionProfile = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentDetachedInteractionProfile", (hand))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentInteractionProfile(
        hand: crate::GlobalNamespace::OVRPlugin_Hand,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_InteractionProfile,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_InteractionProfile = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentInteractionProfile", (hand))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentTrackingTransformPose() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Posef,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentTrackingTransformPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDesiredEyeTextureFormat() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDesiredEyeTextureFormat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDominantHand() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Handedness,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Handedness = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDominantHand", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExternalCameraCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExternalCameraCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEyeFrustum(
        eyeId: crate::GlobalNamespace::OVRPlugin_Eye,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Frustumf> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Frustumf = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEyeFrustum", (eyeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEyeGazesState(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        frameIndex: i32,
        eyeGazesState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_EyeGazesState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEyeGazesState", (stepId, frameIndex, eyeGazesState))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEyeLayerRecommendedResolution(
        recommendedSize: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Sizei,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEyeLayerRecommendedResolution", (recommendedSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEyeRecommendedResolutionScale() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEyeRecommendedResolutionScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEyeTextureSize(
        eyeId: crate::GlobalNamespace::OVRPlugin_Eye,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Sizei> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Sizei = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEyeTextureSize", (eyeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceState(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        frameIndex: i32,
        faceState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_FaceState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFaceState", (stepId, frameIndex, faceState))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceStateInternal(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        frameIndex: i32,
        faceState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_FaceState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFaceStateInternal", (stepId, frameIndex, faceState))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHandNodePoseStateLatency() -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHandNodePoseStateLatency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHandState(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        hand: crate::GlobalNamespace::OVRPlugin_Hand,
        handState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_HandState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHandState", (stepId, hand, handState))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHandTrackingEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHandTrackingEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHeadPoseModifier(
        relativeRotation: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Quatf,
        >,
        relativeTranslation: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Vector3f,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHeadPoseModifier", (relativeRotation, relativeTranslation))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHmdColorDesc() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_ColorSpace,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_ColorSpace = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHmdColorDesc", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInsightPassthroughInitializationState() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInsightPassthroughInitializationState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyboardState(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        keyboardState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_KeyboardState,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyboardState", (stepId, keyboardState))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLayerAndroidSurfaceObject(
        layerId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLayerAndroidSurfaceObject", (layerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLayerRecommendedResolution(
        layerId: i32,
        recommendedSize: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Sizei,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLayerRecommendedResolution", (layerId, recommendedSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLayerTexture(
        layerId: i32,
        stage: i32,
        eyeId: crate::GlobalNamespace::OVRPlugin_Eye,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLayerTexture", (layerId, stage, eyeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLayerTextureStageCount(
        layerId: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLayerTextureStageCount", (layerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalTrackingSpaceRecenterCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLocalTrackingSpaceRecenterCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMesh(
        meshType: crate::GlobalNamespace::OVRPlugin_MeshType,
        mesh: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRPlugin_Mesh>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMesh", (meshType, mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMixedRealityCameraInfo(
        cameraId: i32,
        cameraExtrinsics: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_CameraExtrinsics,
        >,
        cameraIntrinsics: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_CameraIntrinsics,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetMixedRealityCameraInfo",
                (cameraId, cameraExtrinsics, cameraIntrinsics),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNativeOpenXRInstance() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNativeOpenXRInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNativeOpenXRSession() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNativeOpenXRSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeAcceleration(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector3f> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector3f = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodeAcceleration", (nodeId, stepId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeAngularAcceleration(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector3f> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector3f = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodeAngularAcceleration", (nodeId, stepId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeAngularVelocity(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector3f> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector3f = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodeAngularVelocity", (nodeId, stepId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeFrustum2(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        frustum: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Frustumf2,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodeFrustum2", (nodeId, frustum))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeOrientationTracked(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodeOrientationTracked", (nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeOrientationValid(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodeOrientationValid", (nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodePose(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodePose", (nodeId, stepId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodePoseStateAtTime(
        _cordl_time: f64,
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_PoseStatef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_PoseStatef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodePoseStateAtTime", (_cordl_time, nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodePoseStateImmediate(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_PoseStatef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_PoseStatef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodePoseStateImmediate", (nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodePoseStateRaw(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_PoseStatef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_PoseStatef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodePoseStateRaw", (nodeId, stepId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodePositionTracked(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodePositionTracked", (nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodePositionValid(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodePositionValid", (nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodePresent(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodePresent", (nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNodeVelocity(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector3f> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector3f = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNodeVelocity", (nodeId, stepId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPassthroughCapabilities(
        outCapabilities: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_PassthroughCapabilities,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPassthroughCapabilities", (outCapabilities))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPassthroughCapabilityFlags() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_PassthroughCapabilityFlags,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_PassthroughCapabilityFlags = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPassthroughCapabilityFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPassthroughPreferences(
        preferences: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_PassthroughPreferences,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPassthroughPreferences", (preferences))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPerfMetricsFloat(
        perfMetrics: crate::GlobalNamespace::OVRPlugin_PerfMetrics,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<f32>> {
        let __cordl_ret: crate::System::Nullable_1<f32> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPerfMetricsFloat", (perfMetrics))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPerfMetricsInt(
        perfMetrics: crate::GlobalNamespace::OVRPlugin_PerfMetrics,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_ret: crate::System::Nullable_1<i32> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPerfMetricsInt", (perfMetrics))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderModelPaths() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRenderModelPaths", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderModelProperties(
        modelPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        modelProperties: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_RenderModelProperties,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRenderModelProperties", (modelPath, modelProperties))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSkeleton(
        skeletonType: crate::GlobalNamespace::OVRPlugin_SkeletonType,
        skeleton: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Skeleton,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSkeleton", (skeletonType, skeleton))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSkeleton2(
        skeletonType: crate::GlobalNamespace::OVRPlugin_SkeletonType,
        skeleton: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Skeleton2,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSkeleton2", (skeletonType, skeleton))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceBoundary2DCount(
        space: u64,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceBoundary2DCount", (space, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceBoundary2D_Allocator2(
        space: u64,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector2>,
    > {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Vector2,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceBoundary2D", (space, allocator))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceBoundary2D_ByRefMut3(
        space: u64,
        boundary: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceBoundary2D", (space, boundary))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceBoundary2D_NativeArray_1_0(
        space: u64,
        boundary: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceBoundary2D", (space, boundary))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceBoundary2D_NativeArray_1_ByRefMut1(
        space: u64,
        boundary: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector2>,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceBoundary2D", (space, boundary, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceBoundingBox2D(
        space: u64,
        rect: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Rectf>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceBoundingBox2D", (space, rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceBoundingBox3D(
        space: u64,
        bounds: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Boundsf,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceBoundingBox3D", (space, bounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceComponentStatus(
        space: u64,
        componentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        enabled: quest_hook::libil2cpp::ByRefMut<bool>,
        changePending: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetSpaceComponentStatus",
                (space, componentType, enabled, changePending),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceComponentStatusInternal(
        space: u64,
        componentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        enabled: quest_hook::libil2cpp::ByRefMut<bool>,
        changePending: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetSpaceComponentStatusInternal",
                (space, componentType, enabled, changePending),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceContainer(
        space: u64,
        containerUuids: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::System::Guid>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceContainer", (space, containerUuids))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceRoomLayout(
        space: u64,
        roomLayout: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_RoomLayout,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceRoomLayout", (space, roomLayout))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceSemanticLabels(
        space: u64,
        labels: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceSemanticLabels", (space, labels))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceTriangleMesh(
        space: u64,
        vertices: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
        triangles: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceTriangleMesh", (space, vertices, triangles))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceTriangleMeshCounts(
        space: u64,
        vertexCount: quest_hook::libil2cpp::ByRefMut<i32>,
        triangleCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceTriangleMeshCounts", (space, vertexCount, triangleCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceUserId(
        spaceUserHandle: u64,
        spaceUserId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceUserId", (spaceUserHandle, spaceUserId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceUuid(
        space: u64,
        uuid: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceUuid", (space, uuid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemHeadsetType() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SystemHeadset,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SystemHeadset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSystemHeadsetType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemHmd3DofModeEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSystemHmd3DofModeEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemKeyboardDescription(
        keyboardQueryFlags: crate::GlobalNamespace::OVRPlugin_TrackedKeyboardQueryFlags,
        keyboardDescription: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_KeyboardDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetSystemKeyboardDescription",
                (keyboardQueryFlags, keyboardDescription),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeInSeconds() -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTimeInSeconds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackerFrustum(
        trackerId: crate::GlobalNamespace::OVRPlugin_Tracker,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Frustumf> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Frustumf = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTrackerFrustum", (trackerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackerPose(
        trackerId: crate::GlobalNamespace::OVRPlugin_Tracker,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTrackerPose", (trackerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackingCalibratedOrigin() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Posef,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTrackingCalibratedOrigin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackingOriginType() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_TrackingOrigin,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_TrackingOrigin = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTrackingOriginType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackingTransformRawPose() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Posef,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTrackingTransformRawPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackingTransformRelativePose(
        trackingOrigin: crate::GlobalNamespace::OVRPlugin_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTrackingTransformRelativePose", (trackingOrigin))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUseOverriddenExternalCameraFov(
        cameraId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUseOverriddenExternalCameraFov", (cameraId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUseOverriddenExternalCameraStaticPose(
        cameraId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUseOverriddenExternalCameraStaticPose", (cameraId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVirtualKeyboardDirtyTextures(
        textureIds: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureIds,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVirtualKeyboardDirtyTextures", (textureIds))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVirtualKeyboardModelAnimationStates(
        animationStates: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationStates,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVirtualKeyboardModelAnimationStates", (animationStates))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVirtualKeyboardScale(
        scale: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVirtualKeyboardScale", (scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVirtualKeyboardTextureData(
        textureId: u64,
        textureData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureData,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVirtualKeyboardTextureData", (textureId, textureData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GuidToUuidString(
        guid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GuidToUuidString", (guid))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeInsightPassthrough() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeInsightPassthrough", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeMixedReality() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeMixedReality", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsControllerDrivenHandPosesEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsControllerDrivenHandPosesEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInsightPassthroughInitialized() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInsightPassthroughInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInsightPassthroughSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInsightPassthroughSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMixedRealityInitialized() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMixedRealityInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMultimodalHandsControllersSupported() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMultimodalHandsControllersSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOrientationTracked(
        value: crate::GlobalNamespace::OVRPlugin_SpaceLocationFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsOrientationTracked", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOrientationValid(
        value: crate::GlobalNamespace::OVRPlugin_SpaceLocationFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsOrientationValid", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPassthroughShape(
        shape: crate::GlobalNamespace::OVRPlugin_OverlayShape,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPassthroughShape", (shape))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPerfMetricsSupported(
        perfMetrics: crate::GlobalNamespace::OVRPlugin_PerfMetrics,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPerfMetricsSupported", (perfMetrics))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPositionTracked(
        value: crate::GlobalNamespace::OVRPlugin_SpaceLocationFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPositionTracked", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPositionValid(
        value: crate::GlobalNamespace::OVRPlugin_SpaceLocationFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPositionValid", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSuccess(
        result: crate::GlobalNamespace::OVRPlugin_Result,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSuccess", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidBone(
        bone: crate::GlobalNamespace::OVRPlugin_BoneId,
        skeletonType: crate::GlobalNamespace::OVRPlugin_SkeletonType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidBone", (bone, skeletonType))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadRenderModel(
        modelKey: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadRenderModel", (modelKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocateSpace(
        space: u64,
        baseOrigin: crate::GlobalNamespace::OVRPlugin_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocateSpace", (space, baseOrigin))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEditorShutdown() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnEditorShutdown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OverrideExternalCameraFov(
        cameraId: i32,
        useOverriddenFov: bool,
        fov: crate::GlobalNamespace::OVRPlugin_Fovf,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverrideExternalCameraFov", (cameraId, useOverriddenFov, fov))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverrideExternalCameraStaticPose(
        cameraId: i32,
        useOverriddenPose: bool,
        poseInStageOrigin: crate::GlobalNamespace::OVRPlugin_Posef,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OverrideExternalCameraStaticPose",
                (cameraId, useOverriddenPose, poseInStageOrigin),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PollEvent(
        eventDataBuffer: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_EventDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PollEvent", (eventDataBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn QuerySpaces(
        queryInfo: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QuerySpaces", (queryInfo, requestId))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecenterTrackingOrigin(
        flags: crate::GlobalNamespace::OVRPlugin_RecenterFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RecenterTrackingOrigin", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestSceneCapture(
        requestString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RequestSceneCapture", (requestString, requestId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetAppPerfStats() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetAppPerfStats", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetDefaultExternalCamera() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetDefaultExternalCamera", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RetrieveSpaceQueryResults_Allocator0(
        requestId: u64,
        results: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<
                crate::GlobalNamespace::OVRPlugin_SpaceQueryResult,
            >,
        >,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RetrieveSpaceQueryResults", (requestId, results, allocator))?;
        Ok(__cordl_ret.into())
    }
    pub fn RetrieveSpaceQueryResults_u64_ByRefMut1(
        requestId: u64,
        results: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::GlobalNamespace::OVRPlugin_SpaceQueryResult,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RetrieveSpaceQueryResults", (requestId, results))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveSpace(
        space: u64,
        location: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
        mode: crate::GlobalNamespace::OVRPlugin_SpaceStoragePersistenceMode,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveSpace", (space, location, mode, requestId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveSpaceList(
        spaces: crate::Unity::Collections::NativeArray_1<u64>,
        location: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SaveSpaceList", (spaces, location, requestId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendEvent(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        param: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SendEvent", (name, param, source))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendVirtualKeyboardInput(
        inputInfo: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardInputInfo,
        interactorRootPose: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Posef,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SendVirtualKeyboardInput", (inputInfo, interactorRootPose))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBoundaryVisible(value: bool) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetBoundaryVisible", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetClientColorDesc(
        colorSpace: crate::GlobalNamespace::OVRPlugin_ColorSpace,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetClientColorDesc", (colorSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorScaleAndOffset(
        colorScale: crate::UnityEngine::Vector4,
        colorOffset: crate::UnityEngine::Vector4,
        applyToAllLayers: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetColorScaleAndOffset",
                (colorScale, colorOffset, applyToAllLayers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetControllerDrivenHandPoses(
        controllerDrivenHandPoses: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetControllerDrivenHandPoses", (controllerDrivenHandPoses))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetControllerDrivenHandPosesAreNatural(
        controllerDrivenHandPosesAreNatural: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetControllerDrivenHandPosesAreNatural",
                (controllerDrivenHandPosesAreNatural),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetControllerHaptics(
        controllerMask: u32,
        hapticsBuffer: crate::GlobalNamespace::OVRPlugin_HapticsBuffer,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetControllerHaptics", (controllerMask, hapticsBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetControllerHapticsAmplitudeEnvelope(
        controllerMask: crate::GlobalNamespace::OVRPlugin_Controller,
        hapticsVibration: crate::GlobalNamespace::OVRPlugin_HapticsAmplitudeEnvelopeVibration,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetControllerHapticsAmplitudeEnvelope",
                (controllerMask, hapticsVibration),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetControllerHapticsPcm(
        controllerMask: crate::GlobalNamespace::OVRPlugin_Controller,
        hapticsVibration: crate::GlobalNamespace::OVRPlugin_HapticsPcmVibration,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetControllerHapticsPcm", (controllerMask, hapticsVibration))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetControllerLocalizedVibration(
        controllerMask: crate::GlobalNamespace::OVRPlugin_Controller,
        hapticsLocationMask: crate::GlobalNamespace::OVRPlugin_HapticsLocation,
        frequency: f32,
        amplitude: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetControllerLocalizedVibration",
                (controllerMask, hapticsLocationMask, frequency, amplitude),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetControllerVibration(
        controllerMask: u32,
        frequency: f32,
        amplitude: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetControllerVibration", (controllerMask, frequency, amplitude))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDefaultExternalCamera(
        cameraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cameraIntrinsics: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_CameraIntrinsics,
        >,
        cameraExtrinsics: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_CameraExtrinsics,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetDefaultExternalCamera",
                (cameraName, cameraIntrinsics, cameraExtrinsics),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDesiredEyeTextureFormat(
        value: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDesiredEyeTextureFormat", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDeveloperMode(
        active: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDeveloperMode", (active))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetExternalCameraProperties(
        cameraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cameraIntrinsics: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_CameraIntrinsics,
        >,
        cameraExtrinsics: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_CameraExtrinsics,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetExternalCameraProperties",
                (cameraName, cameraIntrinsics, cameraExtrinsics),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetEyeBufferSharpenType(
        sharpenType: crate::GlobalNamespace::OVRPlugin_LayerSharpenType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetEyeBufferSharpenType", (sharpenType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetHandNodePoseStateLatency(
        latencyInSeconds: f64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetHandNodePoseStateLatency", (latencyInSeconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetHeadPoseModifier(
        relativeRotation: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Quatf,
        >,
        relativeTranslation: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Vector3f,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetHeadPoseModifier", (relativeRotation, relativeTranslation))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInsightPassthroughKeyboardHandsIntensity(
        layerId: i32,
        intensity: crate::GlobalNamespace::OVRPlugin_InsightPassthroughKeyboardHandsIntensity,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetInsightPassthroughKeyboardHandsIntensity",
                (layerId, intensity),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInsightPassthroughStyle_OVRPlugin_InsightPassthroughStyle1(
        layerId: i32,
        style: crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetInsightPassthroughStyle", (layerId, style))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInsightPassthroughStyle_OVRPlugin_InsightPassthroughStyle2_0(
        layerId: i32,
        style: crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetInsightPassthroughStyle", (layerId, style))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyboardOverlayUV(
        uv: crate::GlobalNamespace::OVRPlugin_Vector2f,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetKeyboardOverlayUV", (uv))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLogCallback2(
        logCallback: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRPlugin_LogCallback2DelegateType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLogCallback2", (logCallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMultimodalHandsControllersSupported(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMultimodalHandsControllersSupported", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSimultaneousHandsAndControllersEnabled(
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSimultaneousHandsAndControllersEnabled", (enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSpaceComponentStatus(
        space: u64,
        componentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        enable: bool,
        timeout: f64,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetSpaceComponentStatus",
                (space, componentType, enable, timeout, requestId),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTrackingCalibratedOrigin() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTrackingCalibratedOrigin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTrackingOriginType(
        originType: crate::GlobalNamespace::OVRPlugin_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetTrackingOriginType", (originType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVirtualKeyboardModelVisibility(
        visibility: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelVisibility,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetVirtualKeyboardModelVisibility", (visibility))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShareSpaces(
        spaces: crate::Unity::Collections::NativeArray_1<u64>,
        userHandles: crate::Unity::Collections::NativeArray_1<u64>,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShareSpaces", (spaces, userHandles, requestId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowUI(
        ui: crate::GlobalNamespace::OVRPlugin_PlatformUI,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShowUI", (ui))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShutdownInsightPassthrough() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShutdownInsightPassthrough", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShutdownMixedReality() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShutdownMixedReality", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartBodyTracking() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartBodyTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartEyeTracking() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartEyeTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartFaceTracking() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartFaceTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartKeyboardTracking(
        trackedKeyboardId: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartKeyboardTracking", (trackedKeyboardId))?;
        Ok(__cordl_ret.into())
    }
    pub fn StopBodyTracking() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StopBodyTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopEyeTracking() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StopEyeTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopFaceTracking() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StopFaceTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopKeyboardTracking() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StopKeyboardTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SuggestVirtualKeyboardLocation(
        locationInfo: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationInfo,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SuggestVirtualKeyboardLocation", (locationInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn TestBoundaryNode(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        boundaryType: crate::GlobalNamespace::OVRPlugin_BoundaryType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_BoundaryTestResult,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_BoundaryTestResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TestBoundaryNode", (nodeId, boundaryType))?;
        Ok(__cordl_ret.into())
    }
    pub fn TestBoundaryPoint(
        point: crate::GlobalNamespace::OVRPlugin_Vector3f,
        boundaryType: crate::GlobalNamespace::OVRPlugin_BoundaryType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_BoundaryTestResult,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_BoundaryTestResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TestBoundaryPoint", (point, boundaryType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBool(
        b: bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToBool", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryLocateSpace_ByRefMut1(
        space: u64,
        baseOrigin: crate::GlobalNamespace::OVRPlugin_TrackingOrigin,
        pose: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Posef>,
        locationFlags: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_SpaceLocationFlags,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryLocateSpace", (space, baseOrigin, pose, locationFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryLocateSpace_u64_OVRPlugin_TrackingOrigin_ByRefMut0(
        space: u64,
        baseOrigin: crate::GlobalNamespace::OVRPlugin_TrackingOrigin,
        pose: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Posef>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryLocateSpace", (space, baseOrigin, pose))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateExternalCamera() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateExternalCamera", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInsightPassthroughGeometryTransform(
        geometryInstanceHandle: u64,
        transform: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "UpdateInsightPassthroughGeometryTransform",
                (geometryInstanceHandle, transform),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateNodePhysicsPoses(
        frameIndex: i32,
        predictionSeconds: f64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateNodePhysicsPoses", (frameIndex, predictionSeconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePassthroughColorLut(
        colorLut: u64,
        data: crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdatePassthroughColorLut", (colorLut, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AsymmetricFovEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AsymmetricFovEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EyeTextureArrayEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_EyeTextureArrayEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_audioInId() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_audioInId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_audioOutId() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_audioOutId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_batteryLevel() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_batteryLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_batteryStatus() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_BatteryStatus,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_BatteryStatus = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_batteryStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_batteryTemperature() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_batteryTemperature", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bodyTrackingEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_bodyTrackingEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bodyTrackingSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_bodyTrackingSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_chromatic() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_chromatic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cpuLevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_cpuLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeDepth() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeDepth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeFovPremultipliedAlphaModeEnabled() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeFovPremultipliedAlphaModeEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeHeight() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeTrackedFoveatedRenderingEnabled() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeTrackedFoveatedRenderingEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeTrackedFoveatedRenderingSupported() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeTrackedFoveatedRenderingSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeTrackingEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeTrackingEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeTrackingSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeTrackingSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_faceTrackingEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_faceTrackingEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_faceTrackingSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_faceTrackingSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fixedFoveatedRenderingLevel() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_FixedFoveatedRenderingLevel,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_FixedFoveatedRenderingLevel = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fixedFoveatedRenderingLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fixedFoveatedRenderingSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fixedFoveatedRenderingSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_foveatedRenderingLevel() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_FoveatedRenderingLevel,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_FoveatedRenderingLevel = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_foveatedRenderingLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_foveatedRenderingSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_foveatedRenderingSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gpuLevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_gpuLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gpuUtilLevel() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_gpuUtilLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gpuUtilSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_gpuUtilSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasInputFocus() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_hasInputFocus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasVrFocus() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_hasVrFocus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headphonesPresent() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_headphonesPresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hmdPresent() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_hmdPresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_initialized() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_initialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ipd() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ipd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_latency() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_latency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localDimming() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_localDimming", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localDimmingSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_localDimmingSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_monoscopic() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_monoscopic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nativeSDKVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_nativeSDKVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nativeXrApi() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_XrApi,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_XrApi = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_nativeXrApi", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_occlusionMesh() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_occlusionMesh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_position() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_position", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_positionSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_positionSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_positionTracked() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_positionTracked", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_powerSaving() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_powerSaving", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_productName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_productName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_recommendedMSAALevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_recommendedMSAALevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotation() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_rotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shouldQuit() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_shouldQuit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shouldRecenter() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_shouldRecenter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_suggestedCpuPerfLevel() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_suggestedCpuPerfLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_suggestedGpuPerfLevel() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_suggestedGpuPerfLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_systemDisplayFrequenciesAvailable() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_systemDisplayFrequenciesAvailable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_systemDisplayFrequency() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_systemDisplayFrequency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_systemRegion() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SystemRegion,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SystemRegion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_systemRegion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_systemVolume() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_systemVolume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tiledMultiResLevel() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_TiledMultiResLevel,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_TiledMultiResLevel = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_tiledMultiResLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tiledMultiResSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_tiledMultiResSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useDynamicFixedFoveatedRendering() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_useDynamicFixedFoveatedRendering", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useDynamicFoveatedRendering() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_useDynamicFoveatedRendering", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useIPDInPositionTracking() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_useIPDInPositionTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_userPresent() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_userPresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_version() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_version", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vsyncCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_vsyncCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_chromatic(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_chromatic", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cpuLevel(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_cpuLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_eyeDepth(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_eyeDepth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_eyeFovPremultipliedAlphaModeEnabled(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_eyeFovPremultipliedAlphaModeEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_eyeHeight(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_eyeHeight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_eyeTrackedFoveatedRenderingEnabled(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_eyeTrackedFoveatedRenderingEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fixedFoveatedRenderingLevel(
        value: crate::GlobalNamespace::OVRPlugin_FixedFoveatedRenderingLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_fixedFoveatedRenderingLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_foveatedRenderingLevel(
        value: crate::GlobalNamespace::OVRPlugin_FoveatedRenderingLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_foveatedRenderingLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_gpuLevel(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_gpuLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ipd(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ipd", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_localDimming(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_localDimming", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_monoscopic(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_monoscopic", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_occlusionMesh(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_occlusionMesh", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_position(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_position", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rotation(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_rotation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_suggestedCpuPerfLevel(
        value: crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_suggestedCpuPerfLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_suggestedGpuPerfLevel(
        value: crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_suggestedGpuPerfLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_systemDisplayFrequency(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_systemDisplayFrequency", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_tiledMultiResLevel(
        value: crate::GlobalNamespace::OVRPlugin_TiledMultiResLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_tiledMultiResLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useDynamicFixedFoveatedRendering(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_useDynamicFixedFoveatedRendering", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useDynamicFoveatedRendering(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_useDynamicFoveatedRendering", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useIPDInPositionTracking(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_useIPDInPositionTracking", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vsyncCount(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_vsyncCount", (value))?;
        Ok(__cordl_ret.into())
    }
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_AppPerfStats {
    pub FrameStats: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRPlugin_AppPerfFrameStats,
        >,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_BatteryStatus {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_BlendFactor {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
        Ok(__cordl_ret.into())
    }
    pub fn get_OrientationValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_OrientationValid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PositionTracked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_PositionTracked",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PositionValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_PositionValid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+BodyState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_BodyState {
    pub JointLocations: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRPlugin_BodyJointLocation,
        >,
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_BoneId {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_Bool {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_BoundaryGeometry {
    pub BoundaryType: crate::GlobalNamespace::OVRPlugin_BoundaryType,
    pub Points: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Vector3f>,
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_BoundaryType {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_CameraAnchorType {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_CameraDevice {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_CameraDeviceDepthQuality {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_CameraDeviceDepthSensingMode {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_CameraStatus {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_ColorSpace {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+Controller")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_Controller {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+ControllerState4")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+ControllerState5")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+ControllerState6")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+EventDataBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_EventDataBuffer {
    pub EventType: crate::GlobalNamespace::OVRPlugin_EventType,
    pub EventData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_EventType {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_Eye {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+EyeGazesState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_EyeGazesState {
    pub EyeGazes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRPlugin_EyeGazeState,
        >,
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_EyeTextureFormat {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_FaceConstants {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_FaceExpression {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+FaceRegionConfidence")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_FaceRegionConfidence {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_FaceState {
    pub ExpressionWeights: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
    pub ExpressionWeightConfidences: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_FeatureType {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_FixedFoveatedRenderingLevel {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_FoveatedRenderingLevel {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_Hand {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_HandFinger {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_HandFingerPinch {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_HandState {
    pub Status: crate::GlobalNamespace::OVRPlugin_HandStatus,
    pub RootPose: crate::GlobalNamespace::OVRPlugin_Posef,
    pub BoneRotations: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Quatf>,
    >,
    pub Pinches: crate::GlobalNamespace::OVRPlugin_HandFingerPinch,
    pub PinchStrength: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
    pub PointerPose: crate::GlobalNamespace::OVRPlugin_Posef,
    pub HandScale: f32,
    pub HandConfidence: crate::GlobalNamespace::OVRPlugin_TrackingConfidence,
    pub FingerConfidences: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRPlugin_TrackingConfidence,
        >,
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_HandStatus {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_Handedness {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_HapticsConstants {
    #[default]
    MaxSamples = 4000i32,
}
#[cfg(feature = "OVRPlugin+HapticsConstants")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_HapticsConstants =>
    ""."OVRPlugin/HapticsConstants"
);
#[cfg(feature = "OVRPlugin+HapticsDesc")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_HapticsLocation {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_InsightPassthroughColorMapType {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+InsightPassthroughStyleFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_InsightPassthroughStyleFlags {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_InteractionProfile {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_KeyboardDescription {
    pub Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_KeyboardDescriptionConstants {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+Ktx")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Ktx => ""
    ."OVRPlugin/Ktx"
);
#[cfg(feature = "OVRPlugin+Ktx")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_Ktx {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn DestroyKtxTexture(
        texture: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyKtxTexture", (texture))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKtxTextureData(
        texture: crate::System::IntPtr,
        textureData: crate::System::IntPtr,
        bufferSize: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKtxTextureData", (texture, textureData, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKtxTextureHeight(
        texture: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKtxTextureHeight", (texture))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKtxTextureSize(
        texture: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKtxTextureSize", (texture))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKtxTextureWidth(
        texture: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKtxTextureWidth", (texture))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadKtxFromMemory(
        dataPtr: crate::System::IntPtr,
        length: u32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadKtxFromMemory", (dataPtr, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TranscodeKtxTexture(
        texture: crate::System::IntPtr,
        format: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TranscodeKtxTexture", (texture, format))?;
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_LayerDesc {
    pub Shape: crate::GlobalNamespace::OVRPlugin_OverlayShape,
    pub Layout: crate::GlobalNamespace::OVRPlugin_LayerLayout,
    pub TextureSize: crate::GlobalNamespace::OVRPlugin_Sizei,
    pub MipLevels: i32,
    pub SampleCount: i32,
    pub Format: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    pub LayerFlags: i32,
    pub Fov: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Fovf>,
    >,
    pub VisibleRect: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Rectf>,
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+LayerDescInternal")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+LayerFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_LayerFlags {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_LayerLayout {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_LayerSharpenType {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_LayerSubmit {
    pub LayerId: i32,
    pub TextureStage: i32,
    pub ViewportRect: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Recti>,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_LayerSuperSamplingType {
    #[default]
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
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (logLevel, message, _cordl_size, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_LogLevel {
    #[default]
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+Media")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Media => ""
    ."OVRPlugin/Media"
);
#[cfg(feature = "OVRPlugin+Media")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_Media {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn EncodeMrcFrame_IntPtr_IntPtr_Il2CppArray_i32_f64_ByRefMut0(
        textureHandle: crate::System::IntPtr,
        fgTextureHandle: crate::System::IntPtr,
        audioData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        audioFrames: i32,
        audioChannels: i32,
        timestamp: f64,
        poseTime: f64,
        outSyncId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EncodeMrcFrame",
                (
                    textureHandle,
                    fgTextureHandle,
                    audioData,
                    audioFrames,
                    audioChannels,
                    timestamp,
                    poseTime,
                    outSyncId,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeMrcFrame_RenderTexture_Il2CppArray_i32_f64_ByRefMut1(
        frame: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        audioData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        audioFrames: i32,
        audioChannels: i32,
        timestamp: f64,
        poseTime: f64,
        outSyncId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EncodeMrcFrame",
                (
                    frame,
                    audioData,
                    audioFrames,
                    audioChannels,
                    timestamp,
                    poseTime,
                    outSyncId,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInitialized() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMrcActivationMode() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::Media_OVRPlugin_MrcActivationMode,
    > {
        let __cordl_ret: crate::GlobalNamespace::Media_OVRPlugin_MrcActivationMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMrcActivationMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMrcAudioSampleRate() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMrcAudioSampleRate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMrcFrameImageFlipped() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMrcFrameImageFlipped", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMrcFrameSize(
        frameWidth: quest_hook::libil2cpp::ByRefMut<i32>,
        frameHeight: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMrcFrameSize", (frameWidth, frameHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMrcInputVideoBufferType() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::Media_OVRPlugin_InputVideoBufferType,
    > {
        let __cordl_ret: crate::GlobalNamespace::Media_OVRPlugin_InputVideoBufferType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMrcInputVideoBufferType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlatformCameraMode() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::Media_OVRPlugin_PlatformCameraMode,
    > {
        let __cordl_ret: crate::GlobalNamespace::Media_OVRPlugin_PlatformCameraMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPlatformCameraMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCastingToRemoteClient() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCastingToRemoteClient", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMrcActivated() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMrcActivated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMrcEnabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMrcEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetAvailableQueueIndexVulkan(
        queueIndexVk: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetAvailableQueueIndexVulkan", (queueIndexVk))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMrcActivationMode(
        mode: crate::GlobalNamespace::Media_OVRPlugin_MrcActivationMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMrcActivationMode", (mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMrcAudioSampleRate(
        sampleRate: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMrcAudioSampleRate", (sampleRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMrcFrameImageFlipped(
        imageFlipped: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMrcFrameImageFlipped", (imageFlipped))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMrcFrameSize(
        frameWidth: i32,
        frameHeight: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMrcFrameSize", (frameWidth, frameHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMrcHeadsetControllerPose(
        headsetPose: crate::GlobalNamespace::OVRPlugin_Posef,
        leftControllerPose: crate::GlobalNamespace::OVRPlugin_Posef,
        rightControllerPose: crate::GlobalNamespace::OVRPlugin_Posef,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetMrcHeadsetControllerPose",
                (headsetPose, leftControllerPose, rightControllerPose),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMrcInputVideoBufferType(
        videoBufferType: crate::GlobalNamespace::Media_OVRPlugin_InputVideoBufferType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMrcInputVideoBufferType", (videoBufferType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlatformCameraMode(
        mode: crate::GlobalNamespace::Media_OVRPlugin_PlatformCameraMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetPlatformCameraMode", (mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlatformInitialized() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetPlatformInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Shutdown() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Shutdown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SyncMrcFrame(syncId: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SyncMrcFrame", (syncId))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UseMrcDebugCamera() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UseMrcDebugCamera", ())?;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Type: crate::GlobalNamespace::OVRPlugin_MeshType,
    pub NumVertices: u32,
    pub NumIndices: u32,
    pub VertexPositions: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Vector3f>,
    >,
    pub Indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i16>>,
    pub VertexNormals: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Vector3f>,
    >,
    pub VertexUV0: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Vector2f>,
    >,
    pub BlendIndices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Vector4s>,
    >,
    pub BlendWeights: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Vector4f>,
    >,
}
#[cfg(feature = "OVRPlugin+Mesh")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Mesh => ""
    ."OVRPlugin/Mesh"
);
#[cfg(feature = "OVRPlugin+Mesh")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_Mesh {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_MeshConstants {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_MeshType {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_Node {
    #[default]
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_0_1_0 => ""
    ."OVRPlugin/OVRP_0_1_0"
);
#[cfg(feature = "OVRPlugin+OVRP_0_1_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_0_1_0 {
    pub fn ovrp_GetEyeTextureSize(
        eyeId: crate::GlobalNamespace::OVRPlugin_Eye,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Sizei> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Sizei = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetEyeTextureSize", (eyeId))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_0_1_1 => ""
    ."OVRPlugin/OVRP_0_1_1"
);
#[cfg(feature = "OVRPlugin+OVRP_0_1_1")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_0_1_1 {
    pub fn ovrp_SetOverlayQuad2(
        onTop: crate::GlobalNamespace::OVRPlugin_Bool,
        headLocked: crate::GlobalNamespace::OVRPlugin_Bool,
        texture: crate::System::IntPtr,
        device: crate::System::IntPtr,
        pose: crate::GlobalNamespace::OVRPlugin_Posef,
        scale: crate::GlobalNamespace::OVRPlugin_Vector3f,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_SetOverlayQuad2",
                (onTop, headLocked, texture, device, pose, scale),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_0_1_2 => ""
    ."OVRPlugin/OVRP_0_1_2"
);
#[cfg(feature = "OVRPlugin+OVRP_0_1_2")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_0_1_2 {
    pub fn ovrp_GetNodePose(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodePose", (nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetControllerVibration(
        controllerMask: u32,
        frequency: f32,
        amplitude: f32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_SetControllerVibration",
                (controllerMask, frequency, amplitude),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_0_1_3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_0_1_3 => ""
    ."OVRPlugin/OVRP_0_1_3"
);
#[cfg(feature = "OVRPlugin+OVRP_0_1_3")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_0_1_3 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_0_1_3 {
    pub fn ovrp_GetNodeAcceleration(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodeAcceleration", (nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNodeVelocity(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodeVelocity", (nodeId))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_0_5_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_0_5_0 => ""
    ."OVRPlugin/OVRP_0_5_0"
);
#[cfg(feature = "OVRPlugin+OVRP_0_5_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_0_5_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_0_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_0_0 => ""
    ."OVRPlugin/OVRP_1_0_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_0_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_0_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_0_0 {
    pub fn ovrp_GetTrackingCalibratedOrigin() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Posef,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetTrackingCalibratedOrigin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetTrackingOriginType() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_TrackingOrigin,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_TrackingOrigin = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetTrackingOriginType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_RecenterTrackingOrigin(
        flags: u32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_RecenterTrackingOrigin", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetTrackingOriginType(
        originType: crate::GlobalNamespace::OVRPlugin_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetTrackingOriginType", (originType))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_10_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_10_0 => ""
    ."OVRPlugin/OVRP_1_10_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_10_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_10_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_11_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_11_0 => ""
    ."OVRPlugin/OVRP_1_11_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_11_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_11_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_11_0 {
    pub fn ovrp_GetDesiredEyeTextureFormat() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetDesiredEyeTextureFormat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetDesiredEyeTextureFormat(
        value: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetDesiredEyeTextureFormat", (value))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_12_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_12_0 => ""
    ."OVRPlugin/OVRP_1_12_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_12_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_12_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_12_0 {
    pub fn ovrp_GetAppFramerate() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetAppFramerate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetControllerState2(
        controllerMask: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_ControllerState2,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_ControllerState2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetControllerState2", (controllerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNodePoseState(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_PoseStatef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_PoseStatef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodePoseState", (stepId, nodeId))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_15_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_15_0 => ""
    ."OVRPlugin/OVRP_1_15_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_15_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_15_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn ovrp_CalculateLayerDesc(
        shape: crate::GlobalNamespace::OVRPlugin_OverlayShape,
        layout: crate::GlobalNamespace::OVRPlugin_LayerLayout,
        textureSize: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Sizei,
        >,
        mipLevels: i32,
        sampleCount: i32,
        format: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
        layerFlags: i32,
        layerDesc: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_LayerDescInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_CalculateLayerDesc",
                (
                    shape,
                    layout,
                    textureSize,
                    mipLevels,
                    sampleCount,
                    format,
                    layerFlags,
                    layerDesc,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_EnqueueDestroyLayer(
        layerId: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_EnqueueDestroyLayer", (layerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_EnqueueSetupLayer(
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_LayerDescInternal,
        >,
        layerId: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_EnqueueSetupLayer", (desc, layerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_EnqueueSubmitLayer(
        flags: u32,
        textureLeft: crate::System::IntPtr,
        textureRight: crate::System::IntPtr,
        layerId: i32,
        frameIndex: i32,
        pose: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Posef>,
        scale: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Vector3f,
        >,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_EnqueueSubmitLayer",
                (
                    flags,
                    textureLeft,
                    textureRight,
                    layerId,
                    frameIndex,
                    pose,
                    scale,
                    layerIndex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetExternalCameraCount(
        cameraCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetExternalCameraCount", (cameraCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetExternalCameraExtrinsics(
        cameraId: i32,
        cameraExtrinsics: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_CameraExtrinsics,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetExternalCameraExtrinsics", (cameraId, cameraExtrinsics))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetExternalCameraIntrinsics(
        cameraId: i32,
        cameraIntrinsics: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_CameraIntrinsics,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetExternalCameraIntrinsics", (cameraId, cameraIntrinsics))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetExternalCameraName(
        cameraId: i32,
        cameraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetExternalCameraName", (cameraId, cameraName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetEyeTextureArrayEnabled() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetEyeTextureArrayEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetLayerTexturePtr(
        layerId: i32,
        stage: i32,
        eyeId: crate::GlobalNamespace::OVRPlugin_Eye,
        textureHandle: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetLayerTexturePtr", (layerId, stage, eyeId, textureHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetLayerTextureStageCount(
        layerId: i32,
        layerTextureStageCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetLayerTextureStageCount",
                (layerId, layerTextureStageCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetMixedRealityInitialized() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetMixedRealityInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNodeFrustum2(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        nodeFrustum: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Frustumf2,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodeFrustum2", (nodeId, nodeFrustum))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_InitializeMixedReality() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_InitializeMixedReality", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_ShutdownMixedReality() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_ShutdownMixedReality", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UpdateExternalCamera() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_UpdateExternalCamera", ())?;
        Ok(__cordl_ret.into())
    }
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_16_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_16_0 => ""
    ."OVRPlugin/OVRP_1_16_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_16_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_16_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_16_0 {
    pub fn ovrp_CloseCameraDevice(
        cameraDevice: crate::GlobalNamespace::OVRPlugin_CameraDevice,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_CloseCameraDevice", (cameraDevice))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetCameraDeviceColorFrameBgraPixels(
        cameraDevice: crate::GlobalNamespace::OVRPlugin_CameraDevice,
        colorFrameBgraPixels: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        colorFrameRowPitch: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetCameraDeviceColorFrameBgraPixels",
                (cameraDevice, colorFrameBgraPixels, colorFrameRowPitch),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetCameraDeviceColorFrameSize(
        cameraDevice: crate::GlobalNamespace::OVRPlugin_CameraDevice,
        colorFrameSize: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Sizei,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetCameraDeviceColorFrameSize",
                (cameraDevice, colorFrameSize),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetControllerState4(
        controllerMask: u32,
        controllerState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_ControllerState4,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetControllerState4", (controllerMask, controllerState))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_HasCameraDeviceOpened(
        cameraDevice: crate::GlobalNamespace::OVRPlugin_CameraDevice,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_HasCameraDeviceOpened", (cameraDevice))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_IsCameraDeviceAvailable(
        cameraDevice: crate::GlobalNamespace::OVRPlugin_CameraDevice,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_IsCameraDeviceAvailable", (cameraDevice))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_IsCameraDeviceColorFrameAvailable(
        cameraDevice: crate::GlobalNamespace::OVRPlugin_CameraDevice,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_IsCameraDeviceColorFrameAvailable", (cameraDevice))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_OpenCameraDevice(
        cameraDevice: crate::GlobalNamespace::OVRPlugin_CameraDevice,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_OpenCameraDevice", (cameraDevice))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetCameraDevicePreferredColorFrameSize(
        cameraDevice: crate::GlobalNamespace::OVRPlugin_CameraDevice,
        preferredColorFrameSize: crate::GlobalNamespace::OVRPlugin_Sizei,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_SetCameraDevicePreferredColorFrameSize",
                (cameraDevice, preferredColorFrameSize),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UpdateCameraDevices() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_UpdateCameraDevices", ())?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_17_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_17_0 => ""
    ."OVRPlugin/OVRP_1_17_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_17_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_17_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_18_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_18_0 => ""
    ."OVRPlugin/OVRP_1_18_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_18_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_18_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_18_0 {
    pub fn ovrp_GetAppHasInputFocus(
        appHasInputFocus: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetAppHasInputFocus", (appHasInputFocus))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetHandNodePoseStateLatency(
        latencyInSeconds: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetHandNodePoseStateLatency", (latencyInSeconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetHandNodePoseStateLatency(
        latencyInSeconds: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetHandNodePoseStateLatency", (latencyInSeconds))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_19_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_19_0 => ""
    ."OVRPlugin/OVRP_1_19_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_19_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_19_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_1_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_1_0 => ""
    ."OVRPlugin/OVRP_1_1_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_1_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_1_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_1_0 {
    pub fn _ovrp_GetAppLatencyTimings() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("_ovrp_GetAppLatencyTimings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ovrp_GetNativeSDKVersion() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("_ovrp_GetNativeSDKVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ovrp_GetSystemProductName() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("_ovrp_GetSystemProductName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ovrp_GetVersion() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("_ovrp_GetVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetAppHasVrFocus() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetAppHasVrFocus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetAppLatencyTimings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetAppLatencyTimings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetAppMonoscopic() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetAppMonoscopic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetAppShouldQuit() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetAppShouldQuit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetAppShouldRecenter() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetAppShouldRecenter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetAudioInId() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetAudioInId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetAudioOutId() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetAudioOutId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetControllerState(
        controllerMask: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_ControllerState,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_ControllerState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetControllerState", (controllerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetEyeTextureScale() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetEyeTextureScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetInitialized() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNativeSDKVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNativeSDKVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNodeFrustum(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Frustumf> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Frustumf = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodeFrustum", (nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNodeOrientationTracked(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodeOrientationTracked", (nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNodePositionTracked(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodePositionTracked", (nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNodePresent(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodePresent", (nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemBatteryLevel() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemBatteryLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemBatteryStatus() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_BatteryStatus,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_BatteryStatus = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemBatteryStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemBatteryTemperature() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemBatteryTemperature", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemCpuLevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemCpuLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemDisplayFrequency() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemDisplayFrequency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemGpuLevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemGpuLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemPowerSavingMode() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemPowerSavingMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemProductName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemProductName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemVSyncCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemVSyncCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemVolume() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemVolume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetTrackingOrientationEnabled() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetTrackingOrientationEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetTrackingOrientationSupported() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetTrackingOrientationSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetTrackingPositionEnabled() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetTrackingPositionEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetTrackingPositionSupported() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetTrackingPositionSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetUserEyeDepth() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetUserEyeDepth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetUserEyeHeight() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetUserEyeHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetUserIPD() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetUserIPD", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetUserPresent() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetUserPresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetAppMonoscopic(
        value: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetAppMonoscopic", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetEyeTextureScale(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetEyeTextureScale", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetSystemCpuLevel(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetSystemCpuLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetSystemGpuLevel(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetSystemGpuLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetTrackingOrientationEnabled(
        value: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetTrackingOrientationEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetTrackingPositionEnabled(
        value: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetTrackingPositionEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetUserEyeDepth(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetUserEyeDepth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetUserEyeHeight(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetUserEyeHeight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetUserIPD(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetUserIPD", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_ShowSystemUI(
        ui: crate::GlobalNamespace::OVRPlugin_PlatformUI,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_ShowSystemUI", (ui))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_21_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_21_0 => ""
    ."OVRPlugin/OVRP_1_21_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_21_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_21_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_21_0 {
    pub fn ovrp_GetAppAsymmetricFov(
        useAsymmetricFov: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetAppAsymmetricFov", (useAsymmetricFov))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetGPUUtilLevel(
        gpuUtil: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetGPUUtilLevel", (gpuUtil))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetGPUUtilSupported(
        gpuUtilSupported: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetGPUUtilSupported", (gpuUtilSupported))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemDisplayAvailableFrequencies(
        systemDisplayAvailableFrequencies: crate::System::IntPtr,
        numFrequencies: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetSystemDisplayAvailableFrequencies",
                (systemDisplayAvailableFrequencies, numFrequencies),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemDisplayFrequency2(
        systemDisplayFrequency: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemDisplayFrequency2", (systemDisplayFrequency))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetTiledMultiResLevel(
        level: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_FoveatedRenderingLevel,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetTiledMultiResLevel", (level))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetTiledMultiResSupported(
        foveationSupported: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetTiledMultiResSupported", (foveationSupported))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetSystemDisplayFrequency(
        requestedFrequency: f32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetSystemDisplayFrequency", (requestedFrequency))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetTiledMultiResLevel(
        level: crate::GlobalNamespace::OVRPlugin_FoveatedRenderingLevel,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetTiledMultiResLevel", (level))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_28_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_28_0 => ""
    ."OVRPlugin/OVRP_1_28_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_28_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_28_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_28_0 {
    pub fn ovrp_EnqueueSetupLayer2(
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_LayerDescInternal,
        >,
        compositionDepth: i32,
        layerId: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_EnqueueSetupLayer2", (desc, compositionDepth, layerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetDominantHand(
        dominantHand: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Handedness,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetDominantHand", (dominantHand))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SendEvent(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        param: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SendEvent", (name, param))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_29_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_29_0 => ""
    ."OVRPlugin/OVRP_1_29_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_29_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_29_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_29_0 {
    pub fn ovrp_GetHeadPoseModifier(
        relativeRotation: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Quatf,
        >,
        relativeTranslation: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Vector3f,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetHeadPoseModifier",
                (relativeRotation, relativeTranslation),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetLayerAndroidSurfaceObject(
        layerId: i32,
        surfaceObject: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetLayerAndroidSurfaceObject", (layerId, surfaceObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNodePoseStateRaw(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        frameIndex: i32,
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        nodePoseState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_PoseStatef,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetNodePoseStateRaw",
                (stepId, frameIndex, nodeId, nodePoseState),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetHeadPoseModifier(
        relativeRotation: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Quatf,
        >,
        relativeTranslation: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Vector3f,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_SetHeadPoseModifier",
                (relativeRotation, relativeTranslation),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_2_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_2_0 => ""
    ."OVRPlugin/OVRP_1_2_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_2_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_2_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_2_0 {
    pub fn ovrp_SetSystemVSyncCount(
        vsyncCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetSystemVSyncCount", (vsyncCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrpi_SetTrackingCalibratedOrigin() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrpi_SetTrackingCalibratedOrigin", ())?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_30_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_30_0 => ""
    ."OVRPlugin/OVRP_1_30_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_30_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_30_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_30_0 {
    pub fn ovrp_GetCurrentTrackingTransformPose(
        trackingTransformPose: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Posef,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetCurrentTrackingTransformPose", (trackingTransformPose))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetPerfMetricsFloat(
        perfMetrics: crate::GlobalNamespace::OVRPlugin_PerfMetrics,
        value: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetPerfMetricsFloat", (perfMetrics, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetPerfMetricsInt(
        perfMetrics: crate::GlobalNamespace::OVRPlugin_PerfMetrics,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetPerfMetricsInt", (perfMetrics, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetTrackingTransformRawPose(
        trackingTransformRawPose: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Posef,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetTrackingTransformRawPose", (trackingTransformRawPose))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_IsPerfMetricsSupported(
        perfMetrics: crate::GlobalNamespace::OVRPlugin_PerfMetrics,
        isSupported: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_IsPerfMetricsSupported", (perfMetrics, isSupported))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SendEvent2(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        param: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SendEvent2", (name, param, source))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_31_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_31_0 => ""
    ."OVRPlugin/OVRP_1_31_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_31_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_31_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_31_0 {
    pub fn ovrp_GetTimeInSeconds(
        value: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetTimeInSeconds", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetColorScaleAndOffset(
        colorScale: crate::UnityEngine::Vector4,
        colorOffset: crate::UnityEngine::Vector4,
        applyToAllLayers: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_SetColorScaleAndOffset",
                (colorScale, colorOffset, applyToAllLayers),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_32_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_32_0 => ""
    ."OVRPlugin/OVRP_1_32_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_32_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_32_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_32_0 {
    pub fn ovrp_AddCustomMetadata(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        param: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_AddCustomMetadata", (name, param))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_34_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_34_0 => ""
    ."OVRPlugin/OVRP_1_34_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_34_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_34_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_34_0 {
    pub fn ovrp_EnqueueSubmitLayer2(
        flags: u32,
        textureLeft: crate::System::IntPtr,
        textureRight: crate::System::IntPtr,
        layerId: i32,
        frameIndex: i32,
        pose: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Posef>,
        scale: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Vector3f,
        >,
        layerIndex: i32,
        overrideTextureRectMatrix: crate::GlobalNamespace::OVRPlugin_Bool,
        textureRectMatrix: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_TextureRectMatrixf,
        >,
        overridePerLayerColorScaleAndOffset: crate::GlobalNamespace::OVRPlugin_Bool,
        colorScale: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        colorOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_EnqueueSubmitLayer2",
                (
                    flags,
                    textureLeft,
                    textureRight,
                    layerId,
                    frameIndex,
                    pose,
                    scale,
                    layerIndex,
                    overrideTextureRectMatrix,
                    textureRectMatrix,
                    overridePerLayerColorScaleAndOffset,
                    colorScale,
                    colorOffset,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_35_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_35_0 => ""
    ."OVRPlugin/OVRP_1_35_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_35_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_35_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_36_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_36_0 => ""
    ."OVRPlugin/OVRP_1_36_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_36_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_36_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_37_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_37_0 => ""
    ."OVRPlugin/OVRP_1_37_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_37_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_37_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_38_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_38_0 => ""
    ."OVRPlugin/OVRP_1_38_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_38_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_38_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_38_0 {
    pub fn ovrp_GetNodeOrientationValid(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        nodeOrientationValid: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodeOrientationValid", (nodeId, nodeOrientationValid))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNodePositionValid(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        nodePositionValid: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodePositionValid", (nodeId, nodePositionValid))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetTrackingTransformRelativePose(
        trackingTransformRelativePose: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Posef,
        >,
        trackingOrigin: crate::GlobalNamespace::OVRPlugin_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetTrackingTransformRelativePose",
                (trackingTransformRelativePose, trackingOrigin),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_EncodeMrcFrame(
        rawBuffer: crate::System::IntPtr,
        audioDataPtr: crate::System::IntPtr,
        audioDataLen: i32,
        audioChannels: i32,
        timestamp: f64,
        outSyncId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_Media_EncodeMrcFrame",
                (
                    rawBuffer,
                    audioDataPtr,
                    audioDataLen,
                    audioChannels,
                    timestamp,
                    outSyncId,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_EncodeMrcFrameWithDualTextures(
        backgroundTextureHandle: crate::System::IntPtr,
        foregroundTextureHandle: crate::System::IntPtr,
        audioData: crate::System::IntPtr,
        audioDataLen: i32,
        audioChannels: i32,
        timestamp: f64,
        outSyncId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_Media_EncodeMrcFrameWithDualTextures",
                (
                    backgroundTextureHandle,
                    foregroundTextureHandle,
                    audioData,
                    audioDataLen,
                    audioChannels,
                    timestamp,
                    outSyncId,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_GetInitialized(
        initialized: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_GetInitialized", (initialized))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_GetMrcActivationMode(
        activationMode: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::Media_OVRPlugin_MrcActivationMode,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_GetMrcActivationMode", (activationMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_GetMrcAudioSampleRate(
        sampleRate: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_GetMrcAudioSampleRate", (sampleRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_GetMrcFrameImageFlipped(
        flipped: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Bool>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_GetMrcFrameImageFlipped", (flipped))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_GetMrcFrameSize(
        frameWidth: quest_hook::libil2cpp::ByRefMut<i32>,
        frameHeight: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_GetMrcFrameSize", (frameWidth, frameHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_GetMrcInputVideoBufferType(
        inputVideoBufferType: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::Media_OVRPlugin_InputVideoBufferType,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_GetMrcInputVideoBufferType", (inputVideoBufferType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_Initialize() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_IsMrcActivated(
        mrcActivated: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_IsMrcActivated", (mrcActivated))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_IsMrcEnabled(
        mrcEnabled: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_IsMrcEnabled", (mrcEnabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_SetMrcActivationMode(
        activationMode: crate::GlobalNamespace::Media_OVRPlugin_MrcActivationMode,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_SetMrcActivationMode", (activationMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_SetMrcAudioSampleRate(
        sampleRate: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_SetMrcAudioSampleRate", (sampleRate))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_SetMrcFrameImageFlipped(
        flipped: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_SetMrcFrameImageFlipped", (flipped))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_SetMrcFrameSize(
        frameWidth: i32,
        frameHeight: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_SetMrcFrameSize", (frameWidth, frameHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_SetMrcInputVideoBufferType(
        inputVideoBufferType: crate::GlobalNamespace::Media_OVRPlugin_InputVideoBufferType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_SetMrcInputVideoBufferType", (inputVideoBufferType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_Shutdown() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_Shutdown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_SyncMrcFrame(
        syncId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_SyncMrcFrame", (syncId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_Update() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_UseMrcDebugCamera(
        useMrcDebugCamera: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_UseMrcDebugCamera", (useMrcDebugCamera))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetDeveloperMode(
        active: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetDeveloperMode", (active))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_39_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_39_0 => ""
    ."OVRPlugin/OVRP_1_39_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_39_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_39_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_3_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_3_0 => ""
    ."OVRPlugin/OVRP_1_3_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_3_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_3_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_3_0 {
    pub fn ovrp_GetEyeOcclusionMeshEnabled() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetEyeOcclusionMeshEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemHeadphonesPresent() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemHeadphonesPresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetEyeOcclusionMeshEnabled(
        value: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetEyeOcclusionMeshEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_40_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_40_0 => ""
    ."OVRPlugin/OVRP_1_40_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_40_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_40_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_41_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_41_0 => ""
    ."OVRPlugin/OVRP_1_41_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_41_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_41_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_42_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_42_0 => ""
    ."OVRPlugin/OVRP_1_42_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_42_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_42_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_42_0 {
    pub fn ovrp_GetAdaptiveGpuPerformanceScale2(
        adaptiveGpuPerformanceScale: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetAdaptiveGpuPerformanceScale2",
                (adaptiveGpuPerformanceScale),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_43_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_43_0 => ""
    ."OVRPlugin/OVRP_1_43_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_43_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_43_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_44_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_44_0 => ""
    ."OVRPlugin/OVRP_1_44_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_44_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_44_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_44_0 {
    pub fn ovrp_GetHandState(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        hand: crate::GlobalNamespace::OVRPlugin_Hand,
        handState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_HandStateInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetHandState", (stepId, hand, handState))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetHandTrackingEnabled(
        handTrackingEnabled: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetHandTrackingEnabled", (handTrackingEnabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetLocalTrackingSpaceRecenterCount(
        recenterCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetLocalTrackingSpaceRecenterCount", (recenterCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetMesh(
        meshType: crate::GlobalNamespace::OVRPlugin_MeshType,
        meshPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetMesh", (meshType, meshPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSkeleton(
        skeletonType: crate::GlobalNamespace::OVRPlugin_SkeletonType,
        skeleton: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Skeleton,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSkeleton", (skeletonType, skeleton))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetUseOverriddenExternalCameraFov(
        cameraId: i32,
        useOverriddenFov: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetUseOverriddenExternalCameraFov",
                (cameraId, useOverriddenFov),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetUseOverriddenExternalCameraStaticPose(
        cameraId: i32,
        useOverriddenStaticPose: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetUseOverriddenExternalCameraStaticPose",
                (cameraId, useOverriddenStaticPose),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_OverrideExternalCameraFov(
        cameraId: i32,
        useOverriddenFov: crate::GlobalNamespace::OVRPlugin_Bool,
        fov: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Fovf>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_OverrideExternalCameraFov",
                (cameraId, useOverriddenFov, fov),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_OverrideExternalCameraStaticPose(
        cameraId: i32,
        useOverriddenPose: crate::GlobalNamespace::OVRPlugin_Bool,
        poseInStageOrigin: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Posef,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_OverrideExternalCameraStaticPose",
                (cameraId, useOverriddenPose, poseInStageOrigin),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_ResetDefaultExternalCamera() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_ResetDefaultExternalCamera", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetDefaultExternalCamera(
        cameraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cameraIntrinsics: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_CameraIntrinsics,
        >,
        cameraExtrinsics: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_CameraExtrinsics,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_SetDefaultExternalCamera",
                (cameraName, cameraIntrinsics, cameraExtrinsics),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_45_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_45_0 => ""
    ."OVRPlugin/OVRP_1_45_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_45_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_45_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_45_0 {
    pub fn ovrp_GetSystemHmd3DofModeEnabled(
        enabled: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Bool>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemHmd3DofModeEnabled", (enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_SetAvailableQueueIndexVulkan(
        queueIndexVk: u32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_SetAvailableQueueIndexVulkan", (queueIndexVk))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_46_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_46_0 => ""
    ."OVRPlugin/OVRP_1_46_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_46_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_46_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_46_0 {
    pub fn ovrp_GetTiledMultiResDynamic(
        isDynamic: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetTiledMultiResDynamic", (isDynamic))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetTiledMultiResDynamic(
        isDynamic: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetTiledMultiResDynamic", (isDynamic))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_47_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_47_0 => ""
    ."OVRPlugin/OVRP_1_47_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_47_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_47_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_48_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_48_0 => ""
    ."OVRPlugin/OVRP_1_48_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_48_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_48_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_48_0 {
    pub fn ovrp_SetExternalCameraProperties(
        cameraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cameraIntrinsics: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_CameraIntrinsics,
        >,
        cameraExtrinsics: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_CameraExtrinsics,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_SetExternalCameraProperties",
                (cameraName, cameraIntrinsics, cameraExtrinsics),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_49_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_49_0 => ""
    ."OVRPlugin/OVRP_1_49_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_49_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_49_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn ovrp_GetHmdColorDesc(
        colorSpace: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_ColorSpace,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetHmdColorDesc", (colorSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_CreateCustomCameraAnchor(
        anchorName: crate::System::IntPtr,
        anchorHandle: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_CreateCustomCameraAnchor", (anchorName, anchorHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_DestroyCustomCameraAnchor(
        anchorHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_DestroyCustomCameraAnchor", (anchorHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_EncodeMrcFrameDualTexturesWithPoseTime(
        backgroundTextureHandle: crate::System::IntPtr,
        foregroundTextureHandle: crate::System::IntPtr,
        audioData: crate::System::IntPtr,
        audioDataLen: i32,
        audioChannels: i32,
        timestamp: f64,
        poseTime: f64,
        outSyncId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_Media_EncodeMrcFrameDualTexturesWithPoseTime",
                (
                    backgroundTextureHandle,
                    foregroundTextureHandle,
                    audioData,
                    audioDataLen,
                    audioChannels,
                    timestamp,
                    poseTime,
                    outSyncId,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_EncodeMrcFrameWithPoseTime(
        rawBuffer: crate::System::IntPtr,
        audioDataPtr: crate::System::IntPtr,
        audioDataLen: i32,
        audioChannels: i32,
        timestamp: f64,
        poseTime: f64,
        outSyncId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_Media_EncodeMrcFrameWithPoseTime",
                (
                    rawBuffer,
                    audioDataPtr,
                    audioDataLen,
                    audioChannels,
                    timestamp,
                    poseTime,
                    outSyncId,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_EnumerateCameraAnchorHandles(
        anchorCount: quest_hook::libil2cpp::ByRefMut<i32>,
        CameraAnchorHandle: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_Media_EnumerateCameraAnchorHandles",
                (anchorCount, CameraAnchorHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_GetCameraAnchorHandle(
        anchorName: crate::System::IntPtr,
        anchorHandle: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_GetCameraAnchorHandle", (anchorName, anchorHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_GetCameraAnchorName(
        anchorHandle: crate::System::IntPtr,
        cameraName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_GetCameraAnchorName", (anchorHandle, cameraName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_GetCameraAnchorType(
        anchorHandle: crate::System::IntPtr,
        anchorType: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_CameraAnchorType,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_GetCameraAnchorType", (anchorHandle, anchorType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_GetCameraMinMaxDistance(
        anchorHandle: crate::System::IntPtr,
        minDistance: quest_hook::libil2cpp::ByRefMut<f64>,
        maxDistance: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_Media_GetCameraMinMaxDistance",
                (anchorHandle, minDistance, maxDistance),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_GetCurrentCameraAnchorHandle(
        anchorHandle: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_GetCurrentCameraAnchorHandle", (anchorHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_GetCustomCameraAnchorPose(
        anchorHandle: crate::System::IntPtr,
        pose: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Posef>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_GetCustomCameraAnchorPose", (anchorHandle, pose))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_SetCameraMinMaxDistance(
        anchorHandle: crate::System::IntPtr,
        minDistance: f64,
        maxDistance: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_Media_SetCameraMinMaxDistance",
                (anchorHandle, minDistance, maxDistance),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_SetCustomCameraAnchorPose(
        anchorHandle: crate::System::IntPtr,
        pose: crate::GlobalNamespace::OVRPlugin_Posef,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_SetCustomCameraAnchorPose", (anchorHandle, pose))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_SetHeadsetControllerPose(
        headsetPose: crate::GlobalNamespace::OVRPlugin_Posef,
        leftControllerPose: crate::GlobalNamespace::OVRPlugin_Posef,
        rightControllerPose: crate::GlobalNamespace::OVRPlugin_Posef,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_Media_SetHeadsetControllerPose",
                (headsetPose, leftControllerPose, rightControllerPose),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetClientColorDesc(
        colorSpace: crate::GlobalNamespace::OVRPlugin_ColorSpace,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetClientColorDesc", (colorSpace))?;
        Ok(__cordl_ret.into())
    }
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_50_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_50_0 => ""
    ."OVRPlugin/OVRP_1_50_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_50_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_50_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_51_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_51_0 => ""
    ."OVRPlugin/OVRP_1_51_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_51_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_51_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_52_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_52_0 => ""
    ."OVRPlugin/OVRP_1_52_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_52_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_52_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_53_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_53_0 => ""
    ."OVRPlugin/OVRP_1_53_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_53_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_53_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_54_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_54_0 => ""
    ."OVRPlugin/OVRP_1_54_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_54_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_54_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_54_0 {
    pub fn ovrp_Media_SetPlatformInitialized() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_SetPlatformInitialized", ())?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_55_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_55_0 => ""
    ."OVRPlugin/OVRP_1_55_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_55_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_55_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_55_0 {
    pub fn ovrp_GetNativeOpenXRHandles(
        xrInstance: quest_hook::libil2cpp::ByRefMut<u64>,
        xrSession: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNativeOpenXRHandles", (xrInstance, xrSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNativeXrApiType(
        xrApi: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_XrApi>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNativeXrApiType", (xrApi))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSkeleton2(
        skeletonType: crate::GlobalNamespace::OVRPlugin_SkeletonType,
        skeleton: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Skeleton2Internal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSkeleton2", (skeletonType, skeleton))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_PollEvent(
        eventDataBuffer: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_EventDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_PollEvent", (eventDataBuffer))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_55_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_55_1 => ""
    ."OVRPlugin/OVRP_1_55_1"
);
#[cfg(feature = "OVRPlugin+OVRP_1_55_1")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_55_1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_55_1 {
    pub fn ovrp_PollEvent2(
        eventType: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_EventType,
        >,
        eventData: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_PollEvent2", (eventType, eventData))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_56_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_56_0 => ""
    ."OVRPlugin/OVRP_1_56_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_56_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_56_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_57_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_57_0 => ""
    ."OVRPlugin/OVRP_1_57_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_57_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_57_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_57_0 {
    pub fn ovrp_GetEyeFovPremultipliedAlphaMode(
        enabled: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Bool>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetEyeFovPremultipliedAlphaMode", (enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_GetPlatformCameraMode(
        platformCameraMode: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::Media_OVRPlugin_PlatformCameraMode,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_GetPlatformCameraMode", (platformCameraMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_SetPlatformCameraMode(
        platformCameraMode: crate::GlobalNamespace::Media_OVRPlugin_PlatformCameraMode,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_SetPlatformCameraMode", (platformCameraMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetEyeFovPremultipliedAlphaMode(
        enabled: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetEyeFovPremultipliedAlphaMode", (enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetKeyboardOverlayUV(
        uv: crate::GlobalNamespace::OVRPlugin_Vector2f,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetKeyboardOverlayUV", (uv))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_58_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_58_0 => ""
    ."OVRPlugin/OVRP_1_58_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_58_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_58_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_59_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_59_0 => ""
    ."OVRPlugin/OVRP_1_59_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_59_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_59_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_5_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_5_0 => ""
    ."OVRPlugin/OVRP_1_5_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_5_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_5_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_5_0 {
    pub fn ovrp_GetSystemRegion() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SystemRegion,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SystemRegion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemRegion", ())?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_60_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_60_0 => ""
    ."OVRPlugin/OVRP_1_60_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_60_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_60_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_61_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_61_0 => ""
    ."OVRPlugin/OVRP_1_61_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_61_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_61_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_62_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_62_0 => ""
    ."OVRPlugin/OVRP_1_62_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_62_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_62_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_63_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_63_0 => ""
    ."OVRPlugin/OVRP_1_63_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_63_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_63_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_63_0 {
    pub fn ovrp_AddInsightPassthroughSurfaceGeometry(
        layerId: i32,
        meshHandle: u64,
        T_world_model: crate::UnityEngine::Matrix4x4,
        geometryInstanceHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_AddInsightPassthroughSurfaceGeometry",
                (layerId, meshHandle, T_world_model, geometryInstanceHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_CreateInsightTriangleMesh(
        layerId: i32,
        vertices: crate::System::IntPtr,
        vertexCount: i32,
        triangles: crate::System::IntPtr,
        triangleCount: i32,
        meshHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_CreateInsightTriangleMesh",
                (layerId, vertices, vertexCount, triangles, triangleCount, meshHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_DestroyInsightPassthroughGeometryInstance(
        geometryInstanceHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_DestroyInsightPassthroughGeometryInstance",
                (geometryInstanceHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_DestroyInsightTriangleMesh(
        meshHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_DestroyInsightTriangleMesh", (meshHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetInsightPassthroughInitialized() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetInsightPassthroughInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_InitializeInsightPassthrough() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_InitializeInsightPassthrough", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetInsightPassthroughStyle(
        layerId: i32,
        style: crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetInsightPassthroughStyle", (layerId, style))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_ShutdownInsightPassthrough() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_ShutdownInsightPassthrough", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UpdateInsightPassthroughGeometryTransform(
        geometryInstanceHandle: u64,
        T_world_model: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_UpdateInsightPassthroughGeometryTransform",
                (geometryInstanceHandle, T_world_model),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_64_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_64_0 => ""
    ."OVRPlugin/OVRP_1_64_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_64_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_64_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_64_0 {
    pub fn ovrp_LocateSpace(
        location: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Posef,
        >,
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        trackingOrigin: crate::GlobalNamespace::OVRPlugin_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_LocateSpace", (location, space, trackingOrigin))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_65_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_65_0 => ""
    ."OVRPlugin/OVRP_1_65_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_65_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_65_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_65_0 {
    pub fn ovrp_DestroySpace(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_DestroySpace", (space))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_KtxDestroy(
        texture: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_KtxDestroy", (texture))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_KtxGetTextureData(
        texture: crate::System::IntPtr,
        data: crate::System::IntPtr,
        bufferSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_KtxGetTextureData", (texture, data, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_KtxLoadFromMemory(
        data: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        length: u32,
        texture: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_KtxLoadFromMemory", (data, length, texture))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_KtxTextureHeight(
        texture: crate::System::IntPtr,
        height: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_KtxTextureHeight", (texture, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_KtxTextureSize(
        texture: crate::System::IntPtr,
        _cordl_size: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_KtxTextureSize", (texture, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_KtxTextureWidth(
        texture: crate::System::IntPtr,
        width: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_KtxTextureWidth", (texture, width))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_KtxTranscode(
        texture: crate::System::IntPtr,
        format: u32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_KtxTranscode", (texture, format))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_66_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_66_0 => ""
    ."OVRPlugin/OVRP_1_66_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_66_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_66_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_66_0 {
    pub fn ovrp_GetInsightPassthroughInitializationState() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetInsightPassthroughInitializationState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Media_IsCastingToRemoteClient(
        isCasting: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Media_IsCastingToRemoteClient", (isCasting))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_67_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_67_0 => ""
    ."OVRPlugin/OVRP_1_67_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_67_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_67_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_68_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_68_0 => ""
    ."OVRPlugin/OVRP_1_68_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_68_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_68_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn ovrp_GetKeyboardState(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        frameIndex: i32,
        keyboardState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_KeyboardState,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetKeyboardState", (stepId, frameIndex, keyboardState))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetRenderModelPaths(
        index: u32,
        path: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetRenderModelPaths", (index, path))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetRenderModelProperties(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        properties: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_RenderModelPropertiesInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetRenderModelProperties", (path, properties))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemKeyboardDescription(
        keyboardQueryFlags: crate::GlobalNamespace::OVRPlugin_TrackedKeyboardQueryFlags,
        keyboardDescription: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_KeyboardDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetSystemKeyboardDescription",
                (keyboardQueryFlags, keyboardDescription),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_LoadRenderModel(
        modelKey: u64,
        bufferInputCapacity: u32,
        bufferCountOutput: quest_hook::libil2cpp::ByRefMut<u32>,
        buffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_LoadRenderModel",
                (modelKey, bufferInputCapacity, bufferCountOutput, buffer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetInsightPassthroughKeyboardHandsIntensity(
        layerId: i32,
        intensity: crate::GlobalNamespace::OVRPlugin_InsightPassthroughKeyboardHandsIntensity,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_SetInsightPassthroughKeyboardHandsIntensity",
                (layerId, intensity),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_StartKeyboardTracking(
        trackedKeyboardId: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_StartKeyboardTracking", (trackedKeyboardId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_StopKeyboardTracking() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_StopKeyboardTracking", ())?;
        Ok(__cordl_ret.into())
    }
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_69_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_69_0 => ""
    ."OVRPlugin/OVRP_1_69_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_69_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_69_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_69_0 {
    pub fn ovrp_GetNodePoseStateImmediate(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        nodePoseState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_PoseStatef,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodePoseStateImmediate", (nodeId, nodePoseState))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_6_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_6_0 => ""
    ."OVRPlugin/OVRP_1_6_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_6_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_6_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_6_0 {
    pub fn ovrp_GetAppCpuStartToGpuEndTime() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetAppCpuStartToGpuEndTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetControllerHapticsDesc(
        controllerMask: u32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_HapticsDesc> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_HapticsDesc = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetControllerHapticsDesc", (controllerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetControllerHapticsState(
        controllerMask: u32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_HapticsState> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_HapticsState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetControllerHapticsState", (controllerMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetEyeRecommendedResolutionScale() -> quest_hook::libil2cpp::Result<
        f32,
    > {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetEyeRecommendedResolutionScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemRecommendedMSAALevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemRecommendedMSAALevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetTrackingIPDEnabled() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetTrackingIPDEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetControllerHaptics(
        controllerMask: u32,
        hapticsBuffer: crate::GlobalNamespace::OVRPlugin_HapticsBuffer,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetControllerHaptics", (controllerMask, hapticsBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetOverlayQuad3(
        flags: u32,
        textureLeft: crate::System::IntPtr,
        textureRight: crate::System::IntPtr,
        device: crate::System::IntPtr,
        pose: crate::GlobalNamespace::OVRPlugin_Posef,
        scale: crate::GlobalNamespace::OVRPlugin_Vector3f,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_SetOverlayQuad3",
                (flags, textureLeft, textureRight, device, pose, scale, layerIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetTrackingIPDEnabled(
        value: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetTrackingIPDEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_70_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_70_0 => ""
    ."OVRPlugin/OVRP_1_70_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_70_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_70_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_70_0 {
    pub fn ovrp_SetLogCallback2(
        logCallback: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRPlugin_LogCallback2DelegateType,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetLogCallback2", (logCallback))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_71_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_71_0 => ""
    ."OVRPlugin/OVRP_1_71_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_71_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_71_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_71_0 {
    pub fn ovrp_GetSuggestedCpuPerformanceLevel(
        perfLevel: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSuggestedCpuPerformanceLevel", (perfLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSuggestedGpuPerformanceLevel(
        perfLevel: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSuggestedGpuPerformanceLevel", (perfLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_IsInsightPassthroughSupported(
        supported: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_IsInsightPassthroughSupported", (supported))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetSuggestedCpuPerformanceLevel(
        perfLevel: crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetSuggestedCpuPerformanceLevel", (perfLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetSuggestedGpuPerformanceLevel(
        perfLevel: crate::GlobalNamespace::OVRPlugin_ProcessorPerformanceLevel,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetSuggestedGpuPerformanceLevel", (perfLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UnityOpenXR_HookGetInstanceProcAddr(
        func: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_UnityOpenXR_HookGetInstanceProcAddr", (func))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UnityOpenXR_OnAppSpaceChange(
        xrSpace: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_UnityOpenXR_OnAppSpaceChange", (xrSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UnityOpenXR_OnInstanceCreate(
        xrInstance: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_UnityOpenXR_OnInstanceCreate", (xrInstance))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UnityOpenXR_OnInstanceDestroy(
        xrInstance: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_UnityOpenXR_OnInstanceDestroy", (xrInstance))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UnityOpenXR_OnSessionBegin(
        xrSession: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_UnityOpenXR_OnSessionBegin", (xrSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UnityOpenXR_OnSessionCreate(
        xrSession: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_UnityOpenXR_OnSessionCreate", (xrSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UnityOpenXR_OnSessionDestroy(
        xrSession: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_UnityOpenXR_OnSessionDestroy", (xrSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UnityOpenXR_OnSessionEnd(
        xrSession: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_UnityOpenXR_OnSessionEnd", (xrSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UnityOpenXR_OnSessionExiting(
        xrSession: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_UnityOpenXR_OnSessionExiting", (xrSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UnityOpenXR_OnSessionStateChange(
        oldState: i32,
        newState: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_UnityOpenXR_OnSessionStateChange", (oldState, newState))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UnityOpenXR_SetClientVersion(
        majorVersion: i32,
        minorVersion: i32,
        patchVersion: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_UnityOpenXR_SetClientVersion",
                (majorVersion, minorVersion, patchVersion),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_72_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_72_0 => ""
    ."OVRPlugin/OVRP_1_72_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_72_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_72_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_72_0 {
    pub fn ovrp_CreateSpatialAnchor(
        createInfo: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_SpatialAnchorCreateInfo,
        >,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_CreateSpatialAnchor", (createInfo, requestId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_EnumerateSpaceSupportedComponents(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        componentTypesCapacityInput: u32,
        componentTypesCountOutput: quest_hook::libil2cpp::ByRefMut<u32>,
        componentTypes: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_EnumerateSpaceSupportedComponents",
                (
                    space,
                    componentTypesCapacityInput,
                    componentTypesCountOutput,
                    componentTypes,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_EraseSpace(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        location: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_EraseSpace", (space, location, requestId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSpaceBoundary2D(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        boundaryInternal: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_PolygonalBoundary2DInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSpaceBoundary2D", (space, boundaryInternal))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSpaceBoundingBox2D(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        rect: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Rectf>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSpaceBoundingBox2D", (space, rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSpaceBoundingBox3D(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        bounds: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Boundsf,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSpaceBoundingBox3D", (space, bounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSpaceComponentStatus(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        componentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        enabled: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Bool>,
        changePending: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetSpaceComponentStatus",
                (space, componentType, enabled, changePending),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSpaceContainer(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        containerInternal: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_SpaceContainerInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSpaceContainer", (space, containerInternal))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSpaceRoomLayout(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        roomLayoutInternal: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_RoomLayoutInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSpaceRoomLayout", (space, roomLayoutInternal))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSpaceSemanticLabels(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        labelsInternal: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_SpaceSemanticLabelInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSpaceSemanticLabels", (space, labelsInternal))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_QuerySpaces(
        queryInfo: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo,
        >,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_QuerySpaces", (queryInfo, requestId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_RequestSceneCapture(
        request: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_SceneCaptureRequestInternal,
        >,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_RequestSceneCapture", (request, requestId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_RetrieveSpaceQueryResults(
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
        resultCapacityInput: u32,
        resultCountOutput: quest_hook::libil2cpp::ByRefMut<u32>,
        results: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_RetrieveSpaceQueryResults",
                (requestId, resultCapacityInput, resultCountOutput, results),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SaveSpace(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        location: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
        mode: crate::GlobalNamespace::OVRPlugin_SpaceStoragePersistenceMode,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SaveSpace", (space, location, mode, requestId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetSpaceComponentStatus(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        componentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        enable: crate::GlobalNamespace::OVRPlugin_Bool,
        timeout: f64,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_SetSpaceComponentStatus",
                (space, componentType, enable, timeout, requestId),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_73_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_73_0 => ""
    ."OVRPlugin/OVRP_1_73_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_73_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_73_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_74_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_74_0 => ""
    ."OVRPlugin/OVRP_1_74_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_74_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_74_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_74_0 {
    pub fn ovrp_ChangeVirtualKeyboardTextContext(
        textContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_ChangeVirtualKeyboardTextContext", (textContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_CreateVirtualKeyboard(
        createInfo: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardCreateInfo,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_CreateVirtualKeyboard", (createInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_CreateVirtualKeyboardSpace(
        createInfo: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardSpaceCreateInfo,
        keyboardSpace: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_CreateVirtualKeyboardSpace", (createInfo, keyboardSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_DestroyVirtualKeyboard() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_DestroyVirtualKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetRenderModelProperties2(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flags: crate::GlobalNamespace::OVRPlugin_RenderModelFlags,
        properties: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_RenderModelPropertiesInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetRenderModelProperties2", (path, flags, properties))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSpaceUuid(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        uuid: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSpaceUuid", (space, uuid))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetVirtualKeyboardScale(
        location: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetVirtualKeyboardScale", (location))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SendVirtualKeyboardInput(
        inputInfo: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardInputInfo,
        interactorRootPose: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Posef,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SendVirtualKeyboardInput", (inputInfo, interactorRootPose))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SuggestVirtualKeyboardLocation(
        locationInfo: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationInfo,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SuggestVirtualKeyboardLocation", (locationInfo))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_75_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_75_0 => ""
    ."OVRPlugin/OVRP_1_75_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_75_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_75_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_76_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_76_0 => ""
    ."OVRPlugin/OVRP_1_76_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_76_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_76_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_76_0 {
    pub fn ovrp_GetNodePoseStateAtTime(
        _cordl_time: f64,
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        nodePoseState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_PoseStatef,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetNodePoseStateAtTime",
                (_cordl_time, nodeId, nodePoseState),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_78_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_78_0 => ""
    ."OVRPlugin/OVRP_1_78_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_78_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_78_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_78_0 {
    pub fn ovrp_GetBodyState(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        frameIndex: i32,
        bodyState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_BodyStateInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetBodyState", (stepId, frameIndex, bodyState))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetBodyTrackingEnabled(
        value: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Bool>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetBodyTrackingEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetBodyTrackingSupported(
        value: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Bool>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetBodyTrackingSupported", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetControllerSampleRateHz(
        controller: crate::GlobalNamespace::OVRPlugin_Controller,
        sampleRateHz: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetControllerSampleRateHz", (controller, sampleRateHz))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetControllerState5(
        controllerMask: u32,
        controllerState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_ControllerState5,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetControllerState5", (controllerMask, controllerState))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetCurrentInteractionProfile(
        hand: crate::GlobalNamespace::OVRPlugin_Hand,
        interactionProfile: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_InteractionProfile,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetCurrentInteractionProfile", (hand, interactionProfile))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetEyeGazesState(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        frameIndex: i32,
        eyeGazesState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_EyeGazesStateInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetEyeGazesState", (stepId, frameIndex, eyeGazesState))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetEyeTrackingEnabled(
        eyeTrackingEnabled: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetEyeTrackingEnabled", (eyeTrackingEnabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetEyeTrackingSupported(
        eyeTrackingSupported: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetEyeTrackingSupported", (eyeTrackingSupported))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetFaceState(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        frameIndex: i32,
        faceState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_FaceStateInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetFaceState", (stepId, frameIndex, faceState))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetFaceTrackingEnabled(
        faceTrackingEnabled: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetFaceTrackingEnabled", (faceTrackingEnabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetFaceTrackingSupported(
        faceTrackingSupported: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetFaceTrackingSupported", (faceTrackingSupported))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetFoveationEyeTracked(
        isEyeTrackedFoveation: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetFoveationEyeTracked", (isEyeTrackedFoveation))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetFoveationEyeTrackedSupported(
        foveationSupported: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetFoveationEyeTrackedSupported", (foveationSupported))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetLocalDimming(
        localDimmingMode: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetLocalDimming", (localDimmingMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetLocalDimmingSupported(
        localDimmingSupported: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetLocalDimmingSupported", (localDimmingSupported))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetPassthroughCapabilityFlags(
        capabilityFlags: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_PassthroughCapabilityFlags,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetPassthroughCapabilityFlags", (capabilityFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetControllerHapticsAmplitudeEnvelope(
        controllerMask: crate::GlobalNamespace::OVRPlugin_Controller,
        hapticsVibration: crate::GlobalNamespace::OVRPlugin_HapticsAmplitudeEnvelopeVibration,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_SetControllerHapticsAmplitudeEnvelope",
                (controllerMask, hapticsVibration),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetControllerHapticsPcm(
        controllerMask: crate::GlobalNamespace::OVRPlugin_Controller,
        hapticsVibration: crate::GlobalNamespace::OVRPlugin_HapticsPcmVibration,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetControllerHapticsPcm", (controllerMask, hapticsVibration))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetControllerLocalizedVibration(
        controllerMask: crate::GlobalNamespace::OVRPlugin_Controller,
        hapticsLocationMask: crate::GlobalNamespace::OVRPlugin_HapticsLocation,
        frequency: f32,
        amplitude: f32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_SetControllerLocalizedVibration",
                (controllerMask, hapticsLocationMask, frequency, amplitude),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetFoveationEyeTracked(
        isEyeTrackedFoveation: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetFoveationEyeTracked", (isEyeTrackedFoveation))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetLocalDimming(
        localDimmingMode: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetLocalDimming", (localDimmingMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_StartBodyTracking() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_StartBodyTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_StartEyeTracking() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_StartEyeTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_StartFaceTracking() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_StartFaceTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_StopBodyTracking() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_StopBodyTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_StopEyeTracking() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_StopEyeTracking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_StopFaceTracking() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_StopFaceTracking", ())?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_79_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_79_0 => ""
    ."OVRPlugin/OVRP_1_79_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_79_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_79_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_79_0 {
    pub fn ovrp_CreateSpaceUser(
        spaceUserId: quest_hook::libil2cpp::ByRefMut<u64>,
        spaceUserHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_CreateSpaceUser", (spaceUserId, spaceUserHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_DeclareUser(
        userId: quest_hook::libil2cpp::ByRefMut<u64>,
        userHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_DeclareUser", (userId, userHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_DestroySpaceUser(
        userHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_DestroySpaceUser", (userHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSpaceUserId(
        spaceUserHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        spaceUserId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSpaceUserId", (spaceUserHandle, spaceUserId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_LocateSpace2(
        location: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_SpaceLocationf,
        >,
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        trackingOrigin: crate::GlobalNamespace::OVRPlugin_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_LocateSpace2", (location, space, trackingOrigin))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_QplCreateMarkerHandle(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nameHandle: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_QplCreateMarkerHandle", (name, nameHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_QplDestroyMarkerHandle(
        nameHandle: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_QplDestroyMarkerHandle", (nameHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_QplMarkerAnnotation(
        markerId: i32,
        annotationKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        annotationValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        instanceKey: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_QplMarkerAnnotation",
                (markerId, annotationKey, annotationValue, instanceKey),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_QplMarkerEnd(
        markerId: i32,
        resultTypeId: crate::GlobalNamespace::Qpl_OVRPlugin_ResultType,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_QplMarkerEnd",
                (markerId, resultTypeId, instanceKey, timestampMs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_QplMarkerPointCached(
        markerId: i32,
        nameHandle: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_QplMarkerPointCached",
                (markerId, nameHandle, instanceKey, timestampMs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_QplMarkerStart(
        markerId: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_QplMarkerStart", (markerId, instanceKey, timestampMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SaveSpaceList(
        spaces: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        numSpaces: u32,
        location: crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SaveSpaceList", (spaces, numSpaces, location, requestId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_ShareSpaces(
        spaces: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        numSpaces: u32,
        userHandles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        numUsers: u32,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_ShareSpaces",
                (spaces, numSpaces, userHandles, numUsers, requestId),
            )?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_7_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_7_0 => ""
    ."OVRPlugin/OVRP_1_7_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_7_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_7_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_7_0 {
    pub fn ovrp_GetAppChromaticCorrection() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetAppChromaticCorrection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetAppChromaticCorrection(
        value: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetAppChromaticCorrection", (value))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_81_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_81_0 => ""
    ."OVRPlugin/OVRP_1_81_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_81_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_81_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_82_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_82_0 => ""
    ."OVRPlugin/OVRP_1_82_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_82_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_82_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_82_0 {
    pub fn ovrp_GetSpaceTriangleMesh(
        space: quest_hook::libil2cpp::ByRefMut<u64>,
        triangleMeshInternal: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_TriangleMeshInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSpaceTriangleMesh", (space, triangleMeshInternal))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_83_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_83_0 => ""
    ."OVRPlugin/OVRP_1_83_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_83_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_83_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_83_0 {
    pub fn ovrp_GetControllerState6(
        controllerMask: u32,
        controllerState: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_ControllerState6,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetControllerState6", (controllerMask, controllerState))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetVirtualKeyboardDirtyTextures(
        textureIds: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureIdsInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetVirtualKeyboardDirtyTextures", (textureIds))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetVirtualKeyboardModelAnimationStates(
        animationStates: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationStatesInternal,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetVirtualKeyboardModelAnimationStates", (animationStates))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetVirtualKeyboardTextureData(
        textureId: u64,
        textureData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_VirtualKeyboardTextureData,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetVirtualKeyboardTextureData", (textureId, textureData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetVirtualKeyboardModelVisibility(
        visibility: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelVisibility,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetVirtualKeyboardModelVisibility", (visibility))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_84_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_84_0 => ""
    ."OVRPlugin/OVRP_1_84_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_84_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_84_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_84_0 {
    pub fn ovrp_CreatePassthroughColorLut(
        channels: crate::GlobalNamespace::OVRPlugin_PassthroughColorLutChannels,
        resolution: u32,
        data: crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData,
        colorLut: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_CreatePassthroughColorLut",
                (channels, resolution, data, colorLut),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_DestroyPassthroughColorLut(
        colorLut: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_DestroyPassthroughColorLut", (colorLut))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetEyeLayerRecommendedResolution(
        recommendedDimensions: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Sizei,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetEyeLayerRecommendedResolution", (recommendedDimensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetLayerRecommendedResolution(
        layerId: i32,
        recommendedDimensions: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Sizei,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetLayerRecommendedResolution",
                (layerId, recommendedDimensions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetInsightPassthroughStyle2(
        layerId: i32,
        style: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle2,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetInsightPassthroughStyle2", (layerId, style))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_UpdatePassthroughColorLut(
        colorLut: u64,
        data: crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_UpdatePassthroughColorLut", (colorLut, data))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_85_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_85_0 => ""
    ."OVRPlugin/OVRP_1_85_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_85_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_85_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_85_0 {
    pub fn ovrp_GetPassthroughCapabilities(
        capabilityFlags: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_PassthroughCapabilities,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetPassthroughCapabilities", (capabilityFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_OnEditorShutdown() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Result,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_OnEditorShutdown", ())?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_86_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_86_0 => ""
    ."OVRPlugin/OVRP_1_86_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_86_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_86_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_86_0 {
    pub fn ovrp_AreHandPosesGeneratedByControllerData(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        isGeneratedByControllerData: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_AreHandPosesGeneratedByControllerData",
                (stepId, nodeId, isGeneratedByControllerData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetControllerIsInHand(
        stepId: crate::GlobalNamespace::OVRPlugin_Step,
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        isInHand: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Bool>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetControllerIsInHand", (stepId, nodeId, isInHand))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetCurrentDetachedInteractionProfile(
        hand: crate::GlobalNamespace::OVRPlugin_Hand,
        interactionProfile: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_InteractionProfile,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_GetCurrentDetachedInteractionProfile",
                (hand, interactionProfile),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_IsControllerDrivenHandPosesEnabled(
        enabled: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Bool>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_IsControllerDrivenHandPosesEnabled", (enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_IsMultimodalHandsControllersSupported(
        supported: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_Bool,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_IsMultimodalHandsControllersSupported", (supported))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetControllerDrivenHandPoses(
        controllerDrivenHandPoses: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetControllerDrivenHandPoses", (controllerDrivenHandPoses))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetMultimodalHandsControllersSupported(
        supported: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetMultimodalHandsControllersSupported", (supported))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_87_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_87_0 => ""
    ."OVRPlugin/OVRP_1_87_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_87_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_87_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_87_0 {
    pub fn ovrp_AreControllerDrivenHandPosesNatural(
        natural: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_Bool>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_AreControllerDrivenHandPosesNatural", (natural))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetPassthroughPreferences(
        preferences: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_PassthroughPreferences,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetPassthroughPreferences", (preferences))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetControllerDrivenHandPosesAreNatural(
        controllerDrivenHandPosesAreNatural: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ovrp_SetControllerDrivenHandPosesAreNatural",
                (controllerDrivenHandPosesAreNatural),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetEyeBufferSharpenType(
        sharpenType: crate::GlobalNamespace::OVRPlugin_LayerSharpenType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetEyeBufferSharpenType", (sharpenType))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_88_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_88_0 => ""
    ."OVRPlugin/OVRP_1_88_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_88_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_88_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_88_0 {
    pub fn ovrp_SetSimultaneousHandsAndControllersEnabled(
        enabled: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Result> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Result = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetSimultaneousHandsAndControllersEnabled", (enabled))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_8_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_8_0 => ""
    ."OVRPlugin/OVRP_1_8_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_8_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_8_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_8_0 {
    pub fn ovrp_GetBoundaryConfigured() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetBoundaryConfigured", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetBoundaryDimensions(
        boundaryType: crate::GlobalNamespace::OVRPlugin_BoundaryType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Vector3f> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Vector3f = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetBoundaryDimensions", (boundaryType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetBoundaryGeometry(
        boundaryType: crate::GlobalNamespace::OVRPlugin_BoundaryType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_BoundaryGeometry,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_BoundaryGeometry = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetBoundaryGeometry", (boundaryType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetBoundaryVisible() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetBoundaryVisible", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNodeAcceleration2(
        stateId: i32,
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodeAcceleration2", (stateId, nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNodePose2(
        stateId: i32,
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodePose2", (stateId, nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetNodeVelocity2(
        stateId: i32,
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Posef> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Posef = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetNodeVelocity2", (stateId, nodeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_SetBoundaryVisible(
        value: crate::GlobalNamespace::OVRPlugin_Bool,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_SetBoundaryVisible", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_TestBoundaryNode(
        nodeId: crate::GlobalNamespace::OVRPlugin_Node,
        boundaryType: crate::GlobalNamespace::OVRPlugin_BoundaryType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_BoundaryTestResult,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_BoundaryTestResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_TestBoundaryNode", (nodeId, boundaryType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_TestBoundaryPoint(
        point: crate::GlobalNamespace::OVRPlugin_Vector3f,
        boundaryType: crate::GlobalNamespace::OVRPlugin_BoundaryType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_BoundaryTestResult,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_BoundaryTestResult = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_TestBoundaryPoint", (point, boundaryType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_Update2(
        stateId: i32,
        frameIndex: i32,
        predictionSeconds: f64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_Update2", (stateId, frameIndex, predictionSeconds))?;
        Ok(__cordl_ret.into())
    }
}
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+OVRP_1_9_0")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_OVRP_1_9_0 => ""
    ."OVRPlugin/OVRP_1_9_0"
);
#[cfg(feature = "OVRPlugin+OVRP_1_9_0")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_OVRP_1_9_0 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::OVRPlugin_OVRP_1_9_0 {
    pub fn ovrp_GetActiveController() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Controller,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Controller = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetActiveController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetAppPerfStats() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_AppPerfStats,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_AppPerfStats = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetAppPerfStats", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetBoundaryGeometry2(
        boundaryType: crate::GlobalNamespace::OVRPlugin_BoundaryType,
        points: crate::System::IntPtr,
        pointsCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_Bool> {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetBoundaryGeometry2", (boundaryType, points, pointsCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetConnectedControllers() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Controller,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Controller = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetConnectedControllers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_GetSystemHeadsetType() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_SystemHeadset,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SystemHeadset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_GetSystemHeadsetType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ovrp_ResetAppPerfStats() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_Bool,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_Bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ovrp_ResetAppPerfStats", ())?;
        Ok(__cordl_ret.into())
    }
}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_OverlayFlag {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_OverlayShape {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_PassthroughCapabilityFields {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_PassthroughCapabilityFlags {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_PassthroughColorLutChannels {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_PassthroughPreferenceFields {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_PassthroughPreferenceFlags {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_PerfMetrics {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        pinnedArray: crate::GlobalNamespace::OVRPlugin_PinnedArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (pinnedArray))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+PinnedArray_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::OVRPlugin_PinnedArray_1<T> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRPlugin+PinnedArray_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::OVRPlugin_PinnedArray_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "OVRPlugin+PlatformUI")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_PlatformUI {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+ProcessorPerformanceLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_ProcessorPerformanceLevel {
    #[default]
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+Qpl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_Qpl => ""
    ."OVRPlugin/Qpl"
);
#[cfg(feature = "OVRPlugin+Qpl")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_Qpl {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn CreateMarkerHandle(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nameHandle: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateMarkerHandle", (name, nameHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyMarkerHandle(nameHandle: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyMarkerHandle", (nameHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerAnnotation(
        markerId: i32,
        annotationKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        annotationValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        instanceKey: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MarkerAnnotation",
                (markerId, annotationKey, annotationValue, instanceKey),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerEnd(
        markerId: i32,
        resultTypeId: crate::GlobalNamespace::Qpl_OVRPlugin_ResultType,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MarkerEnd", (markerId, resultTypeId, instanceKey, timestampMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerPointCached(
        markerId: i32,
        nameHandle: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MarkerPointCached",
                (markerId, nameHandle, instanceKey, timestampMs),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkerStart(
        markerId: i32,
        instanceKey: i32,
        timestampMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MarkerStart", (markerId, instanceKey, timestampMs))?;
        Ok(__cordl_ret.into())
    }
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
#[derive(Debug, Clone, Default, PartialEq)]
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+RecenterFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_RecenterFlags {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_RenderModelFlags {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_RenderModelProperties {
    pub ModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_RenderModelPropertiesInternal {
    pub ModelName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_Result {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_RoomLayout {
    pub floorUuid: crate::System::Guid,
    pub ceilingUuid: crate::System::Guid,
    pub wallUuids: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::System::Guid>,
    >,
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_SceneCaptureRequestInternal {
    pub requestByteCount: i32,
    pub request: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_OVRPlugin_Sizei0(
        &mut self,
        other: crate::GlobalNamespace::OVRPlugin_Sizei,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+Sizei")]
impl AsRef<crate::System::IEquatable_1<crate::GlobalNamespace::OVRPlugin_Sizei>>
for crate::GlobalNamespace::OVRPlugin_Sizei {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::GlobalNamespace::OVRPlugin_Sizei> {
        todo!()
    }
}
#[cfg(feature = "OVRPlugin+Sizei")]
impl AsMut<crate::System::IEquatable_1<crate::GlobalNamespace::OVRPlugin_Sizei>>
for crate::GlobalNamespace::OVRPlugin_Sizei {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::GlobalNamespace::OVRPlugin_Sizei> {
        todo!()
    }
}
#[cfg(feature = "OVRPlugin+Skeleton")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_Skeleton {
    pub Type: crate::GlobalNamespace::OVRPlugin_SkeletonType,
    pub NumBones: u32,
    pub NumBoneCapsules: u32,
    pub Bones: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Bone>,
    >,
    pub BoneCapsules: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_BoneCapsule>,
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_Skeleton2 {
    pub Type: crate::GlobalNamespace::OVRPlugin_SkeletonType,
    pub NumBones: u32,
    pub NumBoneCapsules: u32,
    pub Bones: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_Bone>,
    >,
    pub BoneCapsules: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::OVRPlugin_BoneCapsule>,
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_SkeletonConstants {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_SkeletonType {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_SpaceComponentType {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_SpaceFilterInfoComponents {
    pub Components: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        >,
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_SpaceFilterInfoIds {
    pub Ids: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::System::Guid>,
    >,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_SpaceLocationFlags {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_SpaceQueryActionType {
    #[default]
    Load = 0i32,
}
#[cfg(feature = "OVRPlugin+SpaceQueryActionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SpaceQueryActionType
    => ""."OVRPlugin/SpaceQueryActionType"
);
#[cfg(feature = "OVRPlugin+SpaceQueryFilterType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_SpaceQueryFilterType {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_SpaceQueryType {
    #[default]
    Action = 0i32,
}
#[cfg(feature = "OVRPlugin+SpaceQueryType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_SpaceQueryType => ""
    ."OVRPlugin/SpaceQueryType"
);
#[cfg(feature = "OVRPlugin+SpaceSemanticLabelInternal")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_SpaceStorageLocation {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_SpaceStoragePersistenceMode {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_Step {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_SystemHeadset {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_SystemRegion {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+TiledMultiResLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_TiledMultiResLevel {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_TrackedKeyboardFlags {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_TrackedKeyboardPresentationStyles {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_TrackedKeyboardQueryFlags {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_Tracker {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_TrackingConfidence {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_TrackingOrigin {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPlugin+UnityOpenXR")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlugin_UnityOpenXR => ""
    ."OVRPlugin/UnityOpenXR"
);
#[cfg(feature = "OVRPlugin+UnityOpenXR")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlugin_UnityOpenXR {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn HookGetInstanceProcAddr(
        func: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HookGetInstanceProcAddr", (func))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnAppSpaceChange(
        xrSpace: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnAppSpaceChange", (xrSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnInstanceCreate(xrInstance: u64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnInstanceCreate", (xrInstance))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnInstanceDestroy(
        xrInstance: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnInstanceDestroy", (xrInstance))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSessionBegin(
        xrSession: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSessionBegin", (xrSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSessionCreate(
        xrSession: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSessionCreate", (xrSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSessionDestroy(
        xrSession: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSessionDestroy", (xrSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSessionEnd(
        xrSession: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSessionEnd", (xrSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSessionExiting(
        xrSession: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSessionExiting", (xrSession))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSessionStateChange(
        oldState: i32,
        newState: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnSessionStateChange", (oldState, newState))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetClientVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetClientVersion", ())?;
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+Vector4f")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+Vector4s")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardCreateInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_VirtualKeyboardInputSource {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_VirtualKeyboardInputStateFlags {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_VirtualKeyboardLocationType {
    #[default]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_VirtualKeyboardModelAnimationStates {
    pub States: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVRPlugin_VirtualKeyboardModelAnimationState,
        >,
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPlugin+VirtualKeyboardSpaceCreateInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPlugin_VirtualKeyboardTextureIds {
    pub TextureIds: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlugin_XrApi {
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Qpl_OVRPlugin_ResultType {
    #[default]
    Cancel = 4i16,
    Fail = 3i16,
    Success = 2i16,
}
#[cfg(feature = "OVRPlugin+Qpl+ResultType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Qpl_OVRPlugin_ResultType => ""
    ."OVRPlugin/Qpl/ResultType"
);
