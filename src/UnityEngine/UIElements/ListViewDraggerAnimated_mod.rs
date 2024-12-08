#[cfg(feature = "UnityEngine+UIElements+ListViewDraggerAnimated")]
#[repr(C)]
#[derive(Debug)]
pub struct ListViewDraggerAnimated {
    __cordl_parent: crate::UnityEngine::UIElements::ListViewDragger,
    pub m_DragStartIndex: i32,
    pub m_CurrentIndex: i32,
    pub m_SelectionHeight: f32,
    pub m_LocalOffsetOnStart: f32,
    pub m_CurrentPointerPosition: crate::UnityEngine::Vector3,
    pub m_Item: *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
    pub m_OffsetItem: *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
    pub _isDragging_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDraggerAnimated")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ListViewDraggerAnimated
    => "UnityEngine.UIElements"."ListViewDraggerAnimated"
);
#[cfg(feature = "UnityEngine+UIElements+ListViewDraggerAnimated")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ListViewDraggerAnimated {
    type Target = crate::UnityEngine::UIElements::ListViewDragger;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDraggerAnimated")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ListViewDraggerAnimated {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDraggerAnimated")]
impl crate::UnityEngine::UIElements::ListViewDraggerAnimated {
    pub fn StartDrag(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StartDragArgs> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StartDragArgs = __cordl_object
            .invoke("StartDrag", (pointerPosition))?;
        Ok(__cordl_ret)
    }
    pub fn get_isDragging(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDragging", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_isDragging(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isDragging", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Animate(
        &mut self,
        element: *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
        paddingTop: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Animate", (element, paddingTop))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        listView: *mut crate::UnityEngine::UIElements::BaseVerticalCollectionView,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (listView))?;
        Ok(__cordl_ret)
    }
    pub fn get_draggedItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::ReusableCollectionItem = __cordl_object
            .invoke("get_draggedItem", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDrop(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrop", (pointerPosition))?;
        Ok(__cordl_ret)
    }
    pub fn get_supportsDragEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_supportsDragEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDrag(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDrag", (pointerPosition))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetDragPosition(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector2,
        dragPosition: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ListViewDragger_DragPosition,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetDragPosition", (pointerPosition, dragPosition))?;
        Ok(__cordl_ret)
    }
    pub fn ClearDragAndDropUI(
        &mut self,
        dragCancelled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearDragAndDropUI", (dragCancelled))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        listView: *mut crate::UnityEngine::UIElements::BaseVerticalCollectionView,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listView))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ListViewDraggerAnimated")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ListViewDraggerAnimated {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
