#[cfg(feature = "System+DateTimeFormat")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeFormat {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+DateTimeFormat")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::DateTimeFormat {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "DateTimeFormat";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+DateTimeFormat")]
impl std::ops::Deref for crate::System::DateTimeFormat {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeFormat")]
impl std::ops::DerefMut for crate::System::DateTimeFormat {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeFormat")]
impl crate::System::DateTimeFormat {
    pub fn Append2DigitNumber(
        result: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Append2DigitNumber")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Append2DigitNumber", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (result, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExpandPredefinedFormat(
        format: crate::System::ReadOnlySpan_1<char>,
        dateTime: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
        dtfi: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
        >,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Globalization::DateTimeFormatInfo,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        4usize,
                    >("ExpandPredefinedFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExpandPredefinedFormat", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (format, dateTime, dtfi, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatCustomized(
        dateTime: crate::System::DateTime,
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        offset: crate::System::TimeSpan,
        result: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::DateTimeFormatInfo,
                            >,
                            crate::System::TimeSpan,
                            quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        5usize,
                    >("FormatCustomized")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FormatCustomized", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (dateTime, format, dtfi, offset, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatCustomizedRoundripTimeZone(
        dateTime: crate::System::DateTime,
        offset: crate::System::TimeSpan,
        result: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            crate::System::TimeSpan,
                            quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("FormatCustomizedRoundripTimeZone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FormatCustomizedRoundripTimeZone", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (dateTime, offset, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatCustomizedTimeZone(
        dateTime: crate::System::DateTime,
        offset: crate::System::TimeSpan,
        format: crate::System::ReadOnlySpan_1<char>,
        tokenLen: i32,
        timeOnly: bool,
        result: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            crate::System::TimeSpan,
                            crate::System::ReadOnlySpan_1<char>,
                            i32,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("FormatCustomizedTimeZone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FormatCustomizedTimeZone", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dateTime, offset, format, tokenLen, timeOnly, result),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatDayOfWeek(
        dayOfWeek: i32,
        repeat: i32,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::DateTimeFormatInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        3usize,
                    >("FormatDayOfWeek")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FormatDayOfWeek", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (dayOfWeek, repeat, dtfi))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatDigits_StringBuilder_i32_i32_0(
        outputBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        value: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("FormatDigits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FormatDigits", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (outputBuffer, value, len))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatDigits__cordl_bool1(
        outputBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        value: i32,
        len: i32,
        overrideLengthLimit: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            i32,
                            i32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("FormatDigits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FormatDigits", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (outputBuffer, value, len, overrideLengthLimit))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatHebrewMonthName(
        _cordl_time: crate::System::DateTime,
        month: i32,
        repeatCount: i32,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::DateTimeFormatInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        4usize,
                    >("FormatHebrewMonthName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FormatHebrewMonthName", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_cordl_time, month, repeatCount, dtfi))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatMonth(
        month: i32,
        repeatCount: i32,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::DateTimeFormatInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        3usize,
                    >("FormatMonth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FormatMonth", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (month, repeatCount, dtfi))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatStringBuilder(
        dateTime: crate::System::DateTime,
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::DateTimeFormatInfo,
                            >,
                            crate::System::TimeSpan,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        4usize,
                    >("FormatStringBuilder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FormatStringBuilder", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder> = unsafe {
            cordl_method_info.invoke_unchecked((), (dateTime, format, dtfi, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_DateTime_Il2CppString_IFormatProvider0(
        dateTime: crate::System::DateTime,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        3usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (dateTime, format, provider))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Format_TimeSpan1(
        dateTime: crate::System::DateTime,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                            crate::System::TimeSpan,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        4usize,
                    >("Format")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Format",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (dateTime, format, provider, offset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRealFormat(
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::DateTimeFormatInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        2usize,
                    >("GetRealFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRealFormat", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked((), (format, dtfi))? };
        Ok(__cordl_ret.into())
    }
    pub fn HebrewFormatDigits(
        outputBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Text::StringBuilder,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HebrewFormatDigits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HebrewFormatDigits", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (outputBuffer, digits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvalidFormatForLocal(
        format: crate::System::ReadOnlySpan_1<char>,
        dateTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::ReadOnlySpan_1<char>, crate::System::DateTime),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("InvalidFormatForLocal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvalidFormatForLocal", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (format, dateTime))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsUseGenitiveForm(
        format: crate::System::ReadOnlySpan_1<char>,
        index: i32,
        tokenLen: i32,
        patternToMatch: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::ReadOnlySpan_1<char>, i32, i32, char),
                        bool,
                        4usize,
                    >("IsUseGenitiveForm")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsUseGenitiveForm", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (format, index, tokenLen, patternToMatch))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseNextChar(
        format: crate::System::ReadOnlySpan_1<char>,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::ReadOnlySpan_1<char>, i32),
                        i32,
                        2usize,
                    >("ParseNextChar")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseNextChar", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (format, pos))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseQuoteString(
        format: crate::System::ReadOnlySpan_1<char>,
        pos: i32,
        result: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        ),
                        i32,
                        3usize,
                    >("ParseQuoteString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseQuoteString", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (format, pos, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseRepeatPattern(
        format: crate::System::ReadOnlySpan_1<char>,
        pos: i32,
        patternChar: char,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::ReadOnlySpan_1<char>, i32, char),
                        i32,
                        3usize,
                    >("ParseRepeatPattern")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseRepeatPattern", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (format, pos, patternChar))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatO(
        dateTime: crate::System::DateTime,
        offset: crate::System::TimeSpan,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            crate::System::TimeSpan,
                            crate::System::Span_1<char>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        bool,
                        4usize,
                    >("TryFormatO")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryFormatO", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (dateTime, offset, destination, charsWritten))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatR(
        dateTime: crate::System::DateTime,
        offset: crate::System::TimeSpan,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            crate::System::TimeSpan,
                            crate::System::Span_1<char>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        bool,
                        4usize,
                    >("TryFormatR")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryFormatR", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (dateTime, offset, destination, charsWritten))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryFormat_DateTime_Span_1_ByRefMut_ReadOnlySpan_1_IFormatProvider0(
        dateTime: crate::System::DateTime,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            crate::System::Span_1<char>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                        ),
                        bool,
                        5usize,
                    >("TryFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryFormat", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dateTime, destination, charsWritten, format, provider),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryFormat_TimeSpan1(
        dateTime: crate::System::DateTime,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::DateTime,
                            crate::System::Span_1<char>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                            crate::System::TimeSpan,
                        ),
                        bool,
                        6usize,
                    >("TryFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryFormat", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (dateTime, destination, charsWritten, format, provider, offset),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteDigits(
        value: u64,
        buffer: crate::System::Span_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64, crate::System::Span_1<char>),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("WriteDigits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteDigits", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (value, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteFourDecimalDigits(
        value: u32,
        buffer: crate::System::Span_1<char>,
        startingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u32, crate::System::Span_1<char>, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteFourDecimalDigits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteFourDecimalDigits", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (value, buffer, startingIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteTwoDecimalDigits(
        value: u32,
        destination: crate::System::Span_1<char>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u32, crate::System::Span_1<char>, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteTwoDecimalDigits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteTwoDecimalDigits", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (value, destination, offset))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+DateTimeFormat")]
impl quest_hook::libil2cpp::ObjectType for crate::System::DateTimeFormat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
