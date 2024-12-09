#[cfg(feature = "System+Globalization+TimeSpanParse")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeSpanParse {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Globalization+TimeSpanParse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::TimeSpanParse =>
    "System.Globalization"."TimeSpanParse"
);
#[cfg(feature = "System+Globalization+TimeSpanParse")]
impl std::ops::Deref for crate::System::Globalization::TimeSpanParse {
    type Target = crate::System::Object;
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
#[derive(Debug, Clone)]
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
        Ok(__cordl_ret)
    }
    pub fn NextNonDigit(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextNonDigit",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn SkipBlanks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SkipBlanks",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
#[derive(Debug, Clone)]
pub struct TimeSpanParse_TimeSpanRawInfo {
    pub _lastSeenTTT: crate::System::Globalization::TimeSpanParse_TTT,
    pub _tokenCount: i32,
    pub _sepCount: i32,
    pub _numCount: i32,
    pub _posLoc: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    pub _negLoc: crate::System::Globalization::TimeSpanFormat_FormatLiterals,
    pub _posLocInit: bool,
    pub _negLocInit: bool,
    pub _fullPosPattern: *mut crate::System::String,
    pub _fullNegPattern: *mut crate::System::String,
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        dtfi: *mut crate::System::Globalization::DateTimeFormatInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (dtfi),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanResult")]
#[repr(C)]
#[derive(Debug, Clone)]
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
        resourceKey: *mut crate::System::String,
        messageArgument: *mut crate::System::Object,
        argumentName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (kind, resourceKey, messageArgument, argumentName),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
#[derive(Debug, Clone)]
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Globalization+TimeSpanParse+TimeSpanTokenizer")]
#[repr(C)]
#[derive(Debug, Clone)]
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_EOL(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_EOL",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_NextChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_NextChar",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
