#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnListView {
    __cordl_parent: crate::UnityEngine::UIElements::BaseListView,
    pub m_Columns: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
    pub m_SortingEnabled: bool,
    pub m_SortColumnDescriptions: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::SortColumnDescriptions,
    >,
    pub m_SortedColumns: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::SortColumnDescription,
        >,
    >,
    pub columnSortingChanged: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub headerContextMenuPopulateEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
            *mut crate::UnityEngine::UIElements::Column,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MultiColumnListView =>
    "UnityEngine.UIElements"."MultiColumnListView"
);
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MultiColumnListView {
    type Target = crate::UnityEngine::UIElements::BaseListView;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MultiColumnListView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView")]
impl crate::UnityEngine::UIElements::MultiColumnListView {
    #[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::MultiColumnListView_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::MultiColumnListView_UxmlTraits;
    pub fn CreateViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionViewController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionViewController,
        > = __cordl_object.invoke("CreateViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateVirtualizationController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateVirtualizationController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Columns1(
        columns: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (columns))?;
        Ok(__cordl_object.into())
    }
    pub fn RaiseColumnSortingChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseColumnSortingChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseHeaderContextMenuPopulate(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
        >,
        column: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Column>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseHeaderContextMenuPopulate", (evt, column))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetViewController(
        &mut self,
        controller: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::CollectionViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetViewController", (controller))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Columns1(
        &mut self,
        columns: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (columns))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_columns(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Columns,
        > = __cordl_object.invoke("get_columns", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sortColumnDescriptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::SortColumnDescriptions>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescriptions,
        > = __cordl_object.invoke("get_sortColumnDescriptions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_viewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MultiColumnListViewController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MultiColumnListViewController,
        > = __cordl_object.invoke("get_viewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_columns(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_columns", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sortColumnDescriptions(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescriptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sortColumnDescriptions", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MultiColumnListView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnListView_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::MultiColumnListView,
        *mut crate::UnityEngine::UIElements::MultiColumnListView_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MultiColumnListView_UxmlFactory =>
    "UnityEngine.UIElements"."MultiColumnListView/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlFactory")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::MultiColumnListView_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::MultiColumnListView,
        *mut crate::UnityEngine::UIElements::MultiColumnListView_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlFactory")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::MultiColumnListView_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlFactory")]
impl crate::UnityEngine::UIElements::MultiColumnListView_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MultiColumnListView_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnListView_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseListView_UxmlTraits,
    pub m_SortingEnabled: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    >,
    pub m_Columns: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlObjectAttributeDescription_1<
            *mut crate::UnityEngine::UIElements::Columns,
        >,
    >,
    pub m_SortColumnDescriptions: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlObjectAttributeDescription_1<
            *mut crate::UnityEngine::UIElements::SortColumnDescriptions,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MultiColumnListView_UxmlTraits => "UnityEngine.UIElements"
    ."MultiColumnListView/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MultiColumnListView_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseListView_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlTraits")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::MultiColumnListView_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlTraits")]
impl crate::UnityEngine::UIElements::MultiColumnListView_UxmlTraits {
    pub fn Init(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (ve, bag, cc))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListView+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MultiColumnListView_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
