#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation")]
#[repr(C)]
#[derive(Debug)]
pub struct Triangulation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::Triangulation =>
    "UnityEngine.ProBuilder.MeshOperations"."Triangulation"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::Triangulation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::Triangulation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation")]
impl crate::UnityEngine::ProBuilder::MeshOperations::Triangulation {
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation+__c__DisplayClass7_0"
    )]
    pub type __c__DisplayClass7_0 = crate::UnityEngine::ProBuilder::MeshOperations::Triangulation___c__DisplayClass7_0;
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation+__c__DisplayClass8_0"
    )]
    pub type __c__DisplayClass8_0 = crate::UnityEngine::ProBuilder::MeshOperations::Triangulation___c__DisplayClass8_0;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::Triangulation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
