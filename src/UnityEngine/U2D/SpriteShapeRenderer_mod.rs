#[cfg(feature = "UnityEngine+U2D+SpriteShapeRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteShapeRenderer {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
}
#[cfg(feature = "UnityEngine+U2D+SpriteShapeRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::U2D::SpriteShapeRenderer =>
    "UnityEngine.U2D"."SpriteShapeRenderer"
);
#[cfg(feature = "UnityEngine+U2D+SpriteShapeRenderer")]
impl std::ops::Deref for crate::UnityEngine::U2D::SpriteShapeRenderer {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+U2D+SpriteShapeRenderer")]
impl std::ops::DerefMut for crate::UnityEngine::U2D::SpriteShapeRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+U2D+SpriteShapeRenderer")]
impl crate::UnityEngine::U2D::SpriteShapeRenderer {}
#[cfg(feature = "UnityEngine+U2D+SpriteShapeRenderer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::U2D::SpriteShapeRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
