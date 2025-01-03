#[cfg(feature = "TMPro+TMP_LineInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TMP_LineInfo {
    pub controlCharacterCount: i32,
    pub characterCount: i32,
    pub visibleCharacterCount: i32,
    pub spaceCount: i32,
    pub wordCount: i32,
    pub firstCharacterIndex: i32,
    pub firstVisibleCharacterIndex: i32,
    pub lastCharacterIndex: i32,
    pub lastVisibleCharacterIndex: i32,
    pub length: f32,
    pub lineHeight: f32,
    pub ascender: f32,
    pub baseline: f32,
    pub descender: f32,
    pub maxAdvance: f32,
    pub width: f32,
    pub marginLeft: f32,
    pub marginRight: f32,
    pub alignment: crate::TMPro::HorizontalAlignmentOptions,
    pub lineExtents: crate::TMPro::Extents,
}
#[cfg(feature = "TMPro+TMP_LineInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_LineInfo => "TMPro"."TMP_LineInfo"
);
#[cfg(feature = "TMPro+TMP_LineInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_LineInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_LineInfo")]
impl crate::TMPro::TMP_LineInfo {}
