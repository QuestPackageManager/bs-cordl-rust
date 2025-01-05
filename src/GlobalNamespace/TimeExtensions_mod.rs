#[cfg(feature = "TimeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TimeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TimeExtensions => ""
    ."TimeExtensions"
);
#[cfg(feature = "TimeExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::TimeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TimeExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::TimeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TimeExtensions")]
impl crate::GlobalNamespace::TimeExtensions {
    pub fn AsUnixTime(
        unixTime: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsUnixTime", (unixTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn DaysToSeconds(days: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DaysToSeconds", (days))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFormattedRemainingTimeTwoOfDaysHoursMinutes(
        timeSpan: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFormattedRemainingTimeTwoOfDaysHoursMinutes", (timeSpan))?;
        Ok(__cordl_ret.into())
    }
    pub fn Hours(_cordl_time: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Hours", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn HoursToSeconds(hours: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HoursToSeconds", (hours))?;
        Ok(__cordl_ret.into())
    }
    pub fn Milliseconds(_cordl_time: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Milliseconds", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn MinSecDurationText(
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MinSecDurationText", (duration))?;
        Ok(__cordl_ret.into())
    }
    pub fn MinSecMillisecDurationText(
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MinSecMillisecDurationText", (duration))?;
        Ok(__cordl_ret.into())
    }
    pub fn Minutes(_cordl_time: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Minutes", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn MinutesToSeconds(minutes: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MinutesToSeconds", (minutes))?;
        Ok(__cordl_ret.into())
    }
    pub fn OneBeatDuration(bpm: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OneBeatDuration", (bpm))?;
        Ok(__cordl_ret.into())
    }
    pub fn Seconds(_cordl_time: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Seconds", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn SecondsToDays(_cordl_time: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SecondsToDays", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn SecondsToHours(_cordl_time: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SecondsToHours", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn SecondsToMinutes_f32_0(seconds: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SecondsToMinutes", (seconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn SecondsToMinutes_i32_1(
        _cordl_time: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SecondsToMinutes", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn TimeToBeat(_cordl_time: f32, bpm: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TimeToBeat", (_cordl_time, bpm))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUnixTime(
        dateTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUnixTime", (dateTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn TotalDays(_cordl_time: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TotalDays", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn TotalHours(_cordl_time: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TotalHours", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn TotalMinutes(_cordl_time: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TotalMinutes", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn TotalSeconds(_cordl_time: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TotalSeconds", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TimeExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TimeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
