#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemSettings"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRuntimeReflectionSystemSettings {
    __cordl_parent: crate::System::Object,
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemSettings"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemSettings =>
    "UnityEngine.Experimental.Rendering"."ScriptableRuntimeReflectionSystemSettings"
);
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemSettings"
)]
impl std::ops::Deref
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemSettings"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemSettings"
)]
impl crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemSettings {}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemSettings"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
