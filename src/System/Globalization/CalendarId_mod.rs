#[cfg(feature = "System+Globalization+CalendarId")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CalendarId {
    #[default]
    CHINESELUNISOLAR = 15u16,
    GREGORIAN = 1u16,
    GREGORIAN_ARABIC = 10u16,
    GREGORIAN_ME_FRENCH = 9u16,
    GREGORIAN_US = 2u16,
    GREGORIAN_XLIT_ENGLISH = 11u16,
    GREGORIAN_XLIT_FRENCH = 12u16,
    HEBREW = 8u16,
    HIJRI = 6u16,
    JAPAN = 3u16,
    JAPANESELUNISOLAR = 14u16,
    JULIAN = 13u16,
    KOREA = 5u16,
    KOREANLUNISOLAR = 20u16,
    LAST_CALENDAR = 23u16,
    LUNAR_ETO_CHN = 17u16,
    LUNAR_ETO_KOR = 18u16,
    LUNAR_ETO_ROKUYOU = 19u16,
    PERSIAN = 22u16,
    SAKA = 16u16,
    TAIWAN = 4u16,
    TAIWANLUNISOLAR = 21u16,
    THAI = 7u16,
    UNINITIALIZED_VALUE = 0u16,
}
#[cfg(feature = "System+Globalization+CalendarId")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Globalization::CalendarId {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "CalendarId";
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
#[cfg(feature = "System+Globalization+CalendarId")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Globalization::CalendarId {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Globalization+CalendarId")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Globalization::CalendarId {
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
#[cfg(feature = "System+Globalization+CalendarId")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Globalization::CalendarId {
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
#[cfg(feature = "System+Globalization+CalendarId")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Globalization::CalendarId {
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
