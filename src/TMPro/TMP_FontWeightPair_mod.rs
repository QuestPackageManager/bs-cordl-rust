#[cfg(feature = "TMPro+TMP_FontWeightPair")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TMP_FontWeightPair {
    pub regularTypeface: *mut crate::TMPro::TMP_FontAsset,
    pub italicTypeface: *mut crate::TMPro::TMP_FontAsset,
}
#[cfg(feature = "TMPro+TMP_FontWeightPair")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_FontWeightPair => "TMPro"
    ."TMP_FontWeightPair"
);
#[cfg(feature = "TMPro+TMP_FontWeightPair")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_FontWeightPair {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_FontWeightPair")]
impl crate::TMPro::TMP_FontWeightPair {}
