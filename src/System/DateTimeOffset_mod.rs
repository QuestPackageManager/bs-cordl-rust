#[cfg(feature = "System+DateTimeOffset")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DateTimeOffset {
    pub _dateTime: crate::System::DateTime,
    pub _offsetMinutes: i16,
}
#[cfg(feature = "System+DateTimeOffset")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::DateTimeOffset {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "DateTimeOffset";
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
#[cfg(feature = "System+DateTimeOffset")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::DateTimeOffset {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+DateTimeOffset")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::DateTimeOffset {
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
#[cfg(feature = "System+DateTimeOffset")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::DateTimeOffset {
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
#[cfg(feature = "System+DateTimeOffset")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::DateTimeOffset {
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
    pub fn AddTicks(
        &mut self,
        ticks: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddTicks",
            (ticks),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare(
        first: crate::System::DateTimeOffset,
        second: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (first, second))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FromFileTime(
        fileTime: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromFileTime", (fileTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromUnixTimeSeconds(
        seconds: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromUnixTimeSeconds", (seconds))?;
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
    pub fn ParseExact(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        styles: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseExact", (input, format, formatProvider, styles))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_DateTimeStyles1(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        styles: crate::System::Globalization::DateTimeStyles,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (input, formatProvider, styles))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_Il2CppString_IFormatProvider0(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (input, formatProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IComparable_CompareTo(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IComparable.CompareTo",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_IDeserializationCallback_OnDeserialization(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
            (sender),
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
    pub fn ToLocalTime_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToLocalTime",
            (),
        )?;
        Ok(__cordl_ret.into())
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
    pub fn ToString_IFormatProvider1(
        &mut self,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (formatProvider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider2(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUniversalTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToUniversalTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormat(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryFormat",
            (destination, charsWritten, format, formatProvider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        styles: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (input, formatProvider, styles, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact_Il2CppArray1(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formats: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        styles: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseExact", (input, formats, formatProvider, styles, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParseExact_Il2CppString0(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        styles: crate::System::Globalization::DateTimeStyles,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseExact", (input, format, formatProvider, styles, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateDate(
        dateTime: crate::System::DateTime,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateDate", (dateTime, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateOffset(
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateOffset", (offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateStyles(
        style: crate::System::Globalization::DateTimeStyles,
        parameterName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::DateTimeStyles> {
        let __cordl_ret: crate::System::Globalization::DateTimeStyles = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateStyles", (style, parameterName))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext6(
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
        Ok(__cordl_ret.into())
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
        calendar: quest_hook::libil2cpp::Gc<crate::System::Globalization::Calendar>,
        offset: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (year, month, day, hour, minute, second, millisecond, calendar, offset),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_ClockDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ClockDateTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_DateTime",
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
    pub fn get_Hour(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Hour",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_LocalDateTime",
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
    pub fn get_Now() -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Now", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Offset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Offset",
            (),
        )?;
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
    pub fn get_UtcDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_UtcDateTime",
            (),
        )?;
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
    pub fn op_Equality(
        left: crate::System::DateTimeOffset,
        right: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        dateTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (dateTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::System::DateTimeOffset,
        right: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        left: crate::System::DateTimeOffset,
        right: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsRef<crate::System::IComparable> for crate::System::DateTimeOffset {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsMut<crate::System::IComparable> for crate::System::DateTimeOffset {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsRef<crate::System::IComparable_1<crate::System::DateTimeOffset>>
for crate::System::DateTimeOffset {
    fn as_ref(&self) -> &crate::System::IComparable_1<crate::System::DateTimeOffset> {
        todo!()
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsMut<crate::System::IComparable_1<crate::System::DateTimeOffset>>
for crate::System::DateTimeOffset {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::System::DateTimeOffset> {
        todo!()
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsRef<crate::System::IEquatable_1<crate::System::DateTimeOffset>>
for crate::System::DateTimeOffset {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::System::DateTimeOffset> {
        todo!()
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsMut<crate::System::IEquatable_1<crate::System::DateTimeOffset>>
for crate::System::DateTimeOffset {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::System::DateTimeOffset> {
        todo!()
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsRef<crate::System::IFormattable> for crate::System::DateTimeOffset {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsMut<crate::System::IFormattable> for crate::System::DateTimeOffset {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsRef<crate::System::ISpanFormattable> for crate::System::DateTimeOffset {
    fn as_ref(&self) -> &crate::System::ISpanFormattable {
        todo!()
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsMut<crate::System::ISpanFormattable> for crate::System::DateTimeOffset {
    fn as_mut(&mut self) -> &mut crate::System::ISpanFormattable {
        todo!()
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsRef<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::DateTimeOffset {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::Serialization::IDeserializationCallback {
        todo!()
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsMut<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::DateTimeOffset {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::IDeserializationCallback {
        todo!()
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::DateTimeOffset {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        todo!()
    }
}
#[cfg(feature = "System+DateTimeOffset")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::DateTimeOffset {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        todo!()
    }
}
