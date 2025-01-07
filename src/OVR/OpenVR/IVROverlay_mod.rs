#[cfg(feature = "OVR+OpenVR+IVROverlay")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IVROverlay {
    pub FindOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__FindOverlay,
    >,
    pub CreateOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__CreateOverlay,
    >,
    pub DestroyOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__DestroyOverlay,
    >,
    pub SetHighQualityOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetHighQualityOverlay,
    >,
    pub GetHighQualityOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetHighQualityOverlay,
    >,
    pub GetOverlayKey: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayKey,
    >,
    pub GetOverlayName: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayName,
    >,
    pub SetOverlayName: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayName,
    >,
    pub GetOverlayImageData: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayImageData,
    >,
    pub GetOverlayErrorNameFromEnum: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayErrorNameFromEnum,
    >,
    pub SetOverlayRenderingPid: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayRenderingPid,
    >,
    pub GetOverlayRenderingPid: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayRenderingPid,
    >,
    pub SetOverlayFlag: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayFlag,
    >,
    pub GetOverlayFlag: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayFlag,
    >,
    pub SetOverlayColor: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayColor,
    >,
    pub GetOverlayColor: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayColor,
    >,
    pub SetOverlayAlpha: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayAlpha,
    >,
    pub GetOverlayAlpha: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayAlpha,
    >,
    pub SetOverlayTexelAspect: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayTexelAspect,
    >,
    pub GetOverlayTexelAspect: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayTexelAspect,
    >,
    pub SetOverlaySortOrder: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlaySortOrder,
    >,
    pub GetOverlaySortOrder: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlaySortOrder,
    >,
    pub SetOverlayWidthInMeters: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayWidthInMeters,
    >,
    pub GetOverlayWidthInMeters: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayWidthInMeters,
    >,
    pub SetOverlayAutoCurveDistanceRangeInMeters: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayAutoCurveDistanceRangeInMeters,
    >,
    pub GetOverlayAutoCurveDistanceRangeInMeters: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayAutoCurveDistanceRangeInMeters,
    >,
    pub SetOverlayTextureColorSpace: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayTextureColorSpace,
    >,
    pub GetOverlayTextureColorSpace: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayTextureColorSpace,
    >,
    pub SetOverlayTextureBounds: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayTextureBounds,
    >,
    pub GetOverlayTextureBounds: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayTextureBounds,
    >,
    pub GetOverlayRenderModel: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayRenderModel,
    >,
    pub SetOverlayRenderModel: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayRenderModel,
    >,
    pub GetOverlayTransformType: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayTransformType,
    >,
    pub SetOverlayTransformAbsolute: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayTransformAbsolute,
    >,
    pub GetOverlayTransformAbsolute: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayTransformAbsolute,
    >,
    pub SetOverlayTransformTrackedDeviceRelative: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceRelative,
    >,
    pub GetOverlayTransformTrackedDeviceRelative: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceRelative,
    >,
    pub SetOverlayTransformTrackedDeviceComponent: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceComponent,
    >,
    pub GetOverlayTransformTrackedDeviceComponent: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceComponent,
    >,
    pub GetOverlayTransformOverlayRelative: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayTransformOverlayRelative,
    >,
    pub SetOverlayTransformOverlayRelative: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayTransformOverlayRelative,
    >,
    pub ShowOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__ShowOverlay,
    >,
    pub HideOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__HideOverlay,
    >,
    pub IsOverlayVisible: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__IsOverlayVisible,
    >,
    pub GetTransformForOverlayCoordinates: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetTransformForOverlayCoordinates,
    >,
    pub PollNextOverlayEvent: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__PollNextOverlayEvent,
    >,
    pub GetOverlayInputMethod: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayInputMethod,
    >,
    pub SetOverlayInputMethod: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayInputMethod,
    >,
    pub GetOverlayMouseScale: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayMouseScale,
    >,
    pub SetOverlayMouseScale: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayMouseScale,
    >,
    pub ComputeOverlayIntersection: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__ComputeOverlayIntersection,
    >,
    pub IsHoverTargetOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__IsHoverTargetOverlay,
    >,
    pub GetGamepadFocusOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetGamepadFocusOverlay,
    >,
    pub SetGamepadFocusOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetGamepadFocusOverlay,
    >,
    pub SetOverlayNeighbor: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayNeighbor,
    >,
    pub MoveGamepadFocusToNeighbor: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__MoveGamepadFocusToNeighbor,
    >,
    pub SetOverlayDualAnalogTransform: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayDualAnalogTransform,
    >,
    pub GetOverlayDualAnalogTransform: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayDualAnalogTransform,
    >,
    pub SetOverlayTexture: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayTexture,
    >,
    pub ClearOverlayTexture: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__ClearOverlayTexture,
    >,
    pub SetOverlayRaw: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayRaw,
    >,
    pub SetOverlayFromFile: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayFromFile,
    >,
    pub GetOverlayTexture: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayTexture,
    >,
    pub ReleaseNativeOverlayHandle: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__ReleaseNativeOverlayHandle,
    >,
    pub GetOverlayTextureSize: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayTextureSize,
    >,
    pub CreateDashboardOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__CreateDashboardOverlay,
    >,
    pub IsDashboardVisible: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__IsDashboardVisible,
    >,
    pub IsActiveDashboardOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__IsActiveDashboardOverlay,
    >,
    pub SetDashboardOverlaySceneProcess: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetDashboardOverlaySceneProcess,
    >,
    pub GetDashboardOverlaySceneProcess: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetDashboardOverlaySceneProcess,
    >,
    pub ShowDashboard: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__ShowDashboard,
    >,
    pub GetPrimaryDashboardDevice: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetPrimaryDashboardDevice,
    >,
    pub ShowKeyboard: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__ShowKeyboard,
    >,
    pub ShowKeyboardForOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__ShowKeyboardForOverlay,
    >,
    pub GetKeyboardText: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetKeyboardText,
    >,
    pub HideKeyboard: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__HideKeyboard,
    >,
    pub SetKeyboardTransformAbsolute: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetKeyboardTransformAbsolute,
    >,
    pub SetKeyboardPositionForOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetKeyboardPositionForOverlay,
    >,
    pub SetOverlayIntersectionMask: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__SetOverlayIntersectionMask,
    >,
    pub GetOverlayFlags: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__GetOverlayFlags,
    >,
    pub ShowMessageOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__ShowMessageOverlay,
    >,
    pub CloseMessageOverlay: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVROverlay__CloseMessageOverlay,
    >,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVROverlay {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVROverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::IVROverlay {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::IVROverlay {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::IVROverlay {
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
#[cfg(feature = "OVR+OpenVR+IVROverlay")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::IVROverlay {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::IVROverlay {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay")]
impl crate::OVR::OpenVR::IVROverlay {
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_ClearOverlayTexture")]
    pub type _ClearOverlayTexture = crate::OVR::OpenVR::IVROverlay__ClearOverlayTexture;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_CloseMessageOverlay")]
    pub type _CloseMessageOverlay = crate::OVR::OpenVR::IVROverlay__CloseMessageOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_ComputeOverlayIntersection")]
    pub type _ComputeOverlayIntersection = crate::OVR::OpenVR::IVROverlay__ComputeOverlayIntersection;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateDashboardOverlay")]
    pub type _CreateDashboardOverlay = crate::OVR::OpenVR::IVROverlay__CreateDashboardOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateOverlay")]
    pub type _CreateOverlay = crate::OVR::OpenVR::IVROverlay__CreateOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_DestroyOverlay")]
    pub type _DestroyOverlay = crate::OVR::OpenVR::IVROverlay__DestroyOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_FindOverlay")]
    pub type _FindOverlay = crate::OVR::OpenVR::IVROverlay__FindOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetDashboardOverlaySceneProcess")]
    pub type _GetDashboardOverlaySceneProcess = crate::OVR::OpenVR::IVROverlay__GetDashboardOverlaySceneProcess;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetGamepadFocusOverlay")]
    pub type _GetGamepadFocusOverlay = crate::OVR::OpenVR::IVROverlay__GetGamepadFocusOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetHighQualityOverlay")]
    pub type _GetHighQualityOverlay = crate::OVR::OpenVR::IVROverlay__GetHighQualityOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetKeyboardText")]
    pub type _GetKeyboardText = crate::OVR::OpenVR::IVROverlay__GetKeyboardText;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAlpha")]
    pub type _GetOverlayAlpha = crate::OVR::OpenVR::IVROverlay__GetOverlayAlpha;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAutoCurveDistanceRangeInMeters")]
    pub type _GetOverlayAutoCurveDistanceRangeInMeters = crate::OVR::OpenVR::IVROverlay__GetOverlayAutoCurveDistanceRangeInMeters;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayColor")]
    pub type _GetOverlayColor = crate::OVR::OpenVR::IVROverlay__GetOverlayColor;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayDualAnalogTransform")]
    pub type _GetOverlayDualAnalogTransform = crate::OVR::OpenVR::IVROverlay__GetOverlayDualAnalogTransform;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayErrorNameFromEnum")]
    pub type _GetOverlayErrorNameFromEnum = crate::OVR::OpenVR::IVROverlay__GetOverlayErrorNameFromEnum;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlag")]
    pub type _GetOverlayFlag = crate::OVR::OpenVR::IVROverlay__GetOverlayFlag;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlags")]
    pub type _GetOverlayFlags = crate::OVR::OpenVR::IVROverlay__GetOverlayFlags;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayImageData")]
    pub type _GetOverlayImageData = crate::OVR::OpenVR::IVROverlay__GetOverlayImageData;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayInputMethod")]
    pub type _GetOverlayInputMethod = crate::OVR::OpenVR::IVROverlay__GetOverlayInputMethod;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayKey")]
    pub type _GetOverlayKey = crate::OVR::OpenVR::IVROverlay__GetOverlayKey;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayMouseScale")]
    pub type _GetOverlayMouseScale = crate::OVR::OpenVR::IVROverlay__GetOverlayMouseScale;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayName")]
    pub type _GetOverlayName = crate::OVR::OpenVR::IVROverlay__GetOverlayName;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderModel")]
    pub type _GetOverlayRenderModel = crate::OVR::OpenVR::IVROverlay__GetOverlayRenderModel;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderingPid")]
    pub type _GetOverlayRenderingPid = crate::OVR::OpenVR::IVROverlay__GetOverlayRenderingPid;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlaySortOrder")]
    pub type _GetOverlaySortOrder = crate::OVR::OpenVR::IVROverlay__GetOverlaySortOrder;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexelAspect")]
    pub type _GetOverlayTexelAspect = crate::OVR::OpenVR::IVROverlay__GetOverlayTexelAspect;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexture")]
    pub type _GetOverlayTexture = crate::OVR::OpenVR::IVROverlay__GetOverlayTexture;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureBounds")]
    pub type _GetOverlayTextureBounds = crate::OVR::OpenVR::IVROverlay__GetOverlayTextureBounds;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureColorSpace")]
    pub type _GetOverlayTextureColorSpace = crate::OVR::OpenVR::IVROverlay__GetOverlayTextureColorSpace;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureSize")]
    pub type _GetOverlayTextureSize = crate::OVR::OpenVR::IVROverlay__GetOverlayTextureSize;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformAbsolute")]
    pub type _GetOverlayTransformAbsolute = crate::OVR::OpenVR::IVROverlay__GetOverlayTransformAbsolute;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformOverlayRelative")]
    pub type _GetOverlayTransformOverlayRelative = crate::OVR::OpenVR::IVROverlay__GetOverlayTransformOverlayRelative;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceComponent")]
    pub type _GetOverlayTransformTrackedDeviceComponent = crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceComponent;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceRelative")]
    pub type _GetOverlayTransformTrackedDeviceRelative = crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceRelative;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformType")]
    pub type _GetOverlayTransformType = crate::OVR::OpenVR::IVROverlay__GetOverlayTransformType;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayWidthInMeters")]
    pub type _GetOverlayWidthInMeters = crate::OVR::OpenVR::IVROverlay__GetOverlayWidthInMeters;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetPrimaryDashboardDevice")]
    pub type _GetPrimaryDashboardDevice = crate::OVR::OpenVR::IVROverlay__GetPrimaryDashboardDevice;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_GetTransformForOverlayCoordinates")]
    pub type _GetTransformForOverlayCoordinates = crate::OVR::OpenVR::IVROverlay__GetTransformForOverlayCoordinates;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_HideKeyboard")]
    pub type _HideKeyboard = crate::OVR::OpenVR::IVROverlay__HideKeyboard;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_HideOverlay")]
    pub type _HideOverlay = crate::OVR::OpenVR::IVROverlay__HideOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_IsActiveDashboardOverlay")]
    pub type _IsActiveDashboardOverlay = crate::OVR::OpenVR::IVROverlay__IsActiveDashboardOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_IsDashboardVisible")]
    pub type _IsDashboardVisible = crate::OVR::OpenVR::IVROverlay__IsDashboardVisible;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_IsHoverTargetOverlay")]
    pub type _IsHoverTargetOverlay = crate::OVR::OpenVR::IVROverlay__IsHoverTargetOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_IsOverlayVisible")]
    pub type _IsOverlayVisible = crate::OVR::OpenVR::IVROverlay__IsOverlayVisible;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_MoveGamepadFocusToNeighbor")]
    pub type _MoveGamepadFocusToNeighbor = crate::OVR::OpenVR::IVROverlay__MoveGamepadFocusToNeighbor;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_PollNextOverlayEvent")]
    pub type _PollNextOverlayEvent = crate::OVR::OpenVR::IVROverlay__PollNextOverlayEvent;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_ReleaseNativeOverlayHandle")]
    pub type _ReleaseNativeOverlayHandle = crate::OVR::OpenVR::IVROverlay__ReleaseNativeOverlayHandle;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetDashboardOverlaySceneProcess")]
    pub type _SetDashboardOverlaySceneProcess = crate::OVR::OpenVR::IVROverlay__SetDashboardOverlaySceneProcess;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetGamepadFocusOverlay")]
    pub type _SetGamepadFocusOverlay = crate::OVR::OpenVR::IVROverlay__SetGamepadFocusOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetHighQualityOverlay")]
    pub type _SetHighQualityOverlay = crate::OVR::OpenVR::IVROverlay__SetHighQualityOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardPositionForOverlay")]
    pub type _SetKeyboardPositionForOverlay = crate::OVR::OpenVR::IVROverlay__SetKeyboardPositionForOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardTransformAbsolute")]
    pub type _SetKeyboardTransformAbsolute = crate::OVR::OpenVR::IVROverlay__SetKeyboardTransformAbsolute;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAlpha")]
    pub type _SetOverlayAlpha = crate::OVR::OpenVR::IVROverlay__SetOverlayAlpha;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAutoCurveDistanceRangeInMeters")]
    pub type _SetOverlayAutoCurveDistanceRangeInMeters = crate::OVR::OpenVR::IVROverlay__SetOverlayAutoCurveDistanceRangeInMeters;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayColor")]
    pub type _SetOverlayColor = crate::OVR::OpenVR::IVROverlay__SetOverlayColor;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayDualAnalogTransform")]
    pub type _SetOverlayDualAnalogTransform = crate::OVR::OpenVR::IVROverlay__SetOverlayDualAnalogTransform;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFlag")]
    pub type _SetOverlayFlag = crate::OVR::OpenVR::IVROverlay__SetOverlayFlag;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFromFile")]
    pub type _SetOverlayFromFile = crate::OVR::OpenVR::IVROverlay__SetOverlayFromFile;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayInputMethod")]
    pub type _SetOverlayInputMethod = crate::OVR::OpenVR::IVROverlay__SetOverlayInputMethod;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayIntersectionMask")]
    pub type _SetOverlayIntersectionMask = crate::OVR::OpenVR::IVROverlay__SetOverlayIntersectionMask;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayMouseScale")]
    pub type _SetOverlayMouseScale = crate::OVR::OpenVR::IVROverlay__SetOverlayMouseScale;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayName")]
    pub type _SetOverlayName = crate::OVR::OpenVR::IVROverlay__SetOverlayName;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayNeighbor")]
    pub type _SetOverlayNeighbor = crate::OVR::OpenVR::IVROverlay__SetOverlayNeighbor;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRaw")]
    pub type _SetOverlayRaw = crate::OVR::OpenVR::IVROverlay__SetOverlayRaw;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderModel")]
    pub type _SetOverlayRenderModel = crate::OVR::OpenVR::IVROverlay__SetOverlayRenderModel;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderingPid")]
    pub type _SetOverlayRenderingPid = crate::OVR::OpenVR::IVROverlay__SetOverlayRenderingPid;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlaySortOrder")]
    pub type _SetOverlaySortOrder = crate::OVR::OpenVR::IVROverlay__SetOverlaySortOrder;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexelAspect")]
    pub type _SetOverlayTexelAspect = crate::OVR::OpenVR::IVROverlay__SetOverlayTexelAspect;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexture")]
    pub type _SetOverlayTexture = crate::OVR::OpenVR::IVROverlay__SetOverlayTexture;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureBounds")]
    pub type _SetOverlayTextureBounds = crate::OVR::OpenVR::IVROverlay__SetOverlayTextureBounds;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureColorSpace")]
    pub type _SetOverlayTextureColorSpace = crate::OVR::OpenVR::IVROverlay__SetOverlayTextureColorSpace;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformAbsolute")]
    pub type _SetOverlayTransformAbsolute = crate::OVR::OpenVR::IVROverlay__SetOverlayTransformAbsolute;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformOverlayRelative")]
    pub type _SetOverlayTransformOverlayRelative = crate::OVR::OpenVR::IVROverlay__SetOverlayTransformOverlayRelative;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceComponent")]
    pub type _SetOverlayTransformTrackedDeviceComponent = crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceComponent;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceRelative")]
    pub type _SetOverlayTransformTrackedDeviceRelative = crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceRelative;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayWidthInMeters")]
    pub type _SetOverlayWidthInMeters = crate::OVR::OpenVR::IVROverlay__SetOverlayWidthInMeters;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowDashboard")]
    pub type _ShowDashboard = crate::OVR::OpenVR::IVROverlay__ShowDashboard;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboard")]
    pub type _ShowKeyboard = crate::OVR::OpenVR::IVROverlay__ShowKeyboard;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboardForOverlay")]
    pub type _ShowKeyboardForOverlay = crate::OVR::OpenVR::IVROverlay__ShowKeyboardForOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowMessageOverlay")]
    pub type _ShowMessageOverlay = crate::OVR::OpenVR::IVROverlay__ShowMessageOverlay;
    #[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowOverlay")]
    pub type _ShowOverlay = crate::OVR::OpenVR::IVROverlay__ShowOverlay;
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ClearOverlayTexture")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__ClearOverlayTexture {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ClearOverlayTexture")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__ClearOverlayTexture {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_ClearOverlayTexture";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ClearOverlayTexture")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__ClearOverlayTexture {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ClearOverlayTexture")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__ClearOverlayTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ClearOverlayTexture")]
impl crate::OVR::OpenVR::IVROverlay__ClearOverlayTexture {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ClearOverlayTexture")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__ClearOverlayTexture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CloseMessageOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__CloseMessageOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CloseMessageOverlay")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__CloseMessageOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_CloseMessageOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CloseMessageOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__CloseMessageOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CloseMessageOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__CloseMessageOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CloseMessageOverlay")]
impl crate::OVR::OpenVR::IVROverlay__CloseMessageOverlay {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CloseMessageOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__CloseMessageOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ComputeOverlayIntersection")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__ComputeOverlayIntersection {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ComputeOverlayIntersection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__ComputeOverlayIntersection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_ComputeOverlayIntersection";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ComputeOverlayIntersection")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__ComputeOverlayIntersection {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ComputeOverlayIntersection")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__ComputeOverlayIntersection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ComputeOverlayIntersection")]
impl crate::OVR::OpenVR::IVROverlay__ComputeOverlayIntersection {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pParams: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayIntersectionParams_t,
        >,
        pResults: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayIntersectionResults_t,
        >,
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
                (ulOverlayHandle, pParams, pResults, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pParams: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayIntersectionParams_t,
        >,
        pResults: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayIntersectionResults_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EndInvoke", (pParams, pResults, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pParams: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayIntersectionParams_t,
        >,
        pResults: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayIntersectionResults_t,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pParams, pResults))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ComputeOverlayIntersection")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__ComputeOverlayIntersection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateDashboardOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__CreateDashboardOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateDashboardOverlay")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__CreateDashboardOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_CreateDashboardOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateDashboardOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__CreateDashboardOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateDashboardOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__CreateDashboardOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateDashboardOverlay")]
impl crate::OVR::OpenVR::IVROverlay__CreateDashboardOverlay {
    pub fn BeginInvoke(
        &mut self,
        pchOverlayKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchOverlayFriendlyName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pMainHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        pThumbnailHandle: quest_hook::libil2cpp::ByRefMut<u64>,
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
                (
                    pchOverlayKey,
                    pchOverlayFriendlyName,
                    pMainHandle,
                    pThumbnailHandle,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pMainHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        pThumbnailHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pMainHandle, pThumbnailHandle, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchOverlayKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchOverlayFriendlyName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pMainHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        pThumbnailHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (pchOverlayKey, pchOverlayFriendlyName, pMainHandle, pThumbnailHandle),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateDashboardOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__CreateDashboardOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__CreateOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateOverlay")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__CreateOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_CreateOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__CreateOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__CreateOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateOverlay")]
impl crate::OVR::OpenVR::IVROverlay__CreateOverlay {
    pub fn BeginInvoke(
        &mut self,
        pchOverlayKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchOverlayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pOverlayHandle: quest_hook::libil2cpp::ByRefMut<u64>,
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
                (pchOverlayKey, pchOverlayName, pOverlayHandle, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pOverlayHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pOverlayHandle, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchOverlayKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchOverlayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pOverlayHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (pchOverlayKey, pchOverlayName, pOverlayHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_CreateOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__CreateOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_DestroyOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__DestroyOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_DestroyOverlay")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__DestroyOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_DestroyOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_DestroyOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__DestroyOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_DestroyOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__DestroyOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_DestroyOverlay")]
impl crate::OVR::OpenVR::IVROverlay__DestroyOverlay {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_DestroyOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__DestroyOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_FindOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__FindOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_FindOverlay")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVROverlay__FindOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_FindOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_FindOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__FindOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_FindOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__FindOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_FindOverlay")]
impl crate::OVR::OpenVR::IVROverlay__FindOverlay {
    pub fn BeginInvoke(
        &mut self,
        pchOverlayKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pOverlayHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchOverlayKey, pOverlayHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pOverlayHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pOverlayHandle, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchOverlayKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pOverlayHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (pchOverlayKey, pOverlayHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_FindOverlay")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVROverlay__FindOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetDashboardOverlaySceneProcess")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetDashboardOverlaySceneProcess {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetDashboardOverlaySceneProcess")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetDashboardOverlaySceneProcess {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetDashboardOverlaySceneProcess";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetDashboardOverlaySceneProcess")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVROverlay__GetDashboardOverlaySceneProcess {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetDashboardOverlaySceneProcess")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__GetDashboardOverlaySceneProcess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetDashboardOverlaySceneProcess")]
impl crate::OVR::OpenVR::IVROverlay__GetDashboardOverlaySceneProcess {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        punProcessId: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, punProcessId, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        punProcessId: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (punProcessId, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        punProcessId: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, punProcessId))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetDashboardOverlaySceneProcess")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetDashboardOverlaySceneProcess {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetGamepadFocusOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetGamepadFocusOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetGamepadFocusOverlay")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetGamepadFocusOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetGamepadFocusOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetGamepadFocusOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetGamepadFocusOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetGamepadFocusOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetGamepadFocusOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetGamepadFocusOverlay")]
impl crate::OVR::OpenVR::IVROverlay__GetGamepadFocusOverlay {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("Invoke", ())?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetGamepadFocusOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetGamepadFocusOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetHighQualityOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetHighQualityOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetHighQualityOverlay")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetHighQualityOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetHighQualityOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetHighQualityOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetHighQualityOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetHighQualityOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetHighQualityOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetHighQualityOverlay")]
impl crate::OVR::OpenVR::IVROverlay__GetHighQualityOverlay {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("Invoke", ())?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetHighQualityOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetHighQualityOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetKeyboardText")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetKeyboardText {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetKeyboardText")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetKeyboardText {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetKeyboardText";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetKeyboardText")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetKeyboardText {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetKeyboardText")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetKeyboardText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetKeyboardText")]
impl crate::OVR::OpenVR::IVROverlay__GetKeyboardText {
    pub fn BeginInvoke(
        &mut self,
        pchText: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        cchText: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchText, cchText, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchText: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        cchText: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("Invoke", (pchText, cchText))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetKeyboardText")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetKeyboardText {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAlpha")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayAlpha {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAlpha")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayAlpha {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayAlpha";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAlpha")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayAlpha {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAlpha")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayAlpha {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAlpha")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayAlpha {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pfAlpha: quest_hook::libil2cpp::ByRefMut<f32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, pfAlpha, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pfAlpha: quest_hook::libil2cpp::ByRefMut<f32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pfAlpha, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pfAlpha: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pfAlpha))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAlpha")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayAlpha {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAutoCurveDistanceRangeInMeters")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayAutoCurveDistanceRangeInMeters {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAutoCurveDistanceRangeInMeters")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayAutoCurveDistanceRangeInMeters {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayAutoCurveDistanceRangeInMeters";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAutoCurveDistanceRangeInMeters")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVROverlay__GetOverlayAutoCurveDistanceRangeInMeters {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAutoCurveDistanceRangeInMeters")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__GetOverlayAutoCurveDistanceRangeInMeters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAutoCurveDistanceRangeInMeters")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayAutoCurveDistanceRangeInMeters {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pfMinDistanceInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
        pfMaxDistanceInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
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
                (
                    ulOverlayHandle,
                    pfMinDistanceInMeters,
                    pfMaxDistanceInMeters,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pfMinDistanceInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
        pfMaxDistanceInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "EndInvoke",
                (pfMinDistanceInMeters, pfMaxDistanceInMeters, result),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pfMinDistanceInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
        pfMaxDistanceInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (ulOverlayHandle, pfMinDistanceInMeters, pfMaxDistanceInMeters),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayAutoCurveDistanceRangeInMeters")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayAutoCurveDistanceRangeInMeters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayColor")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayColor {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayColor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayColor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayColor";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayColor")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayColor {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayColor")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayColor")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayColor {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pfRed: quest_hook::libil2cpp::ByRefMut<f32>,
        pfGreen: quest_hook::libil2cpp::ByRefMut<f32>,
        pfBlue: quest_hook::libil2cpp::ByRefMut<f32>,
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
                (ulOverlayHandle, pfRed, pfGreen, pfBlue, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pfRed: quest_hook::libil2cpp::ByRefMut<f32>,
        pfGreen: quest_hook::libil2cpp::ByRefMut<f32>,
        pfBlue: quest_hook::libil2cpp::ByRefMut<f32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pfRed, pfGreen, pfBlue, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pfRed: quest_hook::libil2cpp::ByRefMut<f32>,
        pfGreen: quest_hook::libil2cpp::ByRefMut<f32>,
        pfBlue: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pfRed, pfGreen, pfBlue))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayColor")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayColor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayDualAnalogTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayDualAnalogTransform {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayDualAnalogTransform")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayDualAnalogTransform {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayDualAnalogTransform";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayDualAnalogTransform")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayDualAnalogTransform {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayDualAnalogTransform")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__GetOverlayDualAnalogTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayDualAnalogTransform")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayDualAnalogTransform {
    pub fn BeginInvoke(
        &mut self,
        ulOverlay: u64,
        eWhich: crate::OVR::OpenVR::EDualAnalogWhich,
        pvCenter: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
        pfRadius: quest_hook::libil2cpp::ByRefMut<f32>,
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
                (ulOverlay, eWhich, pvCenter, pfRadius, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pvCenter: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
        pfRadius: quest_hook::libil2cpp::ByRefMut<f32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pvCenter, pfRadius, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlay: u64,
        eWhich: crate::OVR::OpenVR::EDualAnalogWhich,
        pvCenter: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
        pfRadius: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlay, eWhich, pvCenter, pfRadius))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayDualAnalogTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayDualAnalogTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayErrorNameFromEnum")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayErrorNameFromEnum {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayErrorNameFromEnum")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayErrorNameFromEnum {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayErrorNameFromEnum";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayErrorNameFromEnum")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayErrorNameFromEnum {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayErrorNameFromEnum")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayErrorNameFromEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayErrorNameFromEnum")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayErrorNameFromEnum {
    pub fn BeginInvoke(
        &mut self,
        error: crate::OVR::OpenVR::EVROverlayError,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (error, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        error: crate::OVR::OpenVR::EVROverlayError,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (error))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayErrorNameFromEnum")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayErrorNameFromEnum {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlag")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayFlag {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlag")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayFlag {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayFlag";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlag")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayFlag {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlag")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayFlag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlag")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayFlag {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        eOverlayFlag: crate::OVR::OpenVR::VROverlayFlags,
        pbEnabled: quest_hook::libil2cpp::ByRefMut<bool>,
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
                (ulOverlayHandle, eOverlayFlag, pbEnabled, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pbEnabled: quest_hook::libil2cpp::ByRefMut<bool>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pbEnabled, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        eOverlayFlag: crate::OVR::OpenVR::VROverlayFlags,
        pbEnabled: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, eOverlayFlag, pbEnabled))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlag")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayFlag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlags")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayFlags {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlags")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayFlags {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayFlags";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlags")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayFlags {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlags")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayFlags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlags")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayFlags {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pFlags: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, pFlags, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pFlags: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pFlags, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pFlags: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pFlags))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayFlags")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayFlags {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayImageData")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayImageData {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayImageData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayImageData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayImageData";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayImageData")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayImageData {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayImageData")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayImageData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayImageData")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayImageData {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pvBuffer: crate::System::IntPtr,
        unBufferSize: u32,
        punWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        punHeight: quest_hook::libil2cpp::ByRefMut<u32>,
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
                (
                    ulOverlayHandle,
                    pvBuffer,
                    unBufferSize,
                    punWidth,
                    punHeight,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        punWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        punHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (punWidth, punHeight, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pvBuffer: crate::System::IntPtr,
        unBufferSize: u32,
        punWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        punHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (ulOverlayHandle, pvBuffer, unBufferSize, punWidth, punHeight),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayImageData")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayImageData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayInputMethod")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayInputMethod {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayInputMethod")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayInputMethod {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayInputMethod";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayInputMethod")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayInputMethod {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayInputMethod")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayInputMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayInputMethod")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayInputMethod {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        peInputMethod: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayInputMethod,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, peInputMethod, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peInputMethod: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayInputMethod,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (peInputMethod, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        peInputMethod: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayInputMethod,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, peInputMethod))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayInputMethod")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayInputMethod {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayKey")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayKey {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayKey")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayKey {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayKey";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayKey")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayKey {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayKey")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayKey")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayKey {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferSize: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
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
                (ulOverlayHandle, pchValue, unBufferSize, pError, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (pError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferSize: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pchValue, unBufferSize, pError))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayMouseScale")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayMouseScale {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayMouseScale")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayMouseScale {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayMouseScale";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayMouseScale")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayMouseScale {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayMouseScale")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayMouseScale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayMouseScale")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayMouseScale {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pvecMouseScale: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdVector2_t,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, pvecMouseScale, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pvecMouseScale: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdVector2_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pvecMouseScale, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pvecMouseScale: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pvecMouseScale))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayMouseScale")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayMouseScale {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayName")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayName {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayName")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayName {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayName";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayName")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayName {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayName")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayName")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayName {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferSize: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
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
                (ulOverlayHandle, pchValue, unBufferSize, pError, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (pError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferSize: u32,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pchValue, unBufferSize, pError))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayName")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayName {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderModel")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayRenderModel {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayRenderModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayRenderModel";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderModel")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayRenderModel {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderModel")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayRenderModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderModel")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayRenderModel {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferSize: u32,
        pColor: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdColor_t>,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
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
                (
                    ulOverlayHandle,
                    pchValue,
                    unBufferSize,
                    pColor,
                    pError,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pColor: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdColor_t>,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("EndInvoke", (pColor, pError, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferSize: u32,
        pColor: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdColor_t>,
        pError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVROverlayError>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke(
                "Invoke",
                (ulOverlayHandle, pchValue, unBufferSize, pColor, pError),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayRenderModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderingPid")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayRenderingPid {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderingPid")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayRenderingPid {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayRenderingPid";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderingPid")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayRenderingPid {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderingPid")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayRenderingPid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderingPid")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayRenderingPid {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("Invoke", (ulOverlayHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayRenderingPid")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayRenderingPid {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlaySortOrder")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlaySortOrder {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlaySortOrder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlaySortOrder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlaySortOrder";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlaySortOrder")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlaySortOrder {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlaySortOrder")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlaySortOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlaySortOrder")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlaySortOrder {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        punSortOrder: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, punSortOrder, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        punSortOrder: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (punSortOrder, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        punSortOrder: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, punSortOrder))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlaySortOrder")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlaySortOrder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexelAspect")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayTexelAspect {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexelAspect")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayTexelAspect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayTexelAspect";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexelAspect")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayTexelAspect {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexelAspect")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayTexelAspect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexelAspect")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayTexelAspect {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pfTexelAspect: quest_hook::libil2cpp::ByRefMut<f32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, pfTexelAspect, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pfTexelAspect: quest_hook::libil2cpp::ByRefMut<f32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pfTexelAspect, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pfTexelAspect: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pfTexelAspect))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexelAspect")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayTexelAspect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexture")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayTexture {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexture")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayTexture {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayTexture";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexture")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayTexture {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexture")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexture")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayTexture {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pNativeTextureHandle: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        pNativeTextureRef: crate::System::IntPtr,
        pWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        pNativeFormat: quest_hook::libil2cpp::ByRefMut<u32>,
        pAPIType: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::ETextureType>,
        pColorSpace: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EColorSpace>,
        pTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
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
                (
                    ulOverlayHandle,
                    pNativeTextureHandle,
                    pNativeTextureRef,
                    pWidth,
                    pHeight,
                    pNativeFormat,
                    pAPIType,
                    pColorSpace,
                    pTextureBounds,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pNativeTextureHandle: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        pWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        pNativeFormat: quest_hook::libil2cpp::ByRefMut<u32>,
        pAPIType: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::ETextureType>,
        pColorSpace: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EColorSpace>,
        pTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "EndInvoke",
                (
                    pNativeTextureHandle,
                    pWidth,
                    pHeight,
                    pNativeFormat,
                    pAPIType,
                    pColorSpace,
                    pTextureBounds,
                    result,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pNativeTextureHandle: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        pNativeTextureRef: crate::System::IntPtr,
        pWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        pNativeFormat: quest_hook::libil2cpp::ByRefMut<u32>,
        pAPIType: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::ETextureType>,
        pColorSpace: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EColorSpace>,
        pTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (
                    ulOverlayHandle,
                    pNativeTextureHandle,
                    pNativeTextureRef,
                    pWidth,
                    pHeight,
                    pNativeFormat,
                    pAPIType,
                    pColorSpace,
                    pTextureBounds,
                ),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTexture")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayTexture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureBounds")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayTextureBounds {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureBounds")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayTextureBounds {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayTextureBounds";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureBounds")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayTextureBounds {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureBounds")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayTextureBounds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureBounds")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayTextureBounds {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pOverlayTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
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
                (ulOverlayHandle, pOverlayTextureBounds, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pOverlayTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pOverlayTextureBounds, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pOverlayTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pOverlayTextureBounds))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureBounds")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayTextureBounds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureColorSpace")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayTextureColorSpace {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureColorSpace")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayTextureColorSpace {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayTextureColorSpace";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureColorSpace")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayTextureColorSpace {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureColorSpace")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayTextureColorSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureColorSpace")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayTextureColorSpace {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        peTextureColorSpace: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EColorSpace,
        >,
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
                (ulOverlayHandle, peTextureColorSpace, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peTextureColorSpace: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EColorSpace,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (peTextureColorSpace, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        peTextureColorSpace: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::EColorSpace,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, peTextureColorSpace))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureColorSpace")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayTextureColorSpace {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureSize")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayTextureSize {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureSize")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayTextureSize {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayTextureSize";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureSize")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayTextureSize {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureSize")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayTextureSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureSize")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayTextureSize {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pHeight: quest_hook::libil2cpp::ByRefMut<u32>,
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
                (ulOverlayHandle, pWidth, pHeight, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pWidth, pHeight, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pWidth, pHeight))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTextureSize")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayTextureSize {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformAbsolute")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayTransformAbsolute {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformAbsolute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformAbsolute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayTransformAbsolute";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformAbsolute")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformAbsolute {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformAbsolute")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformAbsolute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformAbsolute")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayTransformAbsolute {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        peTrackingOrigin: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackingUniverseOrigin,
        >,
        pmatTrackingOriginToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
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
                (
                    ulOverlayHandle,
                    peTrackingOrigin,
                    pmatTrackingOriginToOverlayTransform,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peTrackingOrigin: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackingUniverseOrigin,
        >,
        pmatTrackingOriginToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "EndInvoke",
                (peTrackingOrigin, pmatTrackingOriginToOverlayTransform, result),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        peTrackingOrigin: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::ETrackingUniverseOrigin,
        >,
        pmatTrackingOriginToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (ulOverlayHandle, peTrackingOrigin, pmatTrackingOriginToOverlayTransform),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformAbsolute")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformAbsolute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformOverlayRelative")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayTransformOverlayRelative {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformOverlayRelative")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformOverlayRelative {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayTransformOverlayRelative";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformOverlayRelative")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformOverlayRelative {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformOverlayRelative")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformOverlayRelative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformOverlayRelative")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayTransformOverlayRelative {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        ulOverlayHandleParent: quest_hook::libil2cpp::ByRefMut<u64>,
        pmatParentOverlayToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
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
                (
                    ulOverlayHandle,
                    ulOverlayHandleParent,
                    pmatParentOverlayToOverlayTransform,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        ulOverlayHandleParent: quest_hook::libil2cpp::ByRefMut<u64>,
        pmatParentOverlayToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "EndInvoke",
                (ulOverlayHandleParent, pmatParentOverlayToOverlayTransform, result),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        ulOverlayHandleParent: quest_hook::libil2cpp::ByRefMut<u64>,
        pmatParentOverlayToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (
                    ulOverlayHandle,
                    ulOverlayHandleParent,
                    pmatParentOverlayToOverlayTransform,
                ),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformOverlayRelative")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformOverlayRelative {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceComponent")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayTransformTrackedDeviceComponent {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceComponent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceComponent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayTransformTrackedDeviceComponent";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceComponent")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceComponent {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceComponent")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceComponent")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceComponent {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        punDeviceIndex: quest_hook::libil2cpp::ByRefMut<u32>,
        pchComponentName: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unComponentNameSize: u32,
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
                (
                    ulOverlayHandle,
                    punDeviceIndex,
                    pchComponentName,
                    unComponentNameSize,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        punDeviceIndex: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (punDeviceIndex, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        punDeviceIndex: quest_hook::libil2cpp::ByRefMut<u32>,
        pchComponentName: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unComponentNameSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (ulOverlayHandle, punDeviceIndex, pchComponentName, unComponentNameSize),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceComponent")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceComponent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceRelative")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayTransformTrackedDeviceRelative {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceRelative")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceRelative {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayTransformTrackedDeviceRelative";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceRelative")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceRelative {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceRelative")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceRelative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceRelative")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceRelative {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        punTrackedDevice: quest_hook::libil2cpp::ByRefMut<u32>,
        pmatTrackedDeviceToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
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
                (
                    ulOverlayHandle,
                    punTrackedDevice,
                    pmatTrackedDeviceToOverlayTransform,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        punTrackedDevice: quest_hook::libil2cpp::ByRefMut<u32>,
        pmatTrackedDeviceToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "EndInvoke",
                (punTrackedDevice, pmatTrackedDeviceToOverlayTransform, result),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        punTrackedDevice: quest_hook::libil2cpp::ByRefMut<u32>,
        pmatTrackedDeviceToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (ulOverlayHandle, punTrackedDevice, pmatTrackedDeviceToOverlayTransform),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformTrackedDeviceRelative")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformTrackedDeviceRelative {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformType")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayTransformType {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformType {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayTransformType";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformType")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformType {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformType")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformType")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayTransformType {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        peTransformType: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayTransformType,
        >,
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
                (ulOverlayHandle, peTransformType, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        peTransformType: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayTransformType,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (peTransformType, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        peTransformType: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayTransformType,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, peTransformType))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayTransformType")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayTransformType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayWidthInMeters")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetOverlayWidthInMeters {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayWidthInMeters")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetOverlayWidthInMeters {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetOverlayWidthInMeters";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayWidthInMeters")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetOverlayWidthInMeters {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayWidthInMeters")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetOverlayWidthInMeters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayWidthInMeters")]
impl crate::OVR::OpenVR::IVROverlay__GetOverlayWidthInMeters {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pfWidthInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
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
                (ulOverlayHandle, pfWidthInMeters, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pfWidthInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pfWidthInMeters, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pfWidthInMeters: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pfWidthInMeters))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetOverlayWidthInMeters")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetOverlayWidthInMeters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetPrimaryDashboardDevice")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetPrimaryDashboardDevice {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetPrimaryDashboardDevice")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetPrimaryDashboardDevice {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetPrimaryDashboardDevice";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetPrimaryDashboardDevice")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__GetPrimaryDashboardDevice {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetPrimaryDashboardDevice")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__GetPrimaryDashboardDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetPrimaryDashboardDevice")]
impl crate::OVR::OpenVR::IVROverlay__GetPrimaryDashboardDevice {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("Invoke", ())?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetPrimaryDashboardDevice")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetPrimaryDashboardDevice {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetTransformForOverlayCoordinates")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__GetTransformForOverlayCoordinates {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetTransformForOverlayCoordinates")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__GetTransformForOverlayCoordinates {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_GetTransformForOverlayCoordinates";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetTransformForOverlayCoordinates")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVROverlay__GetTransformForOverlayCoordinates {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetTransformForOverlayCoordinates")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__GetTransformForOverlayCoordinates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetTransformForOverlayCoordinates")]
impl crate::OVR::OpenVR::IVROverlay__GetTransformForOverlayCoordinates {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        eTrackingOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        coordinatesInOverlay: crate::OVR::OpenVR::HmdVector2_t,
        pmatTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
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
                (
                    ulOverlayHandle,
                    eTrackingOrigin,
                    coordinatesInOverlay,
                    pmatTransform,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pmatTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pmatTransform, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        eTrackingOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        coordinatesInOverlay: crate::OVR::OpenVR::HmdVector2_t,
        pmatTransform: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix34_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (ulOverlayHandle, eTrackingOrigin, coordinatesInOverlay, pmatTransform),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_GetTransformForOverlayCoordinates")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__GetTransformForOverlayCoordinates {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_HideKeyboard")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__HideKeyboard {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_HideKeyboard")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__HideKeyboard {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_HideKeyboard";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_HideKeyboard")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__HideKeyboard {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_HideKeyboard")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__HideKeyboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_HideKeyboard")]
impl crate::OVR::OpenVR::IVROverlay__HideKeyboard {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_HideKeyboard")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVROverlay__HideKeyboard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_HideOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__HideOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_HideOverlay")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVROverlay__HideOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_HideOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_HideOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__HideOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_HideOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__HideOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_HideOverlay")]
impl crate::OVR::OpenVR::IVROverlay__HideOverlay {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_HideOverlay")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVROverlay__HideOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsActiveDashboardOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__IsActiveDashboardOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsActiveDashboardOverlay")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__IsActiveDashboardOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_IsActiveDashboardOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsActiveDashboardOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__IsActiveDashboardOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsActiveDashboardOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__IsActiveDashboardOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsActiveDashboardOverlay")]
impl crate::OVR::OpenVR::IVROverlay__IsActiveDashboardOverlay {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (ulOverlayHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsActiveDashboardOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__IsActiveDashboardOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsDashboardVisible")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__IsDashboardVisible {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsDashboardVisible")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__IsDashboardVisible {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_IsDashboardVisible";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsDashboardVisible")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__IsDashboardVisible {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsDashboardVisible")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__IsDashboardVisible {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsDashboardVisible")]
impl crate::OVR::OpenVR::IVROverlay__IsDashboardVisible {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", ())?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsDashboardVisible")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__IsDashboardVisible {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsHoverTargetOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__IsHoverTargetOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsHoverTargetOverlay")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__IsHoverTargetOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_IsHoverTargetOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsHoverTargetOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__IsHoverTargetOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsHoverTargetOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__IsHoverTargetOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsHoverTargetOverlay")]
impl crate::OVR::OpenVR::IVROverlay__IsHoverTargetOverlay {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (ulOverlayHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsHoverTargetOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__IsHoverTargetOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsOverlayVisible")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__IsOverlayVisible {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsOverlayVisible")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__IsOverlayVisible {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_IsOverlayVisible";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsOverlayVisible")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__IsOverlayVisible {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsOverlayVisible")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__IsOverlayVisible {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsOverlayVisible")]
impl crate::OVR::OpenVR::IVROverlay__IsOverlayVisible {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (ulOverlayHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_IsOverlayVisible")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__IsOverlayVisible {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_MoveGamepadFocusToNeighbor")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__MoveGamepadFocusToNeighbor {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_MoveGamepadFocusToNeighbor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__MoveGamepadFocusToNeighbor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_MoveGamepadFocusToNeighbor";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_MoveGamepadFocusToNeighbor")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__MoveGamepadFocusToNeighbor {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_MoveGamepadFocusToNeighbor")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__MoveGamepadFocusToNeighbor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_MoveGamepadFocusToNeighbor")]
impl crate::OVR::OpenVR::IVROverlay__MoveGamepadFocusToNeighbor {
    pub fn BeginInvoke(
        &mut self,
        eDirection: crate::OVR::OpenVR::EOverlayDirection,
        ulFrom: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (eDirection, ulFrom, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eDirection: crate::OVR::OpenVR::EOverlayDirection,
        ulFrom: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (eDirection, ulFrom))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_MoveGamepadFocusToNeighbor")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__MoveGamepadFocusToNeighbor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_PollNextOverlayEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__PollNextOverlayEvent {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_PollNextOverlayEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__PollNextOverlayEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_PollNextOverlayEvent";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_PollNextOverlayEvent")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__PollNextOverlayEvent {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_PollNextOverlayEvent")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__PollNextOverlayEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_PollNextOverlayEvent")]
impl crate::OVR::OpenVR::IVROverlay__PollNextOverlayEvent {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        uncbVREvent: u32,
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
                (ulOverlayHandle, pEvent, uncbVREvent, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (pEvent, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pEvent: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VREvent_t>,
        uncbVREvent: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pEvent, uncbVREvent))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_PollNextOverlayEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__PollNextOverlayEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ReleaseNativeOverlayHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__ReleaseNativeOverlayHandle {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ReleaseNativeOverlayHandle")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__ReleaseNativeOverlayHandle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_ReleaseNativeOverlayHandle";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ReleaseNativeOverlayHandle")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__ReleaseNativeOverlayHandle {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ReleaseNativeOverlayHandle")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__ReleaseNativeOverlayHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ReleaseNativeOverlayHandle")]
impl crate::OVR::OpenVR::IVROverlay__ReleaseNativeOverlayHandle {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pNativeTextureHandle: crate::System::IntPtr,
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
                (ulOverlayHandle, pNativeTextureHandle, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pNativeTextureHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pNativeTextureHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ReleaseNativeOverlayHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__ReleaseNativeOverlayHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetDashboardOverlaySceneProcess")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetDashboardOverlaySceneProcess {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetDashboardOverlaySceneProcess")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetDashboardOverlaySceneProcess {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetDashboardOverlaySceneProcess";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetDashboardOverlaySceneProcess")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVROverlay__SetDashboardOverlaySceneProcess {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetDashboardOverlaySceneProcess")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__SetDashboardOverlaySceneProcess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetDashboardOverlaySceneProcess")]
impl crate::OVR::OpenVR::IVROverlay__SetDashboardOverlaySceneProcess {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        unProcessId: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, unProcessId, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        unProcessId: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, unProcessId))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetDashboardOverlaySceneProcess")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetDashboardOverlaySceneProcess {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetGamepadFocusOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetGamepadFocusOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetGamepadFocusOverlay")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetGamepadFocusOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetGamepadFocusOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetGamepadFocusOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetGamepadFocusOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetGamepadFocusOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetGamepadFocusOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetGamepadFocusOverlay")]
impl crate::OVR::OpenVR::IVROverlay__SetGamepadFocusOverlay {
    pub fn BeginInvoke(
        &mut self,
        ulNewFocusOverlay: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulNewFocusOverlay, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulNewFocusOverlay: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulNewFocusOverlay))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetGamepadFocusOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetGamepadFocusOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetHighQualityOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetHighQualityOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetHighQualityOverlay")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetHighQualityOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetHighQualityOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetHighQualityOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetHighQualityOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetHighQualityOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetHighQualityOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetHighQualityOverlay")]
impl crate::OVR::OpenVR::IVROverlay__SetHighQualityOverlay {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetHighQualityOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetHighQualityOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardPositionForOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetKeyboardPositionForOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardPositionForOverlay")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetKeyboardPositionForOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetKeyboardPositionForOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardPositionForOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetKeyboardPositionForOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardPositionForOverlay")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__SetKeyboardPositionForOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardPositionForOverlay")]
impl crate::OVR::OpenVR::IVROverlay__SetKeyboardPositionForOverlay {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        avoidRect: crate::OVR::OpenVR::HmdRect2_t,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, avoidRect, callback, object))?;
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
        ulOverlayHandle: u64,
        avoidRect: crate::OVR::OpenVR::HmdRect2_t,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, avoidRect))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardPositionForOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetKeyboardPositionForOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardTransformAbsolute")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetKeyboardTransformAbsolute {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardTransformAbsolute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetKeyboardTransformAbsolute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetKeyboardTransformAbsolute";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardTransformAbsolute")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetKeyboardTransformAbsolute {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardTransformAbsolute")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__SetKeyboardTransformAbsolute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardTransformAbsolute")]
