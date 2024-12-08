#[cfg(feature = "System+Globalization+DateTimeFormatInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeFormatInfo {
    __cordl_parent: crate::System::Object,
    pub _cultureData: *mut crate::System::Globalization::CultureData,
    pub _name: *mut crate::System::String,
    pub _langName: *mut crate::System::String,
    pub _compareInfo: *mut crate::System::Globalization::CompareInfo,
    pub _cultureInfo: *mut crate::System::Globalization::CultureInfo,
    pub amDesignator: *mut crate::System::String,
    pub pmDesignator: *mut crate::System::String,
    pub dateSeparator: *mut crate::System::String,
    pub generalShortTimePattern: *mut crate::System::String,
    pub generalLongTimePattern: *mut crate::System::String,
    pub timeSeparator: *mut crate::System::String,
    pub monthDayPattern: *mut crate::System::String,
    pub dateTimeOffsetPattern: *mut crate::System::String,
    pub calendar: *mut crate::System::Globalization::Calendar,
    pub firstDayOfWeek: i32,
    pub calendarWeekRule: i32,
    pub fullDateTimePattern: *mut crate::System::String,
    pub abbreviatedDayNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub m_superShortDayNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub dayNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub abbreviatedMonthNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub monthNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub genitiveMonthNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub m_genitiveAbbreviatedMonthNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub leapYearMonthNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub longDatePattern: *mut crate::System::String,
    pub shortDatePattern: *mut crate::System::String,
    pub yearMonthPattern: *mut crate::System::String,
    pub longTimePattern: *mut crate::System::String,
    pub shortTimePattern: *mut crate::System::String,
    pub allYearMonthPatterns: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub allShortDatePatterns: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub allLongDatePatterns: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub allShortTimePatterns: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub allLongTimePatterns: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub m_eraNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub m_abbrevEraNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub m_abbrevEnglishEraNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub optionalCalendars: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Globalization::CalendarId,
    >,
    pub _isReadOnly: bool,
    pub formatFlags: crate::System::Globalization::DateTimeFormatFlags,
    pub _fullTimeSpanPositivePattern: *mut crate::System::String,
    pub _fullTimeSpanNegativePattern: *mut crate::System::String,
    pub _dtfiTokenHash: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
    >,
}
#[cfg(feature = "System+Globalization+DateTimeFormatInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::DateTimeFormatInfo =>
    "System.Globalization"."DateTimeFormatInfo"
);
#[cfg(feature = "System+Globalization+DateTimeFormatInfo")]
impl std::ops::Deref for crate::System::Globalization::DateTimeFormatInfo {
    type Target = crate::System::Object;
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
        temp: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
        >,
        monthPostfix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddMonthNames", (temp, monthPostfix))?;
        Ok(__cordl_ret)
    }
    pub fn ClearTokenHashTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearTokenHashTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompareStringIgnoreCaseOptimized(
        &mut self,
        string1: *mut crate::System::String,
        offset1: i32,
        length1: i32,
        string2: *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
    pub fn CreateTokenHashTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
        > = __cordl_object.invoke("CreateTokenHashTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAbbreviatedDayName(
        &mut self,
        dayofweek: crate::System::DayOfWeek,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetAbbreviatedDayName", (dayofweek))?;
        Ok(__cordl_ret)
    }
    pub fn GetAbbreviatedEraName(
        &mut self,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetAbbreviatedEraName", (era))?;
        Ok(__cordl_ret)
    }
    pub fn GetAbbreviatedMonthName(
        &mut self,
        month: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetAbbreviatedMonthName", (month))?;
        Ok(__cordl_ret)
    }
    pub fn GetAllDateTimePatterns(
        &mut self,
        format: char,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetAllDateTimePatterns", (format))?;
        Ok(__cordl_ret)
    }
    pub fn GetDayName(
        &mut self,
        dayofweek: crate::System::DayOfWeek,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetDayName", (dayofweek))?;
        Ok(__cordl_ret)
    }
    pub fn GetEraName(
        &mut self,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetEraName", (era))?;
        Ok(__cordl_ret)
    }
    pub fn GetFormat(
        &mut self,
        formatType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetFormat", (formatType))?;
        Ok(__cordl_ret)
    }
    pub fn GetMonthName(
        &mut self,
        month: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetMonthName", (month))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn InitializeOverridableProperties(
        &mut self,
        cultureData: *mut crate::System::Globalization::CultureData,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeOverridableProperties", (cultureData, calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn InsertAtCurrentHashNode(
        &mut self,
        hashTable: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
        >,
        str: *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
    pub fn InsertHash(
        &mut self,
        hashTable: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
        >,
        str: *mut crate::System::String,
        tokenType: crate::System::TokenType,
        tokenValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertHash", (hashTable, str, tokenType, tokenValue))?;
        Ok(__cordl_ret)
    }
    pub fn IsAllowedJapaneseTokenFollowedByNonSpaceLetter(
        &mut self,
        tokenString: *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_CultureData_Calendar1(
        cultureData: *mut crate::System::Globalization::CultureData,
        cal: *mut crate::System::Globalization::Calendar,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cultureData, cal))?;
        Ok(__cordl_object)
    }
    pub fn PopulateSpecialTokenHashTable(
        &mut self,
        temp: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Globalization::DateTimeFormatInfo_TokenHashValue,
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CultureData_Calendar1(
        &mut self,
        cultureData: *mut crate::System::Globalization::CultureData,
        cal: *mut crate::System::Globalization::Calendar,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cultureData, cal))?;
        Ok(__cordl_ret)
    }
    pub fn get_AMDesignator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AMDesignator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AbbreviatedDayNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_AbbreviatedDayNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AbbreviatedEnglishEraNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_AbbreviatedEnglishEraNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AbbreviatedEraNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_AbbreviatedEraNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AbbreviatedMonthNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_AbbreviatedMonthNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AllLongDatePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_AllLongDatePatterns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AllLongTimePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_AllLongTimePatterns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AllShortDatePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_AllShortDatePatterns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AllShortTimePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_AllShortTimePatterns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AllYearMonthPatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_AllYearMonthPatterns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Calendar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::Calendar> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::Calendar = __cordl_object
            .invoke("get_Calendar", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CompareInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::CompareInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::CompareInfo = __cordl_object
            .invoke("get_CompareInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Culture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::CultureInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::CultureInfo = __cordl_object
            .invoke("get_Culture", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CultureName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_CultureName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DateSeparator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DateSeparator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DateTimeOffsetPattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DateTimeOffsetPattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DayNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_DayNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EraNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_EraNames", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_FullDateTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_FullDateTimePattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FullTimeSpanNegativePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_FullTimeSpanNegativePattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FullTimeSpanPositivePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_FullTimeSpanPositivePattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GeneralLongTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_GeneralLongTimePattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GeneralShortTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_GeneralShortTimePattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasForceTwoDigitYears(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasForceTwoDigitYears", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasSpacesInDayNames(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasSpacesInDayNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasSpacesInMonthNames(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasSpacesInMonthNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasYearMonthAdjustment(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasYearMonthAdjustment", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadOnly", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LanguageName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_LanguageName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LongDatePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_LongDatePattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LongTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_LongTimePattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MonthDayPattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_MonthDayPattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MonthGenitiveNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_MonthGenitiveNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MonthNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_MonthNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OptionalCalendars(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::System::Globalization::CalendarId>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Globalization::CalendarId,
        > = __cordl_object.invoke("get_OptionalCalendars", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PMDesignator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_PMDesignator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RFC1123Pattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_RFC1123Pattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ShortDatePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ShortDatePattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ShortTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ShortTimePattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SortableDateTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_SortableDateTimePattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TimeSeparator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_TimeSeparator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UnclonedLongDatePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_UnclonedLongDatePatterns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UnclonedLongTimePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_UnclonedLongTimePatterns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UnclonedShortDatePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_UnclonedShortDatePatterns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UnclonedShortTimePatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_UnclonedShortTimePatterns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UnclonedYearMonthPatterns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_UnclonedYearMonthPatterns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UniversalSortableDateTimePattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_UniversalSortableDateTimePattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_YearMonthPattern(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_YearMonthPattern", ())?;
        Ok(__cordl_ret)
    }
    pub fn internalGetAbbreviatedDayOfWeekNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("internalGetAbbreviatedDayOfWeekNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn internalGetAbbreviatedDayOfWeekNamesCore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("internalGetAbbreviatedDayOfWeekNamesCore", ())?;
        Ok(__cordl_ret)
    }
    pub fn internalGetAbbreviatedMonthNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("internalGetAbbreviatedMonthNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn internalGetAbbreviatedMonthNamesCore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("internalGetAbbreviatedMonthNamesCore", ())?;
        Ok(__cordl_ret)
    }
    pub fn internalGetDayOfWeekNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("internalGetDayOfWeekNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn internalGetDayOfWeekNamesCore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("internalGetDayOfWeekNamesCore", ())?;
        Ok(__cordl_ret)
    }
    pub fn internalGetGenitiveMonthNames(
        &mut self,
        abbreviated: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("internalGetGenitiveMonthNames", (abbreviated))?;
        Ok(__cordl_ret)
    }
    pub fn internalGetLeapYearMonthNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("internalGetLeapYearMonthNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn internalGetMonthName(
        &mut self,
        month: i32,
        style: crate::System::Globalization::MonthNameStyles,
        abbreviated: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("internalGetMonthName", (month, style, abbreviated))?;
        Ok(__cordl_ret)
    }
    pub fn internalGetMonthNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("internalGetMonthNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn internalGetMonthNamesCore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("internalGetMonthNamesCore", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Calendar(
        &mut self,
        value: *mut crate::System::Globalization::Calendar,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Calendar", (value))?;
        Ok(__cordl_ret)
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
#[cfg(feature = "System+Globalization+DateTimeFormatInfo+TokenHashValue")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeFormatInfo_TokenHashValue {
    __cordl_parent: crate::System::Object,
    pub tokenString: *mut crate::System::String,
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
    type Target = crate::System::Object;
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
        tokenString: *mut crate::System::String,
        tokenType: crate::System::TokenType,
        tokenValue: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tokenString, tokenType, tokenValue))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        tokenString: *mut crate::System::String,
        tokenType: crate::System::TokenType,
        tokenValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tokenString, tokenType, tokenValue))?;
        Ok(__cordl_ret)
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
