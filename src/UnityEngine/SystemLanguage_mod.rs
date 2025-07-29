#[cfg(feature = "cordl_class_UnityEngine+SystemLanguage")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemLanguage {
    #[default]
    Afrikaans = 0i32,
    Arabic = 1i32,
    Basque = 2i32,
    Belarusian = 3i32,
    Bulgarian = 4i32,
    Catalan = 5i32,
    Chinese = 6i32,
    ChineseSimplified = 40i32,
    ChineseTraditional = 41i32,
    Czech = 7i32,
    Danish = 8i32,
    Dutch = 9i32,
    English = 10i32,
    Estonian = 11i32,
    Faroese = 12i32,
    Finnish = 13i32,
    French = 14i32,
    German = 15i32,
    Greek = 16i32,
    Hebrew = 17i32,
    Hindi = 42i32,
    Hungarian = 18i32,
    Icelandic = 19i32,
    Indonesian = 20i32,
    Italian = 21i32,
    Japanese = 22i32,
    Korean = 23i32,
    Latvian = 24i32,
    Lithuanian = 25i32,
    Norwegian = 26i32,
    Polish = 27i32,
    Portuguese = 28i32,
    Romanian = 29i32,
    Russian = 30i32,
    SerboCroatian = 31i32,
    Slovak = 32i32,
    Slovenian = 33i32,
    Spanish = 34i32,
    Swedish = 35i32,
    Thai = 36i32,
    Turkish = 37i32,
    Ukrainian = 38i32,
    Unknown = 43i32,
    Vietnamese = 39i32,
}
#[cfg(feature = "cordl_class_UnityEngine+SystemLanguage")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::SystemLanguage {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "SystemLanguage";
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
#[cfg(feature = "cordl_class_UnityEngine+SystemLanguage")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::SystemLanguage {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+SystemLanguage")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::SystemLanguage {
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
#[cfg(feature = "cordl_class_UnityEngine+SystemLanguage")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::SystemLanguage {
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
#[cfg(feature = "cordl_class_UnityEngine+SystemLanguage")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::SystemLanguage {
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
