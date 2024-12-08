#[cfg(feature = "UnityEngine+HumanDescription")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HumanDescription {
    pub human: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::HumanBone>,
    pub skeleton: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::SkeletonBone,
    >,
    pub m_ArmTwist: f32,
    pub m_ForeArmTwist: f32,
    pub m_UpperLegTwist: f32,
    pub m_LegTwist: f32,
    pub m_ArmStretch: f32,
    pub m_LegStretch: f32,
    pub m_FeetSpacing: f32,
    pub m_GlobalScale: f32,
    pub m_RootMotionBoneName: *mut crate::System::String,
    pub m_HasTranslationDoF: bool,
    pub m_HasExtraRoot: bool,
    pub m_SkeletonHasParents: bool,
}
#[cfg(feature = "UnityEngine+HumanDescription")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::HumanDescription => "UnityEngine"
    ."HumanDescription"
);
#[cfg(feature = "UnityEngine+HumanDescription")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::HumanDescription {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+HumanDescription")]
impl crate::UnityEngine::HumanDescription {}
