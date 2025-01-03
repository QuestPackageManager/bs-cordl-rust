#[cfg(feature = "UnityEngine+JointSpring")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct JointSpring {
    pub spring: f32,
    pub damper: f32,
    pub targetPosition: f32,
}
#[cfg(feature = "UnityEngine+JointSpring")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::JointSpring => "UnityEngine"
    ."JointSpring"
);
#[cfg(feature = "UnityEngine+JointSpring")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::JointSpring {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+JointSpring")]
impl crate::UnityEngine::JointSpring {}
