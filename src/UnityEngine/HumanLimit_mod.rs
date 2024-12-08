#[cfg(feature = "UnityEngine+HumanLimit")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HumanLimit {
    pub m_Min: crate::UnityEngine::Vector3,
    pub m_Max: crate::UnityEngine::Vector3,
    pub m_Center: crate::UnityEngine::Vector3,
    pub m_AxisLength: f32,
    pub m_UseDefaultValues: i32,
}
#[cfg(feature = "UnityEngine+HumanLimit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::HumanLimit => "UnityEngine"
    ."HumanLimit"
);
#[cfg(feature = "UnityEngine+HumanLimit")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::HumanLimit {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+HumanLimit")]
impl crate::UnityEngine::HumanLimit {}
