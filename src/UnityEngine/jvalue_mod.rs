#[cfg(feature = "UnityEngine+jvalue")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct jvalue {
    padding: quest_hook::libil2cpp::ValueTypePadding<8usize>,
}
#[cfg(feature = "UnityEngine+jvalue")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::jvalue => "UnityEngine"."jvalue"
);
#[cfg(feature = "UnityEngine+jvalue")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::jvalue {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+jvalue")]
impl crate::UnityEngine::jvalue {}
