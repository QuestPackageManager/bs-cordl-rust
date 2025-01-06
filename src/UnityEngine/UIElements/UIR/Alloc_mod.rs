#[cfg(feature = "UnityEngine+UIElements+UIR+Alloc")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Alloc {
    pub start: u32,
    pub _cordl_size: u32,
    pub handle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub shortLived: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Alloc")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::Alloc =>
    "UnityEngine.UIElements.UIR"."Alloc"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Alloc")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::Alloc {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Alloc")]
impl crate::UnityEngine::UIElements::UIR::Alloc {}
