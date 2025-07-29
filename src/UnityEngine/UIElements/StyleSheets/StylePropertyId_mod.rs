#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StylePropertyId")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StylePropertyId {
    #[default]
    AlignContent = 131072i32,
    AlignItems = 131073i32,
    AlignSelf = 131074i32,
    All = 262144i32,
    BackgroundColor = 458752i32,
    BackgroundImage = 458753i32,
    BackgroundPosition = 262145i32,
    BackgroundPositionX = 458754i32,
    BackgroundPositionY = 458755i32,
    BackgroundRepeat = 458756i32,
    BackgroundSize = 458757i32,
    BorderBottomColor = 458758i32,
    BorderBottomLeftRadius = 458759i32,
    BorderBottomRightRadius = 458760i32,
    BorderBottomWidth = 131075i32,
    BorderColor = 262146i32,
    BorderLeftColor = 458761i32,
    BorderLeftWidth = 131076i32,
    BorderRadius = 262147i32,
    BorderRightColor = 458762i32,
    BorderRightWidth = 131077i32,
    BorderTopColor = 458763i32,
    BorderTopLeftRadius = 458764i32,
    BorderTopRightRadius = 458765i32,
    BorderTopWidth = 131078i32,
    BorderWidth = 262148i32,
    Bottom = 131079i32,
    Color = 65536i32,
    Cursor = 196608i32,
    Custom = -1i32,
    Display = 131080i32,
    Flex = 262149i32,
    FlexBasis = 131081i32,
    FlexDirection = 131082i32,
    FlexGrow = 131083i32,
    FlexShrink = 131084i32,
    FlexWrap = 131085i32,
    FontSize = 65537i32,
    Height = 131086i32,
    JustifyContent = 131087i32,
    Left = 131088i32,
    LetterSpacing = 65538i32,
    Margin = 262150i32,
    MarginBottom = 131089i32,
    MarginLeft = 131090i32,
    MarginRight = 131091i32,
    MarginTop = 131092i32,
    MaxHeight = 131093i32,
    MaxWidth = 131094i32,
    MinHeight = 131095i32,
    MinWidth = 131096i32,
    Opacity = 458766i32,
    Overflow = 458767i32,
    Padding = 262151i32,
    PaddingBottom = 131097i32,
    PaddingLeft = 131098i32,
    PaddingRight = 131099i32,
    PaddingTop = 131100i32,
    Position = 131101i32,
    Right = 131102i32,
    Rotate = 327680i32,
    Scale = 327681i32,
    TextOverflow = 196609i32,
    TextShadow = 65539i32,
    Top = 131103i32,
    TransformOrigin = 327682i32,
    Transition = 262152i32,
    TransitionDelay = 393216i32,
    TransitionDuration = 393217i32,
    TransitionProperty = 393218i32,
    TransitionTimingFunction = 393219i32,
    Translate = 327683i32,
    UnityBackgroundImageTintColor = 196610i32,
    UnityBackgroundScaleMode = 262153i32,
    UnityFont = 65540i32,
    UnityFontDefinition = 65541i32,
    UnityFontStyleAndWeight = 65542i32,
    UnityOverflowClipBox = 196611i32,
    UnityParagraphSpacing = 65543i32,
    UnitySliceBottom = 196612i32,
    UnitySliceLeft = 196613i32,
    UnitySliceRight = 196614i32,
    UnitySliceScale = 196615i32,
    UnitySliceTop = 196616i32,
    UnityTextAlign = 65544i32,
    UnityTextOutline = 262154i32,
    UnityTextOutlineColor = 65545i32,
    UnityTextOutlineWidth = 65546i32,
    UnityTextOverflowPosition = 196617i32,
    Unknown = 0i32,
    Visibility = 65547i32,
    WhiteSpace = 65548i32,
    Width = 131104i32,
    WordSpacing = 65549i32,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StylePropertyId")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyId {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.StyleSheets";
    const CLASS_NAME: &'static str = "StylePropertyId";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StylePropertyId")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyId {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StylePropertyId")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyId {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StylePropertyId")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyId {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+StyleSheets+StylePropertyId")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::StyleSheets::StylePropertyId {
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
