#[cfg(feature = "UnityEngine+UIElements+BaseListView")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseListView {
    __cordl_parent: crate::UnityEngine::UIElements::BaseVerticalCollectionView,
    pub m_ShowBoundCollectionSize: bool,
    pub m_ShowFoldoutHeader: bool,
    pub m_HeaderTitle: *mut crate::System::String,
    pub itemsAdded: *mut crate::System::Action_1<
        *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    >,
    pub itemsRemoved: *mut crate::System::Action_1<
        *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    >,
    pub itemsSourceSizeChanged: *mut crate::System::Action,
    pub m_ListViewLabel: *mut crate::UnityEngine::UIElements::Label,
    pub m_Foldout: *mut crate::UnityEngine::UIElements::Foldout,
    pub m_ArraySizeField: *mut crate::UnityEngine::UIElements::TextField,
    pub m_IsOverMultiEditLimit: bool,
    pub m_Footer: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_AddButton: *mut crate::UnityEngine::UIElements::Button,
    pub m_RemoveButton: *mut crate::UnityEngine::UIElements::Button,
    pub m_ItemAddedCallback: *mut crate::System::Action_1<
        *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    >,
    pub m_ItemRemovedCallback: *mut crate::System::Action_1<
        *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    >,
    pub m_ItemsSourceSizeChangedCallback: *mut crate::System::Action,
    pub m_ReorderMode: crate::UnityEngine::UIElements::ListViewReorderMode,
    pub reorderModeChanged: *mut crate::System::Action,
    pub m_MaxMultiEditStr: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+UIElements+BaseListView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::BaseListView =>
    "UnityEngine.UIElements"."BaseListView"
);
#[cfg(feature = "UnityEngine+UIElements+BaseListView")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BaseListView {
    type Target = crate::UnityEngine::UIElements::BaseVerticalCollectionView;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseListView")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BaseListView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseListView")]
impl crate::UnityEngine::UIElements::BaseListView {
    #[cfg(feature = "UnityEngine+UIElements+BaseListView+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::BaseListView_UxmlTraits;
    pub fn AddItems(
        &mut self,
        itemCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddItems", (itemCount))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDragAndDropController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::ICollectionDragAndDropController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::ICollectionDragAndDropController = __cordl_object
            .invoke("CreateDragAndDropController", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateDragger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::ListViewDragger,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::ListViewDragger = __cordl_object
            .invoke("CreateDragger", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateVirtualizationController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateVirtualizationController", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnableFooter(
        &mut self,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableFooter", (enabled))?;
        Ok(__cordl_ret)
    }
    pub fn HandleItemNavigation(
        &mut self,
        moveIn: bool,
        altPressed: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HandleItemNavigation", (moveIn, altPressed))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnAddClicked(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAddClicked", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnArraySizeFieldChanged(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::ChangeEvent_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnArraySizeFieldChanged", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnItemAdded(
        &mut self,
        indices: *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnItemAdded", (indices))?;
        Ok(__cordl_ret)
    }
    pub fn OnItemsRemoved(
        &mut self,
        indices: *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnItemsRemoved", (indices))?;
        Ok(__cordl_ret)
    }
    pub fn OnItemsSourceSizeChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnItemsSourceSizeChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnRemoveClicked(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRemoveClicked", ())?;
        Ok(__cordl_ret)
    }
    pub fn PostRefresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PostRefresh", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetViewController(
        &mut self,
        controller: *mut crate::UnityEngine::UIElements::CollectionViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetViewController", (controller))?;
        Ok(__cordl_ret)
    }
    pub fn SetupArraySizeField(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupArraySizeField", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateArraySizeField(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateArraySizeField", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateListViewLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateListViewLabel", ())?;
        Ok(__cordl_ret)
    }
    pub fn _OnAddClicked_b__36_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnAddClicked>b__36_0", ())?;
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
    pub fn add_reorderModeChanged(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_reorderModeChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_footer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_footer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_reorderMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::ListViewReorderMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::ListViewReorderMode = __cordl_object
            .invoke("get_reorderMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showAddRemoveFooter(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showAddRemoveFooter", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showBoundCollectionSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_showBoundCollectionSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showFoldoutHeader(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showFoldoutHeader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_viewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::BaseListViewController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::BaseListViewController = __cordl_object
            .invoke("get_viewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_reorderModeChanged(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_reorderModeChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_headerTitle(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_headerTitle", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_reorderMode(
        &mut self,
        value: crate::UnityEngine::UIElements::ListViewReorderMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_reorderMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_showAddRemoveFooter(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showAddRemoveFooter", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_showBoundCollectionSize(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showBoundCollectionSize", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_showFoldoutHeader(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showFoldoutHeader", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseListView")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::BaseListView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseListView+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseListView_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseVerticalCollectionView_UxmlTraits,
    pub m_ShowFoldoutHeader: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_HeaderTitle: *mut crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    pub m_ShowAddRemoveFooter: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_ReorderMode: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::ListViewReorderMode,
    >,
    pub m_ShowBoundCollectionSize: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
}
#[cfg(feature = "UnityEngine+UIElements+BaseListView+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::BaseListView_UxmlTraits
    => "UnityEngine.UIElements"."BaseListView/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+BaseListView+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BaseListView_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseVerticalCollectionView_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseListView+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BaseListView_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseListView+UxmlTraits")]
impl crate::UnityEngine::UIElements::BaseListView_UxmlTraits {
    pub fn Init(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
        bag: *mut crate::UnityEngine::UIElements::IUxmlAttributes,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (ve, bag, cc))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "UnityEngine+UIElements+BaseListView+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BaseListView_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}