impl crate::OVR::OpenVR::IVROverlay__SetKeyboardTransformAbsolute {
    pub fn BeginInvoke(
        &mut self,
        eTrackingOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pmatTrackingOriginToKeyboardTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
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
                (
                    eTrackingOrigin,
                    pmatTrackingOriginToKeyboardTransform,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pmatTrackingOriginToKeyboardTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (pmatTrackingOriginToKeyboardTransform, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eTrackingOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pmatTrackingOriginToKeyboardTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (eTrackingOrigin, pmatTrackingOriginToKeyboardTransform))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetKeyboardTransformAbsolute")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetKeyboardTransformAbsolute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAlpha")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayAlpha {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAlpha")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayAlpha {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayAlpha";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAlpha")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayAlpha {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAlpha")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayAlpha {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAlpha")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayAlpha {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        fAlpha: f32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, fAlpha, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        fAlpha: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, fAlpha))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAlpha")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayAlpha {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAutoCurveDistanceRangeInMeters")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayAutoCurveDistanceRangeInMeters {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAutoCurveDistanceRangeInMeters")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayAutoCurveDistanceRangeInMeters {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayAutoCurveDistanceRangeInMeters";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAutoCurveDistanceRangeInMeters")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVROverlay__SetOverlayAutoCurveDistanceRangeInMeters {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAutoCurveDistanceRangeInMeters")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__SetOverlayAutoCurveDistanceRangeInMeters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAutoCurveDistanceRangeInMeters")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayAutoCurveDistanceRangeInMeters {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        fMinDistanceInMeters: f32,
        fMaxDistanceInMeters: f32,
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
                (
                    ulOverlayHandle,
                    fMinDistanceInMeters,
                    fMaxDistanceInMeters,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        fMinDistanceInMeters: f32,
        fMaxDistanceInMeters: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (ulOverlayHandle, fMinDistanceInMeters, fMaxDistanceInMeters),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayAutoCurveDistanceRangeInMeters")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayAutoCurveDistanceRangeInMeters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayColor")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayColor {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayColor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayColor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayColor";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayColor")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayColor {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayColor")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayColor")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayColor {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        fRed: f32,
        fGreen: f32,
        fBlue: f32,
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
                (ulOverlayHandle, fRed, fGreen, fBlue, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        fRed: f32,
        fGreen: f32,
        fBlue: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, fRed, fGreen, fBlue))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayColor")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayColor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayDualAnalogTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayDualAnalogTransform {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayDualAnalogTransform")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayDualAnalogTransform {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayDualAnalogTransform";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayDualAnalogTransform")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayDualAnalogTransform {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayDualAnalogTransform")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__SetOverlayDualAnalogTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayDualAnalogTransform")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayDualAnalogTransform {
    pub fn BeginInvoke(
        &mut self,
        ulOverlay: u64,
        eWhich: crate::OVR::OpenVR::EDualAnalogWhich,
        vCenter: crate::System::IntPtr,
        fRadius: f32,
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
                (ulOverlay, eWhich, vCenter, fRadius, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlay: u64,
        eWhich: crate::OVR::OpenVR::EDualAnalogWhich,
        vCenter: crate::System::IntPtr,
        fRadius: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlay, eWhich, vCenter, fRadius))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayDualAnalogTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayDualAnalogTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFlag")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayFlag {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFlag")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayFlag {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayFlag";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFlag")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayFlag {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFlag")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayFlag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFlag")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayFlag {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        eOverlayFlag: crate::OVR::OpenVR::VROverlayFlags,
        bEnabled: bool,
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
                (ulOverlayHandle, eOverlayFlag, bEnabled, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        eOverlayFlag: crate::OVR::OpenVR::VROverlayFlags,
        bEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, eOverlayFlag, bEnabled))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFlag")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayFlag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFromFile")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayFromFile {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFromFile")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayFromFile {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayFromFile";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFromFile")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayFromFile {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFromFile")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayFromFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFromFile")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayFromFile {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pchFilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, pchFilePath, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pchFilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pchFilePath))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayFromFile")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayFromFile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayInputMethod")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayInputMethod {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayInputMethod")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayInputMethod {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayInputMethod";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayInputMethod")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayInputMethod {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayInputMethod")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayInputMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayInputMethod")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayInputMethod {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        eInputMethod: crate::OVR::OpenVR::VROverlayInputMethod,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, eInputMethod, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        eInputMethod: crate::OVR::OpenVR::VROverlayInputMethod,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, eInputMethod))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayInputMethod")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayInputMethod {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayIntersectionMask")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayIntersectionMask {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayIntersectionMask")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayIntersectionMask {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayIntersectionMask";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayIntersectionMask")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayIntersectionMask {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayIntersectionMask")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayIntersectionMask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayIntersectionMask")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayIntersectionMask {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pMaskPrimitives: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayIntersectionMaskPrimitive_t,
        >,
        unNumMaskPrimitives: u32,
        unPrimitiveSize: u32,
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
                (
                    ulOverlayHandle,
                    pMaskPrimitives,
                    unNumMaskPrimitives,
                    unPrimitiveSize,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pMaskPrimitives: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayIntersectionMaskPrimitive_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pMaskPrimitives, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pMaskPrimitives: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VROverlayIntersectionMaskPrimitive_t,
        >,
        unNumMaskPrimitives: u32,
        unPrimitiveSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (ulOverlayHandle, pMaskPrimitives, unNumMaskPrimitives, unPrimitiveSize),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayIntersectionMask")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayIntersectionMask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayMouseScale")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayMouseScale {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayMouseScale")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayMouseScale {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayMouseScale";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayMouseScale")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayMouseScale {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayMouseScale")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayMouseScale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayMouseScale")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayMouseScale {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pvecMouseScale: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdVector2_t,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, pvecMouseScale, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pvecMouseScale: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdVector2_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pvecMouseScale, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pvecMouseScale: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pvecMouseScale))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayMouseScale")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayMouseScale {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayName")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayName {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayName")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayName {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayName";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayName")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayName {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayName")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayName")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayName {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pchName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, pchName, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pchName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pchName))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayName")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayName {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayNeighbor")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayNeighbor {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayNeighbor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayNeighbor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayNeighbor";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayNeighbor")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayNeighbor {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayNeighbor")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayNeighbor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayNeighbor")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayNeighbor {
    pub fn BeginInvoke(
        &mut self,
        eDirection: crate::OVR::OpenVR::EOverlayDirection,
        ulFrom: u64,
        ulTo: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (eDirection, ulFrom, ulTo, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eDirection: crate::OVR::OpenVR::EOverlayDirection,
        ulFrom: u64,
        ulTo: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (eDirection, ulFrom, ulTo))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayNeighbor")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayNeighbor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRaw")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayRaw {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRaw")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayRaw {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayRaw";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRaw")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayRaw {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRaw")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayRaw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRaw")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayRaw {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pvBuffer: crate::System::IntPtr,
        unWidth: u32,
        unHeight: u32,
        unDepth: u32,
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
                (ulOverlayHandle, pvBuffer, unWidth, unHeight, unDepth, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pvBuffer: crate::System::IntPtr,
        unWidth: u32,
        unHeight: u32,
        unDepth: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pvBuffer, unWidth, unHeight, unDepth))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRaw")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayRaw {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderModel")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayRenderModel {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayRenderModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayRenderModel";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderModel")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayRenderModel {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderModel")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayRenderModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderModel")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayRenderModel {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pchRenderModel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pColor: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdColor_t>,
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
                (ulOverlayHandle, pchRenderModel, pColor, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pColor: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdColor_t>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pColor, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pchRenderModel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pColor: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdColor_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pchRenderModel, pColor))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayRenderModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderingPid")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayRenderingPid {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderingPid")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayRenderingPid {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayRenderingPid";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderingPid")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayRenderingPid {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderingPid")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayRenderingPid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderingPid")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayRenderingPid {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        unPID: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, unPID, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        unPID: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, unPID))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayRenderingPid")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayRenderingPid {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlaySortOrder")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlaySortOrder {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlaySortOrder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlaySortOrder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlaySortOrder";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlaySortOrder")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlaySortOrder {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlaySortOrder")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlaySortOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlaySortOrder")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlaySortOrder {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        unSortOrder: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, unSortOrder, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        unSortOrder: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, unSortOrder))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlaySortOrder")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlaySortOrder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexelAspect")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayTexelAspect {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexelAspect")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayTexelAspect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayTexelAspect";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexelAspect")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayTexelAspect {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexelAspect")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayTexelAspect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexelAspect")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayTexelAspect {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        fTexelAspect: f32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, fTexelAspect, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        fTexelAspect: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, fTexelAspect))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexelAspect")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayTexelAspect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexture")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayTexture {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexture")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayTexture {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayTexture";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexture")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayTexture {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexture")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexture")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayTexture {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pTexture: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::Texture_t>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, pTexture, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pTexture: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::Texture_t>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pTexture, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pTexture: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::Texture_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pTexture))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTexture")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayTexture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureBounds")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayTextureBounds {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureBounds")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayTextureBounds {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayTextureBounds";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureBounds")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayTextureBounds {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureBounds")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayTextureBounds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureBounds")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayTextureBounds {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        pOverlayTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
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
                (ulOverlayHandle, pOverlayTextureBounds, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pOverlayTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pOverlayTextureBounds, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        pOverlayTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, pOverlayTextureBounds))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureBounds")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayTextureBounds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureColorSpace")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayTextureColorSpace {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureColorSpace")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayTextureColorSpace {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayTextureColorSpace";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureColorSpace")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayTextureColorSpace {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureColorSpace")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayTextureColorSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureColorSpace")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayTextureColorSpace {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        eTextureColorSpace: crate::OVR::OpenVR::EColorSpace,
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
                (ulOverlayHandle, eTextureColorSpace, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        eTextureColorSpace: crate::OVR::OpenVR::EColorSpace,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, eTextureColorSpace))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTextureColorSpace")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayTextureColorSpace {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformAbsolute")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayTransformAbsolute {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformAbsolute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformAbsolute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayTransformAbsolute";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformAbsolute")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformAbsolute {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformAbsolute")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformAbsolute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformAbsolute")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayTransformAbsolute {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        eTrackingOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pmatTrackingOriginToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
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
                (
                    ulOverlayHandle,
                    eTrackingOrigin,
                    pmatTrackingOriginToOverlayTransform,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pmatTrackingOriginToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pmatTrackingOriginToOverlayTransform, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        eTrackingOrigin: crate::OVR::OpenVR::ETrackingUniverseOrigin,
        pmatTrackingOriginToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (ulOverlayHandle, eTrackingOrigin, pmatTrackingOriginToOverlayTransform),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformAbsolute")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformAbsolute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformOverlayRelative")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayTransformOverlayRelative {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformOverlayRelative")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformOverlayRelative {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayTransformOverlayRelative";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformOverlayRelative")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformOverlayRelative {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformOverlayRelative")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformOverlayRelative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformOverlayRelative")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayTransformOverlayRelative {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        ulOverlayHandleParent: u64,
        pmatParentOverlayToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
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
                (
                    ulOverlayHandle,
                    ulOverlayHandleParent,
                    pmatParentOverlayToOverlayTransform,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pmatParentOverlayToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pmatParentOverlayToOverlayTransform, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        ulOverlayHandleParent: u64,
        pmatParentOverlayToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (
                    ulOverlayHandle,
                    ulOverlayHandleParent,
                    pmatParentOverlayToOverlayTransform,
                ),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformOverlayRelative")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformOverlayRelative {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceComponent")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayTransformTrackedDeviceComponent {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceComponent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceComponent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayTransformTrackedDeviceComponent";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceComponent")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceComponent {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceComponent")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceComponent")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceComponent {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        unDeviceIndex: u32,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
                (ulOverlayHandle, unDeviceIndex, pchComponentName, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        unDeviceIndex: u32,
        pchComponentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, unDeviceIndex, pchComponentName))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceComponent")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceComponent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceRelative")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayTransformTrackedDeviceRelative {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceRelative")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceRelative {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayTransformTrackedDeviceRelative";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceRelative")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceRelative {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceRelative")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceRelative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceRelative")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceRelative {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        unTrackedDevice: u32,
        pmatTrackedDeviceToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
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
                (
                    ulOverlayHandle,
                    unTrackedDevice,
                    pmatTrackedDeviceToOverlayTransform,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pmatTrackedDeviceToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (pmatTrackedDeviceToOverlayTransform, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        unTrackedDevice: u32,
        pmatTrackedDeviceToOverlayTransform: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::HmdMatrix34_t,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (ulOverlayHandle, unTrackedDevice, pmatTrackedDeviceToOverlayTransform),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayTransformTrackedDeviceRelative")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayTransformTrackedDeviceRelative {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayWidthInMeters")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__SetOverlayWidthInMeters {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayWidthInMeters")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__SetOverlayWidthInMeters {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_SetOverlayWidthInMeters";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayWidthInMeters")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__SetOverlayWidthInMeters {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayWidthInMeters")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__SetOverlayWidthInMeters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayWidthInMeters")]
impl crate::OVR::OpenVR::IVROverlay__SetOverlayWidthInMeters {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        fWidthInMeters: f32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, fWidthInMeters, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        fWidthInMeters: f32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle, fWidthInMeters))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_SetOverlayWidthInMeters")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__SetOverlayWidthInMeters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowDashboard")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__ShowDashboard {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowDashboard")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__ShowDashboard {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_ShowDashboard";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowDashboard")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__ShowDashboard {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowDashboard")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__ShowDashboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowDashboard")]
impl crate::OVR::OpenVR::IVROverlay__ShowDashboard {
    pub fn BeginInvoke(
        &mut self,
        pchOverlayToShow: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (pchOverlayToShow, callback, object))?;
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
        pchOverlayToShow: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (pchOverlayToShow))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowDashboard")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__ShowDashboard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboard")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__ShowKeyboard {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboard")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__ShowKeyboard {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_ShowKeyboard";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboard")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__ShowKeyboard {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboard")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__ShowKeyboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboard")]
