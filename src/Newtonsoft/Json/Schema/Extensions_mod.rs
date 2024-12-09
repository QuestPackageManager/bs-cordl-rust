#[cfg(feature = "Newtonsoft+Json+Schema+Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Schema+Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Schema::Extensions =>
    "Newtonsoft.Json.Schema"."Extensions"
);
#[cfg(feature = "Newtonsoft+Json+Schema+Extensions")]
impl std::ops::Deref for crate::Newtonsoft::Json::Schema::Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+Extensions")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Schema::Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Schema+Extensions")]
impl crate::Newtonsoft::Json::Schema::Extensions {
    #[cfg(feature = "Newtonsoft+Json+Schema+Extensions+__c__DisplayClass0_0")]
    pub type __c__DisplayClass0_0 = crate::Newtonsoft::Json::Schema::Extensions___c__DisplayClass0_0;
    #[cfg(feature = "Newtonsoft+Json+Schema+Extensions+__c__DisplayClass1_0")]
    pub type __c__DisplayClass1_0 = crate::Newtonsoft::Json::Schema::Extensions___c__DisplayClass1_0;
}
#[cfg(feature = "Newtonsoft+Json+Schema+Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Schema::Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
