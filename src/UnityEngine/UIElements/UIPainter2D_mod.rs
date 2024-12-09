#[cfg(feature = "UnityEngine+UIElements+UIPainter2D")]
#[repr(C)]
#[derive(Debug)]
pub struct UIPainter2D {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIPainter2D")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIPainter2D =>
    "UnityEngine.UIElements"."UIPainter2D"
);
#[cfg(feature = "UnityEngine+UIElements+UIPainter2D")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIPainter2D {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIPainter2D")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIPainter2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIPainter2D")]
impl crate::UnityEngine::UIElements::UIPainter2D {}
#[cfg(feature = "UnityEngine+UIElements+UIPainter2D")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::UIPainter2D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
