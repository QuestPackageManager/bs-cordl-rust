#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "DateTimeUtils";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
impl crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    pub const DaysPer100Years: i32 = 36524i32;
    pub const DaysPer400Years: i32 = 146097i32;
    pub const DaysPer4Years: i32 = 1461i32;
    pub const DaysPerYear: i32 = 365i32;
    pub const IsoDateFormat: &'static str = "yyyy-MM-ddTHH:mm:ss.FFFFFFFK";
    pub const TicksPerDay: i64 = 864000000000i64;
    pub fn ConvertDateTimeToJavaScriptTicks_DateTime1(
        dateTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertDateTimeToJavaScriptTicks", (dateTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertDateTimeToJavaScriptTicks_TimeSpan0(
        dateTime: crate::System::DateTime,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertDateTimeToJavaScriptTicks", (dateTime, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertDateTimeToJavaScriptTicks__cordl_bool2(
        dateTime: crate::System::DateTime,
        convertToUtc: bool,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertDateTimeToJavaScriptTicks", (dateTime, convertToUtc))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertJavaScriptTicksToDateTime(
        javaScriptTicks: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertJavaScriptTicksToDateTime", (javaScriptTicks))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyIntToCharArray(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        value: i32,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyIntToCharArray", (chars, start, value, digits))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDateTime(
        dateTimeParser: crate::Newtonsoft::Json::Utilities::DateTimeParser,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDateTime", (dateTimeParser))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureDateTime(
        value: crate::System::DateTime,
        timeZone: crate::Newtonsoft::Json::DateTimeZoneHandling,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureDateTime", (value, timeZone))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDateValues(
        td: crate::System::DateTime,
        year: quest_hook::libil2cpp::ByRefMut<i32>,
        month: quest_hook::libil2cpp::ByRefMut<i32>,
        day: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDateValues", (td, year, month, day))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUtcOffset(
        d: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUtcOffset", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn SwitchToLocalTime(
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SwitchToLocalTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SwitchToUtcTime(
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SwitchToUtcTime", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSerializationMode(
        kind: crate::System::DateTimeKind,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::XmlDateTimeSerializationMode,
    > {
        let __cordl_ret: crate::System::Xml::XmlDateTimeSerializationMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSerializationMode", (kind))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUniversalTicks_DateTime0(
        dateTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUniversalTicks", (dateTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUniversalTicks_TimeSpan1(
        dateTime: crate::System::DateTime,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUniversalTicks", (dateTime, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeExact(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dateTimeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
        dateFormatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryParseDateTimeExact",
                (text, dateTimeZoneHandling, dateFormatString, culture, dt),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeIso(
        text: crate::Newtonsoft::Json::Utilities::StringReference,
        dateTimeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseDateTimeIso", (text, dateTimeZoneHandling, dt))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeMicrosoft(
        text: crate::Newtonsoft::Json::Utilities::StringReference,
        dateTimeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseDateTimeMicrosoft", (text, dateTimeZoneHandling, dt))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeOffsetExact(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dateFormatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryParseDateTimeOffsetExact",
                (text, dateFormatString, culture, dt),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeOffsetIso(
        text: crate::Newtonsoft::Json::Utilities::StringReference,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseDateTimeOffsetIso", (text, dt))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeOffsetMicrosoft(
        text: crate::Newtonsoft::Json::Utilities::StringReference,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseDateTimeOffsetMicrosoft", (text, dt))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeOffset_Il2CppString1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dateFormatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseDateTimeOffset", (s, dateFormatString, culture, dt))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTimeOffset_StringReference0(
        s: crate::Newtonsoft::Json::Utilities::StringReference,
        dateFormatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseDateTimeOffset", (s, dateFormatString, culture, dt))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTime_Il2CppString1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dateTimeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
        dateFormatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryParseDateTime",
                (s, dateTimeZoneHandling, dateFormatString, culture, dt),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDateTime_StringReference0(
        s: crate::Newtonsoft::Json::Utilities::StringReference,
        dateTimeZoneHandling: crate::Newtonsoft::Json::DateTimeZoneHandling,
        dateFormatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        dt: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryParseDateTime",
                (s, dateTimeZoneHandling, dateFormatString, culture, dt),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseMicrosoftDate(
        text: crate::Newtonsoft::Json::Utilities::StringReference,
        ticks: quest_hook::libil2cpp::ByRefMut<i64>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
        kind: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeKind>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseMicrosoftDate", (text, ticks, offset, kind))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryReadOffset(
        offsetText: crate::Newtonsoft::Json::Utilities::StringReference,
        startIndex: i32,
        offset: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryReadOffset", (offsetText, startIndex, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn UniversalTicksToJavaScriptTicks(
        universalTicks: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UniversalTicksToJavaScriptTicks", (universalTicks))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteDateTimeOffset(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        offset: crate::System::TimeSpan,
        format: crate::Newtonsoft::Json::DateFormatHandling,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteDateTimeOffset", (chars, start, offset, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteDateTimeOffsetString(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        value: crate::System::DateTimeOffset,
        format: crate::Newtonsoft::Json::DateFormatHandling,
        formatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteDateTimeOffsetString",
                (writer, value, format, formatString, culture),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteDateTimeString_Il2CppArray_i32_DateTime_Nullable_1_DateTimeKind_DateFormatHandling1(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        value: crate::System::DateTime,
        offset: crate::System::Nullable_1<crate::System::TimeSpan>,
        kind: crate::System::DateTimeKind,
        format: crate::Newtonsoft::Json::DateFormatHandling,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteDateTimeString", (chars, start, value, offset, kind, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteDateTimeString_TextWriter_DateTime_DateFormatHandling_Il2CppString_CultureInfo0(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        value: crate::System::DateTime,
        format: crate::Newtonsoft::Json::DateFormatHandling,
        formatString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteDateTimeString",
                (writer, value, format, formatString, culture),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteDefaultIsoDate(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        dt: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteDefaultIsoDate", (chars, start, dt))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
