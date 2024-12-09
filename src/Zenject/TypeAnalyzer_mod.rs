#[cfg(feature = "Zenject+TypeAnalyzer")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeAnalyzer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+TypeAnalyzer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::TypeAnalyzer => "Zenject"
    ."TypeAnalyzer"
);
#[cfg(feature = "Zenject+TypeAnalyzer")]
impl std::ops::Deref for crate::Zenject::TypeAnalyzer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+TypeAnalyzer")]
impl std::ops::DerefMut for crate::Zenject::TypeAnalyzer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+TypeAnalyzer")]
impl crate::Zenject::TypeAnalyzer {
    pub const ReflectionBakingFactoryMethodName: &'static str = "__zenCreate";
    pub const ReflectionBakingFieldSetterPrefix: &'static str = "__zenFieldSetter";
    pub const ReflectionBakingGetInjectInfoMethodName: &'static str = "__zenCreateInjectTypeInfo";
    pub const ReflectionBakingInjectMethodPrefix: &'static str = "__zenInjectMethod";
    pub const ReflectionBakingPropertySetterPrefix: &'static str = "__zenPropertySetter";
    #[cfg(feature = "Zenject+TypeAnalyzer+__c__DisplayClass23_0")]
    pub type __c__DisplayClass23_0 = crate::Zenject::TypeAnalyzer___c__DisplayClass23_0;
}
#[cfg(feature = "Zenject+TypeAnalyzer")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::TypeAnalyzer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
