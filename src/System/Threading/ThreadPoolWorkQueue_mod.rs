#[cfg(feature = "System+Threading+ThreadPoolWorkQueue")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadPoolWorkQueue {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub queueHead: quest_hook::libil2cpp::Gc<
        crate::System::Threading::ThreadPoolWorkQueue_QueueSegment,
    >,
    pub queueTail: quest_hook::libil2cpp::Gc<
        crate::System::Threading::ThreadPoolWorkQueue_QueueSegment,
    >,
    pub numOutstandingThreadRequests: i32,
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::ThreadPoolWorkQueue =>
    "System.Threading"."ThreadPoolWorkQueue"
);
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue")]
impl std::ops::Deref for crate::System::Threading::ThreadPoolWorkQueue {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue")]
impl std::ops::DerefMut for crate::System::Threading::ThreadPoolWorkQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue")]
impl crate::System::Threading::ThreadPoolWorkQueue {
    #[cfg(feature = "System+Threading+ThreadPoolWorkQueue+QueueSegment")]
    pub type QueueSegment = crate::System::Threading::ThreadPoolWorkQueue_QueueSegment;
    #[cfg(feature = "System+Threading+ThreadPoolWorkQueue+SparseArray_1")]
    pub type SparseArray_1<T: quest_hook::libil2cpp::Type> = crate::System::Threading::ThreadPoolWorkQueue_SparseArray_1<
        T,
    >;
    #[cfg(feature = "System+Threading+ThreadPoolWorkQueue+WorkStealingQueue")]
    pub type WorkStealingQueue = crate::System::Threading::ThreadPoolWorkQueue_WorkStealingQueue;
    pub fn Dequeue(
        &mut self,
        tl: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ThreadPoolWorkQueueThreadLocals,
        >,
        callback: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Threading::IThreadPoolWorkItem>,
        >,
        missedSteal: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dequeue", (tl, callback, missedSteal))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispatch() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Dispatch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Enqueue(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Threading::IThreadPoolWorkItem,
        >,
        forceGlobal: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enqueue", (callback, forceGlobal))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureCurrentThreadHasQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::ThreadPoolWorkQueueThreadLocals,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ThreadPoolWorkQueueThreadLocals,
        > = __cordl_object.invoke("EnsureCurrentThreadHasQueue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureThreadRequested(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureThreadRequested", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalFindAndPop(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Threading::IThreadPoolWorkItem,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("LocalFindAndPop", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkThreadRequestSatisfied(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkThreadRequestSatisfied", ())?;
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
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::ThreadPoolWorkQueue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+QueueSegment")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadPoolWorkQueue_QueueSegment {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub nodes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Threading::IThreadPoolWorkItem,
        >,
    >,
    pub indexes: i32,
    pub Next: quest_hook::libil2cpp::Gc<
        crate::System::Threading::ThreadPoolWorkQueue_QueueSegment,
    >,
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+QueueSegment")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::ThreadPoolWorkQueue_QueueSegment => "System.Threading"
    ."ThreadPoolWorkQueue/QueueSegment"
);
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+QueueSegment")]
impl std::ops::Deref for crate::System::Threading::ThreadPoolWorkQueue_QueueSegment {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+QueueSegment")]
impl std::ops::DerefMut for crate::System::Threading::ThreadPoolWorkQueue_QueueSegment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+QueueSegment")]
impl crate::System::Threading::ThreadPoolWorkQueue_QueueSegment {
    pub fn CompareExchangeIndexes(
        &mut self,
        prevUpper: quest_hook::libil2cpp::ByRefMut<i32>,
        newUpper: i32,
        prevLower: quest_hook::libil2cpp::ByRefMut<i32>,
        newLower: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CompareExchangeIndexes",
                (prevUpper, newUpper, prevLower, newLower),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndexes(
        &mut self,
        upper: quest_hook::libil2cpp::ByRefMut<i32>,
        lower: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetIndexes", (upper, lower))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsUsedUp(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsUsedUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TryDequeue(
        &mut self,
        node: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Threading::IThreadPoolWorkItem>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryDequeue", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnqueue(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Threading::IThreadPoolWorkItem>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryEnqueue", (node))?;
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
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+QueueSegment")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::ThreadPoolWorkQueue_QueueSegment {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+SparseArray_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadPoolWorkQueue_SparseArray_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+SparseArray_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::ThreadPoolWorkQueue_SparseArray_1 < T > => "System.Threading"
    ."ThreadPoolWorkQueue/SparseArray`1" < T >
);
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+SparseArray_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::ThreadPoolWorkQueue_SparseArray_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+SparseArray_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::ThreadPoolWorkQueue_SparseArray_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+SparseArray_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Threading::ThreadPoolWorkQueue_SparseArray_1<T> {
    pub fn Add(&mut self, e: T) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Add", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        initialSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialSize))?;
        Ok(__cordl_object.into())
    }
    pub fn Remove(
        &mut self,
        e: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        initialSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
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
        > = __cordl_object.invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+SparseArray_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::ThreadPoolWorkQueue_SparseArray_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+WorkStealingQueue")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadPoolWorkQueue_WorkStealingQueue {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_array: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Threading::IThreadPoolWorkItem,
        >,
    >,
    pub m_mask: i32,
    pub m_headIndex: i32,
    pub m_tailIndex: i32,
    pub m_foreignLock: crate::System::Threading::SpinLock,
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+WorkStealingQueue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::ThreadPoolWorkQueue_WorkStealingQueue => "System.Threading"
    ."ThreadPoolWorkQueue/WorkStealingQueue"
);
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+WorkStealingQueue")]
impl std::ops::Deref
for crate::System::Threading::ThreadPoolWorkQueue_WorkStealingQueue {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+WorkStealingQueue")]
impl std::ops::DerefMut
for crate::System::Threading::ThreadPoolWorkQueue_WorkStealingQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+WorkStealingQueue")]
impl crate::System::Threading::ThreadPoolWorkQueue_WorkStealingQueue {
    pub fn LocalFindAndPop(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::System::Threading::IThreadPoolWorkItem>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("LocalFindAndPop", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalPop(
        &mut self,
        obj: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Threading::IThreadPoolWorkItem>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("LocalPop", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalPush(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::System::Threading::IThreadPoolWorkItem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LocalPush", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TrySteal_ByRefMut_ByRefMut0(
        &mut self,
        obj: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Threading::IThreadPoolWorkItem>,
        >,
        missedSteal: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TrySteal", (obj, missedSteal))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySteal_i32_1(
        &mut self,
        obj: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Threading::IThreadPoolWorkItem>,
        >,
        missedSteal: quest_hook::libil2cpp::ByRefMut<bool>,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TrySteal", (obj, missedSteal, millisecondsTimeout))?;
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
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueue+WorkStealingQueue")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::ThreadPoolWorkQueue_WorkStealingQueue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
