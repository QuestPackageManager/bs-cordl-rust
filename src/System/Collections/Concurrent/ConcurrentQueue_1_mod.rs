#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ConcurrentQueue_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _crossSegmentLock: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _tail: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T>,
    >,
    pub _head: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Concurrent::ConcurrentQueue_1 < T > =>
    "System.Collections.Concurrent"."ConcurrentQueue`1" < T >
);
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    #[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1+Segment")]
    pub type Segment = crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<
        T,
    >;
    pub fn CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn Enqueue(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enqueue", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnqueueSlow(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnqueueSlow", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn Enumerate(
        &mut self,
        head: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T>,
        >,
        headHead: i32,
        tail: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T>,
        >,
        tailTail: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerator_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<T>,
        > = __cordl_object.invoke("Enumerate", (head, headHead, tail, tailTail))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCount_ConcurrentQueue_1_Segment_i32_1(
        head: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T>,
        >,
        headHead: i32,
        tail: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T>,
        >,
        tailTail: i32,
    ) -> quest_hook::libil2cpp::Result<i64>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCount", (head, headHead, tail, tailTail))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCount_i32_0(
        s: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T>,
        >,
        head: i32,
        tail: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCount", (s, head, tail))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerator_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<T>,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetItemWhenAvailable(
        &mut self,
        segment: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T>,
        >,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("GetItemWhenAvailable", (segment, i))?;
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
    pub fn SnapForObservation(
        &mut self,
        head: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T>,
            >,
        >,
        headHead: quest_hook::libil2cpp::ByRefMut<i32>,
        tail: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T>,
            >,
        >,
        tailTail: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SnapForObservation", (head, headHead, tail, tailTail))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Concurrent_IProducerConsumerCollection_T__TryAdd(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Concurrent.IProducerConsumerCollection<T>.TryAdd",
                (item),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Concurrent_IProducerConsumerCollection_T__TryTake(
        &mut self,
        item: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Concurrent.IProducerConsumerCollection<T>.TryTake",
                (item),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.ICollection.CopyTo", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_get_IsSynchronized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.ICollection.get_IsSynchronized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_get_SyncRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.Collections.ICollection.get_SyncRoot", ())?;
        Ok(__cordl_ret.into())
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
    pub fn ToArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = __cordl_object.invoke("ToArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryDequeue(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryDequeue", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryDequeueSlow(
        &mut self,
        item: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryDequeueSlow", (item))?;
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
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Concurrent::IProducerConsumerCollection_1<T>>
for crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Concurrent::IProducerConsumerCollection_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Concurrent::IProducerConsumerCollection_1<T>>
for crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Concurrent::IProducerConsumerCollection_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<T>>
for crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<T>>
for crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IEnumerable_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IReadOnlyCollection_1<T>>
for crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IReadOnlyCollection_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IReadOnlyCollection_1<T>>
for crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyCollection_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::ICollection>
for crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::ICollection>
for crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
for crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
for crate::System::Collections::Concurrent::ConcurrentQueue_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1+Segment")]
#[repr(C)]
#[derive(Debug)]
pub struct ConcurrentQueue_1_Segment<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _slots: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::System::Collections::Concurrent::Segment_ConcurrentQueue_1_Slot<T>,
        >,
    >,
    pub _slotsMask: i32,
    pub _headAndTail: crate::System::Collections::Concurrent::PaddedHeadAndTail,
    pub _preservedForObservation: bool,
    pub _frozenForEnqueues: bool,
    pub _nextSegment: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1+Segment")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Concurrent::ConcurrentQueue_1_Segment < T > =>
    "System.Collections.Concurrent"."ConcurrentQueue`1/Segment" < T >
);
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1+Segment")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1+Segment")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1+Segment")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T> {
    #[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1+Segment+Slot")]
    pub type Slot = crate::System::Collections::Concurrent::Segment_ConcurrentQueue_1_Slot<
        T,
    >;
    pub fn EnsureFrozenForEnqueues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureFrozenForEnqueues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        boundedLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (boundedLength))?;
        Ok(__cordl_object.into())
    }
    pub fn TryDequeue(
        &mut self,
        item: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryDequeue", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnqueue(&mut self, item: T) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryEnqueue", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        boundedLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (boundedLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Capacity(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Capacity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FreezeOffset(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_FreezeOffset", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1+Segment")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Concurrent::ConcurrentQueue_1_Segment<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1+Segment+Slot")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Segment_ConcurrentQueue_1_Slot<T: quest_hook::libil2cpp::Type> {
    pub Item: T,
    pub SequenceNumber: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1+Segment+Slot")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Concurrent::Segment_ConcurrentQueue_1_Slot < T > =>
    "System.Collections.Concurrent"."ConcurrentQueue`1/Segment/Slot<T>" < T >
);
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1+Segment+Slot")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Collections::Concurrent::Segment_ConcurrentQueue_1_Slot<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Collections+Concurrent+ConcurrentQueue_1+Segment+Slot")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Collections::Concurrent::Segment_ConcurrentQueue_1_Slot<T> {}
