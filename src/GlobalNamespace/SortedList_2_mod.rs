#[cfg(feature = "SortedList_2")]
#[repr(C)]
#[derive(Debug)]
pub struct SortedList_2<
    T: quest_hook::libil2cpp::Type,
    TBase: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _items: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::LinkedList_1<TBase>,
    >,
    pub _sortedListDataProcessor: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ISortedListItemProcessor_1<TBase>,
    >,
    pub _lastUsedNode: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::LinkedListNode_1<TBase>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
    __cordl_phantom_TBase: std::marker::PhantomData<TBase>,
}
#[cfg(feature = "SortedList_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SortedList_2 < T, TBase > => ""
    ."SortedList`2" < T, TBase >
);
#[cfg(feature = "SortedList_2")]
impl<T: quest_hook::libil2cpp::Type, TBase: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::SortedList_2<T, TBase> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SortedList_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    TBase: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::GlobalNamespace::SortedList_2<T, TBase> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SortedList_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    TBase: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::SortedList_2<T, TBase> {
    pub fn InsertInternal(
        &mut self,
        newItem: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<TBase>,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn Insert_LinkedListNode_1_1(
        &mut self,
        newNode: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<TBase>,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn Insert_TBase0(
        &mut self,
        newItem: TBase,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<TBase>,
        >,
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<TBase>,
        > = __cordl_object.invoke("Insert", (newItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        sortedListDataProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISortedListItemProcessor_1<TBase>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sortedListDataProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn Remove(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<TBase>,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn TouchLastUsedNode(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<TBase>,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        sortedListDataProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISortedListItemProcessor_1<TBase>,
        >,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_items(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedList_1<TBase>,
        >,
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedList_1<TBase>,
        > = __cordl_object.invoke("get_items", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SortedList_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    TBase: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SortedList_2<T, TBase> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SortedList_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    TBase: quest_hook::libil2cpp::Type,
> AsRef<crate::GlobalNamespace::ISortedList_1<TBase>>
for crate::GlobalNamespace::SortedList_2<T, TBase> {
    fn as_ref(&self) -> &crate::GlobalNamespace::ISortedList_1<TBase> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SortedList_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    TBase: quest_hook::libil2cpp::Type,
> AsMut<crate::GlobalNamespace::ISortedList_1<TBase>>
for crate::GlobalNamespace::SortedList_2<T, TBase> {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ISortedList_1<TBase> {
        unsafe { std::mem::transmute(self) }
    }
}
