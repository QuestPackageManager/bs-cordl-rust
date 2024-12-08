#[cfg(feature = "System+DateTimeOffset")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DateTimeOffset {
    pub _dateTime: crate::System::DateTime,
    pub _offsetMinutes: i16,
}
#[cfg(feature = "System+DateTimeOffset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeOffset => "System"
    ."DateTimeOffset"
);
#[cfg(feature = "System+DateTimeOffset")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::DateTimeOffset {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl crate::System::DateTimeOffset {
    pub fn get_Month(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Month",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToUniversalTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToUniversalTime",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToLocalTime_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToLocalTime",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToLocalTime__cordl_bool1(
        &mut self,
        throwOnOverflow: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToLocalTime",
            (throwOnOverflow),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_UtcDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_UtcDateTime",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Day(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Day",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IComparable_CompareTo(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IComparable.CompareTo",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_ClockDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ClockDateTime",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Hour(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Hour",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i64_TimeSpan0(
        &mut self,
        ticks: i64,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ticks, offset),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DateTime1(
        &mut self,
        dateTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (dateTime),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DateTime_TimeSpan2(
        &mut self,
        dateTime: crate::System::DateTime,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (dateTime, offset),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_i32_i32_i32_TimeSpan3(
        &mut self,
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (year, month, day, hour, minute, second, offset),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_i32_i32_i32_i32_TimeSpan4(
        &mut self,
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
        millisecond: i32,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (year, month, day, hour, minute, second, millisecond, offset),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_i32_i32_i32_i32_Calendar_TimeSpan5(
        &mut self,
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
        millisecond: i32,
        calendar: *mut crate::System::Globalization::Calendar,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (year, month, day, hour, minute, second, millisecond, calendar, offset),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext6(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (info, context),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddTicks(
        &mut self,
        ticks: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddTicks",
            (ticks),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompareTo(
        &mut self,
        other: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Runtime_Serialization_IDeserializationCallback_OnDeserialization(
        &mut self,
        sender: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
            (sender),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_LocalDateTime",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_TimeOfDay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_TimeOfDay",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Offset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Offset",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Year(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Year",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_DateTimeOffset1(
        &mut self,
        other: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Minute(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Minute",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Ticks(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Ticks",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Runtime.Serialization.ISerializable.GetObjectData",
            (info, context),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_IFormatProvider1(
        &mut self,
        formatProvider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (formatProvider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_String_IFormatProvider2(
        &mut self,
        format: *mut crate::System::String,
        formatProvider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Millisecond(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Millisecond",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryFormat(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
        formatProvider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryFormat",
            (destination, charsWritten, format, formatProvider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Second(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Second",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_DateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_DateTime",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
