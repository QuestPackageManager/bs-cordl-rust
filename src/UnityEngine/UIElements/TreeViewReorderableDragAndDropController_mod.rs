#[cfg(feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController")]
#[repr(C)]
#[derive(Debug)]
pub struct TreeViewReorderableDragAndDropController {
    __cordl_parent: crate::UnityEngine::UIElements::BaseReorderableDragAndDropController,
    pub m_DropData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController_DropData,
    >,
    pub m_TreeView: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::BaseTreeView,
    >,
    pub m_ExpandDropItemScheduledItem: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    >,
    pub m_ExpandDropItemCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
}
#[cfg(feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "TreeViewReorderableDragAndDropController";
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
    pub fn CompareId(
        &mut self,
        id1: i32,
        id2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareId", (id1, id2))?;
        Ok(__cordl_ret.into())
    }
    pub fn DelayExpandDropItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DelayExpandDropItem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DragCleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DragCleanup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpandDropItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandDropItem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleAutoExpand(
        &mut self,
        item: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ReusableCollectionItem,
        >,
        pointerPosition: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAutoExpand", (item, pointerPosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleDragAndDrop(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IListDragAndDropArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::DragVisualMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::DragVisualMode = __cordl_object
            .invoke("HandleDragAndDrop", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        view: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseTreeView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (view))?;
        Ok(__cordl_object.into())
    }
    pub fn OnDrop(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IListDragAndDropArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrop", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn RestoreExpanded(
        &mut self,
        ids: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestoreExpanded", (ids))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupDragAndDrop(
        &mut self,
        itemIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
        skipText: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StartDragArgs> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StartDragArgs = __cordl_object
            .invoke("SetupDragAndDrop", (itemIds, skipText))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        view: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseTreeView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (view))?;
        Ok(__cordl_ret.into())
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
#[cfg(
    feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController+DropData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TreeViewReorderableDragAndDropController_DropData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub expandedIdsBeforeDrag: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub draggedIds: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub lastItemId: i32,
    pub expandItemBeginTimerMs: f32,
    pub expandItemBeginPosition: crate::UnityEngine::Vector2,
}
#[cfg(
    feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController+DropData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController_DropData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "TreeViewReorderableDragAndDropController/DropData";
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
#[cfg(
    feature = "UnityEngine+UIElements+TreeViewReorderableDragAndDropController+DropData"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::TreeViewReorderableDragAndDropController_DropData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
