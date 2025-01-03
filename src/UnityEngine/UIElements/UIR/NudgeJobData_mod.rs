#[cfg(feature = "UnityEngine+UIElements+UIR+NudgeJobData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct NudgeJobData {
    pub src: crate::System::IntPtr,
    pub dst: crate::System::IntPtr,
    pub count: i32,
    pub closingSrc: crate::System::IntPtr,
    pub closingDst: crate::System::IntPtr,
    pub closingCount: i32,
    pub transform: crate::UnityEngine::Matrix4x4,
    pub vertsBeforeUVDisplacement: i32,
    pub vertsAfterUVDisplacement: i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+NudgeJobData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::NudgeJobData =>
    "UnityEngine.UIElements.UIR"."NudgeJobData"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+NudgeJobData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::NudgeJobData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+NudgeJobData")]
impl crate::UnityEngine::UIElements::UIR::NudgeJobData {}
