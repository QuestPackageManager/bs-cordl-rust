#[cfg(feature = "UnityEngine+SortingLayer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SortingLayer {
    pub m_Id: i32,
}
#[cfg(feature = "UnityEngine+SortingLayer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SortingLayer => "UnityEngine"
    ."SortingLayer"
);
#[cfg(feature = "UnityEngine+SortingLayer")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::SortingLayer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+SortingLayer")]
impl crate::UnityEngine::SortingLayer {}
