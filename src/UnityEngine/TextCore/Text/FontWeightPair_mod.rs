#[cfg(feature = "UnityEngine+TextCore+Text+FontWeightPair")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FontWeightPair {
    pub regularTypeface: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::FontAsset,
    >,
    pub italicTypeface: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::FontAsset,
    >,
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontWeightPair")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::FontWeightPair =>
    "UnityEngine.TextCore.Text"."FontWeightPair"
);
#[cfg(feature = "UnityEngine+TextCore+Text+FontWeightPair")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::FontWeightPair {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontWeightPair")]
impl crate::UnityEngine::TextCore::Text::FontWeightPair {}
