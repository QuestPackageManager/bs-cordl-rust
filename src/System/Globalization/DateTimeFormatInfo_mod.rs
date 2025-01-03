#[cfg(feature = "System+Globalization+DateTimeFormatInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeFormatInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cultureData: quest_hook::libil2cpp::Gc<
        crate::System::Globalization::CultureData,
    >,
    pub _name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _langName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _compareInfo: quest_hook::libil2cpp::Gc<
        crate::System::Globalization::CompareInfo,
    >,
    pub _cultureInfo: quest_hook::libil2cpp::Gc<
        crate::System::Globalization::CultureInfo,
    >,
    pub amDesignator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub pmDesignator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub dateSeparator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub generalShortTimePattern: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub generalLongTimePattern: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub timeSeparator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub monthDayPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub dateTimeOffsetPattern: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub calendar: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    pub firstDayOfWeek: i32,
    pub calendarWeekRule: i32,
    pub fullDateTimePattern: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub abbreviatedDayNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub m_superShortDayNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub dayNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub abbreviatedMonthNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub monthNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub genitiveMonthNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub m_genitiveAbbreviatedMonthNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub leapYearMonthNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub longDatePattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub shortDatePattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub yearMonthPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub longTimePattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub shortTimePattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub allYearMonthPatterns: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub allShortDatePatterns: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub allLongDatePatterns: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub allShortTimePatterns: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub allLongTimePatterns: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub m_eraNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub m_abbrevEraNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub m_abbrevEnglishEraNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub optionalCalendars: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::System::Globalization::CalendarId>,
    >,
    pub _isReadOnly: bool,
    pub formatFlags: crate::System::Globalization::DateTimeFormatFlags,
    pub _fullTimeSpanPositivePattern: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _fullTimeSpanNegativePattern: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _dtfiTokenHash: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
        >,
    >,
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::DateTimeFormatInfo =>
    "System.Globalization"."DateTimeFormatInfo"
);
#[cfg(feature = "System+Globalization+DateTimeFormatInfo")]
impl std::ops::Deref for crate::System::Globalization::DateTimeFormatInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo")]
impl std::ops::DerefMut for crate::System::Globalization::DateTimeFormatInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo")]
impl crate::System::Globalization::DateTimeFormatInfo {
    pub const CJKDaySuff: &'static str = "\\u{65e5}";
    pub const CJKHourSuff: &'static str = "\\u{6642}";
    pub const CJKMinuteSuff: &'static str = "\\u{5206}";
    pub const CJKMonthSuff: &'static str = "\\u{6708}";
    pub const CJKSecondSuff: &'static str = "\\u{79d2}";
    pub const CJKYearSuff: &'static str = "\\u{5e74}";
    pub const ChineseHourSuff: &'static str = "\\u{65f6}";
    pub const DEFAULT_ALL_DATETIMES_SIZE: i32 = 132i32;
    pub const EnglishLangName: &'static str = "en";
    pub const GMTName: &'static str = "GMT";
    pub const IgnorableComma: &'static str = ",";
    pub const IgnorablePeriod: &'static str = ".";
    pub const JapaneseEraStart: &'static str = "\\u{5143}";
    pub const JapaneseLangName: &'static str = "ja";
    pub const KoreanDaySuff: &'static str = "\\u{c77c}";
    pub const KoreanHourSuff: &'static str = "\\u{c2dc}";
    pub const KoreanLangName: &'static str = "ko";
    pub const KoreanMinuteSuff: &'static str = "\\u{bd84}";
    pub const KoreanMonthSuff: &'static str = "\\u{c6d4}";
    pub const KoreanSecondSuff: &'static str = "\\u{cd08}";
    pub const KoreanYearSuff: &'static str = "\\u{b144}";
    pub const LocalTimeMark: &'static str = "T";
    pub const RoundtripDateTimeUnfixed: &'static str = "yyyy\\'-\\'MM\\'-\\'ddTHH\\':\\'mm\\':\\'ss zzz";
    pub const RoundtripFormat: &'static str = "yyyy\\'-\\'MM\\'-\\'dd\\'T\\'HH\\':\\'mm\\':\\'ss.fffffffK";
    pub const SECOND_PRIME: i32 = 197i32;
    pub const TOKEN_HASH_SIZE: i32 = 199i32;
    pub const ZuluName: &'static str = "Z";
    pub const dateSeparatorOrTimeZoneOffset: &'static str = "-";
    pub const invariantDateSeparator: &'static str = "/";
    pub const invariantTimeSeparator: &'static str = ":";
    pub const rfc1123Pattern: &'static str = "ddd, dd MMM yyyy HH\\':\\'mm\\':\\'ss \\'GMT\\'";
    pub const sortableDateTimePattern: &'static str = "yyyy\\'-\\'MM\\'-\\'dd\\'T\\'HH\\':\\'mm\\':\\'ss";
    pub const universalSortableDateTimePattern: &'static str = "yyyy\\'-\\'MM\\'-\\'dd HH\\':\\'mm\\':\\'ss\\'Z\\'";
    #[cfg(feature = "System+Globalization+DateTimeFormatInfo+TokenHashValue")]
    pub type TokenHashValue = crate::System::Globalization::DateTimeFormatInfo_TokenHashValue;
    pub fn AddMonthNames(
        &mut self,
        temp: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
            >,
        >,
        monthPostfix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddMonthNames", (temp, monthPostfix))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearTokenHashTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearTokenHashTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareStringIgnoreCaseOptimized(
        &mut self,
        string1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset1: i32,
        length1: i32,
        string2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset2: i32,
        length2: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CompareStringIgnoreCaseOptimized",
                (string1, offset1, length1, string2, offset2, length2),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTokenHashTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
            >,
        > = __cordl_object.invoke("CreateTokenHashTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAbbreviatedDayName(
        &mut self,
        dayofweek: crate::System::DayOfWeek,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetAbbreviatedDayName", (dayofweek))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAbbreviatedEraName(
        &mut self,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetAbbreviatedEraName", (era))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAbbreviatedMonthName(
        &mut self,
        month: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetAbbreviatedMonthName", (month))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllDateTimePatterns(
        &mut self,
        format: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("GetAllDateTimePatterns", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCombinedPatterns(
        patterns1: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        patterns2: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        connectString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCombinedPatterns", (patterns1, patterns2, connectString))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDayName(
        &mut self,
        dayofweek: crate::System::DayOfWeek,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetDayName", (dayofweek))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEraName(
        &mut self,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetEraName", (era))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFormat(
        &mut self,
        formatType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetFormat", (formatType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance(
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInstance", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetJapaneseCalendarDTFI() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetJapaneseCalendarDTFI", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMergedPatterns(
        patterns: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        defaultPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMergedPatterns", (patterns, defaultPattern))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMonthName(
        &mut self,
        month: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetMonthName", (month))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTaiwanCalendarDTFI() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTaiwanCalendarDTFI", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeFormatFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::DateTimeFormatFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Globalization::DateTimeFormatFlags = __cordl_object
            .invoke("InitializeFormatFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeOverridableProperties(
        &mut self,
        cultureData: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureData,
        >,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeOverridableProperties", (cultureData, calendarId))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertAtCurrentHashNode(
        &mut self,
        hashTable: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
            >,
        >,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ch: char,
        tokenType: crate::System::TokenType,
        tokenValue: i32,
        pos: i32,
        hashcode: i32,
        hashProbe: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InsertAtCurrentHashNode",
                (hashTable, str, ch, tokenType, tokenValue, pos, hashcode, hashProbe),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertHash(
        &mut self,
        hashTable: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
            >,
        >,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tokenType: crate::System::TokenType,
        tokenValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertHash", (hashTable, str, tokenType, tokenValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAllowedJapaneseTokenFollowedByNonSpaceLetter(
        &mut self,
        tokenString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nextCh: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "IsAllowedJapaneseTokenFollowedByNonSpaceLetter",
                (tokenString, nextCh),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHebrewChar(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHebrewChar", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsJapaneseCalendar(
        calendar: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsJapaneseCalendar", (calendar))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_CultureData_Calendar1(
        cultureData: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureData,
        >,
        cal: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cultureData, cal))?;
        Ok(__cordl_object.into())
    }
    pub fn PopulateSpecialTokenHashTable(
        &mut self,
        temp: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
            >,
        >,
        useDateSepAsIgnorableSymbol: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PopulateSpecialTokenHashTable",
                (temp, useDateSepAsIgnorableSymbol),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Tokenize(
        &mut self,
        TokenMask: crate::System::TokenType,
        tokenType: quest_hook::libil2cpp::ByRefMut<crate::System::TokenType>,
        tokenValue: quest_hook::libil2cpp::ByRefMut<i32>,
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Tokenize", (TokenMask, tokenType, tokenValue, str))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseHebrewNumber(
        str: quest_hook::libil2cpp::ByRefMut<crate::System::__DTString>,
        badFormat: quest_hook::libil2cpp::ByRefMut<bool>,
        number: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseHebrewNumber", (str, badFormat, number))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateStyles(
        style: crate::System::Globalization::DateTimeStyles,
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateStyles", (style, parameterName))?;
        Ok(__cordl_ret.into())
    }
    pub fn YearMonthAdjustment(
        &mut self,
        year: quest_hook::libil2cpp::ByRefMut<i32>,
        month: quest_hook::libil2cpp::ByRefMut<i32>,
        parsedMonthName: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("YearMonthAdjustment", (year, month, parsedMonthName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CultureData_Calendar1(
        &mut self,
        cultureData: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureData,
        >,
        cal: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cultureData, cal))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AMDesignator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_AMDesignator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AbbreviatedDayNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_AbbreviatedDayNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AbbreviatedEnglishEraNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_AbbreviatedEnglishEraNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AbbreviatedEraNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_AbbreviatedEraNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AbbreviatedMonthNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_AbbreviatedMonthNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AllLongDatePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_AllLongDatePatterns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AllLongTimePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_AllLongTimePatterns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AllShortDatePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_AllShortDatePatterns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AllShortTimePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_AllShortTimePatterns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AllYearMonthPatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_AllYearMonthPatterns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Calendar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::Calendar,
        > = __cordl_object.invoke("get_Calendar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompareInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CompareInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CompareInfo,
        > = __cordl_object.invoke("get_CompareInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Culture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = __cordl_object.invoke("get_Culture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CultureName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_CultureName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DateSeparator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DateSeparator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DateTimeOffsetPattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DateTimeOffsetPattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DayNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_DayNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EraNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_EraNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FormatFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Globalization::DateTimeFormatFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Globalization::DateTimeFormatFlags = __cordl_object
            .invoke("get_FormatFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FullDateTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_FullDateTimePattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FullTimeSpanNegativePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_FullTimeSpanNegativePattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FullTimeSpanPositivePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_FullTimeSpanPositivePattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GeneralLongTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_GeneralLongTimePattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GeneralShortTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_GeneralShortTimePattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasForceTwoDigitYears(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasForceTwoDigitYears", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasSpacesInDayNames(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasSpacesInDayNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasSpacesInMonthNames(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasSpacesInMonthNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasYearMonthAdjustment(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasYearMonthAdjustment", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InvariantInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::DateTimeFormatInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::DateTimeFormatInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InvariantInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LanguageName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_LanguageName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LongDatePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_LongDatePattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LongTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_LongTimePattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MonthDayPattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_MonthDayPattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MonthGenitiveNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_MonthGenitiveNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MonthNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_MonthNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OptionalCalendars(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Globalization::CalendarId>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::Globalization::CalendarId>,
        > = __cordl_object.invoke("get_OptionalCalendars", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PMDesignator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_PMDesignator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RFC1123Pattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_RFC1123Pattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShortDatePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ShortDatePattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShortTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ShortTimePattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SortableDateTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_SortableDateTimePattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TimeSeparator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_TimeSeparator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnclonedLongDatePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_UnclonedLongDatePatterns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnclonedLongTimePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_UnclonedLongTimePatterns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnclonedShortDatePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_UnclonedShortDatePatterns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnclonedShortTimePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_UnclonedShortTimePatterns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnclonedYearMonthPatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_UnclonedYearMonthPatterns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UniversalSortableDateTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_UniversalSortableDateTimePattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_YearMonthPattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_YearMonthPattern", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetAbbreviatedDayOfWeekNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("internalGetAbbreviatedDayOfWeekNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetAbbreviatedDayOfWeekNamesCore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("internalGetAbbreviatedDayOfWeekNamesCore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetAbbreviatedMonthNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("internalGetAbbreviatedMonthNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetAbbreviatedMonthNamesCore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("internalGetAbbreviatedMonthNamesCore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetDayOfWeekNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("internalGetDayOfWeekNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetDayOfWeekNamesCore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("internalGetDayOfWeekNamesCore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetGenitiveMonthNames(
        &mut self,
        abbreviated: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("internalGetGenitiveMonthNames", (abbreviated))?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetLeapYearMonthNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("internalGetLeapYearMonthNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetMonthName(
        &mut self,
        month: i32,
        style: crate::System::Globalization::MonthNameStyles,
        abbreviated: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("internalGetMonthName", (month, style, abbreviated))?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetMonthNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("internalGetMonthNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn internalGetMonthNamesCore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("internalGetMonthNamesCore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Calendar(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Calendar", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::DateTimeFormatInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo")]
impl AsRef<crate::System::ICloneable>
for crate::System::Globalization::DateTimeFormatInfo {
    fn as_ref(&self) -> &crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo")]
impl AsMut<crate::System::ICloneable>
for crate::System::Globalization::DateTimeFormatInfo {
    fn as_mut(&mut self) -> &mut crate::System::ICloneable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo")]
impl AsRef<crate::System::IFormatProvider>
for crate::System::Globalization::DateTimeFormatInfo {
    fn as_ref(&self) -> &crate::System::IFormatProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo")]
impl AsMut<crate::System::IFormatProvider>
for crate::System::Globalization::DateTimeFormatInfo {
    fn as_mut(&mut self) -> &mut crate::System::IFormatProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo+TokenHashValue")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeFormatInfo_TokenHashValue {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub tokenString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub tokenType: crate::System::TokenType,
    pub tokenValue: i32,
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo+TokenHashValue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::DateTimeFormatInfo_TokenHashValue => "System.Globalization"
    ."DateTimeFormatInfo/TokenHashValue"
);
#[cfg(feature = "System+Globalization+DateTimeFormatInfo+TokenHashValue")]
impl std::ops::Deref
for crate::System::Globalization::DateTimeFormatInfo_TokenHashValue {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo+TokenHashValue")]
impl std::ops::DerefMut
for crate::System::Globalization::DateTimeFormatInfo_TokenHashValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo+TokenHashValue")]
impl crate::System::Globalization::DateTimeFormatInfo_TokenHashValue {
    pub fn New(
        tokenString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tokenType: crate::System::TokenType,
        tokenValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tokenString, tokenType, tokenValue))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        tokenString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        tokenType: crate::System::TokenType,
        tokenValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tokenString, tokenType, tokenValue))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo+TokenHashValue")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::DateTimeFormatInfo_TokenHashValue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
