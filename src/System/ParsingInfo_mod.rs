#[cfg(feature = "System+ParsingInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ParsingInfo {
    pub calendar: *mut crate::System::Globalization::Calendar,
    pub dayOfWeek: i32,
    pub timeMark: crate::System::DateTimeParse_TM,
    pub fUseHour12: bool,
    pub fUseTwoDigitYear: bool,
    pub fAllowInnerWhite: bool,
    pub fAllowTrailingWhite: bool,
    pub fCustomNumberParser: bool,
    pub parseNumberDelegate: *mut crate::System::DateTimeParse_MatchNumberDelegate,
}
#[cfg(feature = "System+ParsingInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ParsingInfo => "System"."ParsingInfo"
);
#[cfg(feature = "System+ParsingInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::ParsingInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+ParsingInfo")]
impl crate::System::ParsingInfo {
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
