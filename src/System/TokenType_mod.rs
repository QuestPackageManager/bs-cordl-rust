#[cfg(feature = "System+TokenType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TokenType {
    #[default]
    Am = 3i32,
    DateWordToken = 10i32,
    DayOfWeekToken = 7i32,
    EndOfString = 6i32,
    EraToken = 9i32,
    HebrewNumber = 12i32,
    IgnorableSymbol = 15i32,
    JapaneseEraToken = 13i32,
    MonthToken = 5i32,
    NumberToken = 1i32,
    Pm = 4i32,
    RegularTokenMask = 255i32,
    SEP_Am = 1024i32,
    SEP_Date = 1536i32,
    SEP_DateOrOffset = 3840i32,
    SEP_DaySuff = 2560i32,
    SEP_End = 512i32,
    SEP_HourSuff = 2816i32,
    SEP_LocalTimeMark = 3584i32,
    SEP_MinuteSuff = 3072i32,
    SEP_MonthSuff = 2304i32,
    SEP_Pm = 1280i32,
    SEP_SecondSuff = 3328i32,
    SEP_Space = 768i32,
    SEP_Time = 1792i32,
    SEP_Unk = 256i32,
    SEP_YearSuff = 2048i32,
    SeparatorTokenMask = 65280i32,
    TEraToken = 14i32,
    TimeZoneToken = 8i32,
    UnknownToken = 11i32,
    YearNumberToken = 2i32,
}
#[cfg(feature = "System+TokenType")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::TokenType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "TokenType";
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
#[cfg(feature = "System+TokenType")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::TokenType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+TokenType")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::TokenType {
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
#[cfg(feature = "System+TokenType")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::TokenType {
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
#[cfg(feature = "System+TokenType")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::TokenType {
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
