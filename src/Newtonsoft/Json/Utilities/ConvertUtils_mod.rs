#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils+ConvertResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvertUtils_ConvertResult {
    CannotConvertNull = 1i32,
    NoValidConversion = 3i32,
    NotInstantiableType = 2i32,
    Success = 0i32,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils+ConvertResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult =>
    "Newtonsoft.Json.Utilities"."ConvertUtils/ConvertResult"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ConvertUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::ConvertUtils =>
    "Newtonsoft.Json.Utilities"."ConvertUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::ConvertUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::ConvertUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils")]
impl crate::Newtonsoft::Json::Utilities::ConvertUtils {
    #[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils+__c__DisplayClass8_0")]
    pub type __c__DisplayClass8_0 = crate::Newtonsoft::Json::Utilities::ConvertUtils___c__DisplayClass8_0;
    #[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils+ConvertResult")]
    pub type ConvertResult = crate::Newtonsoft::Json::Utilities::ConvertUtils_ConvertResult;
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ConvertUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::ConvertUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
