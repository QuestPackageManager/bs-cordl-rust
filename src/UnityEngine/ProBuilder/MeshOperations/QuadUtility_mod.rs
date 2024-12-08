#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct QuadUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::QuadUtility =>
    "UnityEngine.ProBuilder.MeshOperations"."QuadUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::QuadUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshOperations::QuadUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
impl crate::UnityEngine::ProBuilder::MeshOperations::QuadUtility {}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::QuadUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
