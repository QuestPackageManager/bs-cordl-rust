#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology")]
#[repr(C)]
#[derive(Debug)]
pub struct SurfaceTopology {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::SurfaceTopology =>
    "UnityEngine.ProBuilder.MeshOperations"."SurfaceTopology"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology")]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::MeshOperations::SurfaceTopology {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::SurfaceTopology {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology")]
impl crate::UnityEngine::ProBuilder::MeshOperations::SurfaceTopology {
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::SurfaceTopology___c;
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::SurfaceTopology {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
