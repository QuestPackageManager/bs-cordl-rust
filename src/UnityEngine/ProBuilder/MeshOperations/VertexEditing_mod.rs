#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
#[repr(C)]
#[derive(Debug)]
pub struct VertexEditing {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::VertexEditing =>
    "UnityEngine.ProBuilder.MeshOperations"."VertexEditing"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::VertexEditing {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::VertexEditing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
impl crate::UnityEngine::ProBuilder::MeshOperations::VertexEditing {}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::VertexEditing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
