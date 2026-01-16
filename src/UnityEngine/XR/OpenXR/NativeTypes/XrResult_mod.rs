#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+NativeTypes+XrResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XrResult {
    #[default]
    ActionTypeMismatch = -27i32,
    ActionsetNotAttached = -46i32,
    ActionsetsAlreadyAttached = -47i32,
    AndroidThreadSettingsFailureKHR = -1000003001i32,
    AndroidThreadSettingsIdInvalidKHR = -1000003000i32,
    ApiLayerNotPresent = -36i32,
    ApiVersionUnsupported = -4i32,
    CallOrderInvalid = -37i32,
    ColorSpaceUnsupportedFB = -1000108000i32,
    ComputeNewSceneNotCompletedMSFT = -1000097000i32,
    ControllerModelKeyInvalidMSFT = -1000055000i32,
    CreateSpatialAnchorFailedMSFT = -1000039001i32,
    DisplayRefreshRateUnsupportedFB = -1000101000i32,
    EnvironmentBlendModeUnsupported = -42i32,
    EnvironmentDepthNotAvailableMETA = 1000291000i32,
    EventUnavailable = 4i32,
    ExtensionDependencyNotEnabled = -1000710001i32,
    ExtensionNotPresent = -9i32,
    FeatureAlreadyCreatedPassthroughFB = -1000118001i32,
    FeatureRequiredPassthroughFB = -1000118002i32,
    FeatureUnsupported = -8i32,
    FileAccessError = -32i32,
    FileContentsInvalid = -33i32,
    FormFactorUnavailable = -35i32,
    FormFactorUnsupported = -34i32,
    FrameDiscarded = 9i32,
    FunctionUnsupported = -7i32,
    FutureInvalidEXT = -1000469002i32,
    FuturePendingEXT = -1000469001i32,
    GraphicsDeviceInvalid = -38i32,
    GraphicsRequirementsCallMissing = -50i32,
    HandleInvalid = -12i32,
    HintAlreadySetQCOM = -1000306000i32,
    IndexOutOfRange = -40i32,
    InitializationFailed = -6i32,
    InstanceLost = -13i32,
    InsufficientResourcesPassthroughFB = -1000118004i32,
    LayerInvalid = -23i32,
    LayerLimitExceeded = -24i32,
    LimitReached = -10i32,
    LocalizationMapAlreadyExistsML = -1000139005i32,
    LocalizationMapCannotExportCloudMapML = -1000139006i32,
    LocalizationMapFailML = -1000139002i32,
    LocalizationMapImportExportPermissionDeniedML = -1000139003i32,
    LocalizationMapIncompatibleML = -1000139000i32,
    LocalizationMapPermissionDeniedML = -1000139004i32,
    LocalizationMapUnavailableML = -1000139001i32,
    LocalizedNameDuplicated = -48i32,
    LocalizedNameInvalid = -49i32,
    LossPending = 3i32,
    MarkerDetectorInvalidCreateInfoML = -1000138003i32,
    MarkerDetectorInvalidDataQueryML = -1000138002i32,
    MarkerDetectorLocateFailedML = -1000138001i32,
    MarkerDetectorPermissionDeniedML = -1000138000i32,
    MarkerIdInvalidVARJO = -1000124001i32,
    MarkerInvalidML = -1000138004i32,
    MarkerNotTrackedVARJO = -1000124000i32,
    MaxResult = 2147483647i32,
    NameDuplicated = -44i32,
    NameInvalid = -45i32,
    NotAnAnchorHTC = -1000319000i32,
    NotPermittedPassthroughFB = -1000118003i32,
    OutOfMemory = -3i32,
    PassthroughColorLutBufferSizeMismatchMETA = -1000266000i32,
    PathCountExceeded = -20i32,
    PathFormatInvalid = -21i32,
    PathInvalid = -19i32,
    PathUnsupported = -22i32,
    PermissionInsufficient = -1000710000i32,
    PlaneDetectionPermissionDeniedEXT = -1000429001i32,
    PoseInvalid = -39i32,
    ReferenceSpaceUnsupported = -31i32,
    RenderModelKeyInvalidFB = -1000119000i32,
    RenderModelUnavailableFB = 1000119020i32,
    ReprojectionModeUnsupportedMSFT = -1000066000i32,
    RuntimeFailure = -2i32,
    RuntimeUnavailable = -51i32,
    SceneComponentIdInvalidMSFT = -1000097001i32,
    SceneComponentTypeMismatchMSFT = -1000097002i32,
    SceneComputeConsistencyMismatchMSFT = -1000097005i32,
    SceneComputeFeatureIncompatibleMSFT = -1000097004i32,
    SceneMarkerDataNotStringMSFT = 1000147000i32,
    SceneMeshBufferIdInvalidMSFT = -1000097003i32,
    SecondaryViewConfigurationTypeNotEnabledMSFT = -1000053000i32,
    SessionLost = -17i32,
    SessionNotFocused = 8i32,
    SessionNotReady = -28i32,
    SessionNotRunning = -16i32,
    SessionNotStopping = -29i32,
    SessionRunning = -14i32,
    SizeInsufficient = -11i32,
    SpaceBoundsUnavailable = 7i32,
    SpaceCloudStorageDisabledFB = -1000169004i32,
    SpaceComponentNotEnabledFB = -1000113001i32,
    SpaceComponentNotSupportedFB = -1000113000i32,
    SpaceComponentStatusAlreadySetFB = -1000113003i32,
    SpaceComponentStatusPendingFB = -1000113002i32,
    SpaceLocalizationFailedFB = -1000169001i32,
    SpaceMappingInsufficientFB = -1000169000i32,
    SpaceNetworkRequestFailedFB = -1000169003i32,
    SpaceNetworkTimeoutFB = -1000169002i32,
    SpaceNotLocatableEXT = -1000429000i32,
    SpatialAnchorNameInvalidMSFT = -1000142002i32,
    SpatialAnchorNameNotFoundMSFT = -1000142001i32,
    Success = 0i32,
    SwapchainFormatUnsupported = -26i32,
    SwapchainRectInvalid = -25i32,
    SystemInvalid = -18i32,
    TimeInvalid = -30i32,
    TimeoutExpired = 1i32,
    UnexpectedStatePassthroughFB = -1000118000i32,
    UnknownPassthroughFB = -1000118050i32,
    ValidationFailure = -1i32,
    ViewConfigurationTypeUnsupported = -41i32,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+NativeTypes+XrResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::OpenXR::NativeTypes::XrResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.NativeTypes";
    const CLASS_NAME: &'static str = "XrResult";
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+NativeTypes+XrResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::XR::OpenXR::NativeTypes::XrResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+NativeTypes+XrResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::XR::OpenXR::NativeTypes::XrResult {
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+NativeTypes+XrResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::XR::OpenXR::NativeTypes::XrResult {
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+NativeTypes+XrResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::XR::OpenXR::NativeTypes::XrResult {
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
