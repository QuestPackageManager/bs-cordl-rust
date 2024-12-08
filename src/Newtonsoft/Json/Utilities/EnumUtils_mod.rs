#[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::EnumUtils =>
    "Newtonsoft.Json.Utilities"."EnumUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::EnumUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::EnumUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils")]
impl crate::Newtonsoft::Json::Utilities::EnumUtils {
    pub const EnumSeparatorChar: char = ',';
    pub const EnumSeparatorString: &'static str = ", ";
    #[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils+__c")]
    pub type __c = crate::Newtonsoft::Json::Utilities::EnumUtils___c;
    #[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils+__c__4_1")]
    pub type __c__4_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::EnumUtils___c__4_1<
        T,
    >;
}
#[cfg(feature = "Newtonsoft+Json+Utilities+EnumUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::EnumUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
