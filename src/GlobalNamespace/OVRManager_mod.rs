#[cfg(feature = "OVRManager")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub disabledCameras: *mut crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::UnityEngine::Camera,
    >,
    pub prevTimeScale: f32,
    pub useRecommendedMSAALevel: bool,
    pub _monoscopic: bool,
    pub _sharpenType: crate::GlobalNamespace::OVRPlugin_LayerSharpenType,
    pub _colorGamut: crate::GlobalNamespace::OVRManager_ColorSpace,
    pub enableDynamicResolution: bool,
    pub minDynamicResolutionScale: f32,
    pub maxDynamicResolutionScale: f32,
    pub minRenderScale: f32,
    pub maxRenderScale: f32,
    pub _headPoseRelativeOffsetRotation: crate::UnityEngine::Vector3,
    pub _headPoseRelativeOffsetTranslation: crate::UnityEngine::Vector3,
    pub profilerTcpPort: i32,
    pub expandMixedRealityCapturePropertySheet: bool,
    pub enableMixedReality: bool,
    pub compositionMethod: crate::GlobalNamespace::OVRManager_CompositionMethod,
    pub extraHiddenLayers: crate::UnityEngine::LayerMask,
    pub extraVisibleLayers: crate::UnityEngine::LayerMask,
    pub dynamicCullingMask: bool,
    pub externalCompositionBackdropColorRift: crate::UnityEngine::Color,
    pub externalCompositionBackdropColorQuest: crate::UnityEngine::Color,
    pub capturingCameraDevice: crate::GlobalNamespace::OVRManager_CameraDevice,
    pub flipCameraFrameHorizontally: bool,
    pub flipCameraFrameVertically: bool,
    pub handPoseStateLatency: f32,
    pub sandwichCompositionRenderLatency: f32,
    pub sandwichCompositionBufferedFrames: i32,
    pub chromaKeyColor: crate::UnityEngine::Color,
    pub chromaKeySimilarity: f32,
    pub chromaKeySmoothRange: f32,
    pub chromaKeySpillRange: f32,
    pub useDynamicLighting: bool,
    pub depthQuality: crate::GlobalNamespace::OVRManager_DepthQuality,
    pub dynamicLightingSmoothFactor: f32,
    pub dynamicLightingDepthVariationClampingValue: f32,
    pub virtualGreenScreenType: crate::GlobalNamespace::OVRManager_VirtualGreenScreenType,
    pub virtualGreenScreenTopY: f32,
    pub virtualGreenScreenBottomY: f32,
    pub virtualGreenScreenApplyDepthCulling: bool,
    pub virtualGreenScreenDepthTolerance: f32,
    pub mrcActivationMode: crate::GlobalNamespace::OVRManager_MrcActivationMode,
    pub instantiateMixedRealityCameraGameObject: *mut crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
    pub launchMultimodalHandsControllersOnStartup: bool,
    pub isInsightPassthroughEnabled: bool,
    pub requestBodyTrackingPermissionOnStartup: bool,
    pub requestFaceTrackingPermissionOnStartup: bool,
    pub requestEyeTrackingPermissionOnStartup: bool,
    pub requestScenePermissionOnStartup: bool,
    pub _localDimming: bool,
    pub _trackingOriginType: crate::GlobalNamespace::OVRManager_TrackingOrigin,
    pub usePositionTracking: bool,
    pub useRotationTracking: bool,
    pub useIPDInPositionTracking: bool,
    pub resetTrackerOnLoad: bool,
    pub AllowRecenter: bool,
    pub LateControllerUpdate: bool,
    pub LateLatching: bool,
    pub _readOnlyControllerDrivenHandPosesType: crate::GlobalNamespace::OVRManager_ControllerDrivenHandPosesType,
    pub controllerDrivenHandPosesType: crate::GlobalNamespace::OVRManager_ControllerDrivenHandPosesType,
    pub _isSupportedPlatform_k__BackingField: bool,
    pub eventListeners: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::GlobalNamespace::OVRManager_EventListener,
    >,
}
#[cfg(feature = "OVRManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager => ""."OVRManager"
);
#[cfg(feature = "OVRManager")]
impl std::ops::Deref for crate::GlobalNamespace::OVRManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRManager")]
impl crate::GlobalNamespace::OVRManager {
    pub const _pixelStepPerFrame: i32 = 32i32;
    #[cfg(feature = "OVRManager+CameraDevice")]
    pub type CameraDevice = crate::GlobalNamespace::OVRManager_CameraDevice;
    #[cfg(feature = "OVRManager+ColorSpace")]
    pub type ColorSpace = crate::GlobalNamespace::OVRManager_ColorSpace;
    #[cfg(feature = "OVRManager+CompositionMethod")]
    pub type CompositionMethod = crate::GlobalNamespace::OVRManager_CompositionMethod;
    #[cfg(feature = "OVRManager+ControllerDrivenHandPosesType")]
    pub type ControllerDrivenHandPosesType = crate::GlobalNamespace::OVRManager_ControllerDrivenHandPosesType;
    #[cfg(feature = "OVRManager+DepthQuality")]
    pub type DepthQuality = crate::GlobalNamespace::OVRManager_DepthQuality;
    #[cfg(feature = "OVRManager+EventListener")]
    type EventListener = crate::GlobalNamespace::OVRManager_EventListener;
    #[cfg(feature = "OVRManager+EyeTextureFormat")]
    pub type EyeTextureFormat = crate::GlobalNamespace::OVRManager_EyeTextureFormat;
    #[cfg(feature = "OVRManager+FixedFoveatedRenderingLevel")]
    pub type FixedFoveatedRenderingLevel = crate::GlobalNamespace::OVRManager_FixedFoveatedRenderingLevel;
    #[cfg(feature = "OVRManager+FoveatedRenderingLevel")]
    pub type FoveatedRenderingLevel = crate::GlobalNamespace::OVRManager_FoveatedRenderingLevel;
    #[cfg(feature = "OVRManager+InstantiateMrcCameraDelegate")]
    pub type InstantiateMrcCameraDelegate = crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate;
    #[cfg(feature = "OVRManager+MrcActivationMode")]
    pub type MrcActivationMode = crate::GlobalNamespace::OVRManager_MrcActivationMode;
    #[cfg(feature = "OVRManager+MrcCameraType")]
    pub type MrcCameraType = crate::GlobalNamespace::OVRManager_MrcCameraType;
    #[cfg(feature = "OVRManager+PassthroughCapabilities")]
    pub type PassthroughCapabilities = crate::GlobalNamespace::OVRManager_PassthroughCapabilities;
    #[cfg(feature = "OVRManager+PassthroughInitializationState")]
    pub type PassthroughInitializationState = crate::GlobalNamespace::OVRManager_PassthroughInitializationState;
    #[cfg(feature = "OVRManager+ProcessorPerformanceLevel")]
    pub type ProcessorPerformanceLevel = crate::GlobalNamespace::OVRManager_ProcessorPerformanceLevel;
    #[cfg(feature = "OVRManager+SystemHeadsetType")]
    pub type SystemHeadsetType = crate::GlobalNamespace::OVRManager_SystemHeadsetType;
    #[cfg(feature = "OVRManager+TiledMultiResLevel")]
    pub type TiledMultiResLevel = crate::GlobalNamespace::OVRManager_TiledMultiResLevel;
    #[cfg(feature = "OVRManager+TrackingOrigin")]
    pub type TrackingOrigin = crate::GlobalNamespace::OVRManager_TrackingOrigin;
    #[cfg(feature = "OVRManager+VirtualGreenScreenType")]
    pub type VirtualGreenScreenType = crate::GlobalNamespace::OVRManager_VirtualGreenScreenType;
    #[cfg(feature = "OVRManager+XRDevice")]
    pub type XRDevice = crate::GlobalNamespace::OVRManager_XRDevice;
    #[cfg(feature = "OVRManager+XrApi")]
    pub type XrApi = crate::GlobalNamespace::OVRManager_XrApi;
    #[cfg(feature = "OVRManager+__c")]
    pub type __c = crate::GlobalNamespace::OVRManager___c;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMixedRealityCaptureConfigurationFileFromCmd() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateMixedRealityCaptureConfigurationFileFromCmd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DeregisterEventListener(
        &mut self,
        listener: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRManager_EventListener,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeregisterEventListener", (listener))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindMainCamera() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindMainCamera", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FixedUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixedUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentDisplaySubsystem() -> quest_hook::libil2cpp::Result<Blacklisted> {
        let __cordl_ret: Blacklisted = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentDisplaySubsystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentDisplaySubsystemDescriptor() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRDisplaySubsystemDescriptor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::XR::XRDisplaySubsystemDescriptor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentDisplaySubsystemDescriptor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentInputSubsystem() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::XRInputSubsystem>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::XR::XRInputSubsystem,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentInputSubsystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOpenVRControllerOffset(
        hand: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        let __cordl_ret: crate::GlobalNamespace::OVRPose = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOpenVRControllerOffset", (hand))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPassthroughCapabilities() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRManager_PassthroughCapabilities,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRManager_PassthroughCapabilities,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPassthroughCapabilities", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpaceWarp() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpaceWarp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasInsightPassthroughInitFailed() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasInsightPassthroughInitFailed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitOVRManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitOVRManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitPermissionRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitPermissionRequest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeInsightPassthrough() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeInsightPassthrough", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAdaptiveResSupportedByEngine() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAdaptiveResSupportedByEngine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInsightPassthroughInitPending() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInsightPassthroughInitPending", ())?;
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
    pub fn IsMultimodalHandsControllersSupported() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMultimodalHandsControllersSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPassthroughRecommended() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPassthroughRecommended", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsUnityAlphaOrBetaVersion() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsUnityAlphaOrBetaVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadMixedRealityCaptureConfigurationFileFromCmd() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadMixedRealityCaptureConfigurationFileFromCmd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MixedRealityEnabledFromCmd() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MixedRealityEnabledFromCmd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_capturingCameraDevice(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRManager_CameraDevice> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_CameraDevice = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_capturingCameraDevice",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_chromaKeyColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_chromaKeyColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_chromaKeySimilarity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_chromaKeySimilarity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_chromaKeySmoothRange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_chromaKeySmoothRange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_chromaKeySpillRange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_chromaKeySpillRange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_compositionMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_CompositionMethod,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_CompositionMethod = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_compositionMethod", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_depthQuality(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRManager_DepthQuality> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_DepthQuality = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_depthQuality", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_dynamicCullingMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_dynamicCullingMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_dynamicLightingDepthVariationClampingValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_dynamicLightingDepthVariationClampingValue",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_dynamicLightingSmoothFactor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_dynamicLightingSmoothFactor",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_enableMixedReality(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_enableMixedReality", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_externalCompositionBackdropColorQuest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_externalCompositionBackdropColorQuest",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_externalCompositionBackdropColorRift(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_externalCompositionBackdropColorRift",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_extraHiddenLayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_extraHiddenLayers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_extraVisibleLayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LayerMask = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_extraVisibleLayers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_flipCameraFrameHorizontally(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_flipCameraFrameHorizontally",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_flipCameraFrameVertically(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_flipCameraFrameVertically",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_handPoseStateLatency(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_handPoseStateLatency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_instantiateMixedRealityCameraGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
        > = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_instantiateMixedRealityCameraGameObject",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_mrcActivationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_MrcActivationMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_MrcActivationMode = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_mrcActivationMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_sandwichCompositionBufferedFrames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_sandwichCompositionBufferedFrames",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_sandwichCompositionRenderLatency(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_sandwichCompositionRenderLatency",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_useDynamicLighting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.get_useDynamicLighting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenApplyDepthCulling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenApplyDepthCulling",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenBottomY(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenBottomY",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenDepthTolerance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenDepthTolerance",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenTopY(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenTopY",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_get_virtualGreenScreenType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_VirtualGreenScreenType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_VirtualGreenScreenType = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.get_virtualGreenScreenType",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_capturingCameraDevice(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_CameraDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_capturingCameraDevice",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_chromaKeyColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.set_chromaKeyColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_chromaKeySimilarity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_chromaKeySimilarity",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_chromaKeySmoothRange(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_chromaKeySmoothRange",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_chromaKeySpillRange(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_chromaKeySpillRange",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_compositionMethod(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_CompositionMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_compositionMethod",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_depthQuality(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_DepthQuality,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OVRMixedRealityCaptureConfiguration.set_depthQuality", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_dynamicCullingMask(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_dynamicCullingMask",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_dynamicLightingDepthVariationClampingValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_dynamicLightingDepthVariationClampingValue",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_dynamicLightingSmoothFactor(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_dynamicLightingSmoothFactor",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_enableMixedReality(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_enableMixedReality",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_externalCompositionBackdropColorQuest(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_externalCompositionBackdropColorQuest",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_externalCompositionBackdropColorRift(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_externalCompositionBackdropColorRift",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_extraHiddenLayers(
        &mut self,
        value: crate::UnityEngine::LayerMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_extraHiddenLayers",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_extraVisibleLayers(
        &mut self,
        value: crate::UnityEngine::LayerMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_extraVisibleLayers",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_flipCameraFrameHorizontally(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_flipCameraFrameHorizontally",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_flipCameraFrameVertically(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_flipCameraFrameVertically",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_handPoseStateLatency(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_handPoseStateLatency",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_instantiateMixedRealityCameraGameObject(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_instantiateMixedRealityCameraGameObject",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_mrcActivationMode(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_MrcActivationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_mrcActivationMode",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_sandwichCompositionBufferedFrames(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_sandwichCompositionBufferedFrames",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_sandwichCompositionRenderLatency(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_sandwichCompositionRenderLatency",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_useDynamicLighting(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_useDynamicLighting",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenApplyDepthCulling(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenApplyDepthCulling",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenBottomY(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenBottomY",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenDepthTolerance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenDepthTolerance",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenTopY(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenTopY",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OVRMixedRealityCaptureConfiguration_set_virtualGreenScreenType(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_VirtualGreenScreenType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OVRMixedRealityCaptureConfiguration.set_virtualGreenScreenType",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OnApplicationFocus(
        &mut self,
        focus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationFocus", (focus))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnApplicationPause(
        &mut self,
        pause: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationPause", (pause))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnApplicationQuit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationQuit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPermissionGranted(
        permissionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnPermissionGranted", (permissionId))?;
        Ok(__cordl_ret.into())
    }
    pub fn PassthroughInitializedOrPending(
        state: crate::GlobalNamespace::OVRManager_PassthroughInitializationState,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PassthroughInitializedOrPending", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn PlatformUIConfirmQuit() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PlatformUIConfirmQuit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterEventListener(
        &mut self,
        listener: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRManager_EventListener,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterEventListener", (listener))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReturnToLauncher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReturnToLauncher", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorScaleAndOffset(
        colorScale: crate::UnityEngine::Vector4,
        colorOffset: crate::UnityEngine::Vector4,
        applyToAllLayers: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetColorScaleAndOffset",
                (colorScale, colorOffset, applyToAllLayers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCurrentXRDevice(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCurrentXRDevice", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDepthSubmission(enable: bool) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDepthSubmission", (enable))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetOpenVRLocalPose(
        leftPos: crate::UnityEngine::Vector3,
        rightPos: crate::UnityEngine::Vector3,
        leftRot: crate::UnityEngine::Quaternion,
        rightRot: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetOpenVRLocalPose", (leftPos, rightPos, leftRot, rightRot))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSpaceWarp(
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSpaceWarp", (enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShutdownInsightPassthrough() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShutdownInsightPassthrough", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StaticInitializeMixedRealityCapture(
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StaticInitializeMixedRealityCapture", (configuration))?;
        Ok(__cordl_ret.into())
    }
    pub fn StaticShutdownMixedRealityCapture(
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StaticShutdownMixedRealityCapture", (configuration))?;
        Ok(__cordl_ret.into())
    }
    pub fn StaticUpdateMixedRealityCapture(
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        trackingOrigin: crate::GlobalNamespace::OVRManager_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "StaticUpdateMixedRealityCapture",
                (configuration, gameObject, trackingOrigin),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateHMDEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateHMDEvents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInsightPassthrough(
        shouldBeEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateInsightPassthrough", (shouldBeEnabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn UseDirectCompositionFromCmd() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UseDirectCompositionFromCmd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UseExternalCompositionFromCmd() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UseExternalCompositionFromCmd", ())?;
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
    pub fn add_AudioInChanged(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_AudioInChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_AudioOutChanged(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_AudioOutChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_DisplayRefreshRateChanged(
        value: quest_hook::libil2cpp::Gc<crate::System::Action_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_DisplayRefreshRateChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_HMDAcquired(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_HMDAcquired", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_HMDLost(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_HMDLost", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_HMDMounted(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_HMDMounted", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_HMDUnmounted(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_HMDUnmounted", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_HSWDismissed(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_HSWDismissed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_InputFocusAcquired(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_InputFocusAcquired", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_InputFocusLost(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_InputFocusLost", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_SceneCaptureComplete(
        value: quest_hook::libil2cpp::Gc<crate::System::Action_2<u64, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_SceneCaptureComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_ShareSpacesComplete(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                u64,
                crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_ShareSpacesComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_SpaceEraseComplete(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                u64,
                bool,
                crate::System::Guid,
                crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_SpaceEraseComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_SpaceListSaveComplete(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                u64,
                crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_SpaceListSaveComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_SpaceQueryComplete(
        value: quest_hook::libil2cpp::Gc<crate::System::Action_2<u64, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_SpaceQueryComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_SpaceQueryResults(
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_SpaceQueryResults", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_SpaceSaveComplete(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                u64,
                crate::GlobalNamespace::OVRSpace,
                bool,
                crate::System::Guid,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_SpaceSaveComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_SpaceSetComponentStatusComplete(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_6<
                u64,
                bool,
                crate::GlobalNamespace::OVRSpace,
                crate::System::Guid,
                crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_SpaceSetComponentStatusComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_SpatialAnchorCreateComplete(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                u64,
                bool,
                crate::GlobalNamespace::OVRSpace,
                crate::System::Guid,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_SpatialAnchorCreateComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_TrackingAcquired(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_TrackingAcquired", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_TrackingLost(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_TrackingLost", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_VrFocusAcquired(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_VrFocusAcquired", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_VrFocusLost(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_VrFocusLost", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSimultaneousHandsAndControllersSupported(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsSimultaneousHandsAndControllersSupported", ())?;
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
    pub fn get_batteryStatus() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_batteryStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_batteryTemperature() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_batteryTemperature", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_boundary() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRBoundary>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRBoundary,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_boundary", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_chromatic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_chromatic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorGamut(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRManager_ColorSpace> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_ColorSpace = __cordl_object
            .invoke("get_colorGamut", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cpuLevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_cpuLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_display() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRDisplay>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRDisplay> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_display", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeFovPremultipliedAlphaModeEnabled() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeFovPremultipliedAlphaModeEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeTextureFormat() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_EyeTextureFormat,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRManager_EyeTextureFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeTextureFormat", ())?;
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
    pub fn get_fixedFoveatedRenderingLevel() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_FixedFoveatedRenderingLevel,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRManager_FixedFoveatedRenderingLevel = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fixedFoveatedRenderingLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fixedFoveatedRenderingSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fixedFoveatedRenderingSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_foveatedRenderingLevel() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_FoveatedRenderingLevel,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRManager_FoveatedRenderingLevel = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_foveatedRenderingLevel", ())?;
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
    pub fn get_headPoseRelativeOffsetRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_headPoseRelativeOffsetRotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headPoseRelativeOffsetTranslation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_headPoseRelativeOffsetTranslation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRManager>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRManager> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_instance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isHmdPresent() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isHmdPresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isPowerSavingActive() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isPowerSavingActive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isSupportedPlatform(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isSupportedPlatform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isUserPresent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isUserPresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_monoscopic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_monoscopic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nativeColorGamut(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRManager_ColorSpace> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_ColorSpace = __cordl_object
            .invoke("get_nativeColorGamut", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pluginVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_pluginVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_profile() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRProfile>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRProfile> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_profile", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_runtimeSettings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRRuntimeSettings>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRRuntimeSettings,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_runtimeSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sdkVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_sdkVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sharpenType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_LayerSharpenType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_LayerSharpenType = __cordl_object
            .invoke("get_sharpenType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_suggestedCpuPerfLevel() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_ProcessorPerformanceLevel,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRManager_ProcessorPerformanceLevel = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_suggestedCpuPerfLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_suggestedGpuPerfLevel() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_ProcessorPerformanceLevel,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRManager_ProcessorPerformanceLevel = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_suggestedGpuPerfLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_systemHeadsetType() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_SystemHeadsetType,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRManager_SystemHeadsetType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_systemHeadsetType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tiledMultiResLevel() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_TiledMultiResLevel,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRManager_TiledMultiResLevel = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_tiledMultiResLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tiledMultiResSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_tiledMultiResSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tracker() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRTracker>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRTracker> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_tracker", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_trackingOriginType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRManager_TrackingOrigin,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_TrackingOrigin = __cordl_object
            .invoke("get_trackingOriginType", ())?;
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
    pub fn get_utilitiesVersion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Version>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Version> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_utilitiesVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_volumeLevel() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_volumeLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vsyncCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_vsyncCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xrApi(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRManager_XrApi> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRManager_XrApi = __cordl_object
            .invoke("get_xrApi", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xrInstance(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("get_xrInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xrSession(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("get_xrSession", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_AudioInChanged(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_AudioInChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_AudioOutChanged(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_AudioOutChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_DisplayRefreshRateChanged(
        value: quest_hook::libil2cpp::Gc<crate::System::Action_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_DisplayRefreshRateChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_HMDAcquired(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_HMDAcquired", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_HMDLost(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_HMDLost", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_HMDMounted(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_HMDMounted", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_HMDUnmounted(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_HMDUnmounted", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_HSWDismissed(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_HSWDismissed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_InputFocusAcquired(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_InputFocusAcquired", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_InputFocusLost(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_InputFocusLost", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_SceneCaptureComplete(
        value: quest_hook::libil2cpp::Gc<crate::System::Action_2<u64, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_SceneCaptureComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_ShareSpacesComplete(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                u64,
                crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_ShareSpacesComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_SpaceEraseComplete(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                u64,
                bool,
                crate::System::Guid,
                crate::GlobalNamespace::OVRPlugin_SpaceStorageLocation,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_SpaceEraseComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_SpaceListSaveComplete(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                u64,
                crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_SpaceListSaveComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_SpaceQueryComplete(
        value: quest_hook::libil2cpp::Gc<crate::System::Action_2<u64, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_SpaceQueryComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_SpaceQueryResults(
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_SpaceQueryResults", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_SpaceSaveComplete(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                u64,
                crate::GlobalNamespace::OVRSpace,
                bool,
                crate::System::Guid,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_SpaceSaveComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_SpaceSetComponentStatusComplete(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_6<
                u64,
                bool,
                crate::GlobalNamespace::OVRSpace,
                crate::System::Guid,
                crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_SpaceSetComponentStatusComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_SpatialAnchorCreateComplete(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                u64,
                bool,
                crate::GlobalNamespace::OVRSpace,
                crate::System::Guid,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_SpatialAnchorCreateComplete", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_TrackingAcquired(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_TrackingAcquired", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_TrackingLost(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_TrackingLost", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_VrFocusAcquired(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_VrFocusAcquired", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_VrFocusLost(
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remove_VrFocusLost", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_boundary(
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRBoundary>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_boundary", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_chromatic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_chromatic", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_colorGamut(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_ColorSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorGamut", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cpuLevel(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_cpuLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_display(
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRDisplay>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_display", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_eyeFovPremultipliedAlphaModeEnabled(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_eyeFovPremultipliedAlphaModeEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_eyeTextureFormat(
        value: crate::GlobalNamespace::OVRManager_EyeTextureFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_eyeTextureFormat", (value))?;
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
        value: crate::GlobalNamespace::OVRManager_FixedFoveatedRenderingLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_fixedFoveatedRenderingLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_foveatedRenderingLevel(
        value: crate::GlobalNamespace::OVRManager_FoveatedRenderingLevel,
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
    pub fn set_hasVrFocus(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_hasVrFocus", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_headPoseRelativeOffsetRotation(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_headPoseRelativeOffsetRotation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_headPoseRelativeOffsetTranslation(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_headPoseRelativeOffsetTranslation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_instance(
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_instance", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isHmdPresent(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_isHmdPresent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isSupportedPlatform(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isSupportedPlatform", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isUserPresent(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isUserPresent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_monoscopic(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_monoscopic", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_runtimeSettings(
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRRuntimeSettings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_runtimeSettings", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sharpenType(
        &mut self,
        value: crate::GlobalNamespace::OVRPlugin_LayerSharpenType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sharpenType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_suggestedCpuPerfLevel(
        value: crate::GlobalNamespace::OVRManager_ProcessorPerformanceLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_suggestedCpuPerfLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_suggestedGpuPerfLevel(
        value: crate::GlobalNamespace::OVRManager_ProcessorPerformanceLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_suggestedGpuPerfLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_tiledMultiResLevel(
        value: crate::GlobalNamespace::OVRManager_TiledMultiResLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_tiledMultiResLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_tracker(
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRTracker>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_tracker", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_trackingOriginType(
        &mut self,
        value: crate::GlobalNamespace::OVRManager_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trackingOriginType", (value))?;
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
    pub fn set_vsyncCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_vsyncCount", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRManager")]
impl AsRef<crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration>
for crate::GlobalNamespace::OVRManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRManager")]
impl AsMut<crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration>
for crate::GlobalNamespace::OVRManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRManager+CameraDevice")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_CameraDevice {
    WebCamera0 = 0i32,
    WebCamera1 = 1i32,
    ZEDCamera = 2i32,
}
#[cfg(feature = "OVRManager+CameraDevice")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager_CameraDevice => ""
    ."OVRManager/CameraDevice"
);
#[cfg(feature = "OVRManager+ColorSpace")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_ColorSpace {
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
#[cfg(feature = "OVRManager+ColorSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager_ColorSpace => ""
    ."OVRManager/ColorSpace"
);
#[cfg(feature = "OVRManager+CompositionMethod")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_CompositionMethod {
    Direct = 1i32,
    External = 0i32,
}
#[cfg(feature = "OVRManager+CompositionMethod")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager_CompositionMethod =>
    ""."OVRManager/CompositionMethod"
);
#[cfg(feature = "OVRManager+ControllerDrivenHandPosesType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_ControllerDrivenHandPosesType {
    ConformingToController = 1i32,
    Natural = 2i32,
    None = 0i32,
}
#[cfg(feature = "OVRManager+ControllerDrivenHandPosesType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRManager_ControllerDrivenHandPosesType => ""
    ."OVRManager/ControllerDrivenHandPosesType"
);
#[cfg(feature = "OVRManager+DepthQuality")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_DepthQuality {
    High = 2i32,
    Low = 0i32,
    Medium = 1i32,
}
#[cfg(feature = "OVRManager+DepthQuality")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager_DepthQuality => ""
    ."OVRManager/DepthQuality"
);
#[cfg(feature = "OVRManager+EventListener")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRManager_EventListener {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRManager+EventListener")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager_EventListener => ""
    ."OVRManager/EventListener"
);
#[cfg(feature = "OVRManager+EventListener")]
impl std::ops::Deref for crate::GlobalNamespace::OVRManager_EventListener {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRManager+EventListener")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRManager_EventListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRManager+EventListener")]
impl crate::GlobalNamespace::OVRManager_EventListener {
    pub fn OnEvent(
        &mut self,
        eventData: crate::GlobalNamespace::OVRPlugin_EventDataBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEvent", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "OVRManager+EventListener")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRManager_EventListener {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRManager+EyeTextureFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_EyeTextureFormat {
    Default = 0i32,
    R11G11B10_FP = 3i32,
    R16G16B16A16_FP = 2i32,
}
#[cfg(feature = "OVRManager+EyeTextureFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager_EyeTextureFormat =>
    ""."OVRManager/EyeTextureFormat"
);
#[cfg(feature = "OVRManager+FixedFoveatedRenderingLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_FixedFoveatedRenderingLevel {
    High = 3i32,
    HighTop = 4i32,
    Low = 1i32,
    Medium = 2i32,
    Off = 0i32,
}
#[cfg(feature = "OVRManager+FixedFoveatedRenderingLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRManager_FixedFoveatedRenderingLevel => ""
    ."OVRManager/FixedFoveatedRenderingLevel"
);
#[cfg(feature = "OVRManager+FoveatedRenderingLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_FoveatedRenderingLevel {
    High = 3i32,
    HighTop = 4i32,
    Low = 1i32,
    Medium = 2i32,
    Off = 0i32,
}
#[cfg(feature = "OVRManager+FoveatedRenderingLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRManager_FoveatedRenderingLevel => ""
    ."OVRManager/FoveatedRenderingLevel"
);
#[cfg(feature = "OVRManager+InstantiateMrcCameraDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRManager_InstantiateMrcCameraDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVRManager+InstantiateMrcCameraDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate => ""
    ."OVRManager/InstantiateMrcCameraDelegate"
);
#[cfg(feature = "OVRManager+InstantiateMrcCameraDelegate")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRManager+InstantiateMrcCameraDelegate")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRManager+InstantiateMrcCameraDelegate")]
impl crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate {
    pub fn BeginInvoke(
        &mut self,
        mainCameraGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        cameraType: crate::GlobalNamespace::OVRManager_MrcCameraType,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (mainCameraGameObject, cameraType, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        mainCameraGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        cameraType: crate::GlobalNamespace::OVRManager_MrcCameraType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("Invoke", (mainCameraGameObject, cameraType))?;
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
#[cfg(feature = "OVRManager+InstantiateMrcCameraDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRManager_InstantiateMrcCameraDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRManager+MrcActivationMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_MrcActivationMode {
    Automatic = 0i32,
    Disabled = 1i32,
}
#[cfg(feature = "OVRManager+MrcActivationMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager_MrcActivationMode =>
    ""."OVRManager/MrcActivationMode"
);
#[cfg(feature = "OVRManager+MrcCameraType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_MrcCameraType {
    Background = 2i32,
    Foreground = 1i32,
    Normal = 0i32,
}
#[cfg(feature = "OVRManager+MrcCameraType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager_MrcCameraType => ""
    ."OVRManager/MrcCameraType"
);
#[cfg(feature = "OVRManager+PassthroughCapabilities")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRManager_PassthroughCapabilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _SupportsPassthrough_k__BackingField: bool,
    pub _SupportsColorPassthrough_k__BackingField: bool,
    pub _MaxColorLutResolution_k__BackingField: u32,
}
#[cfg(feature = "OVRManager+PassthroughCapabilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRManager_PassthroughCapabilities => ""
    ."OVRManager/PassthroughCapabilities"
);
#[cfg(feature = "OVRManager+PassthroughCapabilities")]
impl std::ops::Deref for crate::GlobalNamespace::OVRManager_PassthroughCapabilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRManager+PassthroughCapabilities")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRManager_PassthroughCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRManager+PassthroughCapabilities")]
impl crate::GlobalNamespace::OVRManager_PassthroughCapabilities {
    pub fn New(
        supportsPassthrough: bool,
        supportsColorPassthrough: bool,
        maxColorLutResolution: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (supportsPassthrough, supportsColorPassthrough, maxColorLutResolution),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        supportsPassthrough: bool,
        supportsColorPassthrough: bool,
        maxColorLutResolution: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (supportsPassthrough, supportsColorPassthrough, maxColorLutResolution),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxColorLutResolution(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_MaxColorLutResolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SupportsColorPassthrough(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_SupportsColorPassthrough", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SupportsPassthrough(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_SupportsPassthrough", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRManager+PassthroughCapabilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRManager_PassthroughCapabilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRManager+PassthroughInitializationState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_PassthroughInitializationState {
    Failed = 3i32,
    Initialized = 2i32,
    Pending = 1i32,
    Unspecified = 0i32,
}
#[cfg(feature = "OVRManager+PassthroughInitializationState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRManager_PassthroughInitializationState => ""
    ."OVRManager/PassthroughInitializationState"
);
#[cfg(feature = "OVRManager+ProcessorPerformanceLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_ProcessorPerformanceLevel {
    Boost = 3i32,
    PowerSavings = 0i32,
    SustainedHigh = 2i32,
    SustainedLow = 1i32,
}
#[cfg(feature = "OVRManager+ProcessorPerformanceLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRManager_ProcessorPerformanceLevel => ""
    ."OVRManager/ProcessorPerformanceLevel"
);
#[cfg(feature = "OVRManager+SystemHeadsetType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_SystemHeadsetType {
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
#[cfg(feature = "OVRManager+SystemHeadsetType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager_SystemHeadsetType =>
    ""."OVRManager/SystemHeadsetType"
);
#[cfg(feature = "OVRManager+TiledMultiResLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_TiledMultiResLevel {
    LMSHigh = 3i32,
    LMSHighTop = 4i32,
    LMSLow = 1i32,
    LMSMedium = 2i32,
    Off = 0i32,
}
#[cfg(feature = "OVRManager+TiledMultiResLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager_TiledMultiResLevel
    => ""."OVRManager/TiledMultiResLevel"
);
#[cfg(feature = "OVRManager+TrackingOrigin")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_TrackingOrigin {
    EyeLevel = 0i32,
    FloorLevel = 1i32,
    Stage = 2i32,
}
#[cfg(feature = "OVRManager+TrackingOrigin")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager_TrackingOrigin => ""
    ."OVRManager/TrackingOrigin"
);
#[cfg(feature = "OVRManager+VirtualGreenScreenType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_VirtualGreenScreenType {
    Off = 0i32,
    OuterBoundary = 1i32,
    PlayArea = 2i32,
}
#[cfg(feature = "OVRManager+VirtualGreenScreenType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRManager_VirtualGreenScreenType => ""
    ."OVRManager/VirtualGreenScreenType"
);
#[cfg(feature = "OVRManager+XRDevice")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_XRDevice {
    Oculus = 1i32,
    OpenVR = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "OVRManager+XRDevice")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager_XRDevice => ""
    ."OVRManager/XRDevice"
);
#[cfg(feature = "OVRManager+XrApi")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRManager_XrApi {
    CAPI = 1i32,
    OpenXR = 3i32,
    Unknown = 0i32,
    VRAPI = 2i32,
}
#[cfg(feature = "OVRManager+XrApi")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRManager_XrApi => ""
    ."OVRManager/XrApi"
);
