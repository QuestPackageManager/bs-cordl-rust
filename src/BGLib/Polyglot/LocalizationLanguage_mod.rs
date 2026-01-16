#[cfg(feature = "cordl_class_BGLib+Polyglot+LocalizationLanguage")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum LocalizationLanguage {
    #[default]
    Arabic = 26i32,
    Bosnian = 27i32,
    Bulgarian = 24i32,
    Czech = 20i32,
    Danish = 10i32,
    Debug_English_Reverted = 29i32,
    Debug_Keys = 28i32,
    Debug_Word_With_Max_Length = 30i32,
    Dutch = 13i32,
    English = 0i32,
    Finnish = 15i32,
    French = 1i32,
    German = 3i32,
    Greek = 8i32,
    Hebrew = 25i32,
    Hungarian = 21i32,
    Italian = 4i32,
    Japanese = 16i32,
    Korean = 19i32,
    Norwegian = 11i32,
    Polish = 14i32,
    Portuguese = 6i32,
    Portuguese_Brazil = 5i32,
    Romanian = 22i32,
    Russian = 7i32,
    Simplified_Chinese = 17i32,
    Spanish = 2i32,
    Swedish = 12i32,
    Thai = 23i32,
    Traditional_Chinese = 18i32,
    Turkish = 9i32,
}
#[cfg(feature = "cordl_class_BGLib+Polyglot+LocalizationLanguage")]
unsafe impl quest_hook::libil2cpp::Type for crate::BGLib::Polyglot::LocalizationLanguage {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BGLib.Polyglot";
    const CLASS_NAME: &'static str = "LocalizationLanguage";
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
#[cfg(feature = "cordl_class_BGLib+Polyglot+LocalizationLanguage")]
unsafe impl quest_hook::libil2cpp::Argument for crate::BGLib::Polyglot::LocalizationLanguage {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_BGLib+Polyglot+LocalizationLanguage")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::BGLib::Polyglot::LocalizationLanguage {
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
#[cfg(feature = "cordl_class_BGLib+Polyglot+LocalizationLanguage")]
unsafe impl quest_hook::libil2cpp::Returned for crate::BGLib::Polyglot::LocalizationLanguage {
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
#[cfg(feature = "cordl_class_BGLib+Polyglot+LocalizationLanguage")]
unsafe impl quest_hook::libil2cpp::Return for crate::BGLib::Polyglot::LocalizationLanguage {
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
