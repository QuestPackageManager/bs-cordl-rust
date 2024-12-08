#[cfg(feature = "System+Globalization+CalendarData")]
#[repr(C)]
#[derive(Debug)]
pub struct CalendarData {
    __cordl_parent: crate::System::Object,
    pub sNativeName: *mut crate::System::String,
    pub saShortDates: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub saYearMonths: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub saLongDates: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub sMonthDay: *mut crate::System::String,
    pub saEraNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub saAbbrevEraNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub saAbbrevEnglishEraNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub saDayNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub saAbbrevDayNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub saSuperShortDayNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub saMonthNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub saAbbrevMonthNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub saMonthGenitiveNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub saAbbrevMonthGenitiveNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub saLeapYearMonthNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
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
    type Target = crate::System::Object;
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
    pub fn _ctor_String_i32__cordl_bool1(
        &mut self,
        localeName: *mut crate::System::String,
        calendarId: i32,
        bUseUserOverrides: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (localeName, calendarId, bUseUserOverrides))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeEraNames(
        &mut self,
        localeName: *mut crate::System::String,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeEraNames", (localeName, calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeAbbreviatedEraNames(
        &mut self,
        localeName: *mut crate::System::String,
        calendarId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeAbbreviatedEraNames", (localeName, calendarId))?;
        Ok(__cordl_ret)
    }
    pub fn fill_calendar_data(
        &mut self,
        localeName: *mut crate::System::String,
        datetimeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("fill_calendar_data", (localeName, datetimeIndex))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String_i32__cordl_bool1(
        localeName: *mut crate::System::String,
        calendarId: i32,
        bUseUserOverrides: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (localeName, calendarId, bUseUserOverrides))?;
        Ok(__cordl_object)
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
