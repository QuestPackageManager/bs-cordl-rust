#[cfg(feature = "System+Globalization+FormatProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct FormatProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+FormatProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::FormatProvider =>
    "System.Globalization"."FormatProvider"
);
#[cfg(feature = "System+Globalization+FormatProvider")]
impl std::ops::Deref for crate::System::Globalization::FormatProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider")]
impl std::ops::DerefMut for crate::System::Globalization::FormatProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider")]
impl crate::System::Globalization::FormatProvider {
    #[cfg(feature = "System+Globalization+FormatProvider+Number")]
    pub type Number = crate::System::Globalization::FormatProvider_Number;
    pub fn FormatBigInteger(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        precision: i32,
        scale: i32,
        sign: bool,
        format: crate::System::ReadOnlySpan_1<char>,
        numberFormatInfo: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::NumberFormatInfo,
        >,
        digits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatBigInteger",
                (
                    sb,
                    precision,
                    scale,
                    sign,
                    format,
                    numberFormatInfo,
                    digits,
                    startIndex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryStringToBigInteger(
        s: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        numberFormatInfo: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::NumberFormatInfo,
        >,
        receiver: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        precision: quest_hook::libil2cpp::ByRefMut<i32>,
        scale: quest_hook::libil2cpp::ByRefMut<i32>,
        sign: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryStringToBigInteger",
                (s, styles, numberFormatInfo, receiver, precision, scale, sign),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+FormatProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::FormatProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
#[repr(C)]
#[derive(Debug)]
pub struct FormatProvider_Number {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::FormatProvider_Number =>
    "System.Globalization"."FormatProvider/Number"
);
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
impl std::ops::Deref for crate::System::Globalization::FormatProvider_Number {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
impl std::ops::DerefMut for crate::System::Globalization::FormatProvider_Number {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
impl crate::System::Globalization::FormatProvider_Number {
    #[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
    pub type NumberBuffer = crate::System::Globalization::Number_FormatProvider_NumberBuffer;
    pub fn FindSection(
        format: crate::System::ReadOnlySpan_1<char>,
        section: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindSection", (format, section))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatCurrency(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatCurrency", (sb, number, nMinDigits, nMaxDigits, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatExponent(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        value: i32,
        expChar: char,
        minDigits: i32,
        positiveSign: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatExponent",
                (sb, info, value, expChar, minDigits, positiveSign),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatFixed(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        groupDigits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        sDecimal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sGroup: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatFixed",
                (sb, number, nMinDigits, nMaxDigits, info, groupDigits, sDecimal, sGroup),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatGeneral(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        expChar: char,
        bSuppressScientific: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatGeneral",
                (sb, number, nMinDigits, nMaxDigits, info, expChar, bSuppressScientific),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatNumber(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatNumber", (sb, number, nMinDigits, nMaxDigits, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatPercent(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatPercent", (sb, number, nMinDigits, nMaxDigits, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatScientific(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        expChar: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatScientific",
                (sb, number, nMinDigits, nMaxDigits, info, expChar),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Int32ToDecChars(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
        value: u32,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Int32ToDecChars", (buffer, index, value, digits))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWhite(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsWhite", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchChars_Il2CppObject1(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pEnd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchChars", (p, pEnd, str))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchChars_Il2CppString0(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pEnd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchChars", (p, pEnd, str))?;
        Ok(__cordl_ret.into())
    }
    pub fn NumberToString(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        format: char,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        isDecimal: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "NumberToString",
                (sb, number, format, nMaxDigits, info, isDecimal),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn NumberToStringFormat(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NumberToStringFormat", (sb, number, format, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseFormatSpecifier(
        format: crate::System::ReadOnlySpan_1<char>,
        digits: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseFormatSpecifier", (format, digits))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumber(
        str: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        strEnd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        options: crate::System::Globalization::NumberStyles,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        numfmt: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::NumberFormatInfo,
        >,
        parseDecimal: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ParseNumber",
                (str, strEnd, options, number, sb, numfmt, parseDecimal),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RoundNumber(
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RoundNumber", (number, pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrailingZeros(
        s: crate::System::ReadOnlySpan_1<char>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TrailingZeros", (s, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryStringToNumber(
        str: crate::System::ReadOnlySpan_1<char>,
        options: crate::System::Globalization::NumberStyles,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::Number_FormatProvider_NumberBuffer,
        >,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        numfmt: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::NumberFormatInfo,
        >,
        parseDecimal: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryStringToNumber",
                (str, options, number, sb, numfmt, parseDecimal),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn wcslen(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("wcslen", (s))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::FormatProvider_Number {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Number_FormatProvider_NumberBuffer {
    pub precision: i32,
    pub scale: i32,
    pub sign: bool,
    pub overrideDigits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::Number_FormatProvider_NumberBuffer => "System.Globalization"
    ."FormatProvider/Number/NumberBuffer"
);
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::Number_FormatProvider_NumberBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+FormatProvider+Number+NumberBuffer")]
impl crate::System::Globalization::Number_FormatProvider_NumberBuffer {
    pub fn get_digits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_digits", ())?;
        Ok(__cordl_ret.into())
    }
}
