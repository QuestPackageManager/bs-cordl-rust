#[cfg(feature = "UnityEngine+Android+Permission")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Permission {}
#[cfg(feature = "UnityEngine+Android+Permission")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Android::Permission =>
    "UnityEngine.Android"."Permission"
);
#[cfg(feature = "UnityEngine+Android+Permission")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Android::Permission {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Android+Permission")]
impl crate::UnityEngine::Android::Permission {}