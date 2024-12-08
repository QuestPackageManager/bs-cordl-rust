#[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionTypeAnalyzer {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::Internal::ReflectionTypeAnalyzer =>
    "Zenject.Internal"."ReflectionTypeAnalyzer"
);
#[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer")]
impl std::ops::Deref for crate::Zenject::Internal::ReflectionTypeAnalyzer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer")]
impl std::ops::DerefMut for crate::Zenject::Internal::ReflectionTypeAnalyzer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer")]
impl crate::Zenject::Internal::ReflectionTypeAnalyzer {
    #[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::Zenject::Internal::ReflectionTypeAnalyzer___c__DisplayClass6_0;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer+__c")]
    pub type __c = crate::Zenject::Internal::ReflectionTypeAnalyzer___c;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer+__c__DisplayClass7_0")]
    pub type __c__DisplayClass7_0 = crate::Zenject::Internal::ReflectionTypeAnalyzer___c__DisplayClass7_0;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer+__c__DisplayClass11_0")]
    pub type __c__DisplayClass11_0 = crate::Zenject::Internal::ReflectionTypeAnalyzer___c__DisplayClass11_0;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer+__c__DisplayClass5_1")]
    pub type __c__DisplayClass5_1 = crate::Zenject::Internal::ReflectionTypeAnalyzer___c__DisplayClass5_1;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer+__c__DisplayClass7_1")]
    pub type __c__DisplayClass7_1 = crate::Zenject::Internal::ReflectionTypeAnalyzer___c__DisplayClass7_1;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer+__c__DisplayClass8_0")]
    pub type __c__DisplayClass8_0 = crate::Zenject::Internal::ReflectionTypeAnalyzer___c__DisplayClass8_0;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer+__c__DisplayClass5_0")]
    pub type __c__DisplayClass5_0 = crate::Zenject::Internal::ReflectionTypeAnalyzer___c__DisplayClass5_0;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer+__c__DisplayClass6_1")]
    pub type __c__DisplayClass6_1 = crate::Zenject::Internal::ReflectionTypeAnalyzer___c__DisplayClass6_1;
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::Internal::ReflectionTypeAnalyzer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
