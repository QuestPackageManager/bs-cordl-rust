#[cfg(feature = "UnityEngine+Animations+DiscreteEvaluationAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DiscreteEvaluationAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "UnityEngine+Animations+DiscreteEvaluationAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Animations::DiscreteEvaluationAttribute => "UnityEngine.Animations"
    ."DiscreteEvaluationAttribute"
);
#[cfg(feature = "UnityEngine+Animations+DiscreteEvaluationAttribute")]
impl std::ops::Deref for crate::UnityEngine::Animations::DiscreteEvaluationAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+DiscreteEvaluationAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Animations::DiscreteEvaluationAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+DiscreteEvaluationAttribute")]
impl crate::UnityEngine::Animations::DiscreteEvaluationAttribute {}
#[cfg(feature = "UnityEngine+Animations+DiscreteEvaluationAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Animations::DiscreteEvaluationAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
