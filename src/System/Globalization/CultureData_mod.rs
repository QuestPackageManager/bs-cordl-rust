#[cfg(feature = "System+Globalization+CultureData")]
#[repr(C)]
#[derive(Debug)]
pub struct CultureData {
    __cordl_parent: crate::System::Object,
    pub sAM1159: *mut crate::System::String,
    pub sPM2359: *mut crate::System::String,
    pub sTimeSeparator: *mut crate::System::String,
    pub saLongTimes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub saShortTimes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub iFirstDayOfWeek: i32,
    pub iFirstWeekOfYear: i32,
    pub waCalendars: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub calendars: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Globalization::CalendarData,
    >,
    pub sISO639Language: *mut crate::System::String,
    pub sRealName: *mut crate::System::String,
    pub bUseOverrides: bool,
    pub calendarId: i32,
    pub numberIndex: i32,
    pub iDefaultAnsiCodePage: i32,
    pub iDefaultOemCodePage: i32,
    pub iDefaultMacCodePage: i32,
    pub iDefaultEbcdicCodePage: i32,
    pub isRightToLeft: bool,
    pub sListSeparator: *mut crate::System::String,
}
#[cfg(feature = "System+Globalization+CultureData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::CultureData =>
    "System.Globalization"."CultureData"
);
#[cfg(feature = "System+Globalization+CultureData")]
impl std::ops::Deref for crate::System::Globalization::CultureData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CultureData")]
impl std::ops::DerefMut for crate::System::Globalization::CultureData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CultureData")]
impl crate::System::Globalization::CultureData {
    #[cfg(feature = "System+Globalization+CultureData+NumberFormatEntryManaged")]
    pub type NumberFormatEntryManaged = crate::System::Globalization::CultureData_NumberFormatEntryManaged;
    pub fn get_ShortTimes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_ShortTimes", ())?;
        Ok(__cordl_ret)
    }
    pub fn MonthNames(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("MonthNames", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn get_SAM1159(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_SAM1159", ())?;
        Ok(__cordl_ret)
    }
    pub fn AbbreviatedMonthNames(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("AbbreviatedMonthNames", (calendarId))?;
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
    pub fn LongDates(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("LongDates", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn DayNames(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("DayNames", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn GenitiveMonthNames(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("GenitiveMonthNames", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn AbbreviatedGenitiveMonthNames(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("AbbreviatedGenitiveMonthNames", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn LeapYearMonthNames(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("LeapYearMonthNames", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn MonthDay(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("MonthDay", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn get_SISO639LANGNAME(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_SISO639LANGNAME", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UseUserOverride(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseUserOverride", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IFIRSTWEEKOFYEAR(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_IFIRSTWEEKOFYEAR", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SPM2359(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_SPM2359", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SCOMPAREINFO(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_SCOMPAREINFO", ())?;
        Ok(__cordl_ret)
    }
    pub fn AbbrevEraNames(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("AbbrevEraNames", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn GetNFIValues(
        &mut self,
        nfi: *mut crate::System::Globalization::NumberFormatInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetNFIValues", (nfi))?;
        Ok(__cordl_ret)
    }
    pub fn get_IFIRSTDAYOFWEEK(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_IFIRSTDAYOFWEEK", ())?;
        Ok(__cordl_ret)
    }
    pub fn DateSeparator(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("DateSeparator", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn create_group_sizes_array(
        &mut self,
        gs0: i32,
        gs1: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("create_group_sizes_array", (gs0, gs1))?;
        Ok(__cordl_ret)
    }
    pub fn ShortDates(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("ShortDates", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret)
    }
    pub fn EraNames(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("EraNames", (calendarId))?;
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
    pub fn AbbreviatedEnglishEraNames(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("AbbreviatedEnglishEraNames", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn GetCalendarIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<crate::System::Globalization::CalendarId>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Globalization::CalendarId,
        > = __cordl_object.invoke("GetCalendarIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_STEXTINFO(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_STEXTINFO", ())?;
        Ok(__cordl_ret)
    }
    pub fn fill_culture_data(
        &mut self,
        datetimeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("fill_culture_data", (datetimeIndex))?;
        Ok(__cordl_ret)
    }
    pub fn YearMonths(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("YearMonths", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsInvariantCulture(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInvariantCulture", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LongTimes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_LongTimes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CalendarIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("get_CalendarIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCalendar(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::CalendarData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::CalendarData = __cordl_object
            .invoke("GetCalendar", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn AbbreviatedDayNames(
        &mut self,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("AbbreviatedDayNames", (calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Globalization+CultureData")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::CultureData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+CultureData+NumberFormatEntryManaged")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CultureData_NumberFormatEntryManaged {
    pub currency_decimal_digits: i32,
    pub currency_decimal_separator: i32,
    pub currency_group_separator: i32,
    pub currency_group_sizes0: i32,
    pub currency_group_sizes1: i32,
    pub currency_negative_pattern: i32,
    pub currency_positive_pattern: i32,
    pub currency_symbol: i32,
    pub nan_symbol: i32,
    pub negative_infinity_symbol: i32,
    pub negative_sign: i32,
    pub number_decimal_digits: i32,
    pub number_decimal_separator: i32,
    pub number_group_separator: i32,
    pub number_group_sizes0: i32,
    pub number_group_sizes1: i32,
    pub number_negative_pattern: i32,
    pub per_mille_symbol: i32,
    pub percent_negative_pattern: i32,
    pub percent_positive_pattern: i32,
    pub percent_symbol: i32,
    pub positive_infinity_symbol: i32,
    pub positive_sign: i32,
}
#[cfg(feature = "System+Globalization+CultureData+NumberFormatEntryManaged")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::CultureData_NumberFormatEntryManaged =>
    "System.Globalization"."CultureData/NumberFormatEntryManaged"
);
#[cfg(feature = "System+Globalization+CultureData+NumberFormatEntryManaged")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::CultureData_NumberFormatEntryManaged {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+CultureData+NumberFormatEntryManaged")]
impl crate::System::Globalization::CultureData_NumberFormatEntryManaged {}
