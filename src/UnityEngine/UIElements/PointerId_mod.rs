#[cfg(feature = "UnityEngine+UIElements+PointerId")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+PointerId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PointerId =>
    "UnityEngine.UIElements"."PointerId"
);
#[cfg(feature = "UnityEngine+UIElements+PointerId")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PointerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerId")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PointerId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerId")]
impl crate::UnityEngine::UIElements::PointerId {}
#[cfg(feature = "UnityEngine+UIElements+PointerId")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::PointerId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
