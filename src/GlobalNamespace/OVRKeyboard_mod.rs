#[cfg(feature = "OVRKeyboard")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRKeyboard {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRKeyboard")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRKeyboard => ""."OVRKeyboard"
);
#[cfg(feature = "OVRKeyboard")]
impl std::ops::Deref for crate::GlobalNamespace::OVRKeyboard {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRKeyboard")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRKeyboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRKeyboard")]
impl crate::GlobalNamespace::OVRKeyboard {
    #[cfg(feature = "OVRKeyboard+TrackedKeyboardInfo")]
    pub type TrackedKeyboardInfo = crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo;
    #[cfg(feature = "OVRKeyboard+TrackedKeyboardState")]
    pub type TrackedKeyboardState = crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState;
}
#[cfg(feature = "OVRKeyboard")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRKeyboard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRKeyboard+TrackedKeyboardInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRKeyboard_TrackedKeyboardInfo {
    pub Name: *mut quest_hook::libil2cpp::Il2CppString,
    pub Identifier: u64,
    pub Dimensions: crate::UnityEngine::Vector3,
    pub KeyboardFlags: crate::GlobalNamespace::OVRPlugin_TrackedKeyboardFlags,
    pub SupportedPresentationStyles: crate::GlobalNamespace::OVRPlugin_TrackedKeyboardPresentationStyles,
}
#[cfg(feature = "OVRKeyboard+TrackedKeyboardInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo
    => ""."OVRKeyboard/TrackedKeyboardInfo"
);
#[cfg(feature = "OVRKeyboard+TrackedKeyboardInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRKeyboard+TrackedKeyboardInfo")]
impl crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo {}
#[cfg(feature = "OVRKeyboard+TrackedKeyboardState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRKeyboard_TrackedKeyboardState {
    pub isPositionValid: bool,
    pub isPositionTracked: bool,
    pub isOrientationValid: bool,
    pub isOrientationTracked: bool,
    pub position: crate::UnityEngine::Vector3,
    pub rotation: crate::UnityEngine::Quaternion,
    pub timeInSeconds: f64,
}
#[cfg(feature = "OVRKeyboard+TrackedKeyboardState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRKeyboard_TrackedKeyboardState => ""
    ."OVRKeyboard/TrackedKeyboardState"
);
#[cfg(feature = "OVRKeyboard+TrackedKeyboardState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRKeyboard+TrackedKeyboardState")]
impl crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState {}
