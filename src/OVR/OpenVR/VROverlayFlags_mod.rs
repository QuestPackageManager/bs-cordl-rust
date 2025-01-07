#[cfg(feature = "OVR+OpenVR+VROverlayFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VROverlayFlags {
    #[default]
    AcceptsGamepadEvents = 4i32,
    Curved = 1i32,
    NoDashboardTab = 3i32,
    None = 0i32,
    Panorama = 12i32,
    RGSS4X = 2i32,
    SendVRScrollEvents = 6i32,
    SendVRTouchpadEvents = 7i32,
    ShowGamepadFocus = 5i32,
    ShowTouchPadScrollWheel = 8i32,
    SideBySide_Crossed = 11i32,
    SideBySide_Parallel = 10i32,
    SortWithNonSceneOverlays = 14i32,
    StereoPanorama = 13i32,
    TransferOwnershipToInternalProcess = 9i32,
    VisibleInDashboard = 15i32,
}
#[cfg(feature = "OVR+OpenVR+VROverlayFlags")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::VROverlayFlags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "VROverlayFlags";
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
#[cfg(feature = "OVR+OpenVR+VROverlayFlags")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::VROverlayFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+VROverlayFlags")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::VROverlayFlags {
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
#[cfg(feature = "OVR+OpenVR+VROverlayFlags")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::VROverlayFlags {
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
#[cfg(feature = "OVR+OpenVR+VROverlayFlags")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::VROverlayFlags {
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
