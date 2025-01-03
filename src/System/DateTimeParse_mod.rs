#[cfg(feature = "System+DateTimeParse")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeParse {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+DateTimeParse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeParse => "System"
    ."DateTimeParse"
);
#[cfg(feature = "System+DateTimeParse")]
impl std::ops::Deref for crate::System::DateTimeParse {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeParse")]
impl std::ops::DerefMut for crate::System::DateTimeParse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeParse")]
impl crate::System::DateTimeParse {
    #[cfg(feature = "System+DateTimeParse+DS")]
    pub type DS = crate::System::DateTimeParse_DS;
    #[cfg(feature = "System+DateTimeParse+DTT")]
    pub type DTT = crate::System::DateTimeParse_DTT;
    #[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
    pub type MatchNumberDelegate = crate::System::DateTimeParse_MatchNumberDelegate;
    #[cfg(feature = "System+DateTimeParse+TM")]
    pub type TM = crate::System::DateTimeParse_TM;
    pub fn AdjustHour(
        hour: quest_hook::libil2cpp::ByRefMut<i32>,
        timeMark: crate::System::DateTimeParse_TM,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustHour", (hour, timeMark))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustTimeMark(
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustTimeMark", (dtfi, raw))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustTimeZoneToLocal(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        bTimeOnly: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustTimeZoneToLocal", (result, bTimeOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustTimeZoneToUniversal(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustTimeZoneToUniversal", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckDefaultDateTime(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        cal: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Globalization::Calendar,
        >,
        styles: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckDefaultDateTime", (result, cal, styles))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckNewValue(
        currentValue: quest_hook::libil2cpp::ByRefMut<i32>,
        newValue: i32,
        patternChar: char,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckNewValue", (currentValue, newValue, patternChar, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn DateTimeOffsetTimeZonePostProcessing(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DateTimeOffsetTimeZonePostProcessing", (str, result, styles))?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineTimeZoneAdjustments(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: crate::System::Globalization::DateTimeStyles,
        bTimeOnly: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DetermineTimeZoneAdjustments", (str, result, styles, bTimeOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoStrictParse(
        s: crate::System::ReadOnlySpan_1<char>,
        formatParam: crate::System::ReadOnlySpan_1<char>,
        styles: crate::System::Globalization::DateTimeStyles,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoStrictParse", (s, formatParam, styles, dtfi, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpandPredefinedFormat(
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Globalization::DateTimeFormatInfo,
        >,
        parseInfo: quest_hook::libil2cpp::ByRefMut<crate::System::ParsingInfo>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpandPredefinedFormat", (format, dtfi, parseInfo, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDateOfDSN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDateOfDSN", (result, raw))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDateOfNDS(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDateOfNDS", (result, raw))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDateOfNNDS(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDateOfNNDS", (result, raw, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDateTimeNow(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::DateTimeStyles,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDateTimeNow", (result, styles))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDateTimeParseException(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDateTimeParseException", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfMN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::DateTimeStyles,
        >,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDayOfMN", (result, styles, raw, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfMNN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDayOfMNN", (result, raw, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfNM(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::DateTimeStyles,
        >,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDayOfNM", (result, styles, raw, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfNN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::DateTimeStyles,
        >,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDayOfNN", (result, styles, raw, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfNNN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDayOfNNN", (result, raw, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfNNY(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDayOfNNY", (result, raw, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfYM(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDayOfYM", (result, raw))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfYMN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDayOfYMN", (result, raw))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfYN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDayOfYN", (result, raw))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfYNN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDayOfYNN", (result, raw, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultYear(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::DateTimeStyles,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultYear", (result, styles))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHebrewDayOfNM(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHebrewDayOfNM", (result, raw, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetJapaneseCalendarDefaultInstance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::Calendar,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetJapaneseCalendarDefaultInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMonthDayOrder(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        order: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMonthDayOrder", (pattern, dtfi, order))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTaiwanCalendarDefaultInstance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::Calendar,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTaiwanCalendarDefaultInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeOfN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTimeOfN", (result, raw))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeOfNN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTimeOfNN", (result, raw))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeOfNNN(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTimeOfNNN", (result, raw))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeZoneName(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTimeZoneName", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetYearMonthDayOrder(
        datePattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        order: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetYearMonthDayOrder", (datePattern, dtfi, order))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetYearMonthOrder(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        order: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetYearMonthOrder", (pattern, dtfi, order))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleTimeZone(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HandleTimeZone", (str, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDigit(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDigit", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lex(
        dps: crate::System::DateTimeParse_DS,
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtok: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeToken>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        dtfi: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Globalization::DateTimeFormatInfo,
        >,
        styles: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lex", (dps, str, dtok, raw, result, dtfi, styles))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchAbbreviatedDayName(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchAbbreviatedDayName", (str, dtfi, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchAbbreviatedMonthName(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchAbbreviatedMonthName", (str, dtfi, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchAbbreviatedTimeMark(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeParse_TM>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchAbbreviatedTimeMark", (str, dtfi, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchDayName(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchDayName", (str, dtfi, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchEraName(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchEraName", (str, dtfi, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchHebrewDigits(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        digitLen: i32,
        number: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchHebrewDigits", (str, digitLen, number))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchMonthName(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchMonthName", (str, dtfi, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchTimeMark(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeParse_TM>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchTimeMark", (str, dtfi, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchWord(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchWord", (str, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseByFormat(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        format: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        parseInfo: quest_hook::libil2cpp::ByRefMut<crate::System::ParsingInfo>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseByFormat", (str, format, parseInfo, dtfi, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseDigits_ByRefMut0(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        digitLen: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseDigits", (str, digitLen, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseDigits_i32_ByRefMut1(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        minDigitLen: i32,
        maxDigitLen: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseDigits", (str, minDigitLen, maxDigitLen, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseExactMultiple(
        s: crate::System::ReadOnlySpan_1<char>,
        formats: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        style: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseExactMultiple", (s, formats, dtfi, style))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseExact_ByRefMut1(
        s: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        style: crate::System::Globalization::DateTimeStyles,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseExact", (s, format, dtfi, style, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseExact_ReadOnlySpan_1_ReadOnlySpan_1_DateTimeFormatInfo_DateTimeStyles0(
        s: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        style: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseExact", (s, format, dtfi, style))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseFraction(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseFraction", (str, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseFractionExact(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        maxDigitLen: i32,
        result: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseFractionExact", (str, maxDigitLen, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseISO8601(
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        styles: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseISO8601", (raw, str, styles, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseJapaneseEraStart(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseJapaneseEraStart", (str, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseSign(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseSign", (str, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTimeZone(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseTimeZone", (str, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTimeZoneOffset(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        len: i32,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseTimeZoneOffset", (str, len, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_ByRefMut1(
        s: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        styles: crate::System::Globalization::DateTimeStyles,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (s, dtfi, styles, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_ReadOnlySpan_1_DateTimeFormatInfo_DateTimeStyles0(
        s: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        styles: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (s, dtfi, styles))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessDateTimeSuffix(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtok: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeToken>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessDateTimeSuffix", (result, raw, dtok))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessHebrewTerminalState(
        dps: crate::System::DateTimeParse_DS,
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::DateTimeStyles,
        >,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ProcessHebrewTerminalState",
                (dps, str, result, styles, raw, dtfi),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTerminalState(
        dps: crate::System::DateTimeParse_DS,
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        styles: quest_hook::libil2cpp::ByRefMut<
            crate::System::Globalization::DateTimeStyles,
        >,
        raw: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeRawInfo>,
        dtfi: quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessTerminalState", (dps, str, result, styles, raw, dtfi))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDateDMY(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        day: i32,
        month: i32,
        year: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDateDMY", (result, day, month, year))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDateMDY(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        month: i32,
        day: i32,
        year: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDateMDY", (result, month, day, year))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDateYDM(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        year: i32,
        day: i32,
        month: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDateYDM", (result, year, day, month))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDateYMD(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        year: i32,
        month: i32,
        day: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDateYMD", (result, year, month, day))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryAdjustYear(
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
        year: i32,
        adjustedYear: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryAdjustYear", (result, year, adjustedYear))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExactMultiple_ByRefMut0(
        s: crate::System::ReadOnlySpan_1<char>,
        formats: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        style: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseExactMultiple", (s, formats, dtfi, style, result, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExactMultiple_ReadOnlySpan_1_Il2CppArray_DateTimeFormatInfo_DateTimeStyles_ByRefMut1(
        s: crate::System::ReadOnlySpan_1<char>,
        formats: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        style: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseExactMultiple", (s, formats, dtfi, style, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact_ByRefMut1(
        s: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        style: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseExact", (s, format, dtfi, style, result, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact_ReadOnlySpan_1_ReadOnlySpan_1_DateTimeFormatInfo_DateTimeStyles_ByRefMut0(
        s: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        style: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseExact", (s, format, dtfi, style, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact_ReadOnlySpan_1_ReadOnlySpan_1_DateTimeFormatInfo_DateTimeStyles_ByRefMut2(
        s: crate::System::ReadOnlySpan_1<char>,
        format: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        style: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseExact", (s, format, dtfi, style, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseQuoteString(
        format: crate::System::ReadOnlySpan_1<char>,
        pos: i32,
        result: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        returnValue: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseQuoteString", (format, pos, result, returnValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ByRefMut1(
        s: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        styles: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (s, dtfi, styles, result, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ReadOnlySpan_1_DateTimeFormatInfo_DateTimeStyles_ByRefMut0(
        s: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        styles: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (s, dtfi, styles, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ReadOnlySpan_1_DateTimeFormatInfo_DateTimeStyles_ByRefMut2(
        s: crate::System::ReadOnlySpan_1<char>,
        dtfi: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        >,
        styles: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (s, dtfi, styles, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyValidPunctuation(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VerifyValidPunctuation", (str))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+DateTimeParse")]
impl quest_hook::libil2cpp::ObjectType for crate::System::DateTimeParse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+DateTimeParse+DS")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeParse_DS {
    BEGIN = 0i32,
    DX_DS = 26i32,
    DX_DSN = 27i32,
    DX_MN = 23i32,
    DX_MNN = 25i32,
    DX_NDS = 28i32,
    DX_NM = 24i32,
    DX_NN = 21i32,
    DX_NNDS = 29i32,
    DX_NNN = 22i32,
    DX_NNY = 38i32,
    DX_YM = 33i32,
    DX_YMN = 31i32,
    DX_YN = 32i32,
    DX_YNN = 30i32,
    D_M = 6i32,
    D_MN = 7i32,
    D_MNd = 9i32,
    D_NDS = 10i32,
    D_NM = 8i32,
    D_NN = 4i32,
    D_NNd = 5i32,
    D_Nd = 3i32,
    D_S = 16i32,
    D_Y = 11i32,
    D_YM = 14i32,
    D_YMd = 15i32,
    D_YN = 12i32,
    D_YNd = 13i32,
    ERROR = 20i32,
    N = 1i32,
    NN = 2i32,
    TX_N = 34i32,
    TX_NN = 35i32,
    TX_NNN = 36i32,
    TX_TS = 37i32,
    T_NNt = 19i32,
    T_Nt = 18i32,
    T_S = 17i32,
}
#[cfg(feature = "System+DateTimeParse+DS")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeParse_DS => "System"
    ."DateTimeParse/DS"
);
#[cfg(feature = "System+DateTimeParse+DTT")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeParse_DTT {
    DayOfWeek = 11i32,
    End = 0i32,
    Era = 16i32,
    Max = 20i32,
    MonthDatesep = 8i32,
    MonthEnd = 6i32,
    MonthSpace = 7i32,
    NumAmpm = 2i32,
    NumDatesep = 4i32,
    NumDatesuff = 9i32,
    NumEnd = 1i32,
    NumLocalTimeMark = 19i32,
    NumSpace = 3i32,
    NumTimesep = 5i32,
    NumTimesuff = 10i32,
    NumUTCTimeMark = 17i32,
    TimeZone = 15i32,
    Unk = 18i32,
    YearDateSep = 13i32,
    YearEnd = 14i32,
    YearSpace = 12i32,
}
#[cfg(feature = "System+DateTimeParse+DTT")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeParse_DTT => "System"
    ."DateTimeParse/DTT"
);
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeParse_MatchNumberDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeParse_MatchNumberDelegate =>
    "System"."DateTimeParse/MatchNumberDelegate"
);
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
impl std::ops::Deref for crate::System::DateTimeParse_MatchNumberDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
impl std::ops::DerefMut for crate::System::DateTimeParse_MatchNumberDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
impl crate::System::DateTimeParse_MatchNumberDelegate {
    pub fn Invoke(
        &mut self,
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        digitLen: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (str, digitLen, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+DateTimeParse+MatchNumberDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::DateTimeParse_MatchNumberDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+DateTimeParse+TM")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTimeParse_TM {
    AM = 0i32,
    NotSet = -1i32,
    PM = 1i32,
}
#[cfg(feature = "System+DateTimeParse+TM")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeParse_TM => "System"
    ."DateTimeParse/TM"
);
