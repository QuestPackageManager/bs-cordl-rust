#[cfg(
    feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController+DropData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TreeViewReorderableDragAndDropController_DropData {
    __cordl_parent: crate::System::Object,
    pub expandedIdsBeforeDrag: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub draggedIds: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub lastItemId: i32,
    pub expandItemBeginTimerMs: f32,
    pub expandItemBeginPosition: crate::UnityEngine::Vector2,
}
#[cfg(
    feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController+DropData"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::TreeViewReorderableDragAndDropController_DropData =>
    "UnityEngine.UIElements"."TreeViewReorderableDragAndDropController/DropData"
);
#[cfg(
    feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController+DropData"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController_DropData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController+DropData"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController_DropData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController+DropData"
)]
impl crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController_DropData {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController+DropData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController_DropData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController")]
#[repr(C)]
#[derive(Debug)]
pub struct TreeViewReorderableDragAndDropController {
    __cordl_parent: crate::UnityEngine::UIElements::BaseReorderableDragAndDropController,
    pub m_DropData: *mut crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController_DropData,
    pub m_TreeView: *mut crate::UnityEngine::UIElements::BaseTreeView,
    pub m_ExpandDropItemScheduledItem: *mut crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    pub m_ExpandDropItemCallback: *mut crate::System::Action,
}
#[cfg(feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::TreeViewReorderableDragAndDropController =>
    "UnityEngine.UIElements"."TreeViewReorderableDragAndDropController"
);
#[cfg(feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController {
    type Target = crate::UnityEngine::UIElements::BaseReorderableDragAndDropController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController")]
impl crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController {
    #[cfg(
        feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController+DropData"
    )]
    pub type DropData = crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController_DropData;
    pub fn _ctor(
        &mut self,
        view: *mut crate::UnityEngine::UIElements::BaseTreeView,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (view))?;
        Ok(__cordl_ret)
    }
    pub fn DelayExpandDropItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DelayExpandDropItem", ())?;
        Ok(__cordl_ret)
    }
    pub fn DragCleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DragCleanup", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompareId(
        &mut self,
        id1: i32,
        id2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareId", (id1, id2))?;
        Ok(__cordl_ret)
    }
    pub fn HandleDragAndDrop(
        &mut self,
        args: *mut crate::UnityEngine::UIElements::IListDragAndDropArgs,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::DragVisualMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::DragVisualMode = __cordl_object
            .invoke("HandleDragAndDrop", (args))?;
        Ok(__cordl_ret)
    }
    pub fn SetupDragAndDrop(
        &mut self,
        itemIds: *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
        skipText: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StartDragArgs> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StartDragArgs = __cordl_object
            .invoke("SetupDragAndDrop", (itemIds, skipText))?;
        Ok(__cordl_ret)
    }
    pub fn OnDrop(
        &mut self,
        args: *mut crate::UnityEngine::UIElements::IListDragAndDropArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrop", (args))?;
        Ok(__cordl_ret)
    }
    pub fn RestoreExpanded(
        &mut self,
        ids: *mut crate::System::Collections::Generic::List_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestoreExpanded", (ids))?;
        Ok(__cordl_ret)
    }
    pub fn HandleAutoExpand(
        &mut self,
        item: *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
        pointerPosition: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAutoExpand", (item, pointerPosition))?;
        Ok(__cordl_ret)
    }
    pub fn ExpandDropItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandDropItem", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        view: *mut crate::UnityEngine::UIElements::BaseTreeView,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (view))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
