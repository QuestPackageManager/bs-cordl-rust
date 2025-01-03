#[cfg(feature = "UnityEngineInternal+MathfInternal")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct MathfInternal {}
#[cfg(feature = "UnityEngineInternal+MathfInternal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngineInternal::MathfInternal =>
    "UnityEngineInternal"."MathfInternal"
);
#[cfg(feature = "UnityEngineInternal+MathfInternal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngineInternal::MathfInternal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngineInternal+MathfInternal")]
impl crate::UnityEngineInternal::MathfInternal {}
