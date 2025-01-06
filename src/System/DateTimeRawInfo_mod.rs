#[cfg(feature = "System+DateTimeRawInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DateTimeRawInfo {
    pub num: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub numCount: i32,
    pub month: i32,
    pub year: i32,
    pub dayOfWeek: i32,
    pub era: i32,
    pub timeMark: crate::System::DateTimeParse_TM,
    pub fraction: f64,
    pub hasSameDateAndTimeSeparators: bool,
}
#[cfg(feature = "System+DateTimeRawInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeRawInfo => "System"
    ."DateTimeRawInfo"
);
#[cfg(feature = "System+DateTimeRawInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::DateTimeRawInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+DateTimeRawInfo")]
impl crate::System::DateTimeRawInfo {
    pub fn AddNumber(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddNumber",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNumber(&mut self, index: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetNumber",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        numberBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (numberBuffer),
        )?;
        Ok(__cordl_ret.into())
    }
}
