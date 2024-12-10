#[cfg(feature = "UnityEngine+AnimatorStateInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AnimatorStateInfo {
    pub m_Name: i32,
    pub m_Path: i32,
    pub m_FullPath: i32,
    pub m_NormalizedTime: f32,
    pub m_Length: f32,
    pub m_Speed: f32,
    pub m_SpeedMultiplier: f32,
    pub m_Tag: i32,
    pub m_Loop: i32,
}
#[cfg(feature = "UnityEngine+AnimatorStateInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AnimatorStateInfo => "UnityEngine"
    ."AnimatorStateInfo"
);
#[cfg(feature = "UnityEngine+AnimatorStateInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::AnimatorStateInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+AnimatorStateInfo")]
impl crate::UnityEngine::AnimatorStateInfo {
    pub fn get_normalizedTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_normalizedTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
