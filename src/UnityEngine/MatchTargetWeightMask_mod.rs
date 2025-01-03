#[cfg(feature = "UnityEngine+MatchTargetWeightMask")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct MatchTargetWeightMask {
    pub m_PositionXYZWeight: crate::UnityEngine::Vector3,
    pub m_RotationWeight: f32,
}
#[cfg(feature = "UnityEngine+MatchTargetWeightMask")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::MatchTargetWeightMask =>
    "UnityEngine"."MatchTargetWeightMask"
);
#[cfg(feature = "UnityEngine+MatchTargetWeightMask")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::MatchTargetWeightMask {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+MatchTargetWeightMask")]
impl crate::UnityEngine::MatchTargetWeightMask {}
