#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerationSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct TextGenerationSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub screenRect: crate::UnityEngine::Rect,
    pub margins: crate::UnityEngine::Vector4,
    pub scale: f32,
    pub fontAsset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::FontAsset,
    >,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub spriteAsset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::SpriteAsset,
    >,
    pub styleSheet: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::TextStyleSheet,
    >,
    pub fontStyle: crate::UnityEngine::TextCore::Text::FontStyles,
    pub textSettings: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::TextSettings,
    >,
    pub textAlignment: crate::UnityEngine::TextCore::Text::TextAlignment,
    pub overflowMode: crate::UnityEngine::TextCore::Text::TextOverflowMode,
    pub wordWrap: bool,
    pub wordWrappingRatio: f32,
    pub color: crate::UnityEngine::Color,
    pub fontColorGradient: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::TextColorGradient,
    >,
    pub fontColorGradientPreset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::TextColorGradient,
    >,
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::TextGenerationSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "TextGenerationSettings";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerationSettings")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextGenerationSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerationSettings")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextGenerationSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerationSettings")]
impl crate::UnityEngine::TextCore::Text::TextGenerationSettings {
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Equals", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_TextGenerationSettings0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                        >),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Equals", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetHashCode", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
        right: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                            >,
                        ),
                        bool,
                        2usize,
                    >("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "op_Equality", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (left, right))? };
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
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerationSettings")]
impl AsRef<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    >,
> for crate::UnityEngine::TextCore::Text::TextGenerationSettings {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextGenerationSettings")]
impl AsMut<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    >,
> for crate::UnityEngine::TextCore::Text::TextGenerationSettings {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
