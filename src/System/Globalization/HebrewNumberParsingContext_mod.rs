#[cfg(feature = "System+Globalization+HebrewNumberParsingContext")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HebrewNumberParsingContext {
    pub state: crate::System::Globalization::HebrewNumber_HS,
    pub result: i32,
}
#[cfg(feature = "System+Globalization+HebrewNumberParsingContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::HebrewNumberParsingContext => "System.Globalization"
    ."HebrewNumberParsingContext"
);
#[cfg(feature = "System+Globalization+HebrewNumberParsingContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::HebrewNumberParsingContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+HebrewNumberParsingContext")]
impl crate::System::Globalization::HebrewNumberParsingContext {
    pub fn _ctor(
        &mut self,
        result: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (result),
        )?;
        Ok(__cordl_ret.into())
    }
}
