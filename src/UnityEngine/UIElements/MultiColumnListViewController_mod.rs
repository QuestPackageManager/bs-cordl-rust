#[cfg(feature = "UnityEngine+UIElements+MultiColumnListViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnListViewController {
    __cordl_parent: crate::UnityEngine::UIElements::BaseListViewController,
    pub m_ColumnController: *mut crate::UnityEngine::UIElements::MultiColumnController,
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MultiColumnListViewController => "UnityEngine.UIElements"
    ."MultiColumnListViewController"
);
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListViewController")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MultiColumnListViewController {
    type Target = crate::UnityEngine::UIElements::BaseListViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListViewController")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::MultiColumnListViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListViewController")]
impl crate::UnityEngine::UIElements::MultiColumnListViewController {
    pub fn BindItem(
        &mut self,
        element: *mut crate::UnityEngine::UIElements::VisualElement,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindItem", (element, index))?;
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
    pub fn New(
        columns: *mut crate::UnityEngine::UIElements::Columns,
        sortDescriptions: *mut crate::UnityEngine::UIElements::SortColumnDescriptions,
        sortedColumns: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::SortColumnDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (columns, sortDescriptions, sortedColumns))?;
        Ok(__cordl_object)
    }
    pub fn PrepareView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareView", ())?;
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
    pub fn UpdateReorderClassList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateReorderClassList", ())?;
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
    pub fn get_columnController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::MultiColumnController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::MultiColumnController = __cordl_object
            .invoke("get_columnController", ())?;
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
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnListViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MultiColumnListViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}