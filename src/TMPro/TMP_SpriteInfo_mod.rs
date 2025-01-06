#[cfg(feature = "TMPro+TMP_SpriteInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TMP_SpriteInfo {
    pub spriteIndex: i32,
    pub characterIndex: i32,
    pub vertexIndex: i32,
}
#[cfg(feature = "TMPro+TMP_SpriteInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_SpriteInfo => "TMPro"
    ."TMP_SpriteInfo"
);
#[cfg(feature = "TMPro+TMP_SpriteInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_SpriteInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_SpriteInfo")]
impl crate::TMPro::TMP_SpriteInfo {}
