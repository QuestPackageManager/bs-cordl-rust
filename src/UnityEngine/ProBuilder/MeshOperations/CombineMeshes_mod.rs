#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+CombineMeshes")]
#[repr(C)]
#[derive(Debug)]
pub struct CombineMeshes {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+CombineMeshes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::CombineMeshes =>
    "UnityEngine.ProBuilder.MeshOperations"."CombineMeshes"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+CombineMeshes")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::CombineMeshes {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+CombineMeshes")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::CombineMeshes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+CombineMeshes")]
impl crate::UnityEngine::ProBuilder::MeshOperations::CombineMeshes {
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+CombineMeshes+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::CombineMeshes___c;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+CombineMeshes")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::CombineMeshes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
