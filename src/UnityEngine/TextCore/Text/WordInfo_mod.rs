#[cfg(feature = "UnityEngine+TextCore+Text+WordInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WordInfo {
    pub firstCharacterIndex: i32,
    pub lastCharacterIndex: i32,
    pub characterCount: i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+WordInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::WordInfo =>
    "UnityEngine.TextCore.Text"."WordInfo"
);
#[cfg(feature = "UnityEngine+TextCore+Text+WordInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::WordInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+WordInfo")]
impl crate::UnityEngine::TextCore::Text::WordInfo {}
