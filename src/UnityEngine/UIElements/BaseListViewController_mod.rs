#[cfg(feature = "UnityEngine+UIElements+BaseListViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseListViewController {
    __cordl_parent: crate::UnityEngine::UIElements::CollectionViewController,
    pub itemsSourceSizeChanged: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub itemsAdded: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    >,
    pub itemsRemoved: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+BaseListViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::BaseListViewController
    => "UnityEngine.UIElements"."BaseListViewController"
);
#[cfg(feature = "UnityEngine+UIElements+BaseListViewController")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BaseListViewController {
    type Target = crate::UnityEngine::UIElements::CollectionViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseListViewController")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BaseListViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseListViewController")]
impl crate::UnityEngine::UIElements::BaseListViewController {
    pub fn AddItems(
        &mut self,
        itemCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddItems", (itemCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddToArray(
        source: quest_hook::libil2cpp::Gc<crate::System::Array>,
        itemCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddToArray", (source, itemCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearItems", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureItemSourceCanBeResized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureItemSourceCanBeResized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeBindItem(
        &mut self,
        reusableItem: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ReusableCollectionItem,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeBindItem", (reusableItem, index))?;
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
    pub fn Move(
        &mut self,
        index: i32,
        newIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Move", (index, newIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn NeedsDragHandle(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("NeedsDragHandle", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PostInitRegistration(
        &mut self,
        listItem: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ReusableListViewItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PostInitRegistration", (listItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseItemsAdded(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseItemsAdded", (indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseItemsRemoved(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseItemsRemoved", (indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseOnSizeChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseOnSizeChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFromArray(
        source: quest_hook::libil2cpp::Gc<crate::System::Array>,
        indicesToRemove: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveFromArray", (source, indicesToRemove))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveItem(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveItem", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveItems_List_1_0(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveItems", (indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveItems_i32_1(
        &mut self,
        itemCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveItems", (itemCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn Swap(
        &mut self,
        lhs: i32,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Swap", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn _AddItems_g__IsGenericList_15_0(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("<AddItems>g__IsGenericList|15_0", (t))?;
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
    pub fn add_itemsAdded(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_itemsAdded", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_itemsRemoved(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_itemsRemoved", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_itemsSourceSizeChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_itemsSourceSizeChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_baseListView(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseListView>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseListView,
        > = __cordl_object.invoke("get_baseListView", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_itemsAdded(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_itemsAdded", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_itemsRemoved(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::System::Collections::Generic::IEnumerable_1<i32>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_itemsRemoved", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_itemsSourceSizeChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_itemsSourceSizeChanged", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+BaseListViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BaseListViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
