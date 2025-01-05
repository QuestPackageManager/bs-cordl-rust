#[cfg(feature = "System+Xml+BinXmlDateTime")]
#[repr(C)]
#[derive(Debug)]
pub struct BinXmlDateTime {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Xml+BinXmlDateTime")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::BinXmlDateTime => "System.Xml"
    ."BinXmlDateTime"
);
#[cfg(feature = "System+Xml+BinXmlDateTime")]
impl std::ops::Deref for crate::System::Xml::BinXmlDateTime {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+BinXmlDateTime")]
impl std::ops::DerefMut for crate::System::Xml::BinXmlDateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+BinXmlDateTime")]
impl crate::System::Xml::BinXmlDateTime {
    pub fn BreakDownXsdDate(
        val: i64,
        yr: quest_hook::libil2cpp::ByRefMut<i32>,
        mnth: quest_hook::libil2cpp::ByRefMut<i32>,
        day: quest_hook::libil2cpp::ByRefMut<i32>,
        negTimeZone: quest_hook::libil2cpp::ByRefMut<bool>,
        hr: quest_hook::libil2cpp::ByRefMut<i32>,
        min: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BreakDownXsdDate", (val, yr, mnth, day, negTimeZone, hr, min))?;
        Ok(__cordl_ret.into())
    }
    pub fn BreakDownXsdDateTime(
        val: i64,
        yr: quest_hook::libil2cpp::ByRefMut<i32>,
        mnth: quest_hook::libil2cpp::ByRefMut<i32>,
        day: quest_hook::libil2cpp::ByRefMut<i32>,
        hr: quest_hook::libil2cpp::ByRefMut<i32>,
        min: quest_hook::libil2cpp::ByRefMut<i32>,
        sec: quest_hook::libil2cpp::ByRefMut<i32>,
        ms: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BreakDownXsdDateTime", (val, yr, mnth, day, hr, min, sec, ms))?;
        Ok(__cordl_ret.into())
    }
    pub fn BreakDownXsdTime(
        val: i64,
        hr: quest_hook::libil2cpp::ByRefMut<i32>,
        min: quest_hook::libil2cpp::ByRefMut<i32>,
        sec: quest_hook::libil2cpp::ByRefMut<i32>,
        ms: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BreakDownXsdTime", (val, hr, min, sec, ms))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFractions_DateTime0(
        dt: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFractions", (dt))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFractions_DateTimeOffset1(
        dt: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFractions", (dt))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKatmaiDateTicks(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKatmaiDateTicks", (data, pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKatmaiTimeTicks(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKatmaiTimeTicks", (data, pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKatmaiTimeZoneTicks(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKatmaiTimeZoneTicks", (data, pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn SqlDateTimeToDateTime(
        dateticks: i32,
        timeticks: u32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SqlDateTimeToDateTime", (dateticks, timeticks))?;
        Ok(__cordl_ret.into())
    }
    pub fn SqlDateTimeToString(
        dateticks: i32,
        timeticks: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SqlDateTimeToString", (dateticks, timeticks))?;
        Ok(__cordl_ret.into())
    }
    pub fn SqlSmallDateTimeToDateTime(
        dateticks: i16,
        timeticks: u16,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SqlSmallDateTimeToDateTime", (dateticks, timeticks))?;
        Ok(__cordl_ret.into())
    }
    pub fn SqlSmallDateTimeToString(
        dateticks: i16,
        timeticks: u16,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SqlSmallDateTimeToString", (dateticks, timeticks))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write2Dig(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write2Dig", (sb, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write3Dec(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write3Dec", (sb, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write4DigNeg(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write4DigNeg", (sb, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteDate(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        yr: i32,
        mnth: i32,
        day: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteDate", (sb, yr, mnth, day))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTime(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        hr: i32,
        min: i32,
        sec: i32,
        ms: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteTime", (sb, hr, min, sec, ms))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTimeFullPrecision(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        hr: i32,
        min: i32,
        sec: i32,
        fraction: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteTimeFullPrecision", (sb, hr, min, sec, fraction))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTimeZone_TimeSpan0(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        zone: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteTimeZone", (sb, zone))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTimeZone__cordl_bool_i32_i32_1(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        negTimeZone: bool,
        hr: i32,
        min: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteTimeZone", (sb, negTimeZone, hr, min))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdDateTimeToString(
        val: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdDateTimeToString", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdDateToString(
        val: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdDateToString", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdKatmaiDateOffsetToDateTimeOffset(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdKatmaiDateOffsetToDateTimeOffset", (data, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdKatmaiDateOffsetToString(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdKatmaiDateOffsetToString", (data, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdKatmaiDateTimeOffsetToDateTimeOffset(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdKatmaiDateTimeOffsetToDateTimeOffset", (data, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdKatmaiDateTimeOffsetToString(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdKatmaiDateTimeOffsetToString", (data, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdKatmaiDateTimeToDateTime(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdKatmaiDateTimeToDateTime", (data, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdKatmaiDateTimeToString(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdKatmaiDateTimeToString", (data, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdKatmaiDateToDateTime(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdKatmaiDateToDateTime", (data, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdKatmaiDateToString(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdKatmaiDateToString", (data, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdKatmaiTimeOffsetToDateTimeOffset(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdKatmaiTimeOffsetToDateTimeOffset", (data, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdKatmaiTimeOffsetToString(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdKatmaiTimeOffsetToString", (data, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdKatmaiTimeToDateTime(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdKatmaiTimeToDateTime", (data, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdKatmaiTimeToString(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdKatmaiTimeToString", (data, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn XsdTimeToString(
        val: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XsdTimeToString", (val))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+BinXmlDateTime")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::BinXmlDateTime {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
