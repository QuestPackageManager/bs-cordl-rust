#[cfg(feature = "UnityEngine+UIElements+ReusableListViewItem")]
#[repr(C)]
#[derive(Debug)]
pub struct ReusableListViewItem {
    __cordl_parent: crate::UnityEngine::UIElements::ReusableCollectionItem,
    pub m_Container: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_DragHandle: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_ItemContainer: *mut crate::UnityEngine::UIElements::VisualElement,
}
#[cfg(feature = "UnityEngine+UIElements+ReusableListViewItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ReusableListViewItem =>
    "UnityEngine.UIElements"."ReusableListViewItem"
);
#[cfg(feature = "UnityEngine+UIElements+ReusableListViewItem")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ReusableListViewItem {
    type Target = crate::UnityEngine::UIElements::ReusableCollectionItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableListViewItem")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ReusableListViewItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableListViewItem")]
impl crate::UnityEngine::UIElements::ReusableListViewItem {
    pub fn DetachElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DetachElement", ())?;
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
    pub fn Init(
        &mut self,
        item: *mut crate::UnityEngine::UIElements::VisualElement,
        usesAnimatedDragger: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (item, usesAnimatedDragger))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateHierarchy(
        &mut self,
        root: *mut crate::UnityEngine::UIElements::VisualElement,
        item: *mut crate::UnityEngine::UIElements::VisualElement,
        usesAnimatedDragger: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateHierarchy", (root, item, usesAnimatedDragger))?;
        Ok(__cordl_ret)
    }
    pub fn SetDragGhost(
        &mut self,
        dragGhost: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDragGhost", (dragGhost))?;
        Ok(__cordl_ret)
    }
    pub fn get_rootElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_rootElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDragHandle(
        &mut self,
        needsDragHandle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDragHandle", (needsDragHandle))?;
        Ok(__cordl_ret)
    }
    pub fn PreAttachElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreAttachElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ReusableListViewItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ReusableListViewItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
