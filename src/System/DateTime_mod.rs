#[cfg(feature = "System+DateTime")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DateTime {
    pub _dateData: u64,
}
#[cfg(feature = "System+DateTime")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::DateTime {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "DateTime";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+DateTime")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::DateTime {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+DateTime")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::DateTime {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+DateTime")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::DateTime {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+DateTime")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::DateTime {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+DateTime")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::DateTime {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+DateTime")]
impl crate::System::DateTime {
    pub const DateDataField: &'static str = "dateData";
    pub const DatePartDay: i32 = 3i32;
    pub const DatePartDayOfYear: i32 = 1i32;
    pub const DatePartMonth: i32 = 2i32;
    pub const DatePartYear: i32 = 0i32;
    pub const DaysPer100Years: i32 = 36524i32;
    pub const DaysPer400Years: i32 = 146097i32;
    pub const DaysPer4Years: i32 = 1461i32;
    pub const DaysPerYear: i32 = 365i32;
    pub const DaysTo10000: i32 = 3652059i32;
    pub const DaysTo1601: i32 = 584388i32;
    pub const DaysTo1899: i32 = 693593i32;
    pub const DaysTo1970: i32 = 719162i32;
    pub const DoubleDateOffset: i64 = 599264352000000000i64;
    pub const FileTimeOffset: i64 = 504911232000000000i64;
    pub const FlagsMask: u64 = 13835058055282163712u64;
    pub const KindLocal: u64 = 9223372036854775808u64;
    pub const KindLocalAmbiguousDst: u64 = 13835058055282163712u64;
    pub const KindShift: i32 = 62i32;
    pub const KindUnspecified: u64 = 0u64;
    pub const KindUtc: u64 = 4611686018427387904u64;
    pub const LocalMask: u64 = 9223372036854775808u64;
    pub const MaxMillis: i64 = 315537897600000i64;
    pub const MaxTicks: i64 = 3155378975999999999i64;
    pub const MillisPerDay: i32 = 86400000i32;
    pub const MillisPerHour: i32 = 3600000i32;
    pub const MillisPerMinute: i32 = 60000i32;
    pub const MillisPerSecond: i32 = 1000i32;
    pub const MinTicks: i64 = 0i64;
    pub const OADateMaxAsDouble: f64 = 2958466f64;
    pub const OADateMinAsDouble: f64 = -657435f64;
    pub const OADateMinAsTicks: i64 = 31241376000000000i64;
    pub const TicksCeiling: i64 = 4611686018427387904i64;
    pub const TicksField: &'static str = "ticks";
    pub const TicksMask: u64 = 4611686018427387903u64;
    pub const TicksPerDay: i64 = 864000000000i64;
    pub const TicksPerHour: i64 = 36000000000i64;
    pub const TicksPerMillisecond: i64 = 10000i64;
    pub const TicksPerMinute: i64 = 600000000i64;
    pub const TicksPerSecond: i64 = 10000000i64;
    pub const UnixEpochTicks: i64 = 621355968000000000i64;
    pub fn AddDays(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f64), crate::System::DateTime, 1usize>("AddDays")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddDays", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddMilliseconds(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f64), crate::System::DateTime, 1usize>("AddMilliseconds")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddMilliseconds", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddMonths(
        &mut self,
        months: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), crate::System::DateTime, 1usize>("AddMonths")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddMonths", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, (months))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddSeconds(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f64), crate::System::DateTime, 1usize>("AddSeconds")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddSeconds", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddTicks(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i64), crate::System::DateTime, 1usize>("AddTicks")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddTicks", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddYears(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), crate::System::DateTime, 1usize>("AddYears")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddYears", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Add_TimeSpan0(
        &mut self,
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::TimeSpan),
                crate::System::DateTime,
                1usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Add", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Add_f64_i32_1(
        &mut self,
        value: f64,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f64, i32), crate::System::DateTime, 2usize>("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Add", 2usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, (value, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Compare(
        t1: crate::System::DateTime,
        t2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTime, crate::System::DateTime),
                i32,
                2usize,
            >("Compare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Compare", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (t1, t2)) };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_DateTime1(
        &mut self,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::DateTime), i32, 1usize>("CompareTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompareTo", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_Il2CppObject0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                i32,
                1usize,
            >("CompareTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompareTo", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (value)) };
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
    pub fn DaysInMonth(year: i32, month: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32), i32, 2usize>("DaysInMonth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DaysInMonth", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (year, month)) };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_DateTime1(
        &mut self,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::DateTime), bool, 1usize>("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromBinary(
        dateData: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), crate::System::DateTime, 1usize>("FromBinary")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromBinary", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (dateData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromBinaryRaw(
        dateData: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64),
                crate::System::DateTime,
                1usize,
            >("FromBinaryRaw")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromBinaryRaw", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (dateData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromFileTime(
        fileTime: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i64), crate::System::DateTime, 1usize>("FromFileTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromFileTime", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (fileTime))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromFileTimeUtc(
        fileTime: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64),
                crate::System::DateTime,
                1usize,
            >("FromFileTimeUtc")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromFileTimeUtc", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (fileTime))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDatePart_ByRefMut_ByRefMut_ByRefMut1(
        &mut self,
        year: quest_hook::libil2cpp::ByRefMut<i32>,
        month: quest_hook::libil2cpp::ByRefMut<i32>,
        day: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("GetDatePart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDatePart", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (year, month, day))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDatePart_i32_0(
        &mut self,
        part: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("GetDatePart")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDatePart", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (part)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetHashCode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemTimeAsFileTime() -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i64, 0usize>("GetSystemTimeAsFileTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSystemTimeAsFileTime", 0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TypeCode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::TypeCode, 0usize>("GetTypeCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTypeCode", 0usize
                )
            });
        let __cordl_ret: crate::System::TypeCode = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsAmbiguousDaylightSavingTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsAmbiguousDaylightSavingTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsAmbiguousDaylightSavingTime", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn IsLeapYear(year: i32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), bool, 1usize>("IsLeapYear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsLeapYear", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (year)) };
        Ok(__cordl_ret.into())
    }
    pub fn ParseExact_Il2CppArray_DateTimeStyles2(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formats: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        style: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                    crate::System::Globalization::DateTimeStyles,
                ),
                crate::System::DateTime,
                4usize,
            >("ParseExact")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ParseExact", 4usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (s, formats, provider, style))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseExact_Il2CppString0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                crate::System::DateTime,
                3usize,
            >("ParseExact")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ParseExact", 3usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (s, format, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseExact_Il2CppString_DateTimeStyles1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        style: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                    crate::System::Globalization::DateTimeStyles,
                ),
                crate::System::DateTime,
                4usize,
            >("ParseExact")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ParseExact", 4usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (s, format, provider, style))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Parse_IFormatProvider1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                crate::System::DateTime,
                2usize,
            >("Parse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Parse", 2usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (s, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Parse_IFormatProvider_DateTimeStyles2(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        styles: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                    crate::System::Globalization::DateTimeStyles,
                ),
                crate::System::DateTime,
                3usize,
            >("Parse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Parse", 3usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (s, provider, styles))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Parse_Il2CppString0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::System::DateTime,
                1usize,
            >("Parse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Parse", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (s))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SpecifyKind(
        value: crate::System::DateTime,
        kind: crate::System::DateTimeKind,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTime, crate::System::DateTimeKind),
                crate::System::DateTime,
                2usize,
            >("SpecifyKind")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SpecifyKind", 2usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (value, kind))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Subtract(
        &mut self,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::DateTime),
                crate::System::TimeSpan,
                1usize,
            >("Subtract")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Subtract", 1usize
                )
            });
        let __cordl_ret: crate::System::TimeSpan = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToBoolean(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                bool,
                1usize,
            >("System.IConvertible.ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToByte(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                u8,
                1usize,
            >("System.IConvertible.ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToChar(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                char,
                1usize,
            >("System.IConvertible.ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToChar", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDateTime(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                crate::System::DateTime,
                1usize,
            >("System.IConvertible.ToDateTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToDateTime", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, (provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDecimal(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                crate::System::Decimal,
                1usize,
            >("System.IConvertible.ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToDecimal", 1usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked(self, (provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDouble(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                f64,
                1usize,
            >("System.IConvertible.ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt16(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                i16,
                1usize,
            >("System.IConvertible.ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt32(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                i32,
                1usize,
            >("System.IConvertible.ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt64(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                i64,
                1usize,
            >("System.IConvertible.ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToSByte(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                i8,
                1usize,
            >("System.IConvertible.ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToSingle(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                f32,
                1usize,
            >("System.IConvertible.ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("System.IConvertible.ToType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToType", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt16(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                u16,
                1usize,
            >("System.IConvertible.ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt32(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                u32,
                1usize,
            >("System.IConvertible.ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt64(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                u64,
                1usize,
            >("System.IConvertible.ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("System.Runtime.Serialization.ISerializable.GetObjectData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.Runtime.Serialization.ISerializable.GetObjectData",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TimeToTicks(
        hour: i32,
        minute: i32,
        second: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32), i64, 3usize>("TimeToTicks")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TimeToTicks", 3usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked((), (hour, minute, second))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToBinaryRaw(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i64, 0usize>("ToBinaryRaw")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToBinaryRaw", 0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ToLocalTime_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::DateTime, 0usize>("ToLocalTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToLocalTime", 0usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToLocalTime__cordl_bool1(
        &mut self,
        throwOnOverflow: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), crate::System::DateTime, 1usize>("ToLocalTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToLocalTime", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, (throwOnOverflow))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_IFormatProvider2(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (format)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider3(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (format, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToUniversalTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::DateTime, 0usize>("ToUniversalTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToUniversalTime", 0usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryCreate(
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
        millisecond: i32,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                ),
                bool,
                8usize,
            >("TryCreate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryCreate", 8usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (year, month, day, hour, minute, second, millisecond, result),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryFormat(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                bool,
                4usize,
            >("TryFormat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryFormat", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (destination, charsWritten, format, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParse(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        styles: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                    crate::System::Globalization::DateTimeStyles,
                    quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                ),
                bool,
                4usize,
            >("TryParse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParse", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (s, provider, styles, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        style: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                    crate::System::Globalization::DateTimeStyles,
                    quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                ),
                bool,
                5usize,
            >("TryParseExact")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParseExact", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (s, format, provider, style, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext10(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
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
            method.invoke_unchecked(self, (info, context))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_4(
        &mut self,
        year: i32,
        month: i32,
        day: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32, i32), quest_hook::libil2cpp::Void, 3usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (year, month, day))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_i32_i32_i32_5(
        &mut self,
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, i32, i32, i32),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (year, month, day, hour, minute, second))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_i32_i32_i32_DateTimeKind6(
        &mut self,
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
        kind: crate::System::DateTimeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, i32, i32, i32, crate::System::DateTimeKind),
                quest_hook::libil2cpp::Void,
                7usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (year, month, day, hour, minute, second, kind))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_i32_i32_i32_i32_7(
        &mut self,
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
        millisecond: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, i32, i32, i32, i32),
                quest_hook::libil2cpp::Void,
                7usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (year, month, day, hour, minute, second, millisecond),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_i32_i32_i32_i32_Calendar9(
        &mut self,
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
        millisecond: i32,
        calendar: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (year, month, day, hour, minute, second, millisecond, calendar),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_i32_i32_i32_i32_DateTimeKind8(
        &mut self,
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
        millisecond: i32,
        kind: crate::System::DateTimeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, i32, i32, i32, i32, crate::System::DateTimeKind),
                quest_hook::libil2cpp::Void,
                8usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (year, month, day, hour, minute, second, millisecond, kind),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_0(
        &mut self,
        ticks: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i64), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ticks))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_DateTimeKind2(
        &mut self,
        ticks: i64,
        kind: crate::System::DateTimeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i64, crate::System::DateTimeKind),
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
            method.invoke_unchecked(self, (ticks, kind))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_DateTimeKind__cordl_bool3(
        &mut self,
        ticks: i64,
        kind: crate::System::DateTimeKind,
        isAmbiguousDst: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i64, crate::System::DateTimeKind, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ticks, kind, isAmbiguousDst))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u64_1(
        &mut self,
        dateData: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u64), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dateData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Date(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::DateTime, 0usize>("get_Date")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Date", 0usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Day(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Day")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Day", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_DayOfWeek(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DayOfWeek> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::DayOfWeek, 0usize>("get_DayOfWeek")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_DayOfWeek", 0usize
                )
            });
        let __cordl_ret: crate::System::DayOfWeek = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Hour(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Hour")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Hour", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalKind(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u64, 0usize>("get_InternalKind")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_InternalKind", 0usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalTicks(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i64, 0usize>("get_InternalTicks")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_InternalTicks", 0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Kind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeKind> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::DateTimeKind, 0usize>("get_Kind")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Kind", 0usize
                )
            });
        let __cordl_ret: crate::System::DateTimeKind = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Millisecond(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Millisecond")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Millisecond", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Minute(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Minute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Minute", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Month(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Month")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Month", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Now() -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), crate::System::DateTime, 0usize>("get_Now")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Now", 0usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Second(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Second")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Second", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Ticks(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i64, 0usize>("get_Ticks")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Ticks", 0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_TimeOfDay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::TimeSpan, 0usize>("get_TimeOfDay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_TimeOfDay", 0usize
                )
            });
        let __cordl_ret: crate::System::TimeSpan = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_UtcNow() -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), crate::System::DateTime, 0usize>("get_UtcNow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_UtcNow", 0usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Year(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Year")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Year", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        d: crate::System::DateTime,
        t: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTime, crate::System::TimeSpan),
                crate::System::DateTime,
                2usize,
            >("op_Addition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Addition", 2usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (d, t))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        d1: crate::System::DateTime,
        d2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTime, crate::System::DateTime),
                bool,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Equality", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (d1, d2)) };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan(
        t1: crate::System::DateTime,
        t2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTime, crate::System::DateTime),
                bool,
                2usize,
            >("op_GreaterThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_GreaterThan", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (t1, t2)) };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual(
        t1: crate::System::DateTime,
        t2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTime, crate::System::DateTime),
                bool,
                2usize,
            >("op_GreaterThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_GreaterThanOrEqual", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (t1, t2)) };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        d1: crate::System::DateTime,
        d2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTime, crate::System::DateTime),
                bool,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Inequality", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (d1, d2)) };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan(
        t1: crate::System::DateTime,
        t2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTime, crate::System::DateTime),
                bool,
                2usize,
            >("op_LessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_LessThan", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (t1, t2)) };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual(
        t1: crate::System::DateTime,
        t2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTime, crate::System::DateTime),
                bool,
                2usize,
            >("op_LessThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_LessThanOrEqual", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (t1, t2)) };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_DateTime1(
        d1: crate::System::DateTime,
        d2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTime, crate::System::DateTime),
                crate::System::TimeSpan,
                2usize,
            >("op_Subtraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Subtraction", 2usize
                )
            });
        let __cordl_ret: crate::System::TimeSpan = unsafe {
            method.invoke_unchecked((), (d1, d2))
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_TimeSpan0(
        d: crate::System::DateTime,
        t: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::DateTime, crate::System::TimeSpan),
                crate::System::DateTime,
                2usize,
            >("op_Subtraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "op_Subtraction", 2usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked((), (d, t))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+DateTime")]
impl AsRef<crate::System::IComparable> for crate::System::DateTime {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+DateTime")]
impl AsMut<crate::System::IComparable> for crate::System::DateTime {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+DateTime")]
impl AsRef<crate::System::IComparable_1<crate::System::DateTime>>
for crate::System::DateTime {
    fn as_ref(&self) -> &crate::System::IComparable_1<crate::System::DateTime> {
        todo!()
    }
}
#[cfg(feature = "System+DateTime")]
impl AsMut<crate::System::IComparable_1<crate::System::DateTime>>
for crate::System::DateTime {
    fn as_mut(&mut self) -> &mut crate::System::IComparable_1<crate::System::DateTime> {
        todo!()
    }
}
#[cfg(feature = "System+DateTime")]
impl AsRef<crate::System::IConvertible> for crate::System::DateTime {
    fn as_ref(&self) -> &crate::System::IConvertible {
        todo!()
    }
}
#[cfg(feature = "System+DateTime")]
impl AsMut<crate::System::IConvertible> for crate::System::DateTime {
    fn as_mut(&mut self) -> &mut crate::System::IConvertible {
        todo!()
    }
}
#[cfg(feature = "System+DateTime")]
impl AsRef<crate::System::IEquatable_1<crate::System::DateTime>>
for crate::System::DateTime {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::System::DateTime> {
        todo!()
    }
}
#[cfg(feature = "System+DateTime")]
impl AsMut<crate::System::IEquatable_1<crate::System::DateTime>>
for crate::System::DateTime {
    fn as_mut(&mut self) -> &mut crate::System::IEquatable_1<crate::System::DateTime> {
        todo!()
    }
}
#[cfg(feature = "System+DateTime")]
impl AsRef<crate::System::IFormattable> for crate::System::DateTime {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+DateTime")]
impl AsMut<crate::System::IFormattable> for crate::System::DateTime {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+DateTime")]
impl AsRef<crate::System::ISpanFormattable> for crate::System::DateTime {
    fn as_ref(&self) -> &crate::System::ISpanFormattable {
        todo!()
    }
}
#[cfg(feature = "System+DateTime")]
impl AsMut<crate::System::ISpanFormattable> for crate::System::DateTime {
    fn as_mut(&mut self) -> &mut crate::System::ISpanFormattable {
        todo!()
    }
}
#[cfg(feature = "System+DateTime")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::DateTime {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        todo!()
    }
}
#[cfg(feature = "System+DateTime")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::DateTime {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        todo!()
    }
}
