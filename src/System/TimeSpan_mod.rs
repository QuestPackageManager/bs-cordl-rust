#[cfg(feature = "System+TimeSpan")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TimeSpan {
    pub _ticks: i64,
}
#[cfg(feature = "System+TimeSpan")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::TimeSpan => "System"."TimeSpan"
);
#[cfg(feature = "System+TimeSpan")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::TimeSpan {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+TimeSpan")]
impl crate::System::TimeSpan {
    pub const DaysPerTick: f64 = 0.0000000000011574074074074074f64;
    pub const HoursPerTick: f64 = 0.000000000027777777777777777f64;
    pub const MaxMilliSeconds: i64 = 922337203685477i64;
    pub const MaxSeconds: i64 = 922337203685i64;
    pub const MillisPerDay: i32 = 86400000i32;
    pub const MillisPerHour: i32 = 3600000i32;
    pub const MillisPerMinute: i32 = 60000i32;
    pub const MillisPerSecond: i32 = 1000i32;
    pub const MillisecondsPerTick: f64 = 0.0001f64;
    pub const MinMilliSeconds: i64 = -922337203685477i64;
    pub const MinSeconds: i64 = -922337203685i64;
    pub const MinutesPerTick: f64 = 0.0000000016666666666666667f64;
    pub const SecondsPerTick: f64 = 0.0000001f64;
    pub const TicksPerDay: i64 = 864000000000i64;
    pub const TicksPerHour: i64 = 36000000000i64;
    pub const TicksPerMillisecond: i64 = 10000i64;
    pub const TicksPerMinute: i64 = 600000000i64;
    pub const TicksPerSecond: i64 = 10000000i64;
    pub const TicksPerTenthSecond: i64 = 1000000i64;
    pub fn Add(
        &mut self,
        ts: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Add",
            (ts),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare(
        t1: crate::System::TimeSpan,
        t2: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (t1, t2))?;
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
    pub fn CompareTo_TimeSpan1(
        &mut self,
        value: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Duration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Duration",
            (),
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
    pub fn Equals_TimeSpan1(
        &mut self,
        obj: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FromDays(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromDays", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromHours(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromHours", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromMilliseconds(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromMilliseconds", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromMinutes(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromMinutes", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSeconds(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromSeconds", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromTicks(
        value: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromTicks", (value))?;
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
    pub fn Interval(
        value: f64,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Interval", (value, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn Negate(&mut self) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Negate",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_IFormatProvider1(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (input, formatProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_Il2CppString0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subtract(
        &mut self,
        ts: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Subtract",
            (ts),
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
    pub fn TryParseExact(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseExact", (input, format, formatProvider, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_1(
        &mut self,
        hours: i32,
        minutes: i32,
        seconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (hours, minutes, seconds),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_i32_i32_2(
        &mut self,
        days: i32,
        hours: i32,
        minutes: i32,
        seconds: i32,
        milliseconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (days, hours, minutes, seconds, milliseconds),
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
    pub fn get_Days(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Days",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Hours(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Hours",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Minutes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Minutes",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Seconds(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Seconds",
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
    pub fn get_TotalDays(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_TotalDays",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TotalHours(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_TotalHours",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TotalMilliseconds(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_TotalMilliseconds",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TotalMinutes(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_TotalMinutes",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TotalSeconds(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_TotalSeconds",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        t1: crate::System::TimeSpan,
        t2: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        t1: crate::System::TimeSpan,
        t2: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan(
        t1: crate::System::TimeSpan,
        t2: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual(
        t1: crate::System::TimeSpan,
        t2: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        t1: crate::System::TimeSpan,
        t2: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan(
        t1: crate::System::TimeSpan,
        t2: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual(
        t1: crate::System::TimeSpan,
        t2: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        t1: crate::System::TimeSpan,
        t2: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (t1, t2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        t: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (t))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+TimeSpan")]
impl AsRef<crate::System::IComparable> for crate::System::TimeSpan {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+TimeSpan")]
impl AsMut<crate::System::IComparable> for crate::System::TimeSpan {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+TimeSpan")]
impl AsRef<crate::System::IComparable_1<crate::System::TimeSpan>>
for crate::System::TimeSpan {
    fn as_ref(&self) -> &crate::System::IComparable_1<crate::System::TimeSpan> {
        todo!()
    }
}
#[cfg(feature = "System+TimeSpan")]
impl AsMut<crate::System::IComparable_1<crate::System::TimeSpan>>
for crate::System::TimeSpan {
    fn as_mut(&mut self) -> &mut crate::System::IComparable_1<crate::System::TimeSpan> {
        todo!()
    }
}
#[cfg(feature = "System+TimeSpan")]
impl AsRef<crate::System::IEquatable_1<crate::System::TimeSpan>>
for crate::System::TimeSpan {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::System::TimeSpan> {
        todo!()
    }
}
#[cfg(feature = "System+TimeSpan")]
impl AsMut<crate::System::IEquatable_1<crate::System::TimeSpan>>
for crate::System::TimeSpan {
    fn as_mut(&mut self) -> &mut crate::System::IEquatable_1<crate::System::TimeSpan> {
        todo!()
    }
}
#[cfg(feature = "System+TimeSpan")]
impl AsRef<crate::System::IFormattable> for crate::System::TimeSpan {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+TimeSpan")]
impl AsMut<crate::System::IFormattable> for crate::System::TimeSpan {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+TimeSpan")]
impl AsRef<crate::System::ISpanFormattable> for crate::System::TimeSpan {
    fn as_ref(&self) -> &crate::System::ISpanFormattable {
        todo!()
    }
}
#[cfg(feature = "System+TimeSpan")]
impl AsMut<crate::System::ISpanFormattable> for crate::System::TimeSpan {
    fn as_mut(&mut self) -> &mut crate::System::ISpanFormattable {
        todo!()
    }
}
