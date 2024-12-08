#[cfg(feature = "UnityEngine+UIElements+Columns")]
#[repr(C)]
#[derive(Debug)]
pub struct Columns {
    __cordl_parent: crate::System::Object,
    pub m_Columns: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub m_DisplayColumns: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub m_VisibleColumns: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub m_VisibleColumnsDirty: bool,
    pub m_StretchMode: crate::UnityEngine::UIElements::Columns_StretchMode,
    pub m_Reorderable: bool,
    pub m_Resizable: bool,
    pub m_ResizePreview: bool,
    pub m_PrimaryColumnName: *mut crate::System::String,
    pub changed: *mut crate::System::Action_1<
        crate::UnityEngine::UIElements::ColumnsDataType,
    >,
    pub columnAdded: *mut crate::System::Action_2<
        *mut crate::UnityEngine::UIElements::Column,
        i32,
    >,
    pub columnRemoved: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub columnChanged: *mut crate::System::Action_2<
        *mut crate::UnityEngine::UIElements::Column,
        crate::UnityEngine::UIElements::ColumnDataType,
    >,
    pub columnResized: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
    pub columnReordered: *mut crate::System::Action_3<
        *mut crate::UnityEngine::UIElements::Column,
        i32,
        i32,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Columns")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Columns =>
    "UnityEngine.UIElements"."Columns"
);
#[cfg(feature = "UnityEngine+UIElements+Columns")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Columns {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Columns")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Columns {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Columns")]
impl crate::UnityEngine::UIElements::Columns {
    #[cfg(feature = "UnityEngine+UIElements+Columns+__c")]
    pub type __c = crate::UnityEngine::UIElements::Columns___c;
    #[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectFactory_1")]
    pub type UxmlObjectFactory_1<T: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::Columns_UxmlObjectFactory_1<
        T,
    >;
    #[cfg(feature = "UnityEngine+UIElements+Columns+StretchMode")]
    pub type StretchMode = crate::UnityEngine::UIElements::Columns_StretchMode;
    #[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectTraits_1")]
    pub type UxmlObjectTraits_1<T: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::Columns_UxmlObjectTraits_1<
        T,
    >;
    pub fn add_columnRemoved(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::UnityEngine::UIElements::Column>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_columnRemoved", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_resizePreview(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_resizePreview", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadOnly", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateVisibleColumns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVisibleColumns", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitOrderColumns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitOrderColumns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_visibleList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::UIElements::Column,
        > = __cordl_object.invoke("get_visibleList", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_columnReordered(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::UnityEngine::UIElements::Column,
            i32,
            i32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_columnReordered", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOf", (column))?;
        Ok(__cordl_ret)
    }
    pub fn ReorderDisplay(
        &mut self,
        from: i32,
        to: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReorderDisplay", (from, to))?;
        Ok(__cordl_ret)
    }
    pub fn NotifyChange(
        &mut self,
        _cordl_type: crate::UnityEngine::UIElements::ColumnsDataType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyChange", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn set_reorderable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_reorderable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::UIElements::Column,
        >,
        arrayIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, arrayIndex))?;
        Ok(__cordl_ret)
    }
    pub fn remove_columnAdded(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::UnityEngine::UIElements::Column,
            i32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_columnAdded", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_columnChanged(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::UnityEngine::UIElements::Column,
            crate::UnityEngine::UIElements::ColumnDataType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_columnChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_displayList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::UIElements::Column,
        > = __cordl_object.invoke("get_displayList", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_columnResized(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::UnityEngine::UIElements::Column>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_columnResized", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Remove(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Remove", (column))?;
        Ok(__cordl_ret)
    }
    pub fn set_primaryColumnName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_primaryColumnName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_columnAdded(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::UnityEngine::UIElements::Column,
            i32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_columnAdded", (value))?;
        Ok(__cordl_ret)
    }
    pub fn DirtyVisibleColumns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DirtyVisibleColumns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_resizePreview(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_resizePreview", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnResized(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnResized", (column))?;
        Ok(__cordl_ret)
    }
    pub fn get_resizable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_resizable", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::UnityEngine::UIElements::Column,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::UnityEngine::UIElements::Column,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_changed(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::UnityEngine::UIElements::ColumnsDataType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_changed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_columnResized(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::UnityEngine::UIElements::Column>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_columnResized", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_columnRemoved(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::UnityEngine::UIElements::Column>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_columnRemoved", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_changed(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::UnityEngine::UIElements::ColumnsDataType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_changed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_reorderable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_reorderable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_stretchMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Columns_StretchMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Columns_StretchMode = __cordl_object
            .invoke("get_stretchMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_stretchMode(
        &mut self,
        value: crate::UnityEngine::UIElements::Columns_StretchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_stretchMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_columnChanged(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::UnityEngine::UIElements::Column,
            crate::UnityEngine::UIElements::ColumnDataType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_columnChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn IsPrimary(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPrimary", (column))?;
        Ok(__cordl_ret)
    }
    pub fn Add(
        &mut self,
        item: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (item))?;
        Ok(__cordl_ret)
    }
    pub fn Contains_Column0(
        &mut self,
        item: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (item))?;
        Ok(__cordl_ret)
    }
    pub fn Contains_String1(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (name))?;
        Ok(__cordl_ret)
    }
    pub fn Insert(
        &mut self,
        index: i32,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Insert", (index, column))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_i32_0(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::Column> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::Column = __cordl_object
            .invoke("get_Item", (index))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_String1(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::Column> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::Column = __cordl_object
            .invoke("get_Item", (name))?;
        Ok(__cordl_ret)
    }
    pub fn set_resizable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_resizable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnColumnChanged(
        &mut self,
        column: *mut crate::UnityEngine::UIElements::Column,
        _cordl_type: crate::UnityEngine::UIElements::ColumnDataType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnColumnChanged", (column, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn get_primaryColumnName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_primaryColumnName", ())?;
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
    pub fn remove_columnReordered(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::UnityEngine::UIElements::Column,
            i32,
            i32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_columnReordered", (value))?;
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
#[cfg(feature = "UnityEngine+UIElements+Columns")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::Columns {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Columns+StretchMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Columns_StretchMode {
    Grow = 0i32,
    GrowAndFill = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+Columns+StretchMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Columns_StretchMode =>
    "UnityEngine.UIElements"."Columns/StretchMode"
);
#[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectFactory_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Columns_UxmlObjectFactory_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlObjectFactory_2<
        T,
        *mut crate::UnityEngine::UIElements::Columns_UxmlObjectTraits_1<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectFactory_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Columns_UxmlObjectFactory_1 < T > =>
    "UnityEngine.UIElements"."Columns/UxmlObjectFactory`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectFactory_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::Columns_UxmlObjectFactory_1<T> {
    type Target = crate::UnityEngine::UIElements::UxmlObjectFactory_2<
        T,
        *mut crate::UnityEngine::UIElements::Columns_UxmlObjectTraits_1<T>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectFactory_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::Columns_UxmlObjectFactory_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectFactory_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::Columns_UxmlObjectFactory_1<T> {
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
#[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectFactory_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Columns_UxmlObjectFactory_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectTraits_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Columns_UxmlObjectTraits_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlObjectTraits_1<T>,
    pub m_PrimaryColumnName: *mut crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    pub m_StretchMode: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::Columns_StretchMode,
    >,
    pub m_Reorderable: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_Resizable: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_ResizePreview: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_Columns: *mut crate::UnityEngine::UIElements::UxmlObjectListAttributeDescription_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectTraits_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Columns_UxmlObjectTraits_1 < T > =>
    "UnityEngine.UIElements"."Columns/UxmlObjectTraits`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectTraits_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::Columns_UxmlObjectTraits_1<T> {
    type Target = crate::UnityEngine::UIElements::UxmlObjectTraits_1<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectTraits_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::Columns_UxmlObjectTraits_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectTraits_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::Columns_UxmlObjectTraits_1<T> {
    pub fn Init(
        &mut self,
        obj: quest_hook::libil2cpp::ByRefMut<T>,
        bag: *mut crate::UnityEngine::UIElements::IUxmlAttributes,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (obj, bag, cc))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
#[cfg(feature = "UnityEngine+UIElements+Columns+UxmlObjectTraits_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Columns_UxmlObjectTraits_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
