#[cfg(feature = "System+Number")]
#[repr(C)]
#[derive(Debug)]
pub struct Number {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Number")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Number => "System"."Number"
);
#[cfg(feature = "System+Number")]
impl std::ops::Deref for crate::System::Number {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Number")]
impl std::ops::DerefMut for crate::System::Number {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Number")]
impl crate::System::Number {
    #[cfg(feature = "System+Number+NumberBuffer")]
    pub type NumberBuffer = crate::System::Number_NumberBuffer;
    pub fn DecimalToNumber(
        value: crate::System::Decimal,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecimalToNumber", (value, number))?;
        Ok(__cordl_ret.into())
    }
    pub fn DigitsToInt(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DigitsToInt", (p, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoubleToNumber(
        value: f64,
        precision: i32,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoubleToNumber", (value, precision, number))?;
        Ok(__cordl_ret.into())
    }
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
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        nMinDigits: i32,
        nMaxDigits: i32,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatCurrency", (sb, number, nMinDigits, nMaxDigits, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatDecimal(
        value: crate::System::Decimal,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatDecimal", (value, format, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatDouble_ByRefMut_f64_ReadOnlySpan_1_NumberFormatInfo1(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        value: f64,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatDouble", (sb, value, format, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatDouble_f64_Il2CppString_NumberFormatInfo0(
        value: f64,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatDouble", (value, format, info))?;
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
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
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
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
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
    pub fn FormatInt32(
        value: i32,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatInt32", (value, format, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatInt64(
        value: i64,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatInt64", (value, format, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatNumber(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
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
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
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
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
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
    pub fn FormatSingle_ByRefMut_f32_ReadOnlySpan_1_NumberFormatInfo1(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        value: f32,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatSingle", (sb, value, format, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatSingle_f32_Il2CppString_NumberFormatInfo0(
        value: f32,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatSingle", (value, format, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatUInt32(
        value: u32,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatUInt32", (value, format, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatUInt64(
        value: u64,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatUInt64", (value, format, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn High32(value: u64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("High32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Int32ToHexChars(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: u32,
        hexBase: i32,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Int32ToHexChars", (buffer, value, hexBase, digits))?;
        Ok(__cordl_ret.into())
    }
    pub fn Int32ToHexStr(
        value: i32,
        hexBase: char,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Int32ToHexStr", (value, hexBase, digits))?;
        Ok(__cordl_ret.into())
    }
    pub fn Int32ToNumber(
        value: i32,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Int32ToNumber", (value, number))?;
        Ok(__cordl_ret.into())
    }
    pub fn Int64DivMod1E9(
        value: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Int64DivMod1E9", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Int64ToHexStr(
        value: i64,
        hexBase: char,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Int64ToHexStr", (value, hexBase, digits))?;
        Ok(__cordl_ret.into())
    }
    pub fn Int64ToNumber(
        input: i64,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Int64ToNumber", (input, number))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDigit(ch: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDigit", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWhite(ch: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsWhite", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn Low32(value: u64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Low32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchChars(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pEnd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchChars", (p, pEnd, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Mul32x32To64(a: u32, b: u32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Mul32x32To64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Mul64Lossy(
        a: u64,
        b: u64,
        pexp: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Mul64Lossy", (a, b, pexp))?;
        Ok(__cordl_ret.into())
    }
    pub fn NegativeInt32ToDecStr(
        value: i32,
        digits: i32,
        sNegative: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NegativeInt32ToDecStr", (value, digits, sNegative))?;
        Ok(__cordl_ret.into())
    }
    pub fn NegativeInt64ToDecStr(
        input: i64,
        digits: i32,
        sNegative: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NegativeInt64ToDecStr", (input, digits, sNegative))?;
        Ok(__cordl_ret.into())
    }
    pub fn NumberBufferToDecimal(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        value: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NumberBufferToDecimal", (number, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn NumberBufferToDouble(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        value: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NumberBufferToDouble", (number, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn NumberToDouble(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NumberToDouble", (number))?;
        Ok(__cordl_ret.into())
    }
    pub fn NumberToInt32(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NumberToInt32", (number, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn NumberToInt64(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        value: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NumberToInt64", (number, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn NumberToString(
        sb: quest_hook::libil2cpp::ByRefMut<crate::System::Text::ValueStringBuilder>,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
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
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NumberToStringFormat", (sb, number, format, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn NumberToUInt32(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        value: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NumberToUInt32", (number, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn NumberToUInt64(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        value: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NumberToUInt64", (number, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseDecimal(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseDecimal", (value, styles, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseDouble(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseDouble", (value, styles, info))?;
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
    pub fn ParseInt32(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseInt32", (value, styles, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseInt64(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseInt64", (value, styles, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNumber(
        str: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppObject>,
        strEnd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        styles: crate::System::Globalization::NumberStyles,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        parseDecimal: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseNumber", (str, strEnd, styles, number, info, parseDecimal))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseSingle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseSingle", (value, styles, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseUInt32(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseUInt32", (value, styles, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseUInt64(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseUInt64", (value, styles, info))?;
        Ok(__cordl_ret.into())
    }
    pub fn RoundNumber(
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RoundNumber", (number, pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn StringToNumber(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        parseDecimal: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringToNumber", (value, styles, number, info, parseDecimal))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowOverflowOrFormatException(
        overflow: bool,
        overflowResourceKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowOverflowOrFormatException", (overflow, overflowResourceKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrailingZeros(
        value: crate::System::ReadOnlySpan_1<char>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TrailingZeros", (value, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryCopyTo(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryCopyTo", (source, destination, charsWritten))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatDecimal(
        value: crate::System::Decimal,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryFormatDecimal",
                (value, format, info, destination, charsWritten),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatDouble(
        value: f64,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryFormatDouble",
                (value, format, info, destination, charsWritten),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatInt32(
        value: i32,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryFormatInt32",
                (value, format, provider, destination, charsWritten),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatInt64(
        value: i64,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryFormatInt64",
                (value, format, provider, destination, charsWritten),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatSingle(
        value: f32,
        format: crate::System::ReadOnlySpan_1<char>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryFormatSingle",
                (value, format, info, destination, charsWritten),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatUInt32(
        value: u32,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryFormatUInt32",
                (value, format, provider, destination, charsWritten),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatUInt64(
        value: u64,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryFormatUInt64",
                (value, format, provider, destination, charsWritten),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryInt32ToHexStr(
        value: i32,
        hexBase: char,
        digits: i32,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryInt32ToHexStr",
                (value, hexBase, digits, destination, charsWritten),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryInt64ToHexStr(
        value: i64,
        hexBase: char,
        digits: i32,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryInt64ToHexStr",
                (value, hexBase, digits, destination, charsWritten),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryNegativeInt32ToDecStr(
        value: i32,
        digits: i32,
        sNegative: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryNegativeInt32ToDecStr",
                (value, digits, sNegative, destination, charsWritten),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryNegativeInt64ToDecStr(
        input: i64,
        digits: i32,
        sNegative: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryNegativeInt64ToDecStr",
                (input, digits, sNegative, destination, charsWritten),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDecimal(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseDecimal", (value, styles, info, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDouble(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseDouble", (value, styles, info, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseInt32(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseInt32", (value, styles, info, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseInt32IntegerStyle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
        failureIsOverflow: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryParseInt32IntegerStyle",
                (value, styles, info, result, failureIsOverflow),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseInt64(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseInt64", (value, styles, info, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseInt64IntegerStyle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<i64>,
        failureIsOverflow: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryParseInt64IntegerStyle",
                (value, styles, info, result, failureIsOverflow),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseSingle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseSingle", (value, styles, info, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseUInt32(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseUInt32", (value, styles, info, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseUInt32HexNumberStyle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<u32>,
        failureIsOverflow: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryParseUInt32HexNumberStyle",
                (value, styles, info, result, failureIsOverflow),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseUInt32IntegerStyle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<u32>,
        failureIsOverflow: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryParseUInt32IntegerStyle",
                (value, styles, info, result, failureIsOverflow),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseUInt64(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseUInt64", (value, styles, info, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseUInt64HexNumberStyle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<u64>,
        failureIsOverflow: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryParseUInt64HexNumberStyle",
                (value, styles, info, result, failureIsOverflow),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseUInt64IntegerStyle(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<u64>,
        failureIsOverflow: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryParseUInt64IntegerStyle",
                (value, styles, info, result, failureIsOverflow),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryStringToNumber(
        value: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::NumberStyles,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        parseDecimal: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryStringToNumber", (value, styles, number, info, parseDecimal))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryUInt32ToDecStr(
        value: u32,
        digits: i32,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryUInt32ToDecStr", (value, digits, destination, charsWritten))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryUInt64ToDecStr(
        value: u64,
        digits: i32,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryUInt64ToDecStr", (value, digits, destination, charsWritten))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32ToDecChars(
        bufferEnd: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: u32,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt32ToDecChars", (bufferEnd, value, digits))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32ToDecStr(
        value: u32,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt32ToDecStr", (value, digits))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32ToNumber(
        value: u32,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt32ToNumber", (value, number))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64ToDecStr(
        value: u64,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt64ToDecStr", (value, digits))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64ToNumber(
        value: u64,
        number: quest_hook::libil2cpp::ByRefMut<crate::System::Number_NumberBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt64ToNumber", (value, number))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs(value: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Number")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Number {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NumberBuffer_Number_DigitsAndNullTerminator {}
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::NumberBuffer_Number_DigitsAndNullTerminator => "System"
    ."Number/NumberBuffer/DigitsAndNullTerminator"
);
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::NumberBuffer_Number_DigitsAndNullTerminator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
impl crate::System::NumberBuffer_Number_DigitsAndNullTerminator {}
#[cfg(feature = "System+Number+NumberBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Number_NumberBuffer {
    pub precision: i32,
    pub scale: i32,
    pub _sign: i32,
    pub _digits: crate::System::NumberBuffer_Number_DigitsAndNullTerminator,
    pub _allDigits: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Number+NumberBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Number_NumberBuffer => "System"
    ."Number/NumberBuffer"
);
#[cfg(feature = "System+Number+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Number_NumberBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Number+NumberBuffer")]
impl crate::System::Number_NumberBuffer {
    #[cfg(feature = "System+Number+NumberBuffer+DigitsAndNullTerminator")]
    pub type DigitsAndNullTerminator = crate::System::NumberBuffer_Number_DigitsAndNullTerminator;
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
    pub fn get_sign(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_sign",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sign(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_sign",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
