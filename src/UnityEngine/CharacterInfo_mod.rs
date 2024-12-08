#[cfg(feature = "UnityEngine+CharacterInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CharacterInfo {
    pub index: i32,
    pub uv: crate::UnityEngine::Rect,
    pub vert: crate::UnityEngine::Rect,
    pub width: f32,
    pub _cordl_size: i32,
    pub style: crate::UnityEngine::FontStyle,
    pub flipped: bool,
}
#[cfg(feature = "UnityEngine+CharacterInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CharacterInfo => "UnityEngine"
    ."CharacterInfo"
);
#[cfg(feature = "UnityEngine+CharacterInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::CharacterInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+CharacterInfo")]
impl crate::UnityEngine::CharacterInfo {}
