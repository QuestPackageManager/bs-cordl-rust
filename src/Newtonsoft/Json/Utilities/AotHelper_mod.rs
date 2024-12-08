#[cfg(feature = "Newtonsoft+Json+Utilities+AotHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AotHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+AotHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::AotHelper =>
    "Newtonsoft.Json.Utilities"."AotHelper"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+AotHelper")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::AotHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+AotHelper")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::AotHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+AotHelper")]
impl crate::Newtonsoft::Json::Utilities::AotHelper {
    #[cfg(feature = "Newtonsoft+Json+Utilities+AotHelper+__c__3_2")]
    pub type __c__3_2<
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > = crate::Newtonsoft::Json::Utilities::AotHelper___c__3_2<TKey, TValue>;
    #[cfg(feature = "Newtonsoft+Json+Utilities+AotHelper+__c__2_1")]
    pub type __c__2_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::AotHelper___c__2_1<
        T,
    >;
    #[cfg(feature = "Newtonsoft+Json+Utilities+AotHelper+__c__1_1")]
    pub type __c__1_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::AotHelper___c__1_1<
        T,
    >;
}
#[cfg(feature = "Newtonsoft+Json+Utilities+AotHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::AotHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
