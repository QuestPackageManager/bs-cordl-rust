#[cfg(feature = "BeatmapDataSortedListForTypeAndIds_1")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataSortedListForTypeAndIds_1<TBase: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _items: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::System::ValueTuple_2<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                i32,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ISortedList_1<TBase>>,
        >,
    >,
    pub _sortedListsDataProcessors: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ISortedListItemProcessor_1<TBase>,
            >,
        >,
    >,
    pub _itemToNodeMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            TBase,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::LinkedListNode_1<TBase>,
            >,
        >,
    >,
    __cordl_phantom_TBase: std::marker::PhantomData<TBase>,
}
#[cfg(feature = "BeatmapDataSortedListForTypeAndIds_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapDataSortedListForTypeAndIds_1 < TBase > => ""
    ."BeatmapDataSortedListForTypeAndIds`1" < TBase >
);
#[cfg(feature = "BeatmapDataSortedListForTypeAndIds_1")]
impl<TBase: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::BeatmapDataSortedListForTypeAndIds_1<TBase> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataSortedListForTypeAndIds_1")]
impl<TBase: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::BeatmapDataSortedListForTypeAndIds_1<TBase> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataSortedListForTypeAndIds_1")]
impl<
    TBase: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::BeatmapDataSortedListForTypeAndIds_1<TBase> {
    pub fn GetCount<T>(
        &mut self,
        typeIdentifier: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetCount", (typeIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetItems<T>(
        &mut self,
        typeIdentifier: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerable_1<T>>,
    >
    where
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        > = __cordl_object.invoke("GetItems", (typeIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetList(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        typeIdentifier: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ISortedList_1<TBase>>,
    >
    where
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISortedList_1<TBase>,
        > = __cordl_object.invoke("GetList", (_cordl_type, typeIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertItem(
        &mut self,
        item: TBase,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<TBase>,
        >,
    >
    where
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<TBase>,
        > = __cordl_object.invoke("InsertItem", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveItem(
        &mut self,
        item: TBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveItem", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveList(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        typeIdentifier: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveList", (_cordl_type, typeIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sortedListHeads(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Collections::Generic::LinkedListNode_1<TBase>,
            >,
        >,
    >
    where
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Collections::Generic::LinkedListNode_1<TBase>,
            >,
        > = __cordl_object.invoke("get_sortedListHeads", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataSortedListForTypeAndIds_1")]
impl<TBase: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataSortedListForTypeAndIds_1<TBase> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
