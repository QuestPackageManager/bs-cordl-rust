#[cfg(feature = "cordl_class_UnityEngine+TextCore+LowLevel+FontEngineError")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum FontEngineError {
    #[default]
    Atlas_Generation_Cancelled = 100i32,
    Invalid_Character_Code = 17i32,
    Invalid_Face = 35i32,
    Invalid_File = 4i32,
    Invalid_File_Format = 2i32,
    Invalid_File_Path = 1i32,
    Invalid_File_Structure = 3i32,
    Invalid_Glyph_Index = 16i32,
    Invalid_Library = 33i32,
    Invalid_Library_or_Face = 41i32,
    Invalid_Pixel_Size = 23i32,
    Invalid_SharedTextureData = 101i32,
    Invalid_Table = 8i32,
    OpenTypeLayoutLookup_Mismatch = 116i32,
    Success = 0i32,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+LowLevel+FontEngineError")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::TextCore::LowLevel::FontEngineError
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.LowLevel";
    const CLASS_NAME: &'static str = "FontEngineError";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+LowLevel+FontEngineError")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::TextCore::LowLevel::FontEngineError
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+LowLevel+FontEngineError")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::TextCore::LowLevel::FontEngineError
{
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
#[cfg(feature = "cordl_class_UnityEngine+TextCore+LowLevel+FontEngineError")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::TextCore::LowLevel::FontEngineError
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+LowLevel+FontEngineError")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::TextCore::LowLevel::FontEngineError
{
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
