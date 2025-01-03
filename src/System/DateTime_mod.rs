#[cfg(feature = "System+DateTime")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DateTime {
    pub _dateData: u64,
}
#[cfg(feature = "System+DateTime")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DateTime => "System"."DateTime"
);
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
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddDays",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddMilliseconds(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddMilliseconds",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddMonths(
        &mut self,
        months: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddMonths",
            (months),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSeconds(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddSeconds",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTicks(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddTicks",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddYears(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddYears",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_TimeSpan0(
        &mut self,
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Add",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_f64_i32_1(
        &mut self,
        value: f64,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Add",
            (value, scale),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare(
        t1: crate::System::DateTime,
        t2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_DateTime1(
        &mut self,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_Il2CppObject0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DateToTicks(
        year: i32,
        month: i32,
        day: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DateToTicks", (year, month, day))?;
        Ok(__cordl_ret.into())
    }
    pub fn DaysInMonth(year: i32, month: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DaysInMonth", (year, month))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_DateTime1(
        &mut self,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FromBinary(
        dateData: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBinary", (dateData))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromBinaryRaw(
        dateData: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBinaryRaw", (dateData))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromFileTime(
        fileTime: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromFileTime", (fileTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromFileTimeUtc(
        fileTime: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromFileTimeUtc", (fileTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDatePart_ByRefMut_ByRefMut_ByRefMut1(
        &mut self,
        year: quest_hook::libil2cpp::ByRefMut<i32>,
        month: quest_hook::libil2cpp::ByRefMut<i32>,
        day: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDatePart",
            (year, month, day),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDatePart_i32_0(
        &mut self,
        part: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDatePart",
            (part),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemTimeAsFileTime() -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSystemTimeAsFileTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TypeCode> {
        let __cordl_ret: crate::System::TypeCode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetTypeCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAmbiguousDaylightSavingTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsAmbiguousDaylightSavingTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLeapYear(year: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLeapYear", (year))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseExact_Il2CppArray_DateTimeStyles2(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formats: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        style: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseExact", (s, formats, provider, style))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseExact_Il2CppString0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseExact", (s, format, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseExact_Il2CppString_DateTimeStyles1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        style: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseExact", (s, format, provider, style))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_IFormatProvider1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (s, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_IFormatProvider_DateTimeStyles2(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        styles: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (s, provider, styles))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_Il2CppString0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpecifyKind(
        value: crate::System::DateTime,
        kind: crate::System::DateTimeKind,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SpecifyKind", (value, kind))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subtract(
        &mut self,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Subtract",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToBoolean(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToBoolean",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToByte(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToByte",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToChar(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToChar",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDateTime(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToDateTime",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDecimal(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToDecimal",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDouble(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToDouble",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt16(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToInt16",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt32(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToInt32",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt64(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToInt64",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToSByte(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToSByte",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToSingle(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToSingle",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToType",
            (_cordl_type, provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt16(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToUInt16",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt32(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToUInt32",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt64(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToUInt64",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Runtime.Serialization.ISerializable.GetObjectData",
            (info, context),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TimeToTicks(
        hour: i32,
        minute: i32,
        second: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TimeToTicks", (hour, minute, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBinaryRaw(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToBinaryRaw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLocalTime_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToLocalTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToLocalTime__cordl_bool1(
        &mut self,
        throwOnOverflow: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToLocalTime",
            (throwOnOverflow),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_IFormatProvider2(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider3(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUniversalTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToUniversalTime",
            (),
        )?;
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
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryCreate",
                (year, month, day, hour, minute, second, millisecond, result),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormat(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryFormat",
            (destination, charsWritten, format, provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        styles: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (s, provider, styles, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        style: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseExact", (s, format, provider, style, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext10(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (info, context),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_4(
        &mut self,
        year: i32,
        month: i32,
        day: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (year, month, day),
        )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (year, month, day, hour, minute, second),
        )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (year, month, day, hour, minute, second, kind),
        )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (year, month, day, hour, minute, second, millisecond),
        )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (year, month, day, hour, minute, second, millisecond, calendar),
        )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (year, month, day, hour, minute, second, millisecond, kind),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_0(
        &mut self,
        ticks: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ticks),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_DateTimeKind2(
        &mut self,
        ticks: i64,
        kind: crate::System::DateTimeKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ticks, kind),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_DateTimeKind__cordl_bool3(
        &mut self,
        ticks: i64,
        kind: crate::System::DateTimeKind,
        isAmbiguousDst: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ticks, kind, isAmbiguousDst),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u64_1(
        &mut self,
        dateData: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (dateData),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Date(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Date",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Day(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Day",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DayOfWeek(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DayOfWeek> {
        let __cordl_ret: crate::System::DayOfWeek = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_DayOfWeek",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Hour(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Hour",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalKind(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_InternalKind",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalTicks(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_InternalTicks",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Kind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeKind> {
        let __cordl_ret: crate::System::DateTimeKind = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Kind",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Millisecond(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Millisecond",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Minute(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Minute",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Month(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Month",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Now() -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Now", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Second(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Second",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Ticks(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Ticks",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TimeOfDay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_TimeOfDay",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UtcNow() -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UtcNow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Year(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Year",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        d: crate::System::DateTime,
        t: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (d, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        d1: crate::System::DateTime,
        d2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan(
        t1: crate::System::DateTime,
        t2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual(
        t1: crate::System::DateTime,
        t2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        d1: crate::System::DateTime,
        d2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan(
        t1: crate::System::DateTime,
        t2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual(
        t1: crate::System::DateTime,
        t2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_DateTime1(
        d1: crate::System::DateTime,
        d2: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_TimeSpan0(
        d: crate::System::DateTime,
        t: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (d, t))?;
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
