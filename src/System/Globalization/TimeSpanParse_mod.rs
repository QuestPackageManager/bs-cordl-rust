#[cfg(feature = "System+Globalization+TimeSpanParse")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeSpanParse {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+TimeSpanParse")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Globalization::TimeSpanParse {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "TimeSpanParse";
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
#[cfg(feature = "System+Globalization+TimeSpanParse")]
impl std::ops::Deref for crate::System::Globalization::TimeSpanParse {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse")]
impl std::ops::DerefMut for crate::System::Globalization::TimeSpanParse {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse")]
impl crate::System::Globalization::TimeSpanParse {
    #[cfg(feature = "System+Globalization+TimeSpanParse+ParseFailureKind")]
    pub type ParseFailureKind = crate::System::Globalization::TimeSpanParse_ParseFailureKind;
    #[cfg(feature = "System+Globalization+TimeSpanParse+StringParser")]
    pub type StringParser = crate::System::Globalization::TimeSpanParse_StringParser;
    #[cfg(feature = "System+Globalization+TimeSpanParse+TTT")]
    pub type TTT = crate::System::Globalization::TimeSpanParse_TTT;
    #[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanRawInfo")]
    pub type TimeSpanRawInfo = crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo;
    #[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanResult")]
    pub type TimeSpanResult = crate::System::Globalization::TimeSpanParse_TimeSpanResult;
    #[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanStandardStyles")]
    pub type TimeSpanStandardStyles = crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles;
    #[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanToken")]
    pub type TimeSpanToken = crate::System::Globalization::TimeSpanParse_TimeSpanToken;
    #[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanTokenizer")]
    pub type TimeSpanTokenizer = crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer;
    pub fn Parse(
        input: crate::System::ReadOnlySpan_1<char>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                        ),
                        crate::System::TimeSpan,
                        2usize,
                    >("Parse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Parse",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::TimeSpan = unsafe {
            method.invoke_unchecked((), (input, formatProvider))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseExactDigits_ByRefMut0(
        tokenizer: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer,
        >,
        minDigitLength: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        bool,
                        3usize,
                    >("ParseExactDigits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseExactDigits", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (tokenizer, minDigitLength, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseExactDigits_i32_ByRefMut_ByRefMut1(
        tokenizer: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer,
        >,
        minDigitLength: i32,
        maxDigitLength: i32,
        zeroes: quest_hook::libil2cpp::ByRefMut<i32>,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        bool,
                        5usize,
                    >("ParseExactDigits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseExactDigits", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (tokenizer, minDigitLength, maxDigitLength, zeroes, result),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseExactLiteral(
        tokenizer: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer,
        >,
        enquotedString: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        ),
                        bool,
                        2usize,
                    >("ParseExactLiteral")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseExactLiteral", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (tokenizer, enquotedString))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Pow10(pow: i32) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), i64, 1usize>("Pow10")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Pow10",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (pow))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTerminalState(
        raw: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo,
        >,
        style: crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo,
                            >,
                            crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        3usize,
                    >("ProcessTerminalState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessTerminalState", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (raw, style, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTerminal_D(
        raw: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo,
        >,
        style: crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo,
                            >,
                            crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        3usize,
                    >("ProcessTerminal_D")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessTerminal_D", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (raw, style, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTerminal_DHMSF(
        raw: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo,
        >,
        style: crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo,
                            >,
                            crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        3usize,
                    >("ProcessTerminal_DHMSF")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessTerminal_DHMSF", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (raw, style, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTerminal_HM(
        raw: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo,
        >,
        style: crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo,
                            >,
                            crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        3usize,
                    >("ProcessTerminal_HM")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessTerminal_HM", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (raw, style, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTerminal_HMS_F_D(
        raw: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo,
        >,
        style: crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo,
                            >,
                            crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        3usize,
                    >("ProcessTerminal_HMS_F_D")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessTerminal_HMS_F_D", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (raw, style, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTerminal_HM_S_D(
        raw: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo,
        >,
        style: crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo,
                            >,
                            crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        3usize,
                    >("ProcessTerminal_HM_S_D")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessTerminal_HM_S_D", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (raw, style, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseByFormat(
        input: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::TimeSpanStyles,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            crate::System::ReadOnlySpan_1<char>,
                            crate::System::Globalization::TimeSpanStyles,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        4usize,
                    >("TryParseByFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryParseByFormat", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (input, format, styles, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact(
        input: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        styles: crate::System::Globalization::TimeSpanStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                            crate::System::Globalization::TimeSpanStyles,
                            quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
                        ),
                        bool,
                        5usize,
                    >("TryParseExact")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryParseExact", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (input, format, formatProvider, styles, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExactTimeSpan(
        input: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        styles: crate::System::Globalization::TimeSpanStyles,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                            crate::System::Globalization::TimeSpanStyles,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        5usize,
                    >("TryParseExactTimeSpan")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryParseExactTimeSpan", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (input, format, formatProvider, styles, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseTimeSpan(
        input: crate::System::ReadOnlySpan_1<char>,
        style: crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles,
                            quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        4usize,
                    >("TryParseTimeSpan")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryParseTimeSpan", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (input, style, formatProvider, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseTimeSpanConstant(
        input: crate::System::ReadOnlySpan_1<char>,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        2usize,
                    >("TryParseTimeSpanConstant")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryParseTimeSpanConstant", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (input, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryTimeToTicks(
        positive: bool,
        days: crate::System::Globalization::TimeSpanParse_TimeSpanToken,
        hours: crate::System::Globalization::TimeSpanParse_TimeSpanToken,
        minutes: crate::System::Globalization::TimeSpanParse_TimeSpanToken,
        seconds: crate::System::Globalization::TimeSpanParse_TimeSpanToken,
        fraction: crate::System::Globalization::TimeSpanParse_TimeSpanToken,
        result: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            bool,
                            crate::System::Globalization::TimeSpanParse_TimeSpanToken,
                            crate::System::Globalization::TimeSpanParse_TimeSpanToken,
                            crate::System::Globalization::TimeSpanParse_TimeSpanToken,
                            crate::System::Globalization::TimeSpanParse_TimeSpanToken,
                            crate::System::Globalization::TimeSpanParse_TimeSpanToken,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                        ),
                        bool,
                        7usize,
                    >("TryTimeToTicks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryTimeToTicks", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (positive, days, hours, minutes, seconds, fraction, result),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::TimeSpanParse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+ParseFailureKind")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimeSpanParse_ParseFailureKind {
    #[default]
    ArgumentNull = 1u8,
    Format = 2u8,
    FormatWithParameter = 3u8,
    None = 0u8,
    Overflow = 4u8,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+ParseFailureKind")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::TimeSpanParse_ParseFailureKind {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "TimeSpanParse/ParseFailureKind";
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
#[cfg(feature = "System+Globalization+TimeSpanParse+ParseFailureKind")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Globalization::TimeSpanParse_ParseFailureKind {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+ParseFailureKind")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Globalization::TimeSpanParse_ParseFailureKind {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+ParseFailureKind")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Globalization::TimeSpanParse_ParseFailureKind {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+ParseFailureKind")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Globalization::TimeSpanParse_ParseFailureKind {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+StringParser")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeSpanParse_StringParser {
    pub _str: crate::System::ReadOnlySpan_1<char>,
    pub _ch: char,
    pub _pos: i32,
    pub _len: i32,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+StringParser")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::TimeSpanParse_StringParser {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "TimeSpanParse/StringParser";
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
#[cfg(feature = "System+Globalization+TimeSpanParse+StringParser")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Globalization::TimeSpanParse_StringParser {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+StringParser")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Globalization::TimeSpanParse_StringParser {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+StringParser")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Globalization::TimeSpanParse_StringParser {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+StringParser")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Globalization::TimeSpanParse_StringParser {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+StringParser")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::TimeSpanParse_StringParser {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+StringParser")]
impl crate::System::Globalization::TimeSpanParse_StringParser {
    pub fn NextChar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("NextChar")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextChar", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextNonDigit(&mut self) -> quest_hook::libil2cpp::Result<char> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), char, 0usize>("NextNonDigit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextNonDigit", 0usize
                        )
                    })
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ParseInt(
        &mut self,
        max: i32,
        i: quest_hook::libil2cpp::ByRefMut<i32>,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        3usize,
                    >("ParseInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseInt", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (max, i, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseTime(
        &mut self,
        _cordl_time: quest_hook::libil2cpp::ByRefMut<i64>,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<i64>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        2usize,
                    >("ParseTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseTime", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (_cordl_time, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SkipBlanks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("SkipBlanks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SkipBlanks", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParse(
        &mut self,
        input: crate::System::ReadOnlySpan_1<char>,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        2usize,
                    >("TryParse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryParse", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (input, result))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TTT")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimeSpanParse_TTT {
    #[default]
    End = 1u8,
    None = 0u8,
    Num = 2u8,
    NumOverflow = 4u8,
    Sep = 3u8,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TTT")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::TimeSpanParse_TTT {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "TimeSpanParse/TTT";
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TTT")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Globalization::TimeSpanParse_TTT {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TTT")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Globalization::TimeSpanParse_TTT {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TTT")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Globalization::TimeSpanParse_TTT {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TTT")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Globalization::TimeSpanParse_TTT {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanRawInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeSpanParse_TimeSpanRawInfo {
    pub _lastSeenTTT: crate::System::Globalization::TimeSpanParse_TTT,
    pub _tokenCount: i32,
    pub _sepCount: i32,
    pub _numCount: i32,
    pub _posLoc: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    pub _negLoc: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    pub _posLocInit: bool,
    pub _negLocInit: bool,
    pub _fullPosPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _fullNegPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _numbers0: crate::System::Globalization::TimeSpanParse_TimeSpanToken,
    pub _numbers1: crate::System::Globalization::TimeSpanParse_TimeSpanToken,
    pub _numbers2: crate::System::Globalization::TimeSpanParse_TimeSpanToken,
    pub _numbers3: crate::System::Globalization::TimeSpanParse_TimeSpanToken,
    pub _numbers4: crate::System::Globalization::TimeSpanParse_TimeSpanToken,
    pub _literals0: crate::System::ReadOnlySpan_1<char>,
    pub _literals1: crate::System::ReadOnlySpan_1<char>,
    pub _literals2: crate::System::ReadOnlySpan_1<char>,
    pub _literals3: crate::System::ReadOnlySpan_1<char>,
    pub _literals4: crate::System::ReadOnlySpan_1<char>,
    pub _literals5: crate::System::ReadOnlySpan_1<char>,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanRawInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "TimeSpanParse/TimeSpanRawInfo";
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanRawInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanRawInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanRawInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanRawInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanRawInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanRawInfo")]
impl crate::System::Globalization::TimeSpanParse_TimeSpanRawInfo {
    pub fn AddNum(
        &mut self,
        num: crate::System::Globalization::TimeSpanParse_TimeSpanToken,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Globalization::TimeSpanParse_TimeSpanToken,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        2usize,
                    >("AddNum")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "AddNum",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (num, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddSep(
        &mut self,
        sep: crate::System::ReadOnlySpan_1<char>,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::ReadOnlySpan_1<char>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        2usize,
                    >("AddSep")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "AddSep",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (sep, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn FullAppCompatMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Globalization::TimeSpanFormat_FormatLiterals),
                        bool,
                        1usize,
                    >("FullAppCompatMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FullAppCompatMatch", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pattern))? };
        Ok(__cordl_ret.into())
    }
    pub fn FullDHMMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Globalization::TimeSpanFormat_FormatLiterals),
                        bool,
                        1usize,
                    >("FullDHMMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FullDHMMatch", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pattern))? };
        Ok(__cordl_ret.into())
    }
    pub fn FullDHMSMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Globalization::TimeSpanFormat_FormatLiterals),
                        bool,
                        1usize,
                    >("FullDHMSMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FullDHMSMatch", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pattern))? };
        Ok(__cordl_ret.into())
    }
    pub fn FullDMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Globalization::TimeSpanFormat_FormatLiterals),
                        bool,
                        1usize,
                    >("FullDMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FullDMatch", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pattern))? };
        Ok(__cordl_ret.into())
    }
    pub fn FullHMMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Globalization::TimeSpanFormat_FormatLiterals),
                        bool,
                        1usize,
                    >("FullHMMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FullHMMatch", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pattern))? };
        Ok(__cordl_ret.into())
    }
    pub fn FullHMSFMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Globalization::TimeSpanFormat_FormatLiterals),
                        bool,
                        1usize,
                    >("FullHMSFMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FullHMSFMatch", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pattern))? };
        Ok(__cordl_ret.into())
    }
    pub fn FullHMSMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Globalization::TimeSpanFormat_FormatLiterals),
                        bool,
                        1usize,
                    >("FullHMSMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FullHMSMatch", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pattern))? };
        Ok(__cordl_ret.into())
    }
    pub fn FullMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Globalization::TimeSpanFormat_FormatLiterals),
                        bool,
                        1usize,
                    >("FullMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FullMatch", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pattern))? };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Globalization::DateTimeFormatInfo,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Init",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dtfi))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PartialAppCompatMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Globalization::TimeSpanFormat_FormatLiterals),
                        bool,
                        1usize,
                    >("PartialAppCompatMatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PartialAppCompatMatch", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (pattern))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessToken(
        &mut self,
        tok: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanToken,
        >,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanToken,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Globalization::TimeSpanParse_TimeSpanResult,
                            >,
                        ),
                        bool,
                        2usize,
                    >("ProcessToken")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessToken", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (tok, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_NegativeInvariant(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Globalization::TimeSpanFormat_FormatLiterals,
                        0usize,
                    >("get_NegativeInvariant")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_NegativeInvariant", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Globalization::TimeSpanFormat_FormatLiterals = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_NegativeLocalized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Globalization::TimeSpanFormat_FormatLiterals,
                        0usize,
                    >("get_NegativeLocalized")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_NegativeLocalized", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Globalization::TimeSpanFormat_FormatLiterals = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_PositiveInvariant(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Globalization::TimeSpanFormat_FormatLiterals,
                        0usize,
                    >("get_PositiveInvariant")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_PositiveInvariant", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Globalization::TimeSpanFormat_FormatLiterals = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_PositiveLocalized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Globalization::TimeSpanFormat_FormatLiterals,
                        0usize,
                    >("get_PositiveLocalized")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_PositiveLocalized", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Globalization::TimeSpanFormat_FormatLiterals = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanResult")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeSpanParse_TimeSpanResult {
    pub parsedTimeSpan: crate::System::TimeSpan,
    pub _throwOnFailure: bool,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::TimeSpanParse_TimeSpanResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "TimeSpanParse/TimeSpanResult";
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Globalization::TimeSpanParse_TimeSpanResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Globalization::TimeSpanParse_TimeSpanResult {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Globalization::TimeSpanParse_TimeSpanResult {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Globalization::TimeSpanParse_TimeSpanResult {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::TimeSpanParse_TimeSpanResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanResult")]
impl crate::System::Globalization::TimeSpanParse_TimeSpanResult {
    pub fn SetFailure(
        &mut self,
        kind: crate::System::Globalization::TimeSpanParse_ParseFailureKind,
        resourceKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        messageArgument: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        argumentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Globalization::TimeSpanParse_ParseFailureKind,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        4usize,
                    >("SetFailure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetFailure", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (kind, resourceKey, messageArgument, argumentName),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        throwOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (throwOnFailure))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanStandardStyles")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimeSpanParse_TimeSpanStandardStyles {
    #[default]
    Any = 3u8,
    Invariant = 1u8,
    Localized = 2u8,
    None = 0u8,
    RequireFull = 4u8,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanStandardStyles")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "TimeSpanParse/TimeSpanStandardStyles";
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanStandardStyles")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanStandardStyles")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanStandardStyles")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanStandardStyles")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Globalization::TimeSpanParse_TimeSpanStandardStyles {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanToken")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeSpanParse_TimeSpanToken {
    pub _ttt: crate::System::Globalization::TimeSpanParse_TTT,
    pub _num: i32,
    pub _zeroes: i32,
    pub _sep: crate::System::ReadOnlySpan_1<char>,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanToken")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::TimeSpanParse_TimeSpanToken {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "TimeSpanParse/TimeSpanToken";
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanToken")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Globalization::TimeSpanParse_TimeSpanToken {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanToken")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Globalization::TimeSpanParse_TimeSpanToken {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanToken")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Globalization::TimeSpanParse_TimeSpanToken {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanToken")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Globalization::TimeSpanParse_TimeSpanToken {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanToken")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::TimeSpanParse_TimeSpanToken {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanToken")]
impl crate::System::Globalization::TimeSpanParse_TimeSpanToken {
    pub fn IsInvalidFraction(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("IsInvalidFraction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsInvalidFraction", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TimeSpanParse_TTT0(
        &mut self,
        _cordl_type: crate::System::Globalization::TimeSpanParse_TTT,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Globalization::TimeSpanParse_TTT),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TimeSpanParse_TTT_i32_i32_ReadOnlySpan_1_3(
        &mut self,
        _cordl_type: crate::System::Globalization::TimeSpanParse_TTT,
        number: i32,
        leadingZeroes: i32,
        separator: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Globalization::TimeSpanParse_TTT,
                            i32,
                            i32,
                            crate::System::ReadOnlySpan_1<char>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (_cordl_type, number, leadingZeroes, separator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        number: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (number))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_2(
        &mut self,
        number: i32,
        leadingZeroes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (number, leadingZeroes))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanTokenizer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeSpanParse_TimeSpanTokenizer {
    pub _value: crate::System::ReadOnlySpan_1<char>,
    pub _pos: i32,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanTokenizer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "TimeSpanParse/TimeSpanTokenizer";
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanTokenizer")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanTokenizer")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanTokenizer")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanTokenizer")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer {
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
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanTokenizer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanTokenizer")]
impl crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer {
    pub fn BackOne(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("BackOne")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "BackOne",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::TimeSpanParse_TimeSpanToken,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Globalization::TimeSpanParse_TimeSpanToken,
                        0usize,
                    >("GetNextToken")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNextToken", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Globalization::TimeSpanParse_TimeSpanToken = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ReadOnlySpan_1_0(
        &mut self,
        input: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::ReadOnlySpan_1<char>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        input: crate::System::ReadOnlySpan_1<char>,
        startPosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::ReadOnlySpan_1<char>, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (input, startPosition))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_EOL(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_EOL")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_EOL",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_NextChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), char, 0usize>("get_NextChar")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_NextChar", 0usize
                        )
                    })
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
