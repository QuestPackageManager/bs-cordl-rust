#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1")]
#[repr(C)]
#[derive(Debug)]
pub struct MultipleSortedBeatmapObjectsListsEnumerator_1<
    T: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _priorityQueue: quest_hook::libil2cpp::Gc<
        crate::Priority_Queue::StablePriorityQueue_1<
            *mut crate::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1_BeatmapObjectListNode<
                T,
            >,
        >,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1 < T > => ""
    ."MultipleSortedBeatmapObjectsListsEnumerator`1" < T >
);
#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1<T> {
    #[cfg(
        feature = "MultipleSortedBeatmapObjectsListsEnumerator_1+BeatmapObjectListNode"
    )]
    pub type BeatmapObjectListNode = crate::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1_BeatmapObjectListNode<
        T,
    >;
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::System::ValueTuple_2<T, i32>,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::System::ValueTuple_2<T, i32>,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        dataList: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::ValueTuple_2<
                    *mut crate::System::Collections::Generic::IReadOnlyList_1<T>,
                    i32,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataList))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        dataList: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::ValueTuple_2<
                    *mut crate::System::Collections::Generic::IReadOnlyList_1<T>,
                    i32,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataList))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
for crate::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
for crate::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1+BeatmapObjectListNode")]
#[repr(C)]
#[derive(Debug)]
pub struct MultipleSortedBeatmapObjectsListsEnumerator_1_BeatmapObjectListNode<
    T: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Priority_Queue::StablePriorityQueueNode,
    pub _dataList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<T>,
    >,
    pub _typeId: i32,
    pub _idx: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1+BeatmapObjectListNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1_BeatmapObjectListNode
    < T > => ""."MultipleSortedBeatmapObjectsListsEnumerator`1/BeatmapObjectListNode" < T
    >
);
#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1+BeatmapObjectListNode")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1_BeatmapObjectListNode<
    T,
> {
    type Target = crate::Priority_Queue::StablePriorityQueueNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1+BeatmapObjectListNode")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1_BeatmapObjectListNode<
    T,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1+BeatmapObjectListNode")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1_BeatmapObjectListNode<
    T,
> {
    pub fn MoveToNextItem(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveToNextItem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        dataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<T>,
        >,
        typeId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dataList, typeId))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        dataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<T>,
        >,
        typeId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dataList, typeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_typeId(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_typeId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("get_value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultipleSortedBeatmapObjectsListsEnumerator_1+BeatmapObjectListNode")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultipleSortedBeatmapObjectsListsEnumerator_1_BeatmapObjectListNode<
    T,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
