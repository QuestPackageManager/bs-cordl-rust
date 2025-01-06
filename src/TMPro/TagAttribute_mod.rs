#[cfg(feature = "TMPro+TagAttribute")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TagAttribute {
    pub startIndex: i32,
    pub length: i32,
    pub hashCode: i32,
}
#[cfg(feature = "TMPro+TagAttribute")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TagAttribute => "TMPro"."TagAttribute"
);
#[cfg(feature = "TMPro+TagAttribute")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TagAttribute {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TagAttribute")]
impl crate::TMPro::TagAttribute {}
