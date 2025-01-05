#[cfg(feature = "System+Guid")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Guid {
    pub _a: i32,
    pub _b: i16,
    pub _c: i16,
    pub _d: u8,
    pub _e: u8,
    pub _f: u8,
    pub _g: u8,
    pub _h: u8,
    pub _i: u8,
    pub _j: u8,
    pub _k: u8,
}
#[cfg(feature = "System+Guid")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Guid => "System"."Guid"
);
#[cfg(feature = "System+Guid")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Guid {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Guid")]
impl crate::System::Guid {
    #[cfg(feature = "System+Guid+GuidParseThrowStyle")]
    pub type GuidParseThrowStyle = crate::System::Guid_GuidParseThrowStyle;
    #[cfg(feature = "System+Guid+GuidResult")]
    pub type GuidResult = crate::System::Guid_GuidResult;
    #[cfg(feature = "System+Guid+GuidStyles")]
    pub type GuidStyles = crate::System::Guid_GuidStyles;
    #[cfg(feature = "System+Guid+ParseFailureKind")]
    pub type ParseFailureKind = crate::System::Guid_ParseFailureKind;
    pub fn CompareTo_Guid1(
        &mut self,
        value: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_Il2CppObject0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn EatAllWhitespace(
        str: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EatAllWhitespace", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Guid1(
        &mut self,
        g: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (g),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (o),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResult(
        &mut self,
        me: u32,
        them: u32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetResult",
            (me, them),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn HexToChar(a: i32) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HexToChar", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn HexsToChars(
        guidChars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: i32,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HexsToChars", (guidChars, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn HexsToCharsHexOutput(
        guidChars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: i32,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HexsToCharsHexOutput", (guidChars, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHexPrefix(
        str: crate::System::ReadOnlySpan_1<char>,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHexPrefix", (str, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn NewGuid() -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_ret: crate::System::Guid = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NewGuid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_Il2CppString0(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_ret: crate::System::Guid = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_ReadOnlySpan_1_1(
        input: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_ret: crate::System::Guid = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn StringToInt_ByRefMut_i32_ByRefMut1(
        str: crate::System::ReadOnlySpan_1<char>,
        parsePos: quest_hook::libil2cpp::ByRefMut<i32>,
        requiredLength: i32,
        flags: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
        parseResult: quest_hook::libil2cpp::ByRefMut<crate::System::Guid_GuidResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "StringToInt",
                (str, parsePos, requiredLength, flags, result, parseResult),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StringToInt_i32_ByRefMut0(
        str: crate::System::ReadOnlySpan_1<char>,
        requiredLength: i32,
        flags: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
        parseResult: quest_hook::libil2cpp::ByRefMut<crate::System::Guid_GuidResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringToInt", (str, requiredLength, flags, result, parseResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn StringToLong(
        str: crate::System::ReadOnlySpan_1<char>,
        parsePos: quest_hook::libil2cpp::ByRefMut<i32>,
        flags: i32,
        result: quest_hook::libil2cpp::ByRefMut<i64>,
        parseResult: quest_hook::libil2cpp::ByRefMut<crate::System::Guid_GuidResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringToLong", (str, parsePos, flags, result, parseResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn StringToShort_ByRefMut_i32_ByRefMut1(
        str: crate::System::ReadOnlySpan_1<char>,
        parsePos: quest_hook::libil2cpp::ByRefMut<i32>,
        requiredLength: i32,
        flags: i32,
        result: quest_hook::libil2cpp::ByRefMut<i16>,
        parseResult: quest_hook::libil2cpp::ByRefMut<crate::System::Guid_GuidResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "StringToShort",
                (str, parsePos, requiredLength, flags, result, parseResult),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StringToShort_i32_ByRefMut0(
        str: crate::System::ReadOnlySpan_1<char>,
        requiredLength: i32,
        flags: i32,
        result: quest_hook::libil2cpp::ByRefMut<i16>,
        parseResult: quest_hook::libil2cpp::ByRefMut<crate::System::Guid_GuidResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringToShort", (str, requiredLength, flags, result, parseResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_ISpanFormattable_TryFormat(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.ISpanFormattable.TryFormat",
            (destination, charsWritten, format, provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByteArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToByteArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider2(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormat(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryFormat",
            (destination, charsWritten, format),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact_Il2CppString_Il2CppString0(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseExact", (input, format, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact_ReadOnlySpan_1_ReadOnlySpan_1_1(
        input: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseExact", (input, format, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseGuid(
        guidString: crate::System::ReadOnlySpan_1<char>,
        flags: crate::System::Guid_GuidStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Guid_GuidResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseGuid", (guidString, flags, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseGuidWithDashes(
        guidString: crate::System::ReadOnlySpan_1<char>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Guid_GuidResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseGuidWithDashes", (guidString, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseGuidWithHexPrefix(
        guidString: crate::System::ReadOnlySpan_1<char>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Guid_GuidResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseGuidWithHexPrefix", (guidString, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseGuidWithNoStyle(
        guidString: crate::System::ReadOnlySpan_1<char>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Guid_GuidResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseGuidWithNoStyle", (guidString, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_Il2CppString0(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ReadOnlySpan_1_1(
        input: crate::System::ReadOnlySpan_1<char>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteByteHelper(
        &mut self,
        destination: crate::System::Span_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteByteHelper",
            (destination),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (b),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString4(
        &mut self,
        g: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (g),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ReadOnlySpan_1_1(
        &mut self,
        b: crate::System::ReadOnlySpan_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (b),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i16_i16_Il2CppArray2(
        &mut self,
        a: i32,
        b: i16,
        c: i16,
        d: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i16_i16_u8_u8_u8_u8_u8_u8_u8_u8_3(
        &mut self,
        a: i32,
        b: i16,
        c: i16,
        d: u8,
        e: u8,
        f: u8,
        g: u8,
        h: u8,
        i: u8,
        j: u8,
        k: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d, e, f, g, h, i, j, k),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        a: crate::System::Guid,
        b: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        a: crate::System::Guid,
        b: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Guid")]
impl AsRef<crate::System::IComparable> for crate::System::Guid {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Guid")]
impl AsMut<crate::System::IComparable> for crate::System::Guid {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Guid")]
impl AsRef<crate::System::IComparable_1<crate::System::Guid>> for crate::System::Guid {
    fn as_ref(&self) -> &crate::System::IComparable_1<crate::System::Guid> {
        todo!()
    }
}
#[cfg(feature = "System+Guid")]
impl AsMut<crate::System::IComparable_1<crate::System::Guid>> for crate::System::Guid {
    fn as_mut(&mut self) -> &mut crate::System::IComparable_1<crate::System::Guid> {
        todo!()
    }
}
#[cfg(feature = "System+Guid")]
impl AsRef<crate::System::IEquatable_1<crate::System::Guid>> for crate::System::Guid {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::System::Guid> {
        todo!()
    }
}
#[cfg(feature = "System+Guid")]
impl AsMut<crate::System::IEquatable_1<crate::System::Guid>> for crate::System::Guid {
    fn as_mut(&mut self) -> &mut crate::System::IEquatable_1<crate::System::Guid> {
        todo!()
    }
}
#[cfg(feature = "System+Guid")]
impl AsRef<crate::System::IFormattable> for crate::System::Guid {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Guid")]
impl AsMut<crate::System::IFormattable> for crate::System::Guid {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Guid")]
impl AsRef<crate::System::ISpanFormattable> for crate::System::Guid {
    fn as_ref(&self) -> &crate::System::ISpanFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Guid")]
impl AsMut<crate::System::ISpanFormattable> for crate::System::Guid {
    fn as_mut(&mut self) -> &mut crate::System::ISpanFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Guid+GuidParseThrowStyle")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Guid_GuidParseThrowStyle {
    #[default]
    All = 1i32,
    AllButOverflow = 2i32,
    None = 0i32,
}
#[cfg(feature = "System+Guid+GuidParseThrowStyle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Guid_GuidParseThrowStyle => "System"
    ."Guid/GuidParseThrowStyle"
);
#[cfg(feature = "System+Guid+GuidResult")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Guid_GuidResult {
    pub _parsedGuid: crate::System::Guid,
    pub _throwStyle: crate::System::Guid_GuidParseThrowStyle,
    pub _failure: crate::System::Guid_ParseFailureKind,
    pub _failureMessageID: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _failureMessageFormatArgument: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _failureArgumentName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
}
#[cfg(feature = "System+Guid+GuidResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Guid_GuidResult => "System"
    ."Guid/GuidResult"
);
#[cfg(feature = "System+Guid+GuidResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Guid_GuidResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Guid+GuidResult")]
impl crate::System::Guid_GuidResult {
    pub fn GetGuidParseException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetGuidParseException",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        canThrow: crate::System::Guid_GuidParseThrowStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (canThrow),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFailure_Exception0(
        &mut self,
        nativeException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (nativeException),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFailure_Guid_ParseFailureKind_Il2CppString1(
        &mut self,
        failure: crate::System::Guid_ParseFailureKind,
        failureMessageID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (failure, failureMessageID),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFailure_Guid_ParseFailureKind_Il2CppString_Il2CppObject2(
        &mut self,
        failure: crate::System::Guid_ParseFailureKind,
        failureMessageID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        failureMessageFormatArgument: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (failure, failureMessageID, failureMessageFormatArgument),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFailure_Guid_ParseFailureKind_Il2CppString_Il2CppObject_Il2CppString_Exception3(
        &mut self,
        failure: crate::System::Guid_ParseFailureKind,
        failureMessageID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        failureMessageFormatArgument: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        failureArgumentName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (
                failure,
                failureMessageID,
                failureMessageFormatArgument,
                failureArgumentName,
                innerException,
            ),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Guid+GuidStyles")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Guid_GuidStyles {
    #[default]
    AllowBraces = 2i32,
    AllowDashes = 4i32,
    AllowHexPrefix = 8i32,
    AllowParenthesis = 1i32,
    Any = 15i32,
    BraceFormat = 96i32,
    DigitFormat = 64i32,
    HexFormat = 160i32,
    None = 0i32,
    ParenthesisFormat = 80i32,
    RequireBraces = 32i32,
    RequireHexPrefix = 128i32,
    RequireParenthesis = 16i32,
}
#[cfg(feature = "System+Guid+GuidStyles")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Guid_GuidStyles => "System"
    ."Guid/GuidStyles"
);
#[cfg(feature = "System+Guid+ParseFailureKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Guid_ParseFailureKind {
    #[default]
    ArgumentNull = 1i32,
    Format = 2i32,
    FormatWithInnerException = 5i32,
    FormatWithParameter = 3i32,
    NativeException = 4i32,
    None = 0i32,
}
#[cfg(feature = "System+Guid+ParseFailureKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Guid_ParseFailureKind => "System"
    ."Guid/ParseFailureKind"
);
