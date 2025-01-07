#[cfg(feature = "System+__DTString")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct __DTString {
    pub Value: crate::System::ReadOnlySpan_1<char>,
    pub Index: i32,
    pub m_current: char,
    pub m_info: quest_hook::libil2cpp::Gc<crate::System::Globalization::CompareInfo>,
    pub m_checkDigitToken: bool,
}
#[cfg(feature = "System+__DTString")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::__DTString {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "__DTString";
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
#[cfg(feature = "System+__DTString")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::__DTString {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+__DTString")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::__DTString {
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
#[cfg(feature = "System+__DTString")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::__DTString {
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
#[cfg(feature = "System+__DTString")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::__DTString {
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
#[cfg(feature = "System+__DTString")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::__DTString {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+__DTString")]
impl crate::System::__DTString {
    pub fn Advance(&mut self, count: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Advance",
            (count),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AtEnd(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AtEnd",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConsumeSubString(
        &mut self,
        sub: crate::System::DTSubString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ConsumeSubString",
            (sub),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetChar",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDigit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDigit",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextDigit(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetNextDigit",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRegularToken(
        &mut self,
        tokenType: quest_hook::libil2cpp::ByRefMut<crate::System::TokenType>,
        tokenValue: quest_hook::libil2cpp::ByRefMut<i32>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetRegularToken",
            (tokenType, tokenValue, dtfi),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRepeatCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetRepeatCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSeparatorToken(
        &mut self,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        indexBeforeSeparator: quest_hook::libil2cpp::ByRefMut<i32>,
        charBeforeSeparator: quest_hook::libil2cpp::ByRefMut<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TokenType> {
        let __cordl_ret: crate::System::TokenType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSeparatorToken",
            (dtfi, indexBeforeSeparator, charBeforeSeparator),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSubString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DTSubString> {
        let __cordl_ret: crate::System::DTSubString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetSubString",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchLongestWords(
        &mut self,
        words: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        maxMatchStrLen: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MatchLongestWords",
            (words, maxMatchStrLen),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchSpecifiedWord(
        &mut self,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MatchSpecifiedWord",
            (target),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchSpecifiedWords(
        &mut self,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        checkWordBoundary: bool,
        matchLength: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MatchSpecifiedWords",
            (target, checkWordBoundary, matchLength),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Match_Il2CppString0(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Match",
            (str),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Match__cordl_char1(
        &mut self,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Match",
            (ch),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveLeadingInQuoteSpaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveLeadingInQuoteSpaces",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveTrailingInQuoteSpaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveTrailingInQuoteSpaces",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SkipWhiteSpaceCurrent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SkipWhiteSpaceCurrent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SkipWhiteSpaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SkipWhiteSpaces",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TrimTail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TrimTail",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ReadOnlySpan_1_DateTimeFormatInfo1(
        &mut self,
        str: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (str, dtfi),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        str: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        checkDigitToken: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (str, dtfi, checkDigitToken),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompareInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CompareInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CompareInfo,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_CompareInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Length",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
