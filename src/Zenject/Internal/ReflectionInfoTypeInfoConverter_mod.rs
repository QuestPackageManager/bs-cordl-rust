#[cfg(feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionInfoTypeInfoConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::Internal::ReflectionInfoTypeInfoConverter => "Zenject.Internal"
    ."ReflectionInfoTypeInfoConverter"
);
#[cfg(feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter")]
impl std::ops::Deref for crate::Zenject::Internal::ReflectionInfoTypeInfoConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter")]
impl std::ops::DerefMut for crate::Zenject::Internal::ReflectionInfoTypeInfoConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter")]
impl crate::Zenject::Internal::ReflectionInfoTypeInfoConverter {
    #[cfg(feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter+__c")]
    pub type __c = crate::Zenject::Internal::ReflectionInfoTypeInfoConverter___c;
    #[cfg(
        feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter+__c__DisplayClass0_0"
    )]
    pub type __c__DisplayClass0_0 = crate::Zenject::Internal::ReflectionInfoTypeInfoConverter___c__DisplayClass0_0;
    #[cfg(
        feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter+__c__DisplayClass4_0"
    )]
    pub type __c__DisplayClass4_0 = crate::Zenject::Internal::ReflectionInfoTypeInfoConverter___c__DisplayClass4_0;
    #[cfg(
        feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter+__c__DisplayClass8_0"
    )]
    pub type __c__DisplayClass8_0 = crate::Zenject::Internal::ReflectionInfoTypeInfoConverter___c__DisplayClass8_0;
    #[cfg(
        feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter+__c__DisplayClass8_1"
    )]
    pub type __c__DisplayClass8_1 = crate::Zenject::Internal::ReflectionInfoTypeInfoConverter___c__DisplayClass8_1;
    #[cfg(
        feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter+__c__DisplayClass9_0"
    )]
    pub type __c__DisplayClass9_0 = crate::Zenject::Internal::ReflectionInfoTypeInfoConverter___c__DisplayClass9_0;
}
#[cfg(feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::Internal::ReflectionInfoTypeInfoConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
