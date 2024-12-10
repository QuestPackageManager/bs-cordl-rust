#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeParser")]
#[repr(C)]
#[derive(Debug, Clone)]
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
    pub _text: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub _end: i32,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeParser")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::DateTimeParser =>
    "Newtonsoft.Json.Utilities"."DateTimeParser"
);
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
