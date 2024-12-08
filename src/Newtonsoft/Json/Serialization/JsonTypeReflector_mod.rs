#[cfg(feature = "Newtonsoft+Json+Serialization+JsonTypeReflector")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonTypeReflector {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonTypeReflector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonTypeReflector =>
    "Newtonsoft.Json.Serialization"."JsonTypeReflector"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonTypeReflector")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::JsonTypeReflector {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonTypeReflector")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Serialization::JsonTypeReflector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonTypeReflector")]
impl crate::Newtonsoft::Json::Serialization::JsonTypeReflector {
    pub const ArrayValuesPropertyName: &'static str = "$values";
    pub const ConcurrentDictionaryTypeName: &'static str = "System.Collections.Concurrent.ConcurrentDictionary`2";
    pub const IdPropertyName: &'static str = "$id";
    pub const RefPropertyName: &'static str = "$ref";
    pub const ShouldSerializePrefix: &'static str = "ShouldSerialize";
    pub const SpecifiedPostfix: &'static str = "Specified";
    pub const TypePropertyName: &'static str = "$type";
    pub const ValuePropertyName: &'static str = "$value";
    #[cfg(feature = "Newtonsoft+Json+Serialization+JsonTypeReflector+__c")]
    pub type __c = crate::Newtonsoft::Json::Serialization::JsonTypeReflector___c;
    #[cfg(
        feature = "Newtonsoft+Json+Serialization+JsonTypeReflector+__c__DisplayClass22_0"
    )]
    pub type __c__DisplayClass22_0 = crate::Newtonsoft::Json::Serialization::JsonTypeReflector___c__DisplayClass22_0;
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonTypeReflector")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonTypeReflector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
