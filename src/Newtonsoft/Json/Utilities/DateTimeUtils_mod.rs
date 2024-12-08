#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct DateTimeUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::DateTimeUtils =>
    "Newtonsoft.Json.Utilities"."DateTimeUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
impl crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    pub const DaysPer100Years: i32 = 36524i32;
    pub const DaysPer400Years: i32 = 146097i32;
    pub const DaysPer4Years: i32 = 1461i32;
    pub const DaysPerYear: i32 = 365i32;
    pub const IsoDateFormat: &'static str = "yyyy-MM-ddTHH:mm:ss.FFFFFFFK";
    pub const TicksPerDay: i64 = 864000000000i64;
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DateTimeUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::DateTimeUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
