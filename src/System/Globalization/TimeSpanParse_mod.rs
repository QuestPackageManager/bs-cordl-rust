#[cfg(feature = "System+Globalization+TimeSpanParse")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeSpanParse {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+TimeSpanParse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::TimeSpanParse =>
    "System.Globalization"."TimeSpanParse"
);
#[cfg(feature = "System+Globalization+TimeSpanParse")]
impl std::ops::Deref for crate::System::Globalization::TimeSpanParse {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse")]
impl std::ops::DerefMut for crate::System::Globalization::TimeSpanParse {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (input, formatProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseExactDigits_ByRefMut0(
        tokenizer: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer,
        >,
        minDigitLength: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseExactDigits", (tokenizer, minDigitLength, result))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ParseExactDigits",
                (tokenizer, minDigitLength, maxDigitLength, zeroes, result),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseExactLiteral(
        tokenizer: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanTokenizer,
        >,
        enquotedString: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseExactLiteral", (tokenizer, enquotedString))?;
        Ok(__cordl_ret.into())
    }
    pub fn Pow10(pow: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Pow10", (pow))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessTerminalState", (raw, style, result))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessTerminal_D", (raw, style, result))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessTerminal_DHMSF", (raw, style, result))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessTerminal_HM", (raw, style, result))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessTerminal_HMS_F_D", (raw, style, result))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessTerminal_HM_S_D", (raw, style, result))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseByFormat", (input, format, styles, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact(
        input: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        styles: crate::System::Globalization::TimeSpanStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseExact", (input, format, formatProvider, styles, result))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryParseExactTimeSpan",
                (input, format, formatProvider, styles, result),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseTimeSpan", (input, style, formatProvider, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseTimeSpanConstant(
        input: crate::System::ReadOnlySpan_1<char>,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseTimeSpanConstant", (input, result))?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryTimeToTicks",
                (positive, days, hours, minutes, seconds, fraction, result),
            )?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSpanParse_ParseFailureKind {
    ArgumentNull = 1u8,
    Format = 2u8,
    FormatWithParameter = 3u8,
    None = 0u8,
    Overflow = 4u8,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+ParseFailureKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::TimeSpanParse_ParseFailureKind => "System.Globalization"
    ."TimeSpanParse/ParseFailureKind"
);
#[cfg(feature = "System+Globalization+TimeSpanParse+StringParser")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TimeSpanParse_StringParser {
    pub _str: crate::System::ReadOnlySpan_1<char>,
    pub _ch: char,
    pub _pos: i32,
    pub _len: i32,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+StringParser")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::TimeSpanParse_StringParser => "System.Globalization"
    ."TimeSpanParse/StringParser"
);
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextChar",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextNonDigit(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextNonDigit",
            (),
        )?;
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
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseInt",
            (max, i, result),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTime(
        &mut self,
        _cordl_time: quest_hook::libil2cpp::ByRefMut<i64>,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseTime",
            (_cordl_time, result),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SkipBlanks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SkipBlanks",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse(
        &mut self,
        input: crate::System::ReadOnlySpan_1<char>,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryParse",
            (input, result),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TTT")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSpanParse_TTT {
    End = 1u8,
    None = 0u8,
    Num = 2u8,
    NumOverflow = 4u8,
    Sep = 3u8,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TTT")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::TimeSpanParse_TTT =>
    "System.Globalization"."TimeSpanParse/TTT"
);
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanRawInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::TimeSpanParse_TimeSpanRawInfo => "System.Globalization"
    ."TimeSpanParse/TimeSpanRawInfo"
);
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
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddNum",
            (num, result),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSep(
        &mut self,
        sep: crate::System::ReadOnlySpan_1<char>,
        result: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::TimeSpanParse_TimeSpanResult,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddSep",
            (sep, result),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FullAppCompatMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FullAppCompatMatch",
            (pattern),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FullDHMMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FullDHMMatch",
            (pattern),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FullDHMSMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FullDHMSMatch",
            (pattern),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FullDMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FullDMatch",
            (pattern),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FullHMMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FullHMMatch",
            (pattern),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FullHMSFMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FullHMSFMatch",
            (pattern),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FullHMSMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FullHMSMatch",
            (pattern),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FullMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FullMatch",
            (pattern),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (dtfi),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PartialAppCompatMatch(
        &mut self,
        pattern: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PartialAppCompatMatch",
            (pattern),
        )?;
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
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ProcessToken",
            (tok, result),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NegativeInvariant(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    > {
        let __cordl_ret: crate::System::Globalization::TimeSpanFormat_FormatLiterals = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_NegativeInvariant",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NegativeLocalized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    > {
        let __cordl_ret: crate::System::Globalization::TimeSpanFormat_FormatLiterals = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_NegativeLocalized",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PositiveInvariant(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    > {
        let __cordl_ret: crate::System::Globalization::TimeSpanFormat_FormatLiterals = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_PositiveInvariant",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PositiveLocalized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    > {
        let __cordl_ret: crate::System::Globalization::TimeSpanFormat_FormatLiterals = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_PositiveLocalized",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanResult")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TimeSpanParse_TimeSpanResult {
    pub parsedTimeSpan: crate::System::TimeSpan,
    pub _throwOnFailure: bool,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::TimeSpanParse_TimeSpanResult => "System.Globalization"
    ."TimeSpanParse/TimeSpanResult"
);
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
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (kind, resourceKey, messageArgument, argumentName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        throwOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (throwOnFailure),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanStandardStyles")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSpanParse_TimeSpanStandardStyles {
    Any = 3u8,
    Invariant = 1u8,
    Localized = 2u8,
    None = 0u8,
    RequireFull = 4u8,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanStandardStyles")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::TimeSpanParse_TimeSpanStandardStyles =>
    "System.Globalization"."TimeSpanParse/TimeSpanStandardStyles"
);
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanToken")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TimeSpanParse_TimeSpanToken {
    pub _ttt: crate::System::Globalization::TimeSpanParse_TTT,
    pub _num: i32,
    pub _zeroes: i32,
    pub _sep: crate::System::ReadOnlySpan_1<char>,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanToken")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::TimeSpanParse_TimeSpanToken => "System.Globalization"
    ."TimeSpanParse/TimeSpanToken"
);
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
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsInvalidFraction",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TimeSpanParse_TTT0(
        &mut self,
        _cordl_type: crate::System::Globalization::TimeSpanParse_TTT,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_type),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TimeSpanParse_TTT_i32_i32_ReadOnlySpan_1_3(
        &mut self,
        _cordl_type: crate::System::Globalization::TimeSpanParse_TTT,
        number: i32,
        leadingZeroes: i32,
        separator: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_type, number, leadingZeroes, separator),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        number: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (number),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_2(
        &mut self,
        number: i32,
        leadingZeroes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (number, leadingZeroes),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanTokenizer")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TimeSpanParse_TimeSpanTokenizer {
    pub _value: crate::System::ReadOnlySpan_1<char>,
    pub _pos: i32,
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanTokenizer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::TimeSpanParse_TimeSpanTokenizer => "System.Globalization"
    ."TimeSpanParse/TimeSpanTokenizer"
);
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "BackOne",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::TimeSpanParse_TimeSpanToken,
    > {
        let __cordl_ret: crate::System::Globalization::TimeSpanParse_TimeSpanToken = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetNextToken",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ReadOnlySpan_1_0(
        &mut self,
        input: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (input),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        input: crate::System::ReadOnlySpan_1<char>,
        startPosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (input, startPosition),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EOL(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_EOL",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NextChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_NextChar",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
