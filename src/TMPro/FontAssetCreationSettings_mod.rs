#[cfg(feature = "TMPro+FontAssetCreationSettings")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FontAssetCreationSettings {
    pub sourceFontFileName: *mut quest_hook::libil2cpp::Il2CppString,
    pub sourceFontFileGUID: *mut quest_hook::libil2cpp::Il2CppString,
    pub pointSizeSamplingMode: i32,
    pub pointSize: i32,
    pub padding: i32,
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
#[cfg(feature = "TMPro+FontAssetCreationSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::FontAssetCreationSettings => "TMPro"
    ."FontAssetCreationSettings"
);
#[cfg(feature = "TMPro+FontAssetCreationSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::TMPro::FontAssetCreationSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+FontAssetCreationSettings")]
impl crate::TMPro::FontAssetCreationSettings {
    pub fn _ctor(
        &mut self,
        sourceFontFileGUID: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        pointSize: i32,
        pointSizeSamplingMode: i32,
        padding: i32,
        packingMode: i32,
        atlasWidth: i32,
        atlasHeight: i32,
        characterSelectionMode: i32,
        characterSet: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        renderMode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                sourceFontFileGUID,
                pointSize,
                pointSizeSamplingMode,
                padding,
                packingMode,
                atlasWidth,
                atlasHeight,
                characterSelectionMode,
                characterSet,
                renderMode,
            ),
        )?;
        Ok(__cordl_ret.into())
    }
}
