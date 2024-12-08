#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::ReflectionUtils =>
    "Newtonsoft.Json.Utilities"."ReflectionUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::ReflectionUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::ReflectionUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
impl crate::Newtonsoft::Json::Utilities::ReflectionUtils {
    #[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils+__c")]
    pub type __c = crate::Newtonsoft::Json::Utilities::ReflectionUtils___c;
    #[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils+__c__DisplayClass31_0")]
    pub type __c__DisplayClass31_0 = crate::Newtonsoft::Json::Utilities::ReflectionUtils___c__DisplayClass31_0;
    #[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils+__c__DisplayClass44_0")]
    pub type __c__DisplayClass44_0 = crate::Newtonsoft::Json::Utilities::ReflectionUtils___c__DisplayClass44_0;
    #[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils+__c__DisplayClass44_1")]
    pub type __c__DisplayClass44_1 = crate::Newtonsoft::Json::Utilities::ReflectionUtils___c__DisplayClass44_1;
    #[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils+__c__DisplayClass45_0")]
    pub type __c__DisplayClass45_0 = crate::Newtonsoft::Json::Utilities::ReflectionUtils___c__DisplayClass45_0;
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::ReflectionUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
