#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteRendererGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteRendererGroup {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteRendererGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::U2D::SpriteRendererGroup =>
    "UnityEngine.Experimental.U2D"."SpriteRendererGroup"
);
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteRendererGroup")]
impl std::ops::Deref for crate::UnityEngine::Experimental::U2D::SpriteRendererGroup {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteRendererGroup")]
impl std::ops::DerefMut for crate::UnityEngine::Experimental::U2D::SpriteRendererGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteRendererGroup")]
impl crate::UnityEngine::Experimental::U2D::SpriteRendererGroup {}
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteRendererGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::U2D::SpriteRendererGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
