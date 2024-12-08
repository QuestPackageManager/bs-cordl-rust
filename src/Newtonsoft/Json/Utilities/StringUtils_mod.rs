#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils+SeparatedCaseState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringUtils_SeparatedCaseState {
    Lower = 1i32,
    NewWord = 3i32,
    Start = 0i32,
    Upper = 2i32,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils+SeparatedCaseState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::StringUtils_SeparatedCaseState =>
    "Newtonsoft.Json.Utilities"."StringUtils/SeparatedCaseState"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct StringUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::StringUtils =>
    "Newtonsoft.Json.Utilities"."StringUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::StringUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::StringUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils")]
impl crate::Newtonsoft::Json::Utilities::StringUtils {
    pub const CarriageReturn: char = '\r';
    pub const CarriageReturnLineFeed: &'static str = "\\r\\n";
    pub const Empty: &'static str = "";
    pub const LineFeed: char = '\n';
    pub const Tab: char = '\t';
    #[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils+SeparatedCaseState")]
    pub type SeparatedCaseState = crate::Newtonsoft::Json::Utilities::StringUtils_SeparatedCaseState;
    #[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils+__c__DisplayClass14_0_1")]
    pub type __c__DisplayClass14_0_1<TSource: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::StringUtils___c__DisplayClass14_0_1<
        TSource,
    >;
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::StringUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
