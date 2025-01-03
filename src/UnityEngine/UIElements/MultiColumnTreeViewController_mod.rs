#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiColumnTreeViewController {
    __cordl_parent: crate::UnityEngine::UIElements::BaseTreeViewController,
    pub m_ColumnController: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::MultiColumnController,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MultiColumnTreeViewController => "UnityEngine.UIElements"
    ."MultiColumnTreeViewController"
);
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeViewController")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MultiColumnTreeViewController {
    type Target = crate::UnityEngine::UIElements::BaseTreeViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeViewController")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::MultiColumnTreeViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeViewController")]
impl crate::UnityEngine::UIElements::MultiColumnTreeViewController {
    pub fn BindItem(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BindItem", (element, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyItem(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyItem", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeMakeItem(
        &mut self,
        reusableItem: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ReusableCollectionItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeMakeItem", (reusableItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("MakeItem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        columns: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
        sortDescriptions: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescriptions,
        >,
        sortedColumns: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::UIElements::SortColumnDescription,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (columns, sortDescriptions, sortedColumns))?;
        Ok(__cordl_object.into())
    }
    pub fn PrepareView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareView", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnbindItem(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnbindItem", (element, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        columns: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Columns>,
        sortDescriptions: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::SortColumnDescriptions,
        >,
        sortedColumns: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::UIElements::SortColumnDescription,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (columns, sortDescriptions, sortedColumns))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_columnController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MultiColumnController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MultiColumnController,
        > = __cordl_object.invoke("get_columnController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_header(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Internal::MultiColumnCollectionHeader,
        > = __cordl_object.invoke("get_header", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+MultiColumnTreeViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MultiColumnTreeViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
