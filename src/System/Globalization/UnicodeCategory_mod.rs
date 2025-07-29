#[cfg(feature = "cordl_class_System+Globalization+UnicodeCategory")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnicodeCategory {
    #[default]
    ClosePunctuation = 21i32,
    ConnectorPunctuation = 18i32,
    Control = 14i32,
    CurrencySymbol = 26i32,
    DashPunctuation = 19i32,
    DecimalDigitNumber = 8i32,
    EnclosingMark = 7i32,
    FinalQuotePunctuation = 23i32,
    Format = 15i32,
    InitialQuotePunctuation = 22i32,
    LetterNumber = 9i32,
    LineSeparator = 12i32,
    LowercaseLetter = 1i32,
    MathSymbol = 25i32,
    ModifierLetter = 3i32,
    ModifierSymbol = 27i32,
    NonSpacingMark = 5i32,
    OpenPunctuation = 20i32,
    OtherLetter = 4i32,
    OtherNotAssigned = 29i32,
    OtherNumber = 10i32,
    OtherPunctuation = 24i32,
    OtherSymbol = 28i32,
    ParagraphSeparator = 13i32,
    PrivateUse = 17i32,
    SpaceSeparator = 11i32,
    SpacingCombiningMark = 6i32,
    Surrogate = 16i32,
    TitlecaseLetter = 2i32,
    UppercaseLetter = 0i32,
}
#[cfg(feature = "cordl_class_System+Globalization+UnicodeCategory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::UnicodeCategory {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "UnicodeCategory";
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
#[cfg(feature = "cordl_class_System+Globalization+UnicodeCategory")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Globalization::UnicodeCategory {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Globalization+UnicodeCategory")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Globalization::UnicodeCategory {
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
#[cfg(feature = "cordl_class_System+Globalization+UnicodeCategory")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Globalization::UnicodeCategory {
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
#[cfg(feature = "cordl_class_System+Globalization+UnicodeCategory")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Globalization::UnicodeCategory {
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
