#[cfg(feature = "TMPro+TMP_Settings")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_Settings {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_enableWordWrapping: bool,
    pub m_enableKerning: bool,
    pub m_enableExtraPadding: bool,
    pub m_enableTintAllSprites: bool,
    pub m_enableParseEscapeCharacters: bool,
    pub m_EnableRaycastTarget: bool,
    pub m_GetFontFeaturesAtRuntime: bool,
    pub m_missingGlyphCharacter: i32,
    pub m_warningsDisabled: bool,
    pub m_defaultFontAsset: *mut crate::TMPro::TMP_FontAsset,
    pub m_defaultFontAssetPath: *mut crate::System::String,
    pub m_defaultFontSize: f32,
    pub m_defaultAutoSizeMinRatio: f32,
    pub m_defaultAutoSizeMaxRatio: f32,
    pub m_defaultTextMeshProTextContainerSize: crate::UnityEngine::Vector2,
    pub m_defaultTextMeshProUITextContainerSize: crate::UnityEngine::Vector2,
    pub m_autoSizeTextContainer: bool,
    pub m_IsTextObjectScaleStatic: bool,
    pub m_fallbackFontAssets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::TMPro::TMP_FontAsset,
    >,
    pub m_matchMaterialPreset: bool,
    pub m_defaultSpriteAsset: *mut crate::TMPro::TMP_SpriteAsset,
    pub m_defaultSpriteAssetPath: *mut crate::System::String,
    pub m_enableEmojiSupport: bool,
    pub m_MissingCharacterSpriteUnicode: u32,
    pub m_defaultColorGradientPresetsPath: *mut crate::System::String,
    pub m_defaultStyleSheet: *mut crate::TMPro::TMP_StyleSheet,
    pub m_StyleSheetsResourcePath: *mut crate::System::String,
    pub m_leadingCharacters: *mut crate::UnityEngine::TextAsset,
    pub m_followingCharacters: *mut crate::UnityEngine::TextAsset,
    pub m_linebreakingRules: *mut crate::TMPro::TMP_Settings_LineBreakingTable,
    pub m_UseModernHangulLineBreakingRules: bool,
}
#[cfg(feature = "TMPro+TMP_Settings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Settings => "TMPro"."TMP_Settings"
);
#[cfg(feature = "TMPro+TMP_Settings")]
impl std::ops::Deref for crate::TMPro::TMP_Settings {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Settings")]
impl std::ops::DerefMut for crate::TMPro::TMP_Settings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Settings")]
impl crate::TMPro::TMP_Settings {
    #[cfg(feature = "TMPro+TMP_Settings+LineBreakingTable")]
    pub type LineBreakingTable = crate::TMPro::TMP_Settings_LineBreakingTable;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+TMP_Settings")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_Settings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_Settings+LineBreakingTable")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_Settings_LineBreakingTable {
    __cordl_parent: crate::System::Object,
    pub leadingCharacters: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        char,
    >,
    pub followingCharacters: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        char,
    >,
}
#[cfg(feature = "TMPro+TMP_Settings+LineBreakingTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Settings_LineBreakingTable => "TMPro"
    ."TMP_Settings/LineBreakingTable"
);
#[cfg(feature = "TMPro+TMP_Settings+LineBreakingTable")]
impl std::ops::Deref for crate::TMPro::TMP_Settings_LineBreakingTable {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Settings+LineBreakingTable")]
impl std::ops::DerefMut for crate::TMPro::TMP_Settings_LineBreakingTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Settings+LineBreakingTable")]
impl crate::TMPro::TMP_Settings_LineBreakingTable {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TMPro+TMP_Settings+LineBreakingTable")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_Settings_LineBreakingTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
