#[cfg(feature = "cordl_class_UnityEngine+U2D+SpriteShapeRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteShapeRenderer {
    __cordl_parent: crate::UnityEngine::Renderer,
}
#[cfg(feature = "cordl_class_UnityEngine+U2D+SpriteShapeRenderer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::U2D::SpriteShapeRenderer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.U2D";
    const CLASS_NAME: &'static str = "SpriteShapeRenderer";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "cordl_class_UnityEngine+U2D+SpriteShapeRenderer")]
impl std::ops::Deref for crate::UnityEngine::U2D::SpriteShapeRenderer {
    type Target = crate::UnityEngine::Renderer;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+U2D+SpriteShapeRenderer")]
impl std::ops::DerefMut for crate::UnityEngine::U2D::SpriteShapeRenderer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+U2D+SpriteShapeRenderer")]
impl crate::UnityEngine::U2D::SpriteShapeRenderer {}
#[cfg(feature = "cordl_class_UnityEngine+U2D+SpriteShapeRenderer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::U2D::SpriteShapeRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
