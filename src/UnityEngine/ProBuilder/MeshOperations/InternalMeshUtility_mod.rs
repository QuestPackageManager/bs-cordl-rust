#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalMeshUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility =>
    "UnityEngine.ProBuilder.MeshOperations"."InternalMeshUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
impl crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility {
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility___c;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
