#[cfg(feature = "System+Globalization+HijriCalendar")]
#[repr(C)]
#[derive(Debug)]
pub struct HijriCalendar {
    __cordl_parent: crate::System::Globalization::Calendar,
    pub m_HijriAdvance: i32,
}
#[cfg(feature = "System+Globalization+HijriCalendar")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Globalization::HijriCalendar {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "HijriCalendar";
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
#[cfg(feature = "System+Globalization+HijriCalendar")]
impl std::ops::Deref for crate::System::Globalization::HijriCalendar {
    type Target = crate::System::Globalization::Calendar;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+HijriCalendar")]
impl std::ops::DerefMut for crate::System::Globalization::HijriCalendar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+HijriCalendar")]
impl crate::System::Globalization::HijriCalendar {
    pub fn CheckEraRange(
        era: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CheckEraRange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "CheckEraRange", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (era))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckTicksRange(
        ticks: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64),
                quest_hook::libil2cpp::Void,
                1usize,
            >("CheckTicksRange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "CheckTicksRange", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ticks))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckYearMonthRange(
        year: i32,
        month: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CheckYearMonthRange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "CheckYearMonthRange",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (year, month, era))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckYearRange(
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CheckYearRange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "CheckYearRange", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (year, era))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DaysUpToHijriYear(
        &mut self,
        HijriYear: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i64, 1usize>("DaysUpToHijriYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "DaysUpToHijriYear", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, (HijriYear))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAbsoluteDateHijri(
        &mut self,
        y: i32,
        m: i32,
        d: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32, i32), i64, 3usize>("GetAbsoluteDateHijri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "GetAbsoluteDateHijri",
                    3usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, (y, m, d))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAdvanceHijriDate() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("GetAdvanceHijriDate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "GetAdvanceHijriDate",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDatePart(
        &mut self,
        ticks: i64,
        part: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i64, i32), i32, 2usize>("GetDatePart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "GetDatePart", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (ticks, part))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfMonth(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::DateTime), i32, 1usize>("GetDayOfMonth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "GetDayOfMonth", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (_cordl_time))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfWeek(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::DayOfWeek> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::DateTime),
                crate::System::DayOfWeek,
                1usize,
            >("GetDayOfWeek")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "GetDayOfWeek", 1usize
                )
            });
        let __cordl_ret: crate::System::DayOfWeek = unsafe {
            method.invoke_unchecked(self, (_cordl_time))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDaysInMonth(
        &mut self,
        year: i32,
        month: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32, i32), i32, 3usize>("GetDaysInMonth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "GetDaysInMonth", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (year, month, era))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDaysInYear(
        &mut self,
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), i32, 2usize>("GetDaysInYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "GetDaysInYear", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (year, era))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEra(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::DateTime), i32, 1usize>("GetEra")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "GetEra", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (_cordl_time))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMonth(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::DateTime), i32, 1usize>("GetMonth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "GetMonth", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (_cordl_time))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMonthsInYear(
        &mut self,
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), i32, 2usize>("GetMonthsInYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "GetMonthsInYear", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (year, era))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetYear(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::DateTime), i32, 1usize>("GetYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "GetYear", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (_cordl_time))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsLeapYear(
        &mut self,
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), bool, 2usize>("IsLeapYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "IsLeapYear", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (year, era))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, i32, i32, i32, i32, i32),
                crate::System::DateTime,
                8usize,
            >("ToDateTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "ToDateTime", 8usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (year, month, day, hour, minute, second, millisecond, era),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToFourDigitYear(&mut self, year: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("ToFourDigitYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "ToFourDigitYear", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (year))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Eras(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                0usize,
            >("get_Eras")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "get_Eras", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_HijriAdjustment(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_HijriAdjustment")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "get_HijriAdjustment",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_ID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "get_ID", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxSupportedDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::DateTime,
                0usize,
            >("get_MaxSupportedDateTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "get_MaxSupportedDateTime",
                    0usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_MinSupportedDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::DateTime,
                0usize,
            >("get_MinSupportedDateTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "get_MinSupportedDateTime",
                    0usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_TwoDigitYearMax(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_TwoDigitYearMax")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "get_TwoDigitYearMax",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_TwoDigitYearMax(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Globalization::HijriCalendar as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_TwoDigitYearMax")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Globalization::HijriCalendar as
                    quest_hook::libil2cpp::Type > ::class(), "set_TwoDigitYearMax",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+HijriCalendar")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::HijriCalendar {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
