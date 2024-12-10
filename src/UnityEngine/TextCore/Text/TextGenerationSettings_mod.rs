#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerationSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct TextGenerationSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub text: *mut quest_hook::libil2cpp::Il2CppString,
    pub screenRect: crate::UnityEngine::Rect,
    pub margins: crate::UnityEngine::Vector4,
    pub scale: f32,
    pub fontAsset: *mut crate::UnityEngine::TextCore::Text::FontAsset,
    pub material: *mut crate::UnityEngine::Material,
    pub spriteAsset: *mut crate::UnityEngine::TextCore::Text::SpriteAsset,
    pub styleSheet: *mut crate::UnityEngine::TextCore::Text::TextStyleSheet,
    pub fontStyle: crate::UnityEngine::TextCore::Text::FontStyles,
    pub textSettings: *mut crate::UnityEngine::TextCore::Text::TextSettings,
    pub textAlignment: crate::UnityEngine::TextCore::Text::TextAlignment,
    pub overflowMode: crate::UnityEngine::TextCore::Text::TextOverflowMode,
    pub wordWrap: bool,
    pub wordWrappingRatio: f32,
    pub color: crate::UnityEngine::Color,
    pub fontColorGradient: *mut crate::UnityEngine::TextCore::Text::TextColorGradient,
    pub fontColorGradientPreset: *mut crate::UnityEngine::TextCore::Text::TextColorGradient,
    pub tintSprites: bool,
    pub overrideRichTextColors: bool,
    pub shouldConvertToLinearSpace: bool,
    pub fontSize: f32,
    pub autoSize: bool,
    pub fontSizeMin: f32,
    pub fontSizeMax: f32,
    pub enableKerning: bool,
    pub richText: bool,
    pub isRightToLeft: bool,
    pub extraPadding: f32,
    pub parseControlCharacters: bool,
    pub isOrthographic: bool,
    pub tagNoParsing: bool,
    pub characterSpacing: f32,
    pub wordSpacing: f32,
    pub lineSpacing: f32,
    pub paragraphSpacing: f32,
    pub lineSpacingMax: f32,
    pub textWrappingMode: crate::UnityEngine::TextCore::Text::TextWrappingMode,
    pub maxVisibleCharacters: i32,
    pub maxVisibleWords: i32,
    pub maxVisibleLines: i32,
    pub firstVisibleCharacter: i32,
    pub useMaxVisibleDescender: bool,
    pub fontWeight: crate::UnityEngine::TextCore::Text::TextFontWeight,
    pub pageToDisplay: i32,
    pub horizontalMapping: crate::UnityEngine::TextCore::Text::TextureMapping,
    pub verticalMapping: crate::UnityEngine::TextCore::Text::TextureMapping,
    pub uvLineOffset: f32,
    pub geometrySortingOrder: crate::UnityEngine::TextCore::Text::VertexSortingOrder,
    pub inverseYAxis: bool,
    pub charWidthMaxAdj: f32,
    pub inputSource: crate::UnityEngine::TextCore::Text::TextInputSource,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerationSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::Text::TextGenerationSettings => "UnityEngine.TextCore.Text"
    ."TextGenerationSettings"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerationSettings")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextGenerationSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerationSettings")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextGenerationSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerationSettings")]
impl crate::UnityEngine::TextCore::Text::TextGenerationSettings {
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_TextGenerationSettings0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerationSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::TextGenerationSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
