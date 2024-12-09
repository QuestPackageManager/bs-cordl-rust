#[cfg(feature = "System+Runtime+Serialization+FormatterServices")]
#[repr(C)]
#[derive(Debug)]
pub struct FormatterServices {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Serialization+FormatterServices")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::FormatterServices => "System.Runtime.Serialization"
    ."FormatterServices"
);
#[cfg(feature = "System+Runtime+Serialization+FormatterServices")]
impl std::ops::Deref for crate::System::Runtime::Serialization::FormatterServices {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+FormatterServices")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::FormatterServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+FormatterServices")]
impl crate::System::Runtime::Serialization::FormatterServices {
    #[cfg(
        feature = "System+Runtime+Serialization+FormatterServices+__c__DisplayClass9_0"
    )]
    pub type __c__DisplayClass9_0 = crate::System::Runtime::Serialization::FormatterServices___c__DisplayClass9_0;
}
#[cfg(feature = "System+Runtime+Serialization+FormatterServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::FormatterServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
