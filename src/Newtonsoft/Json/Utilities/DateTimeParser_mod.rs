#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeParser")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DateTimeParser {
    pub Year: i32,
    pub Month: i32,
    pub Day: i32,
    pub Hour: i32,
    pub Minute: i32,
    pub Second: i32,
    pub Fraction: i32,
    pub ZoneHour: i32,
    pub ZoneMinute: i32,
    pub Zone: crate::Newtonsoft::Json::Utilities::ParserTimeZone,
    pub _text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    pub _end: i32,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeParser")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::DateTimeParser {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "DateTimeParser";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::Newtonsoft::Json::Utilities::DateTimeParser {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Newtonsoft::Json::Utilities::DateTimeParser {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::Newtonsoft::Json::Utilities::DateTimeParser {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::Newtonsoft::Json::Utilities::DateTimeParser {
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
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeParser")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Newtonsoft::Json::Utilities::DateTimeParser {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeParser")]
impl crate::Newtonsoft::Json::Utilities::DateTimeParser {
    pub const MaxFractionDigits: i16 = 7i16;
    pub fn Parse(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Parse",
            (text, startIndex, length),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse2Digit(
        &mut self,
        start: i32,
        num: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Parse2Digit",
            (start, num),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse4Digit(
        &mut self,
        start: i32,
        num: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Parse4Digit",
            (start, num),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseChar(
        &mut self,
        start: i32,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseChar",
            (start, ch),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseDate(&mut self, start: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseDate",
            (start),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTime(
        &mut self,
        start: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseTime",
            (start),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTimeAndZoneAndWhitespace(
        &mut self,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseTimeAndZoneAndWhitespace",
            (start),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseZone(&mut self, start: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseZone",
            (start),
        )?;
        Ok(__cordl_ret.into())
    }
}
