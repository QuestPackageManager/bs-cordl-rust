#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetCreationEditorSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FontAssetCreationEditorSettings {
    pub sourceFontFileGUID: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub faceIndex: i32,
    pub pointSizeSamplingMode: i32,
    pub pointSize: i32,
    pub padding: i32,
    pub paddingMode: i32,
    pub packingMode: i32,
    pub atlasWidth: i32,
    pub atlasHeight: i32,
    pub characterSetSelectionMode: i32,
    pub characterSequence: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub referencedFontAssetGUID: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub referencedTextAssetGUID: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub fontStyle: i32,
    pub fontStyleModifier: f32,
    pub renderMode: i32,
    pub includeFontFeatures: bool,
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetCreationEditorSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::FontAssetCreationEditorSettings {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "FontAssetCreationEditorSettings";
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
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetCreationEditorSettings")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::TextCore::Text::FontAssetCreationEditorSettings {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetCreationEditorSettings")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::TextCore::Text::FontAssetCreationEditorSettings {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetCreationEditorSettings")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::TextCore::Text::FontAssetCreationEditorSettings {
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
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetCreationEditorSettings")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::TextCore::Text::FontAssetCreationEditorSettings {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
