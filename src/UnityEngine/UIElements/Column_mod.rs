#[cfg(feature = "UnityEngine+UIElements+Column")]
#[repr(C)]
#[derive(Debug)]
pub struct Column {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Name: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Title: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Icon: crate::UnityEngine::UIElements::Background,
    pub m_Visible: bool,
    pub m_Width: crate::UnityEngine::UIElements::Length,
    pub m_MinWidth: crate::UnityEngine::UIElements::Length,
    pub m_MaxWidth: crate::UnityEngine::UIElements::Length,
    pub m_DesiredWidth: f32,
    pub m_Stretchable: bool,
    pub m_Sortable: bool,
    pub m_Optional: bool,
    pub m_Resizable: bool,
    pub m_MakeHeader: *mut crate::System::Func_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_BindHeader: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_UnbindHeader: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_DestroyHeader: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_MakeCell: *mut crate::System::Func_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_BindCell: *mut crate::System::Action_2<
        *mut crate::UnityEngine::UIElements::VisualElement,
        i32,
    >,
    pub m_UnbindCellItem: *mut crate::System::Action_2<
        *mut crate::UnityEngine::UIElements::VisualElement,
        i32,
    >,
    pub _destroyCell_k__BackingField: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UIElements::VisualElement,
    >,
    pub _collection_k__BackingField: *mut crate::UnityEngine::UIElements::Columns,
    pub changed: *mut crate::System::Action_2<
        *mut crate::UnityEngine::UIElements::Column,
        crate::UnityEngine::UIElements::ColumnDataType,
    >,
    pub resized: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UIElements::Column,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Column")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Column =>
    "UnityEngine.UIElements"."Column"
);
#[cfg(feature = "UnityEngine+UIElements+Column")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Column {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Column")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Column {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Column")]
impl crate::UnityEngine::UIElements::Column {
    #[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectFactory_1")]
    pub type UxmlObjectFactory_1<T: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::Column_UxmlObjectFactory_1<
        T,
    >;
    #[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectTraits_1")]
    pub type UxmlObjectTraits_1<T: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::Column_UxmlObjectTraits_1<
        T,
    >;
    pub fn GetMaxWidth(
        &mut self,
        layoutWidth: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetMaxWidth", (layoutWidth))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMinWidth(
        &mut self,
        layoutWidth: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetMinWidth", (layoutWidth))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWidth(&mut self, layoutWidth: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetWidth", (layoutWidth))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyChange(
        &mut self,
        _cordl_type: crate::UnityEngine::UIElements::ColumnDataType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyChange", (_cordl_type))?;
        Ok(__cordl_ret.into())
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
    pub fn add_changed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::Column,
                crate::UnityEngine::UIElements::ColumnDataType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_changed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_resized(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::Column>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_resized", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bindCell(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::VisualElement,
                i32,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::VisualElement,
                i32,
            >,
        > = __cordl_object.invoke("get_bindCell", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bindHeader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        > = __cordl_object.invoke("get_bindHeader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_collection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Columns,
        > = __cordl_object.invoke("get_collection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_desiredWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_desiredWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_destroyCell(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        > = __cordl_object.invoke("get_destroyCell", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_destroyHeader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        > = __cordl_object.invoke("get_destroyHeader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_displayIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_displayIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_icon(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Background> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Background = __cordl_object
            .invoke("get_icon", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_index(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_index", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_makeCell(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        > = __cordl_object.invoke("get_makeCell", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_makeHeader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        > = __cordl_object.invoke("get_makeHeader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Length = __cordl_object
            .invoke("get_maxWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Length = __cordl_object
            .invoke("get_minWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_optional(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_optional", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_resizable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_resizable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sortable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_sortable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stretchable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_stretchable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_title(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_title", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unbindCell(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::VisualElement,
                i32,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::VisualElement,
                i32,
            >,
        > = __cordl_object.invoke("get_unbindCell", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unbindHeader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        > = __cordl_object.invoke("get_unbindHeader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_visible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_visible", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_visibleIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_visibleIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_width(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::Length = __cordl_object
            .invoke("get_width", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_changed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::UnityEngine::UIElements::Column,
                crate::UnityEngine::UIElements::ColumnDataType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_changed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_resized(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UIElements::Column>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_resized", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_collection(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_collection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_desiredWidth(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_desiredWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_makeCell(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_makeCell", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_makeHeader(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::UnityEngine::UIElements::VisualElement>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_makeHeader", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_minWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_minWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_name", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_optional(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_optional", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_sortable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sortable", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_stretchable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_stretchable", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_title(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_title", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_visible(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_visible", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_width(
        &mut self,
        value: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_width", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Column")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::Column {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectFactory_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Column_UxmlObjectFactory_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlObjectFactory_2<
        T,
        *mut crate::UnityEngine::UIElements::Column_UxmlObjectTraits_1<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectFactory_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Column_UxmlObjectFactory_1 < T > =>
    "UnityEngine.UIElements"."Column/UxmlObjectFactory`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectFactory_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::Column_UxmlObjectFactory_1<T> {
    type Target = crate::UnityEngine::UIElements::UxmlObjectFactory_2<
        T,
        *mut crate::UnityEngine::UIElements::Column_UxmlObjectTraits_1<T>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectFactory_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::Column_UxmlObjectFactory_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectFactory_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::Column_UxmlObjectFactory_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectFactory_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Column_UxmlObjectFactory_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectTraits_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Column_UxmlObjectTraits_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlObjectTraits_1<T>,
    pub m_Name: *mut crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    pub m_Text: *mut crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    pub m_Visible: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_Width: *mut crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    pub m_MinWidth: *mut crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    pub m_MaxWidth: *mut crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    pub m_Stretch: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_Sortable: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_Optional: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_Resizable: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_HeaderTemplateId: *mut crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    pub m_CellTemplateId: *mut crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectTraits_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Column_UxmlObjectTraits_1 < T > =>
    "UnityEngine.UIElements"."Column/UxmlObjectTraits`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectTraits_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::Column_UxmlObjectTraits_1<T> {
    type Target = crate::UnityEngine::UIElements::UxmlObjectTraits_1<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectTraits_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::Column_UxmlObjectTraits_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectTraits_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::Column_UxmlObjectTraits_1<T> {
    #[cfg(
        feature = "UnityEngine+UIElements+Column+UxmlObjectTraits_1+__c__DisplayClass15_0"
    )]
    pub type __c__DisplayClass15_0 = crate::UnityEngine::UIElements::UxmlObjectTraits_1_Column___c__DisplayClass15_0<
        T,
    >;
    #[cfg(
        feature = "UnityEngine+UIElements+Column+UxmlObjectTraits_1+__c__DisplayClass15_1"
    )]
    pub type __c__DisplayClass15_1 = crate::UnityEngine::UIElements::UxmlObjectTraits_1_Column___c__DisplayClass15_1<
        T,
    >;
    pub fn Init(
        &mut self,
        obj: quest_hook::libil2cpp::ByRefMut<T>,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ParseLength(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultValue: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseLength", (str, defaultValue))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Column+UxmlObjectTraits_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Column_UxmlObjectTraits_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
