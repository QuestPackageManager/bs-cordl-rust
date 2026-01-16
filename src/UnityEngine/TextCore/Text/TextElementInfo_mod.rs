#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextElementInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextElementInfo {
    pub character: u32,
    pub index: i32,
    pub elementType: crate::UnityEngine::TextCore::Text::TextElementType,
    pub stringLength: i32,
    pub textElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::TextElement,
    >,
    pub alternativeGlyph: quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
    pub fontAsset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::FontAsset,
    >,
    pub spriteAsset: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::SpriteAsset,
    >,
    pub spriteIndex: i32,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub materialReferenceIndex: i32,
    pub isUsingAlternateTypeface: bool,
    pub pointSize: f32,
    pub lineNumber: i32,
    pub pageNumber: i32,
    pub vertexIndex: i32,
    pub vertexTopLeft: crate::UnityEngine::TextCore::Text::TextVertex,
    pub vertexBottomLeft: crate::UnityEngine::TextCore::Text::TextVertex,
    pub vertexTopRight: crate::UnityEngine::TextCore::Text::TextVertex,
    pub vertexBottomRight: crate::UnityEngine::TextCore::Text::TextVertex,
    pub topLeft: crate::UnityEngine::Vector3,
    pub bottomLeft: crate::UnityEngine::Vector3,
    pub topRight: crate::UnityEngine::Vector3,
    pub bottomRight: crate::UnityEngine::Vector3,
    pub origin: f32,
    pub ascender: f32,
    pub baseLine: f32,
    pub descender: f32,
    pub adjustedAscender: f32,
    pub adjustedDescender: f32,
    pub adjustedHorizontalAdvance: f32,
    pub xAdvance: f32,
    pub aspectRatio: f32,
    pub scale: f32,
    pub color: crate::UnityEngine::Color32,
    pub underlineColor: crate::UnityEngine::Color32,
    pub underlineVertexIndex: i32,
    pub strikethroughColor: crate::UnityEngine::Color32,
    pub strikethroughVertexIndex: i32,
    pub highlightColor: crate::UnityEngine::Color32,
    pub highlightState: crate::UnityEngine::TextCore::Text::HighlightState,
    pub style: crate::UnityEngine::TextCore::Text::FontStyles,
    pub isVisible: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextElementInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::TextElementInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "TextElementInfo";
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextElementInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::TextCore::Text::TextElementInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextElementInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::TextCore::Text::TextElementInfo {
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextElementInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::TextCore::Text::TextElementInfo {
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextElementInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::TextCore::Text::TextElementInfo {
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+Text+TextElementInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::TextElementInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextElementInfo")]
impl crate::UnityEngine::TextCore::Text::TextElementInfo {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
