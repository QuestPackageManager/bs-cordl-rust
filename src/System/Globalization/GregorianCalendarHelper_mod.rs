#[cfg(feature = "System+Globalization+GregorianCalendarHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct GregorianCalendarHelper {
    __cordl_parent: crate::System::Object,
    pub m_maxYear: i32,
    pub m_minYear: i32,
    pub m_Cal: *mut crate::System::Globalization::Calendar,
    pub m_EraInfo: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Globalization::EraInfo,
    >,
    pub m_eras: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub m_minDate: crate::System::DateTime,
}
#[cfg(feature = "System+Globalization+GregorianCalendarHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::GregorianCalendarHelper
    => "System.Globalization"."GregorianCalendarHelper"
);
#[cfg(feature = "System+Globalization+GregorianCalendarHelper")]
impl std::ops::Deref for crate::System::Globalization::GregorianCalendarHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+GregorianCalendarHelper")]
impl std::ops::DerefMut for crate::System::Globalization::GregorianCalendarHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+GregorianCalendarHelper")]
impl crate::System::Globalization::GregorianCalendarHelper {
    pub fn CheckTicksRange(
        &mut self,
        ticks: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckTicksRange", (ticks))?;
        Ok(__cordl_ret)
    }
    pub fn GetDatePart(
        &mut self,
        ticks: i64,
        part: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetDatePart", (ticks, part))?;
        Ok(__cordl_ret)
    }
    pub fn GetDayOfMonth(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetDayOfMonth", (_cordl_time))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetEra(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetEra", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn GetGregorianYear(
        &mut self,
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetGregorianYear", (year, era))?;
        Ok(__cordl_ret)
    }
    pub fn GetMonth(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMonth", (_cordl_time))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetYear(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetYear", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn GetYearOffset(
        &mut self,
        year: i32,
        era: i32,
        throwOnError: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetYearOffset", (year, era, throwOnError))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn IsValidYear(
        &mut self,
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValidYear", (year, era))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        cal: *mut crate::System::Globalization::Calendar,
        eraInfo: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Globalization::EraInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cal, eraInfo))?;
        Ok(__cordl_object)
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
        Ok(__cordl_ret)
    }
    pub fn ToFourDigitYear(
        &mut self,
        year: i32,
        twoDigitYearMax: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ToFourDigitYear", (year, twoDigitYearMax))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        cal: *mut crate::System::Globalization::Calendar,
        eraInfo: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Globalization::EraInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cal, eraInfo))?;
        Ok(__cordl_ret)
    }
    pub fn get_Eras(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("get_Eras", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MaxYear(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxYear", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Globalization+GregorianCalendarHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::GregorianCalendarHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
