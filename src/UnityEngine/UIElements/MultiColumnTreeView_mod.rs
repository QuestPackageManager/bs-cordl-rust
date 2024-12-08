#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnTreeView {
    __cordl_parent: crate::UnityEngine::UIElements::BaseTreeView,
    pub m_Columns: *mut crate::UnityEngine::UIElements::Columns,
    pub m_SortingEnabled: bool,
    pub m_SortColumnDescriptions: *mut crate::UnityEngine::UIElements::SortColumnDescriptions,
    pub m_SortedColumns: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescription,
    >,
    pub columnSortingChanged: *mut crate::System::Action,
    pub headerContextMenuPopulateEvent: *mut crate::System::Action_2<
        *mut crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
        *mut crate::UnityEngine::UIElements::Column,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MultiColumnTreeView =>
    "UnityEngine.UIElements"."MultiColumnTreeView"
);
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MultiColumnTreeView {
    type Target = crate::UnityEngine::UIElements::BaseTreeView;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MultiColumnTreeView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView")]
impl crate::UnityEngine::UIElements::MultiColumnTreeView {
    #[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::MultiColumnTreeView_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::MultiColumnTreeView_UxmlTraits;
    pub fn CreateViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::CollectionViewController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::CollectionViewController = __cordl_object
            .invoke("CreateViewController", ())?;
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Columns1(
        columns: *mut crate::UnityEngine::UIElements::Columns,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (columns))?;
        Ok(__cordl_object)
    }
    pub fn RaiseColumnSortingChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseColumnSortingChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn RaiseHeaderContextMenuPopulate(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
        column: *mut crate::UnityEngine::UIElements::Column,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseHeaderContextMenuPopulate", (evt, column))?;
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
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Columns1(
        &mut self,
        columns: *mut crate::UnityEngine::UIElements::Columns,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (columns))?;
        Ok(__cordl_ret)
    }
    pub fn get_columns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::Columns> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::Columns = __cordl_object
            .invoke("get_columns", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sortColumnDescriptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::SortColumnDescriptions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::SortColumnDescriptions = __cordl_object
            .invoke("get_sortColumnDescriptions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_viewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::MultiColumnTreeViewController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::MultiColumnTreeViewController = __cordl_object
            .invoke("get_viewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_columns(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::Columns,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_columns", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_sortColumnDescriptions(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::SortColumnDescriptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sortColumnDescriptions", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_sortingEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sortingEnabled", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MultiColumnTreeView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnTreeView_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::MultiColumnTreeView,
        *mut crate::UnityEngine::UIElements::MultiColumnTreeView_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MultiColumnTreeView_UxmlFactory =>
    "UnityEngine.UIElements"."MultiColumnTreeView/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlFactory")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::MultiColumnTreeView_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::MultiColumnTreeView,
        *mut crate::UnityEngine::UIElements::MultiColumnTreeView_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlFactory")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::MultiColumnTreeView_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlFactory")]
impl crate::UnityEngine::UIElements::MultiColumnTreeView_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MultiColumnTreeView_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnTreeView_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseTreeView_UxmlTraits,
    pub m_SortingEnabled: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_Columns: *mut crate::UnityEngine::UIElements::UxmlObjectAttributeDescription_1<
        *mut crate::UnityEngine::UIElements::Columns,
    >,
    pub m_SortColumnDescriptions: *mut crate::UnityEngine::UIElements::UxmlObjectAttributeDescription_1<
        *mut crate::UnityEngine::UIElements::SortColumnDescriptions,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MultiColumnTreeView_UxmlTraits => "UnityEngine.UIElements"
    ."MultiColumnTreeView/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MultiColumnTreeView_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseTreeView_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlTraits")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::MultiColumnTreeView_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlTraits")]
impl crate::UnityEngine::UIElements::MultiColumnTreeView_UxmlTraits {
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
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeView+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MultiColumnTreeView_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
