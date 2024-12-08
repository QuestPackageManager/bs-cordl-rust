#[cfg(feature = "UnityEngine+UIElements+MultiColumnController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnController {
    __cordl_parent: crate::System::Object,
    pub columnSortingChanged: *mut crate::System::Action,
    pub headerContextMenuPopulateEvent: *mut crate::System::Action_2<
        *mut crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub m_View: *mut crate::UnityEngine::UIElements::BaseVerticalCollectionView,
    pub m_HeaderContainer: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_MultiColumnHeader: *mut crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader,
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MultiColumnController
    => "UnityEngine.UIElements"."MultiColumnController"
);
#[cfg(feature = "UnityEngine+UIElements+MultiColumnController")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MultiColumnController {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnController")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MultiColumnController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnController")]
impl crate::UnityEngine::UIElements::MultiColumnController {
    pub fn add_columnSortingChanged(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_columnSortingChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnSortingChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnSortingChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnHorizontalScrollerValueChanged(
        &mut self,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnHorizontalScrollerValueChanged", (v))?;
        Ok(__cordl_ret)
    }
    pub fn OnContextMenuPopulateEvent(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnContextMenuPopulateEvent", (evt, column))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateContentContainer(
        &mut self,
        collectionView: *mut crate::UnityEngine::UIElements::BaseVerticalCollectionView,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateContentContainer", (collectionView))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnContainerGeometryChanged(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnContainerGeometryChanged", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnRemoved(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnRemoved", (column))?;
        Ok(__cordl_ret)
    }
    pub fn remove_headerContextMenuPopulateEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
            *mut crate::UnityEngine::UIElements::Column,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_headerContextMenuPopulateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        columns: *mut crate::UnityEngine::UIElements::Columns,
        sortDescriptions: *mut crate::UnityEngine::UIElements::SortColumnDescriptions,
        sortedColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::SortColumnDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (columns, sortDescriptions, sortedColumns))?;
        Ok(__cordl_ret)
    }
    pub fn OnViewDataRestored(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnViewDataRestored", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnsChanged(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        _cordl_type: crate::UnityEngine::UIElements::ColumnDataType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnsChanged", (column, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn get_header(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader = __cordl_object
            .invoke("get_header", ())?;
        Ok(__cordl_ret)
    }
    pub fn DestroyItem(
        &mut self,
        element: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyItem", (element))?;
        Ok(__cordl_ret)
    }
    pub fn OnViewportGeometryChanged(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnViewportGeometryChanged", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnReordered(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        from: i32,
        to: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnReordered", (column, from, to))?;
        Ok(__cordl_ret)
    }
    pub fn PrepareView(
        &mut self,
        collectionView: *mut crate::UnityEngine::UIElements::BaseVerticalCollectionView,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareView", (collectionView))?;
        Ok(__cordl_ret)
    }
    pub fn add_headerContextMenuPopulateEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
            *mut crate::UnityEngine::UIElements::Column,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_headerContextMenuPopulateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnAdded(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnAdded", (column, index))?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnChanged(
        &mut self,
        _cordl_type: crate::UnityEngine::UIElements::ColumnsDataType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnChanged", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn remove_columnSortingChanged(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_columnSortingChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn MakeItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("MakeItem", ())?;
        Ok(__cordl_ret)
    }
    pub fn BindItem<T>(
        &mut self,
        element: *mut crate::UnityEngine::UIElements::VisualElement,
        index: i32,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindItem", (element, index, item))?;
        Ok(__cordl_ret)
    }
    pub fn UnbindItem(
        &mut self,
        element: *mut crate::UnityEngine::UIElements::VisualElement,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnbindItem", (element, index))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnResized(
        &mut self,
        index: i32,
        width: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnResized", (index, width))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        columns: *mut crate::UnityEngine::UIElements::Columns,
        sortDescriptions: *mut crate::UnityEngine::UIElements::SortColumnDescriptions,
        sortedColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::SortColumnDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (columns, sortDescriptions, sortedColumns))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MultiColumnController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
