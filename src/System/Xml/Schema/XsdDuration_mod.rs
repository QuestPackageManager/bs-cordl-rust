#[cfg(feature = "System+Xml+Schema+XsdDuration")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::XsdDuration {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "XsdDuration";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Xml::Schema::XsdDuration {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Xml::Schema::XsdDuration {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Xml::Schema::XsdDuration {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Xml::Schema::XsdDuration {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_XsdDuration_DurationType1(
        &mut self,
        durationType: crate::System::Xml::Schema::XsdDuration_DurationType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (durationType),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToTimeSpan_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TimeSpan> {
        let __cordl_ret: crate::System::TimeSpan = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToTimeSpan",
            (),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn TryParseDigits(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        eatDigits: bool,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
        numDigits: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParseDigits", (s, offset, eatDigits, result, numDigits))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ByRefMut0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Xml::Schema::XsdDuration>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (s, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_XsdDuration_DurationType_ByRefMut1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        durationType: crate::System::Xml::Schema::XsdDuration_DurationType,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Xml::Schema::XsdDuration>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (s, durationType, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryToTimeSpan_ByRefMut0(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryToTimeSpan",
            (result),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryToTimeSpan_XsdDuration_DurationType_ByRefMut1(
        &mut self,
        durationType: crate::System::Xml::Schema::XsdDuration_DurationType,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryToTimeSpan",
            (durationType, result),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString3(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (s),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_XsdDuration_DurationType4(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        durationType: crate::System::Xml::Schema::XsdDuration_DurationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (s, durationType),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
    pub fn get_IsNegative(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNegative",
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
    pub fn get_Months(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Months",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Nanoseconds(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Nanoseconds",
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
    pub fn get_Years(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Years",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+DurationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XsdDuration_DurationType {
    #[default]
    DayTimeDuration = 2i32,
    Duration = 0i32,
    YearMonthDuration = 1i32,
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+DurationType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::XsdDuration_DurationType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "XsdDuration/DurationType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+DurationType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Xml::Schema::XsdDuration_DurationType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+DurationType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Xml::Schema::XsdDuration_DurationType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+DurationType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Xml::Schema::XsdDuration_DurationType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+DurationType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Xml::Schema::XsdDuration_DurationType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+Parts")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XsdDuration_Parts {
    #[default]
    HasDays = 4i32,
    HasHours = 8i32,
    HasMinutes = 16i32,
    HasMonths = 2i32,
    HasNone = 0i32,
    HasSeconds = 32i32,
    HasYears = 1i32,
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+Parts")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::XsdDuration_Parts {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "XsdDuration/Parts";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+Parts")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Xml::Schema::XsdDuration_Parts {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+Parts")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Xml::Schema::XsdDuration_Parts {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+Parts")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Xml::Schema::XsdDuration_Parts {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDuration+Parts")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Xml::Schema::XsdDuration_Parts {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
