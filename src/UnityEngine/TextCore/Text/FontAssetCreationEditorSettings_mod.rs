#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetCreationEditorSettings")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FontAssetCreationEditorSettings {
    pub sourceFontFileGUID: *mut quest_hook::libil2cpp::Il2CppString,
    pub faceIndex: i32,
    pub pointSizeSamplingMode: i32,
    pub pointSize: i32,
    pub padding: i32,
    pub paddingMode: i32,
    pub packingMode: i32,
    pub atlasWidth: i32,
    pub atlasHeight: i32,
    pub characterSetSelectionMode: i32,
    pub characterSequence: *mut quest_hook::libil2cpp::Il2CppString,
    pub referencedFontAssetGUID: *mut quest_hook::libil2cpp::Il2CppString,
    pub referencedTextAssetGUID: *mut quest_hook::libil2cpp::Il2CppString,
    pub fontStyle: i32,
    pub fontStyleModifier: f32,
    pub renderMode: i32,
    pub includeFontFeatures: bool,
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetCreationEditorSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::Text::FontAssetCreationEditorSettings =>
    "UnityEngine.TextCore.Text"."FontAssetCreationEditorSettings"
);
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetCreationEditorSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::FontAssetCreationEditorSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetCreationEditorSettings")]
impl crate::UnityEngine::TextCore::Text::FontAssetCreationEditorSettings {}
