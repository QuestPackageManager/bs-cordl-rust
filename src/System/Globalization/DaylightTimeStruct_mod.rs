#[cfg(feature = "System+Globalization+DaylightTimeStruct")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DaylightTimeStruct {
    pub Start: crate::System::DateTime,
    pub End: crate::System::DateTime,
    pub Delta: crate::System::TimeSpan,
}
#[cfg(feature = "System+Globalization+DaylightTimeStruct")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::DaylightTimeStruct =>
    "System.Globalization"."DaylightTimeStruct"
);
#[cfg(feature = "System+Globalization+DaylightTimeStruct")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::DaylightTimeStruct {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+DaylightTimeStruct")]
impl crate::System::Globalization::DaylightTimeStruct {
    pub fn _ctor(
        &mut self,
        start: crate::System::DateTime,
        end: crate::System::DateTime,
        delta: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (start, end, delta),
        )?;
        Ok(__cordl_ret)
    }
}
