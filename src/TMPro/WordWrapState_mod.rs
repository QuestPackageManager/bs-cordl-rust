#[cfg(feature = "cordl_class_TMPro+WordWrapState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WordWrapState {
    pub previous_WordBreak: i32,
    pub total_CharacterCount: i32,
    pub visible_CharacterCount: i32,
    pub visibleSpaceCount: i32,
    pub visible_SpriteCount: i32,
    pub visible_LinkCount: i32,
    pub firstCharacterIndex: i32,
    pub firstVisibleCharacterIndex: i32,
    pub lastCharacterIndex: i32,
    pub lastVisibleCharIndex: i32,
    pub lineNumber: i32,
    pub maxCapHeight: f32,
    pub maxAscender: f32,
    pub maxDescender: f32,
    pub startOfLineAscender: f32,
    pub maxLineAscender: f32,
    pub maxLineDescender: f32,
    pub pageAscender: f32,
    pub horizontalAlignment: crate::TMPro::HorizontalAlignmentOptions,
    pub marginLeft: f32,
    pub marginRight: f32,
    pub xAdvance: f32,
    pub preferredWidth: f32,
    pub preferredHeight: f32,
    pub renderedWidth: f32,
    pub renderedHeight: f32,
    pub previousLineScale: f32,
    pub wordCount: i32,
    pub fontStyle: crate::TMPro::FontStyles,
    pub italicAngle: i32,
    pub fontScaleMultiplier: f32,
    pub currentFontSize: f32,
    pub baselineOffset: f32,
    pub lineOffset: f32,
    pub isDrivenLineSpacing: bool,
    pub lastBaseGlyphIndex: i32,
    pub cSpace: f32,
    pub mSpace: f32,
    pub textInfo: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_TextInfo>,
    pub lineInfo: crate::TMPro::TMP_LineInfo,
    pub vertexColor: crate::UnityEngine::Color32,
    pub underlineColor: crate::UnityEngine::Color32,
    pub strikethroughColor: crate::UnityEngine::Color32,
    pub highlightState: crate::TMPro::HighlightState,
    pub basicStyleStack: crate::TMPro::TMP_FontStyleStack,
    pub italicAngleStack: crate::TMPro::TMP_TextProcessingStack_1<i32>,
    pub colorStack: crate::TMPro::TMP_TextProcessingStack_1<crate::UnityEngine::Color32>,
    pub underlineColorStack: crate::TMPro::TMP_TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub strikethroughColorStack: crate::TMPro::TMP_TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub highlightColorStack: crate::TMPro::TMP_TextProcessingStack_1<
        crate::UnityEngine::Color32,
    >,
    pub highlightStateStack: crate::TMPro::TMP_TextProcessingStack_1<
        crate::TMPro::HighlightState,
    >,
    pub colorGradientStack: crate::TMPro::TMP_TextProcessingStack_1<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_ColorGradient>,
    >,
    pub sizeStack: crate::TMPro::TMP_TextProcessingStack_1<f32>,
    pub indentStack: crate::TMPro::TMP_TextProcessingStack_1<f32>,
    pub fontWeightStack: crate::TMPro::TMP_TextProcessingStack_1<
        crate::TMPro::FontWeight,
    >,
    pub styleStack: crate::TMPro::TMP_TextProcessingStack_1<i32>,
    pub baselineStack: crate::TMPro::TMP_TextProcessingStack_1<f32>,
    pub actionStack: crate::TMPro::TMP_TextProcessingStack_1<i32>,
    pub materialReferenceStack: crate::TMPro::TMP_TextProcessingStack_1<
        crate::TMPro::MaterialReference,
    >,
    pub lineJustificationStack: crate::TMPro::TMP_TextProcessingStack_1<
        crate::TMPro::HorizontalAlignmentOptions,
    >,
    pub spriteAnimationID: i32,
    pub currentFontAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    pub currentSpriteAsset: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_SpriteAsset>,
    pub currentMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub currentMaterialIndex: i32,
    pub meshExtents: crate::TMPro::Extents,
    pub tagNoParsing: bool,
    pub isNonBreakingSpace: bool,
    pub fxRotation: crate::UnityEngine::Quaternion,
    pub fxScale: crate::UnityEngine::Vector3,
}
#[cfg(feature = "cordl_class_TMPro+WordWrapState")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::WordWrapState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "WordWrapState";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_TMPro+WordWrapState")]
unsafe impl quest_hook::libil2cpp::Argument for crate::TMPro::WordWrapState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_TMPro+WordWrapState")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::TMPro::WordWrapState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_TMPro+WordWrapState")]
unsafe impl quest_hook::libil2cpp::Returned for crate::TMPro::WordWrapState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_TMPro+WordWrapState")]
unsafe impl quest_hook::libil2cpp::Return for crate::TMPro::WordWrapState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_TMPro+WordWrapState")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::WordWrapState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+WordWrapState")]
impl crate::TMPro::WordWrapState {}
