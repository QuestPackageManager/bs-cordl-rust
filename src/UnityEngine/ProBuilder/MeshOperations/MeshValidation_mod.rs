#[cfg(
    feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation+AttributeValidationStrategy"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshValidation_AttributeValidationStrategy {
    Nullify = 1i32,
    Resize = 0i32,
}
#[cfg(
    feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation+AttributeValidationStrategy"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::MeshValidation_AttributeValidationStrategy
    => "UnityEngine.ProBuilder.MeshOperations"
    ."MeshValidation/AttributeValidationStrategy"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshValidation {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::MeshValidation =>
    "UnityEngine.ProBuilder.MeshOperations"."MeshValidation"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation")]
impl crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation {
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation+AttributeValidationStrategy"
    )]
    pub type AttributeValidationStrategy = crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation_AttributeValidationStrategy;
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation+__c__DisplayClass10_0"
    )]
    pub type __c__DisplayClass10_0 = crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation___c__DisplayClass10_0;
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation+__c__DisplayClass5_0"
    )]
    pub type __c__DisplayClass5_0 = crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation___c__DisplayClass5_0;
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation___c;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
