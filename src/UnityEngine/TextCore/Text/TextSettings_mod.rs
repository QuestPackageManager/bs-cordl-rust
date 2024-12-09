#[cfg(feature = "UnityEngine+TextCore+Text+TextSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct TextSettings {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_Version: *mut crate::System::String,
    pub m_DefaultFontAsset: *mut crate::UnityEngine::TextCore::Text::FontAsset,
    pub m_DefaultFontAssetPath: *mut crate::System::String,
    pub m_FallbackFontAssets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::TextCore::Text::FontAsset,
    >,
    pub m_MatchMaterialPreset: bool,
    pub m_MissingCharacterUnicode: i32,
    pub m_ClearDynamicDataOnBuild: bool,
    pub m_DefaultSpriteAsset: *mut crate::UnityEngine::TextCore::Text::SpriteAsset,
    pub m_DefaultSpriteAssetPath: *mut crate::System::String,
    pub m_FallbackSpriteAssets: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::TextCore::Text::SpriteAsset,
    >,
    pub m_MissingSpriteCharacterUnicode: u32,
    pub m_DefaultStyleSheet: *mut crate::UnityEngine::TextCore::Text::TextStyleSheet,
    pub m_StyleSheetsResourcePath: *mut crate::System::String,
    pub m_DefaultColorGradientPresetsPath: *mut crate::System::String,
    pub m_UnicodeLineBreakingRules: *mut crate::UnityEngine::TextCore::Text::UnicodeLineBreakingRules,
    pub m_UseModernHangulLineBreakingRules: bool,
    pub m_DisplayWarnings: bool,
    pub m_FontLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::UnityEngine::TextCore::Text::FontAsset,
    >,
    pub m_FontReferences: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::TextCore::Text::TextSettings_FontReferenceMap,
    >,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextSettings =>
    "UnityEngine.TextCore.Text"."TextSettings"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextSettings")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextSettings {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextSettings")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextSettings")]
impl crate::UnityEngine::TextCore::Text::TextSettings {
    #[cfg(feature = "UnityEngine+TextCore+Text+TextSettings+FontReferenceMap")]
    pub type FontReferenceMap = crate::UnityEngine::TextCore::Text::TextSettings_FontReferenceMap;
    pub fn GetCachedFontAssetInternal(
        &mut self,
        font: *mut crate::UnityEngine::Font,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::TextCore::Text::FontAsset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextCore::Text::FontAsset = __cordl_object
            .invoke("GetCachedFontAssetInternal", (font))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeFontReferenceLookup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeFontReferenceLookup", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
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
    pub fn get_clearDynamicDataOnBuild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_clearDynamicDataOnBuild", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultColorGradientPresetsPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_defaultColorGradientPresetsPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultFontAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::TextCore::Text::FontAsset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextCore::Text::FontAsset = __cordl_object
            .invoke("get_defaultFontAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultFontAssetPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_defaultFontAssetPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultSpriteAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::TextCore::Text::SpriteAsset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextCore::Text::SpriteAsset = __cordl_object
            .invoke("get_defaultSpriteAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultSpriteAssetPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_defaultSpriteAssetPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultStyleSheet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::TextCore::Text::TextStyleSheet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextCore::Text::TextStyleSheet = __cordl_object
            .invoke("get_defaultStyleSheet", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_displayWarnings(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_displayWarnings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fallbackFontAssets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::FontAsset,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::FontAsset,
        > = __cordl_object.invoke("get_fallbackFontAssets", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fallbackSpriteAssets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::SpriteAsset,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::SpriteAsset,
        > = __cordl_object.invoke("get_fallbackSpriteAssets", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lineBreakingRules(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::TextCore::Text::UnicodeLineBreakingRules,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextCore::Text::UnicodeLineBreakingRules = __cordl_object
            .invoke("get_lineBreakingRules", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_matchMaterialPreset(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_matchMaterialPreset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_missingCharacterUnicode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_missingCharacterUnicode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_missingSpriteCharacterUnicode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("get_missingSpriteCharacterUnicode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_styleSheetsResourcePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_styleSheetsResourcePath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_useModernHangulLineBreakingRules(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_useModernHangulLineBreakingRules", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_version", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_clearDynamicDataOnBuild(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clearDynamicDataOnBuild", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_defaultColorGradientPresetsPath(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultColorGradientPresetsPath", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_defaultFontAsset(
        &mut self,
        value: *mut crate::UnityEngine::TextCore::Text::FontAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultFontAsset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_defaultFontAssetPath(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultFontAssetPath", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_defaultSpriteAsset(
        &mut self,
        value: *mut crate::UnityEngine::TextCore::Text::SpriteAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultSpriteAsset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_defaultSpriteAssetPath(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultSpriteAssetPath", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_defaultStyleSheet(
        &mut self,
        value: *mut crate::UnityEngine::TextCore::Text::TextStyleSheet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultStyleSheet", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_displayWarnings(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_displayWarnings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fallbackFontAssets(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::FontAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fallbackFontAssets", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fallbackSpriteAssets(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::TextCore::Text::SpriteAsset,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fallbackSpriteAssets", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_lineBreakingRules(
        &mut self,
        value: *mut crate::UnityEngine::TextCore::Text::UnicodeLineBreakingRules,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lineBreakingRules", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_matchMaterialPreset(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_matchMaterialPreset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_missingCharacterUnicode(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_missingCharacterUnicode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_missingSpriteCharacterUnicode(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_missingSpriteCharacterUnicode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_styleSheetsResourcePath(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_styleSheetsResourcePath", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_useModernHangulLineBreakingRules(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useModernHangulLineBreakingRules", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_version(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_version", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::TextSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextSettings+FontReferenceMap")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextSettings_FontReferenceMap {
    pub font: *mut crate::UnityEngine::Font,
    pub fontAsset: *mut crate::UnityEngine::TextCore::Text::FontAsset,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextSettings+FontReferenceMap")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::Text::TextSettings_FontReferenceMap =>
    "UnityEngine.TextCore.Text"."TextSettings/FontReferenceMap"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextSettings+FontReferenceMap")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::TextSettings_FontReferenceMap {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextSettings+FontReferenceMap")]
impl crate::UnityEngine::TextCore::Text::TextSettings_FontReferenceMap {
    pub fn _ctor(
        &mut self,
        font: *mut crate::UnityEngine::Font,
        fontAsset: *mut crate::UnityEngine::TextCore::Text::FontAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (font, fontAsset),
        )?;
        Ok(__cordl_ret)
    }
}
