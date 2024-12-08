#[cfg(feature = "UnityEngine+UIElements+DragAndDropUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct DragAndDropUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+DragAndDropUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DragAndDropUtility =>
    "UnityEngine.UIElements"."DragAndDropUtility"
);
#[cfg(feature = "UnityEngine+UIElements+DragAndDropUtility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DragAndDropUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DragAndDropUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DragAndDropUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DragAndDropUtility")]
impl crate::UnityEngine::UIElements::DragAndDropUtility {}
#[cfg(feature = "UnityEngine+UIElements+DragAndDropUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DragAndDropUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
