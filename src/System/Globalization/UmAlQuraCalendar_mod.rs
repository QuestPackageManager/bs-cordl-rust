#[cfg(feature = "System+Globalization+UmAlQuraCalendar")]
#[repr(C)]
#[derive(Debug)]
pub struct UmAlQuraCalendar {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
}
#[cfg(feature = "System+Globalization+UmAlQuraCalendar")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::UmAlQuraCalendar =>
    "System.Globalization"."UmAlQuraCalendar"
);
#[cfg(feature = "System+Globalization+UmAlQuraCalendar")]
impl std::ops::Deref for crate::System::Globalization::UmAlQuraCalendar {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+UmAlQuraCalendar")]
impl std::ops::DerefMut for crate::System::Globalization::UmAlQuraCalendar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+UmAlQuraCalendar")]
impl crate::System::Globalization::UmAlQuraCalendar {
    #[cfg(feature = "System+Globalization+UmAlQuraCalendar+DateMapping")]
    pub type DateMapping = crate::System::Globalization::UmAlQuraCalendar_DateMapping;
    pub fn CheckEraRange(
        era: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckEraRange", (era))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckTicksRange(
        ticks: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckTicksRange", (ticks))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckYearMonthRange(
        year: i32,
        month: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckYearMonthRange", (year, month, era))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckYearRange(
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckYearRange", (year, era))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertGregorianToHijri(
        _cordl_time: crate::System::DateTime,
        HijriYear: quest_hook::libil2cpp::ByRefMut<i32>,
        HijriMonth: quest_hook::libil2cpp::ByRefMut<i32>,
        HijriDay: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertGregorianToHijri",
                (_cordl_time, HijriYear, HijriMonth, HijriDay),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertHijriToGregorian(
        HijriYear: i32,
        HijriMonth: i32,
        HijriDay: i32,
        yg: quest_hook::libil2cpp::ByRefMut<i32>,
        mg: quest_hook::libil2cpp::ByRefMut<i32>,
        dg: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertHijriToGregorian",
                (HijriYear, HijriMonth, HijriDay, yg, mg, dg),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAbsoluteDateUmAlQura(
        year: i32,
        month: i32,
        day: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAbsoluteDateUmAlQura", (year, month, day))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDatePart(
        &mut self,
        _cordl_time: crate::System::DateTime,
        part: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetDatePart", (_cordl_time, part))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfMonth(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetDayOfMonth", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfWeek(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::DayOfWeek> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DayOfWeek = __cordl_object
            .invoke("GetDayOfWeek", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDaysInMonth(
        &mut self,
        year: i32,
        month: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetDaysInMonth", (year, month, era))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDaysInYear(
        &mut self,
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetDaysInYear", (year, era))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEra(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetEra", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMonth(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMonth", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMonthsInYear(
        &mut self,
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMonthsInYear", (year, era))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetYear(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetYear", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitDateMapping() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Globalization::UmAlQuraCalendar_DateMapping,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Globalization::UmAlQuraCalendar_DateMapping,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitDateMapping", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLeapYear(
        &mut self,
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsLeapYear", (year, era))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RealGetDaysInYear(year: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RealGetDaysInYear", (year))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDateTime(
        &mut self,
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
        millisecond: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke(
                "ToDateTime",
                (year, month, day, hour, minute, second, millisecond, era),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFourDigitYear(&mut self, year: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ToFourDigitYear", (year))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BaseCalendarID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BaseCalendarID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Eras(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = __cordl_object.invoke("get_Eras", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxSupportedDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_MaxSupportedDateTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MinSupportedDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_MinSupportedDateTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TwoDigitYearMax(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TwoDigitYearMax", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_TwoDigitYearMax(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TwoDigitYearMax", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+UmAlQuraCalendar")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::UmAlQuraCalendar {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Globalization+UmAlQuraCalendar+DateMapping")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct UmAlQuraCalendar_DateMapping {
    pub HijriMonthsLengthFlags: i32,
    pub GregorianDate: crate::System::DateTime,
}
#[cfg(feature = "System+Globalization+UmAlQuraCalendar+DateMapping")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::UmAlQuraCalendar_DateMapping => "System.Globalization"
    ."UmAlQuraCalendar/DateMapping"
);
#[cfg(feature = "System+Globalization+UmAlQuraCalendar+DateMapping")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::UmAlQuraCalendar_DateMapping {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+UmAlQuraCalendar+DateMapping")]
impl crate::System::Globalization::UmAlQuraCalendar_DateMapping {
    pub fn _ctor(
        &mut self,
        MonthsLengthFlags: i32,
        GYear: i32,
        GMonth: i32,
        GDay: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (MonthsLengthFlags, GYear, GMonth, GDay),
        )?;
        Ok(__cordl_ret.into())
    }
}
