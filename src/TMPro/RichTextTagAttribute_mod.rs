#[cfg(feature = "TMPro+RichTextTagAttribute")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RichTextTagAttribute {
    pub nameHashCode: i32,
    pub valueHashCode: i32,
    pub valueType: crate::TMPro::TagValueType,
    pub valueStartIndex: i32,
    pub valueLength: i32,
    pub unitType: crate::TMPro::TagUnitType,
}
#[cfg(feature = "TMPro+RichTextTagAttribute")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::RichTextTagAttribute => "TMPro"
    ."RichTextTagAttribute"
);
#[cfg(feature = "TMPro+RichTextTagAttribute")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::RichTextTagAttribute {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+RichTextTagAttribute")]
impl crate::TMPro::RichTextTagAttribute {}
