#[cfg(feature = "System+DateTimeFormat")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeFormat {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+DateTimeFormat")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeFormat => "System"
    ."DateTimeFormat"
);
#[cfg(feature = "System+DateTimeFormat")]
impl std::ops::Deref for crate::System::DateTimeFormat {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeFormat")]
impl std::ops::DerefMut for crate::System::DateTimeFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeFormat")]
impl crate::System::DateTimeFormat {
    pub fn Append2DigitNumber(
        result: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Append2DigitNumber", (result, val))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpandPredefinedFormat", (format, dateTime, dtfi, offset))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatCustomized", (dateTime, format, dtfi, offset, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatCustomizedRoundripTimeZone(
        dateTime: crate::System::DateTime,
        offset: crate::System::TimeSpan,
        result: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatCustomizedRoundripTimeZone", (dateTime, offset, result))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatCustomizedTimeZone",
                (dateTime, offset, format, tokenLen, timeOnly, result),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatDayOfWeek(
        dayOfWeek: i32,
        repeat: i32,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatDayOfWeek", (dayOfWeek, repeat, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatDigits_Gc_i32_i32_0(
        outputBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        value: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatDigits", (outputBuffer, value, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatDigits__cordl_bool1(
        outputBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        value: i32,
        len: i32,
        overrideLengthLimit: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatDigits", (outputBuffer, value, len, overrideLengthLimit))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatHebrewMonthName", (_cordl_time, month, repeatCount, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatMonth(
        month: i32,
        repeatCount: i32,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatMonth", (month, repeatCount, dtfi))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatStringBuilder", (dateTime, format, dtfi, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_DateTime_Gc_Gc0(
        dateTime: crate::System::DateTime,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dateTime, format, provider))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dateTime, format, provider, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRealFormat(
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRealFormat", (format, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn HebrewFormatDigits(
        outputBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HebrewFormatDigits", (outputBuffer, digits))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidFormatForLocal(
        format: crate::System::ReadOnlySpan_1<char>,
        dateTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvalidFormatForLocal", (format, dateTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsUseGenitiveForm(
        format: crate::System::ReadOnlySpan_1<char>,
        index: i32,
        tokenLen: i32,
        patternToMatch: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsUseGenitiveForm", (format, index, tokenLen, patternToMatch))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseNextChar(
        format: crate::System::ReadOnlySpan_1<char>,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseNextChar", (format, pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseQuoteString(
        format: crate::System::ReadOnlySpan_1<char>,
        pos: i32,
        result: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseQuoteString", (format, pos, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseRepeatPattern(
        format: crate::System::ReadOnlySpan_1<char>,
        pos: i32,
        patternChar: char,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseRepeatPattern", (format, pos, patternChar))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatO(
        dateTime: crate::System::DateTime,
        offset: crate::System::TimeSpan,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFormatO", (dateTime, offset, destination, charsWritten))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormatR(
        dateTime: crate::System::DateTime,
        offset: crate::System::TimeSpan,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryFormatR", (dateTime, offset, destination, charsWritten))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormat_DateTime_Span_1_ByRefMut_ReadOnlySpan_1_Gc0(
        dateTime: crate::System::DateTime,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryFormat",
                (dateTime, destination, charsWritten, format, provider),
            )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryFormat",
                (dateTime, destination, charsWritten, format, provider, offset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteDigits(
        value: u64,
        buffer: crate::System::Span_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteDigits", (value, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteFourDecimalDigits(
        value: u32,
        buffer: crate::System::Span_1<char>,
        startingIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteFourDecimalDigits", (value, buffer, startingIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTwoDecimalDigits(
        value: u32,
        destination: crate::System::Span_1<char>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteTwoDecimalDigits", (value, destination, offset))?;
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
