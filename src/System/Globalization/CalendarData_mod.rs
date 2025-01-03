#[cfg(feature = "System+Globalization+CalendarData")]
#[repr(C)]
#[derive(Debug)]
pub struct CalendarData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub sNativeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub saShortDates: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub saYearMonths: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub saLongDates: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub sMonthDay: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub saEraNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub saAbbrevEraNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub saAbbrevEnglishEraNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub saDayNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub saAbbrevDayNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub saSuperShortDayNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub saMonthNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub saAbbrevMonthNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub saMonthGenitiveNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub saAbbrevMonthGenitiveNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub saLeapYearMonthNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub iTwoDigitYearMax: i32,
    pub iCurrentEra: i32,
    pub bUseUserOverrides: bool,
}
#[cfg(feature = "System+Globalization+CalendarData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::CalendarData =>
    "System.Globalization"."CalendarData"
);
#[cfg(feature = "System+Globalization+CalendarData")]
impl std::ops::Deref for crate::System::Globalization::CalendarData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CalendarData")]
impl std::ops::DerefMut for crate::System::Globalization::CalendarData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CalendarData")]
impl crate::System::Globalization::CalendarData {
    pub const MAX_CALENDARS: i32 = 23i32;
    pub fn CalendarIdToCultureName(
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalendarIdToCultureName", (calendarId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCalendarData(
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CalendarData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CalendarData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCalendarData", (calendarId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetJapaneseEnglishEraNames() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetJapaneseEnglishEraNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetJapaneseEraNames() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetJapaneseEraNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeAbbreviatedEraNames(
        &mut self,
        localeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeAbbreviatedEraNames", (localeName, calendarId))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeEraNames(
        &mut self,
        localeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeEraNames", (localeName, calendarId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_i32__cordl_bool1(
        localeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        calendarId: i32,
        bUseUserOverrides: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (localeName, calendarId, bUseUserOverrides))?;
        Ok(__cordl_object.into())
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
    pub fn _ctor_Il2CppString_i32__cordl_bool1(
        &mut self,
        localeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        calendarId: i32,
        bUseUserOverrides: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (localeName, calendarId, bUseUserOverrides))?;
        Ok(__cordl_ret.into())
    }
    pub fn fill_calendar_data(
        &mut self,
        localeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        datetimeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("fill_calendar_data", (localeName, datetimeIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn nativeGetCalendarData(
        data: quest_hook::libil2cpp::Gc<crate::System::Globalization::CalendarData>,
        localeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("nativeGetCalendarData", (data, localeName, calendarId))?;
        Ok(__cordl_ret.into())
    }
    pub fn nativeGetTwoDigitYearMax(calID: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("nativeGetTwoDigitYearMax", (calID))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+CalendarData")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::CalendarData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
