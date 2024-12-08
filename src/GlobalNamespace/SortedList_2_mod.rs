#[cfg(feature = "SortedList_2")]
#[repr(C)]
#[derive(Debug)]
pub struct SortedList_2<
    T: quest_hook::libil2cpp::Type,
    TBase: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Object,
    pub _items: *mut crate::System::Collections::Generic::LinkedList_1<TBase>,
    pub _sortedListDataProcessor: *mut ISortedListItemProcessor_1<TBase>,
    pub _lastUsedNode: *mut crate::System::Collections::Generic::LinkedListNode_1<TBase>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
    __cordl_phantom_TBase: std::marker::PhantomData<TBase>,
}
#[cfg(feature = "SortedList_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SortedList_2 < T, TBase > => ""."SortedList`2" < T,
    TBase >
);
#[cfg(feature = "SortedList_2")]
impl<T: quest_hook::libil2cpp::Type, TBase: quest_hook::libil2cpp::Type> std::ops::Deref
for SortedList_2<T, TBase> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SortedList_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    TBase: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for SortedList_2<T, TBase> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SortedList_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    TBase: quest_hook::libil2cpp::Type,
> SortedList_2<T, TBase> {
    pub fn get_items(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::LinkedList_1<TBase>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::LinkedList_1<TBase> = __cordl_object
            .invoke("get_items", ())?;
        Ok(__cordl_ret)
    }
    pub fn InsertInternal(
        &mut self,
        newItem: *mut crate::System::Collections::Generic::LinkedListNode_1<TBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertInternal", (newItem))?;
        Ok(__cordl_ret)
    }
    pub fn get_count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_count", ())?;
        Ok(__cordl_ret)
    }
    pub fn Insert_TBase0(
        &mut self,
        newItem: TBase,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::LinkedListNode_1<TBase>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::LinkedListNode_1<
            TBase,
        > = __cordl_object.invoke("Insert", (newItem))?;
        Ok(__cordl_ret)
    }
    pub fn Insert_LinkedListNode_1_1(
        &mut self,
        newNode: *mut crate::System::Collections::Generic::LinkedListNode_1<TBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Insert", (newNode))?;
        Ok(__cordl_ret)
    }
    pub fn Remove(
        &mut self,
        node: *mut crate::System::Collections::Generic::LinkedListNode_1<TBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (node))?;
        Ok(__cordl_ret)
    }
    pub fn TouchLastUsedNode(
        &mut self,
        node: *mut crate::System::Collections::Generic::LinkedListNode_1<TBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TouchLastUsedNode", (node))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        sortedListDataProcessor: *mut ISortedListItemProcessor_1<TBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sortedListDataProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        sortedListDataProcessor: *mut ISortedListItemProcessor_1<TBase>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sortedListDataProcessor))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SortedList_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    TBase: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType for SortedList_2<T, TBase> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
