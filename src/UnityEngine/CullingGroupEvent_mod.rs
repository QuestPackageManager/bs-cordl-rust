#[cfg(feature = "UnityEngine+CullingGroupEvent")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CullingGroupEvent {
    pub m_Index: i32,
    pub m_PrevState: u8,
    pub m_ThisState: u8,
}
#[cfg(feature = "UnityEngine+CullingGroupEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CullingGroupEvent => "UnityEngine"
    ."CullingGroupEvent"
);
#[cfg(feature = "UnityEngine+CullingGroupEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::CullingGroupEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+CullingGroupEvent")]
impl crate::UnityEngine::CullingGroupEvent {}
