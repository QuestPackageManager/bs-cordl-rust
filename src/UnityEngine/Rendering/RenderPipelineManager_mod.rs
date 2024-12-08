#[cfg(feature = "UnityEngine+Rendering+RenderPipelineManager")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderPipelineManager {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::RenderPipelineManager =>
    "UnityEngine.Rendering"."RenderPipelineManager"
);
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineManager")]
impl std::ops::Deref for crate::UnityEngine::Rendering::RenderPipelineManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineManager")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::RenderPipelineManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineManager")]
impl crate::UnityEngine::Rendering::RenderPipelineManager {}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RenderPipelineManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
