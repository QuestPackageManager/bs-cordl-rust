#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+Cookie")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Cookie {
    pub instanceID: i32,
    pub scale: f32,
    pub sizes: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+Cookie")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::GlobalIllumination::Cookie =>
    "UnityEngine.Experimental.GlobalIllumination"."Cookie"
);
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+Cookie")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Experimental::GlobalIllumination::Cookie {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+Cookie")]
impl crate::UnityEngine::Experimental::GlobalIllumination::Cookie {}