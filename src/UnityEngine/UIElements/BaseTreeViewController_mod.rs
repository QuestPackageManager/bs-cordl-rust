#[cfg(feature = "UnityEngine+UIElements+BaseTreeViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseTreeViewController {
    __cordl_parent: crate::UnityEngine::UIElements::CollectionViewController,
    pub m_TreeItems: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        crate::UnityEngine::UIElements::TreeItem,
    >,
    pub m_RootIndices: *mut crate::System::Collections::Generic::List_1<i32>,
    pub m_ItemWrappers: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::TreeViewItemWrapper,
    >,
    pub m_TreeItemIdsWithItemWrappers: *mut crate::System::Collections::Generic::HashSet_1<
        i32,
    >,
    pub m_WrapperInsertionList: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::TreeViewItemWrapper,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+BaseTreeViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::BaseTreeViewController
    => "UnityEngine.UIElements"."BaseTreeViewController"
);
#[cfg(feature = "UnityEngine+UIElements+BaseTreeViewController")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BaseTreeViewController {
    type Target = crate::UnityEngine::UIElements::CollectionViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseTreeViewController")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BaseTreeViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseTreeViewController")]
impl crate::UnityEngine::UIElements::BaseTreeViewController {
    #[cfg(
        feature = "UnityEngine+UIElements+BaseTreeViewController+__c__DisplayClass20_0"
    )]
    pub type __c__DisplayClass20_0 = crate::UnityEngine::UIElements::BaseTreeViewController___c__DisplayClass20_0;
    pub fn CanChangeExpandedState(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanChangeExpandedState", (id))?;
        Ok(__cordl_ret)
    }
    pub fn CollapseItem(
        &mut self,
        id: i32,
        collapseAllChildren: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CollapseItem", (id, collapseAllChildren))?;
        Ok(__cordl_ret)
    }
    pub fn CollapseItemByIndex(
        &mut self,
        index: i32,
        collapseAllChildren: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CollapseItemByIndex", (index, collapseAllChildren))?;
        Ok(__cordl_ret)
    }
    pub fn CreateWrappers(
        &mut self,
        treeViewItemIds: *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
        depth: i32,
        wrappers: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::TreeViewItemWrapper,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateWrappers", (treeViewItemIds, depth, wrappers))?;
        Ok(__cordl_ret)
    }
    pub fn Exists(&mut self, id: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Exists", (id))?;
        Ok(__cordl_ret)
    }
    pub fn ExpandItem(
        &mut self,
        id: i32,
        expandAllChildren: bool,
        refresh: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandItem", (id, expandAllChildren, refresh))?;
        Ok(__cordl_ret)
    }
    pub fn ExpandItemByIndex(
        &mut self,
        index: i32,
        expandAllChildren: bool,
        refresh: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandItemByIndex", (index, expandAllChildren, refresh))?;
        Ok(__cordl_ret)
    }
    pub fn GetAllItemIds(
        &mut self,
        rootIds: *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<i32> = __cordl_object
            .invoke("GetAllItemIds", (rootIds))?;
        Ok(__cordl_ret)
    }
    pub fn GetChildIndexForId(&mut self, id: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetChildIndexForId", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetChildrenIds(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<i32> = __cordl_object
            .invoke("GetChildrenIds", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetChildrenIdsByIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<i32> = __cordl_object
            .invoke("GetChildrenIdsByIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetIdForIndex(&mut self, index: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetIdForIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetIndentationDepth(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetIndentationDepth", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetIndentationDepthByIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetIndentationDepthByIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetIndexForId(&mut self, id: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetIndexForId", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetParentId(&mut self, id: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetParentId", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetRootItemIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<i32> = __cordl_object
            .invoke("GetRootItemIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasChildren(&mut self, id: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasChildren", (id))?;
        Ok(__cordl_ret)
    }
    pub fn HasChildrenByIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasChildrenByIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeBindItem(
        &mut self,
        reusableItem: *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeBindItem", (reusableItem, index))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeDestroyItem(
        &mut self,
        reusableItem: *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeDestroyItem", (reusableItem))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeMakeItem(
        &mut self,
        reusableItem: *mut crate::UnityEngine::UIElements::ReusableCollectionItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeMakeItem", (reusableItem))?;
        Ok(__cordl_ret)
    }
    pub fn IsExpanded(&mut self, id: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsExpanded", (id))?;
        Ok(__cordl_ret)
    }
    pub fn IsExpandedByIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsExpandedByIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn IsIndexValid(&mut self, index: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsIndexValid", (index))?;
        Ok(__cordl_ret)
    }
    pub fn Move(
        &mut self,
        id: i32,
        newParentId: i32,
        childIndex: i32,
        rebuildTree: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Move", (id, newParentId, childIndex, rebuildTree))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnItemPointerUp(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::PointerUpEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnItemPointerUp", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnToggleValueChanged(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::ChangeEvent_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnToggleValueChanged", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn PostInitRegistration(
        &mut self,
        treeItem: *mut crate::UnityEngine::UIElements::ReusableTreeViewItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PostInitRegistration", (treeItem))?;
        Ok(__cordl_ret)
    }
    pub fn RaiseItemParentChanged(
        &mut self,
        id: i32,
        newParentId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseItemParentChanged", (id, newParentId))?;
        Ok(__cordl_ret)
    }
    pub fn RebuildTree(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RebuildTree", ())?;
        Ok(__cordl_ret)
    }
    pub fn RegenerateWrappers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegenerateWrappers", ())?;
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
    pub fn get_baseTreeView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::BaseTreeView,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::BaseTreeView = __cordl_object
            .invoke("get_baseTreeView", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_itemsSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("get_itemsSource", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_itemsSource(
        &mut self,
        value: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_itemsSource", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseTreeViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BaseTreeViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}