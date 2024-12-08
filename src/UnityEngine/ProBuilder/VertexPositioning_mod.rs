#[cfg(feature = "UnityEngine+ProBuilder+VertexPositioning")]
#[repr(C)]
#[derive(Debug)]
pub struct VertexPositioning {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+VertexPositioning")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::VertexPositioning =>
    "UnityEngine.ProBuilder"."VertexPositioning"
);
#[cfg(feature = "UnityEngine+ProBuilder+VertexPositioning")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::VertexPositioning {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+VertexPositioning")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::VertexPositioning {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+VertexPositioning")]
impl crate::UnityEngine::ProBuilder::VertexPositioning {}
#[cfg(feature = "UnityEngine+ProBuilder+VertexPositioning")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::VertexPositioning {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
