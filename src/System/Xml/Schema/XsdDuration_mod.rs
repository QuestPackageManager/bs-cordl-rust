#[cfg(feature = "System+Xml+Schema+XsdDuration")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XsdDuration {
    pub years: i32,
    pub months: i32,
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    pub nanoseconds: u32,
}
#[cfg(feature = "System+Xml+Schema+XsdDuration")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdDuration =>
    "System.Xml.Schema"."XsdDuration"
);
#[cfg(feature = "System+Xml+Schema+XsdDuration")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Schema::XsdDuration {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration")]
impl crate::System::Xml::Schema::XsdDuration {
    #[cfg(feature = "System+Xml+Schema+XsdDuration+DurationType")]
    pub type DurationType = crate::System::Xml::Schema::XsdDuration_DurationType;
    #[cfg(feature = "System+Xml+Schema+XsdDuration+Parts")]
    pub type Parts = crate::System::Xml::Schema::XsdDuration_Parts;
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_XsdDuration_DurationType1(
        &mut self,
        durationType: crate::System::Xml::Schema::XsdDuration_DurationType,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (durationType),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToTimeSpan_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToTimeSpan",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToTimeSpan_XsdDuration_DurationType1(
        &mut self,
        durationType: crate::System::Xml::Schema::XsdDuration_DurationType,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToTimeSpan",
            (durationType),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryToTimeSpan_ByRefMut0(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_ret: *mut crate::System::Exception = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryToTimeSpan",
            (result),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryToTimeSpan_XsdDuration_DurationType_ByRefMut1(
        &mut self,
        durationType: crate::System::Xml::Schema::XsdDuration_DurationType,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_ret: *mut crate::System::Exception = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryToTimeSpan",
            (durationType, result),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString3(
        &mut self,
        s: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (s),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString_XsdDuration_DurationType4(
        &mut self,
        s: *mut quest_hook::libil2cpp::Il2CppString,
        durationType: crate::System::Xml::Schema::XsdDuration_DurationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (s, durationType),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TimeSpan1(
        &mut self,
        timeSpan: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (timeSpan),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TimeSpan_XsdDuration_DurationType2(
        &mut self,
        timeSpan: crate::System::TimeSpan,
        durationType: crate::System::Xml::Schema::XsdDuration_DurationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (timeSpan, durationType),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_i32_i32_i32_i32_i32_i32_i32_0(
        &mut self,
        isNegative: bool,
        years: i32,
        months: i32,
        days: i32,
        hours: i32,
        minutes: i32,
        seconds: i32,
        nanoseconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (isNegative, years, months, days, hours, minutes, seconds, nanoseconds),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Days(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Days",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Hours(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Hours",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNegative(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNegative",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Minutes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Minutes",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Months(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Months",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Nanoseconds(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Nanoseconds",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Seconds(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Seconds",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Years(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Years",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+DurationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XsdDuration_DurationType {
    DayTimeDuration = 2i32,
    Duration = 0i32,
    YearMonthDuration = 1i32,
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+DurationType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdDuration_DurationType =>
    "System.Xml.Schema"."XsdDuration/DurationType"
);
#[cfg(feature = "System+Xml+Schema+XsdDuration+Parts")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XsdDuration_Parts {
    HasDays = 4i32,
    HasHours = 8i32,
    HasMinutes = 16i32,
    HasMonths = 2i32,
    HasNone = 0i32,
    HasSeconds = 32i32,
    HasYears = 1i32,
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+Parts")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XsdDuration_Parts =>
    "System.Xml.Schema"."XsdDuration/Parts"
);
