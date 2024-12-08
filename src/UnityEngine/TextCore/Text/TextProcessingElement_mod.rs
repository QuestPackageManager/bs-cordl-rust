#[cfg(feature = "UnityEngine+TextCore+Text+TextProcessingElement")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextProcessingElement {
    pub elementType: crate::UnityEngine::TextCore::Text::TextProcessingElementType,
    pub unicode: u32,
    pub stringIndex: i32,
    pub length: i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextProcessingElement")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::Text::TextProcessingElement => "UnityEngine.TextCore.Text"
    ."TextProcessingElement"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextProcessingElement")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::TextProcessingElement {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextProcessingElement")]
impl crate::UnityEngine::TextCore::Text::TextProcessingElement {}
