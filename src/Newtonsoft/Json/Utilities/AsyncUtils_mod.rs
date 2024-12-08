#[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::AsyncUtils =>
    "Newtonsoft.Json.Utilities"."AsyncUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::AsyncUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::AsyncUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils")]
impl crate::Newtonsoft::Json::Utilities::AsyncUtils {
    #[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils+__c")]
    pub type __c = crate::Newtonsoft::Json::Utilities::AsyncUtils___c;
    #[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils+__c__6_1")]
    pub type __c__6_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::AsyncUtils___c__6_1<
        T,
    >;
}
#[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::AsyncUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
