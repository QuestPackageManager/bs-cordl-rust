#[cfg(feature = "UnityEngine+Mathf")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Mathf {}
#[cfg(feature = "UnityEngine+Mathf")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Mathf => "UnityEngine"."Mathf"
);
#[cfg(feature = "UnityEngine+Mathf")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Mathf {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Mathf")]
impl crate::UnityEngine::Mathf {}
