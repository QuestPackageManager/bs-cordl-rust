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
    pub m_defaultFontAssetPath: *mut quest_hook::libil2cpp::Il2CppString,
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
    pub m_defaultSpriteAssetPath: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_enableEmojiSupport: bool,
    pub m_MissingCharacterSpriteUnicode: u32,
    pub m_defaultColorGradientPresetsPath: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_defaultStyleSheet: *mut crate::TMPro::TMP_StyleSheet,
    pub m_StyleSheetsResourcePath: *mut quest_hook::libil2cpp::Il2CppString,
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
    pub fn GetCharacters(
        file: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<i32, char>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<i32, char>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCharacters", (file))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFontAsset() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFontAsset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSettings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Settings>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Settings> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSpriteAsset() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSpriteAsset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStyleSheet() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_StyleSheet>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_StyleSheet> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetStyleSheet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadDefaultSettings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Settings>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Settings> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadDefaultSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadLinebreakingRules() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadLinebreakingRules", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_autoSizeTextContainer() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_autoSizeTextContainer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultColorGradientPresetsPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultColorGradientPresetsPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultFontAsset() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultFontAsset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultFontAssetPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultFontAssetPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultFontSize() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultFontSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultSpriteAsset() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultSpriteAsset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultSpriteAssetPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultSpriteAssetPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultStyleSheet() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_StyleSheet>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_StyleSheet> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultStyleSheet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultTextAutoSizingMaxRatio() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultTextAutoSizingMaxRatio", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultTextAutoSizingMinRatio() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultTextAutoSizingMinRatio", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultTextMeshProTextContainerSize() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Vector2,
    > {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultTextMeshProTextContainerSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultTextMeshProUITextContainerSize() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Vector2,
    > {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultTextMeshProUITextContainerSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableEmojiSupport() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_enableEmojiSupport", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableExtraPadding() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_enableExtraPadding", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableKerning() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_enableKerning", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableParseEscapeCharacters() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_enableParseEscapeCharacters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableRaycastTarget() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_enableRaycastTarget", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableTintAllSprites() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_enableTintAllSprites", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableWordWrapping() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_enableWordWrapping", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fallbackFontAssets() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::TMPro::TMP_FontAsset>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::TMPro::TMP_FontAsset>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fallbackFontAssets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_followingCharacters() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_followingCharacters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_getFontFeaturesAtRuntime() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_getFontFeaturesAtRuntime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Settings>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Settings> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_instance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isTextObjectScaleStatic() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isTextObjectScaleStatic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leadingCharacters() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_leadingCharacters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_linebreakingRules() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Settings_LineBreakingTable>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::TMPro::TMP_Settings_LineBreakingTable,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_linebreakingRules", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_matchMaterialPreset() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_matchMaterialPreset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_missingCharacterSpriteUnicode() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_missingCharacterSpriteUnicode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_missingGlyphCharacter() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_missingGlyphCharacter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_styleSheetsResourcePath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_styleSheetsResourcePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useModernHangulLineBreakingRules() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_useModernHangulLineBreakingRules", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_version() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_version", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_warningsDisabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_warningsDisabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enableEmojiSupport(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_enableEmojiSupport", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isTextObjectScaleStatic(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_isTextObjectScaleStatic", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_missingCharacterSpriteUnicode(
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_missingCharacterSpriteUnicode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_missingGlyphCharacter(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_missingGlyphCharacter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useModernHangulLineBreakingRules(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_useModernHangulLineBreakingRules", (value))?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
