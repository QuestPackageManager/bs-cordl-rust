#[cfg(feature = "UnityEngine+Rendering+GraphicsSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphicsSettings {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+Rendering+GraphicsSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::GraphicsSettings =>
    "UnityEngine.Rendering"."GraphicsSettings"
);
#[cfg(feature = "UnityEngine+Rendering+GraphicsSettings")]
impl std::ops::Deref for crate::UnityEngine::Rendering::GraphicsSettings {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GraphicsSettings")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::GraphicsSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GraphicsSettings")]
impl crate::UnityEngine::Rendering::GraphicsSettings {}
#[cfg(feature = "UnityEngine+Rendering+GraphicsSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::GraphicsSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
