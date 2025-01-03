#[cfg(feature = "UnityEngine+TextCore+Text+PageInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PageInfo {
    pub firstCharacterIndex: i32,
    pub lastCharacterIndex: i32,
    pub ascender: f32,
    pub baseLine: f32,
    pub descender: f32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+PageInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::PageInfo =>
    "UnityEngine.TextCore.Text"."PageInfo"
);
#[cfg(feature = "UnityEngine+TextCore+Text+PageInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::PageInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+PageInfo")]
impl crate::UnityEngine::TextCore::Text::PageInfo {}
