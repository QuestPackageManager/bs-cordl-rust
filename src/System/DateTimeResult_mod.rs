#[cfg(feature = "System+DateTimeResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DateTimeResult {
    pub Year: i32,
    pub Month: i32,
    pub Day: i32,
    pub Hour: i32,
    pub Minute: i32,
    pub Second: i32,
    pub fraction: f64,
    pub era: i32,
    pub flags: crate::System::ParseFlags,
    pub timeZoneOffset: crate::System::TimeSpan,
    pub calendar: *mut crate::System::Globalization::Calendar,
    pub parsedDate: crate::System::DateTime,
    pub failure: crate::System::ParseFailureKind,
    pub failureMessageID: *mut crate::System::String,
    pub failureMessageFormatArgument: *mut crate::System::Object,
    pub failureArgumentName: *mut crate::System::String,
    pub originalDateTimeString: crate::System::ReadOnlySpan_1<char>,
    pub failedFormatSpecifier: crate::System::ReadOnlySpan_1<char>,
}
#[cfg(feature = "System+DateTimeResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DateTimeResult => "System"
    ."DateTimeResult"
);
#[cfg(feature = "System+DateTimeResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::DateTimeResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+DateTimeResult")]
impl crate::System::DateTimeResult {
    pub fn Init(
        &mut self,
        originalDateTimeString: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (originalDateTimeString),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetBadDateTimeFailure(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetBadDateTimeFailure",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetBadFormatSpecifierFailure_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetBadFormatSpecifierFailure",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetBadFormatSpecifierFailure_ReadOnlySpan_1_1(
        &mut self,
        failedFormatSpecifier: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetBadFormatSpecifierFailure",
            (failedFormatSpecifier),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetDate(
        &mut self,
        year: i32,
        month: i32,
        day: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetDate",
            (year, month, day),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetFailure_Object1(
        &mut self,
        failure: crate::System::ParseFailureKind,
        failureMessageID: *mut crate::System::String,
        failureMessageFormatArgument: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (failure, failureMessageID, failureMessageFormatArgument),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetFailure_Object_String2(
        &mut self,
        failure: crate::System::ParseFailureKind,
        failureMessageID: *mut crate::System::String,
        failureMessageFormatArgument: *mut crate::System::Object,
        failureArgumentName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (
                failure,
                failureMessageID,
                failureMessageFormatArgument,
                failureArgumentName,
            ),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetFailure_ParseFailureKind_String0(
        &mut self,
        failure: crate::System::ParseFailureKind,
        failureMessageID: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (failure, failureMessageID),
        )?;
        Ok(__cordl_ret)
    }
}