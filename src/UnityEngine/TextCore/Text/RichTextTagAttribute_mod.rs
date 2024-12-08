#[cfg(feature = "UnityEngine+TextCore+Text+RichTextTagAttribute")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RichTextTagAttribute {
    pub nameHashCode: i32,
    pub valueHashCode: i32,
    pub valueType: crate::UnityEngine::TextCore::Text::TagValueType,
    pub valueStartIndex: i32,
    pub valueLength: i32,
    pub unitType: crate::UnityEngine::TextCore::Text::TagUnitType,
}
#[cfg(feature = "UnityEngine+TextCore+Text+RichTextTagAttribute")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::Text::RichTextTagAttribute => "UnityEngine.TextCore.Text"
    ."RichTextTagAttribute"
);
#[cfg(feature = "UnityEngine+TextCore+Text+RichTextTagAttribute")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::RichTextTagAttribute {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+RichTextTagAttribute")]
impl crate::UnityEngine::TextCore::Text::RichTextTagAttribute {}