impl crate::OVR::OpenVR::IVROverlay__ShowKeyboard {
    pub fn BeginInvoke(
        &mut self,
        eInputMode: i32,
        eLineInputMode: i32,
        pchDescription: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        unCharMax: u32,
        pchExistingText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bUseMinimalMode: bool,
        uUserValue: u64,
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
                (
                    eInputMode,
                    eLineInputMode,
                    pchDescription,
                    unCharMax,
                    pchExistingText,
                    bUseMinimalMode,
                    uUserValue,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eInputMode: i32,
        eLineInputMode: i32,
        pchDescription: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        unCharMax: u32,
        pchExistingText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bUseMinimalMode: bool,
        uUserValue: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (
                    eInputMode,
                    eLineInputMode,
                    pchDescription,
                    unCharMax,
                    pchExistingText,
                    bUseMinimalMode,
                    uUserValue,
                ),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboard")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVROverlay__ShowKeyboard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboardForOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__ShowKeyboardForOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboardForOverlay")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__ShowKeyboardForOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_ShowKeyboardForOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboardForOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__ShowKeyboardForOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboardForOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__ShowKeyboardForOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboardForOverlay")]
impl crate::OVR::OpenVR::IVROverlay__ShowKeyboardForOverlay {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        eInputMode: i32,
        eLineInputMode: i32,
        pchDescription: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        unCharMax: u32,
        pchExistingText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bUseMinimalMode: bool,
        uUserValue: u64,
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
                (
                    ulOverlayHandle,
                    eInputMode,
                    eLineInputMode,
                    pchDescription,
                    unCharMax,
                    pchExistingText,
                    bUseMinimalMode,
                    uUserValue,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
        eInputMode: i32,
        eLineInputMode: i32,
        pchDescription: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        unCharMax: u32,
        pchExistingText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bUseMinimalMode: bool,
        uUserValue: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke(
                "Invoke",
                (
                    ulOverlayHandle,
                    eInputMode,
                    eLineInputMode,
                    pchDescription,
                    unCharMax,
                    pchExistingText,
                    bUseMinimalMode,
                    uUserValue,
                ),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowKeyboardForOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__ShowKeyboardForOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowMessageOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__ShowMessageOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowMessageOverlay")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVROverlay__ShowMessageOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_ShowMessageOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowMessageOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__ShowMessageOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowMessageOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__ShowMessageOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowMessageOverlay")]
impl crate::OVR::OpenVR::IVROverlay__ShowMessageOverlay {
    pub fn BeginInvoke(
        &mut self,
        pchText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchCaption: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchButton0Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchButton1Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchButton2Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchButton3Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
                (
                    pchText,
                    pchCaption,
                    pchButton0Text,
                    pchButton1Text,
                    pchButton2Text,
                    pchButton3Text,
                    callback,
                    object,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::VRMessageOverlayResponse> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::VRMessageOverlayResponse = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        pchText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchCaption: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchButton0Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchButton1Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchButton2Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchButton3Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::VRMessageOverlayResponse> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::VRMessageOverlayResponse = __cordl_object
            .invoke(
                "Invoke",
                (
                    pchText,
                    pchCaption,
                    pchButton0Text,
                    pchButton1Text,
                    pchButton2Text,
                    pchButton3Text,
                ),
            )?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowMessageOverlay")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVROverlay__ShowMessageOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowOverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct IVROverlay__ShowOverlay {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowOverlay")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVROverlay__ShowOverlay {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "_ShowOverlay";
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowOverlay")]
impl std::ops::Deref for crate::OVR::OpenVR::IVROverlay__ShowOverlay {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowOverlay")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVROverlay__ShowOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowOverlay")]
impl crate::OVR::OpenVR::IVROverlay__ShowOverlay {
    pub fn BeginInvoke(
        &mut self,
        ulOverlayHandle: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (ulOverlayHandle, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        ulOverlayHandle: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVROverlayError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVROverlayError = __cordl_object
            .invoke("Invoke", (ulOverlayHandle))?;
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
#[cfg(feature = "OVR+OpenVR+IVROverlay+_ShowOverlay")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVROverlay__ShowOverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
