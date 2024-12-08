#[cfg(feature = "System+Globalization+TimeSpanFormat+FormatLiterals")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TimeSpanFormat_FormatLiterals {
    pub AppCompatLiteral: *mut crate::System::String,
    pub dd: i32,
    pub hh: i32,
    pub mm: i32,
    pub ss: i32,
    pub ff: i32,
    pub _literals: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
}
#[cfg(feature = "System+Globalization+TimeSpanFormat+FormatLiterals")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Globalization::TimeSpanFormat_FormatLiterals => "System.Globalization"
    ."TimeSpanFormat/FormatLiterals"
);
#[cfg(feature = "System+Globalization+TimeSpanFormat+FormatLiterals")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Globalization::TimeSpanFormat_FormatLiterals {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Globalization+TimeSpanFormat+FormatLiterals")]
impl crate::System::Globalization::TimeSpanFormat_FormatLiterals {
    pub fn Init(
        &mut self,
        format: crate::System::ReadOnlySpan_1<char>,
        useInvariantFieldLengths: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (format, useInvariantFieldLengths),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_DayHourSep(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_DayHourSep",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_End(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_End",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_HourMinuteSep(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_HourMinuteSep",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_MinuteSecondSep(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_MinuteSecondSep",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_SecondFractionSep(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_SecondFractionSep",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Start",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Globalization+TimeSpanFormat+Pattern")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSpanFormat_Pattern {
    Full = 2i32,
    Minimum = 1i32,
    None = 0i32,
}
#[cfg(feature = "System+Globalization+TimeSpanFormat+Pattern")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::TimeSpanFormat_Pattern =>
    "System.Globalization"."TimeSpanFormat/Pattern"
);
#[cfg(feature = "System+Globalization+TimeSpanFormat")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeSpanFormat {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Globalization+TimeSpanFormat")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::TimeSpanFormat =>
    "System.Globalization"."TimeSpanFormat"
);
#[cfg(feature = "System+Globalization+TimeSpanFormat")]
impl std::ops::Deref for crate::System::Globalization::TimeSpanFormat {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+TimeSpanFormat")]
impl std::ops::DerefMut for crate::System::Globalization::TimeSpanFormat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+TimeSpanFormat")]
impl crate::System::Globalization::TimeSpanFormat {
    #[cfg(feature = "System+Globalization+TimeSpanFormat+Pattern")]
    pub type Pattern = crate::System::Globalization::TimeSpanFormat_Pattern;
    #[cfg(feature = "System+Globalization+TimeSpanFormat+FormatLiterals")]
    pub type FormatLiterals = crate::System::Globalization::TimeSpanFormat_FormatLiterals;
}
#[cfg(feature = "System+Globalization+TimeSpanFormat")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::TimeSpanFormat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
