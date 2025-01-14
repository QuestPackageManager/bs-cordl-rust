#[cfg(feature = "System+Globalization+GregorianCalendarHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct GregorianCalendarHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_maxYear: i32,
    pub m_minYear: i32,
    pub m_Cal: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    pub m_EraInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Globalization::EraInfo>,
        >,
    >,
    pub m_eras: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub m_minDate: crate::System::DateTime,
}
#[cfg(feature = "System+Globalization+GregorianCalendarHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::GregorianCalendarHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "GregorianCalendarHelper";
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
#[cfg(feature = "System+Globalization+GregorianCalendarHelper")]
impl std::ops::Deref for crate::System::Globalization::GregorianCalendarHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i64), quest_hook::libil2cpp::Void, 1usize>("CheckTicksRange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CheckTicksRange", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ticks))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DateToTicks(
        year: i32,
        month: i32,
        day: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32), i64, 3usize>("DateToTicks")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DateToTicks", 3usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (year, month, day))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAbsoluteDate(
        year: i32,
        month: i32,
        day: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32), i64, 3usize>("GetAbsoluteDate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetAbsoluteDate", 3usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (year, month, day))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDatePart(
        &mut self,
        ticks: i64,
        part: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i64, i32), i32, 2usize>("GetDatePart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDatePart", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (ticks, part)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfMonth(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::DateTime), i32, 1usize>("GetDayOfMonth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDayOfMonth", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (_cordl_time)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDayOfWeek(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::DayOfWeek> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::DateTime),
                crate::System::DayOfWeek,
                1usize,
            >("GetDayOfWeek")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDayOfWeek", 1usize
                )
            });
        let __cordl_ret: crate::System::DayOfWeek = unsafe {
            method.invoke_unchecked(self, (_cordl_time))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDaysInMonth(
        &mut self,
        year: i32,
        month: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32, i32), i32, 3usize>("GetDaysInMonth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDaysInMonth", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (year, month, era))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDaysInYear(
        &mut self,
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), i32, 2usize>("GetDaysInYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDaysInYear", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (year, era)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetEra(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::DateTime), i32, 1usize>("GetEra")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetEra", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (_cordl_time)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetGregorianYear(
        &mut self,
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), i32, 2usize>("GetGregorianYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetGregorianYear", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (year, era)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetMonth(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::DateTime), i32, 1usize>("GetMonth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetMonth", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (_cordl_time)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetMonthsInYear(
        &mut self,
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), i32, 2usize>("GetMonthsInYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetMonthsInYear", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (year, era)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetYear(
        &mut self,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::DateTime), i32, 1usize>("GetYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetYear", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (_cordl_time)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetYearOffset(
        &mut self,
        year: i32,
        era: i32,
        throwOnError: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32, bool), i32, 3usize>("GetYearOffset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetYearOffset", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (year, era, throwOnError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsLeapYear(
        &mut self,
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), bool, 2usize>("IsLeapYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsLeapYear", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (year, era)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidYear(
        &mut self,
        year: i32,
        era: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), bool, 2usize>("IsValidYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsValidYear", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (year, era)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        cal: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
        eraInfo: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Globalization::EraInfo>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cal, eraInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn TimeToTicks(
        hour: i32,
        minute: i32,
        second: i32,
        millisecond: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32, i32), i64, 4usize>("TimeToTicks")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TimeToTicks", 4usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (hour, minute, second, millisecond))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, i32, i32, i32, i32, i32),
                crate::System::DateTime,
                8usize,
            >("ToDateTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToDateTime", 8usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (year, month, day, hour, minute, second, millisecond, era),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToFourDigitYear(
        &mut self,
        year: i32,
        twoDigitYearMax: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), i32, 2usize>("ToFourDigitYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToFourDigitYear", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (year, twoDigitYearMax))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        cal: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
        eraInfo: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Globalization::EraInfo>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::EraInfo,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cal, eraInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Eras(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                0usize,
            >("get_Eras")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Eras", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxYear(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_MaxYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_MaxYear", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
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
