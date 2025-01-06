#[cfg(feature = "UnityEngine+PenData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PenData {
    pub position: crate::UnityEngine::Vector2,
    pub tilt: crate::UnityEngine::Vector2,
    pub penStatus: crate::UnityEngine::PenStatus,
    pub twist: f32,
    pub pressure: f32,
    pub contactType: crate::UnityEngine::PenEventType,
    pub deltaPos: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+PenData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PenData => "UnityEngine"."PenData"
);
#[cfg(feature = "UnityEngine+PenData")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::PenData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+PenData")]
impl crate::UnityEngine::PenData {}
