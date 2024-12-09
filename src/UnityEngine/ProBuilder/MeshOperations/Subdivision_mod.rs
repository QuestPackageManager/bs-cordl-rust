#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Subdivision")]
#[repr(C)]
#[derive(Debug)]
pub struct Subdivision {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Subdivision")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::Subdivision =>
    "UnityEngine.ProBuilder.MeshOperations"."Subdivision"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Subdivision")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::Subdivision {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Subdivision")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshOperations::Subdivision {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Subdivision")]
impl crate::UnityEngine::ProBuilder::MeshOperations::Subdivision {}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Subdivision")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::Subdivision {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
