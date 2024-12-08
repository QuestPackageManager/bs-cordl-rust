#[cfg(feature = "UnityEngine+UIElements+BaseFieldMouseDragger")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseFieldMouseDragger {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+BaseFieldMouseDragger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::BaseFieldMouseDragger
    => "UnityEngine.UIElements"."BaseFieldMouseDragger"
);
#[cfg(feature = "UnityEngine+UIElements+BaseFieldMouseDragger")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BaseFieldMouseDragger {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseFieldMouseDragger")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BaseFieldMouseDragger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseFieldMouseDragger")]
impl crate::UnityEngine::UIElements::BaseFieldMouseDragger {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetDragZone_Rect1(
        &mut self,
        dragElement: *mut crate::UnityEngine::UIElements::VisualElement,
        hotZone: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDragZone", (dragElement, hotZone))?;
        Ok(__cordl_ret)
    }
    pub fn SetDragZone_VisualElement0(
        &mut self,
        dragElement: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDragZone", (dragElement))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseFieldMouseDragger")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BaseFieldMouseDragger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
