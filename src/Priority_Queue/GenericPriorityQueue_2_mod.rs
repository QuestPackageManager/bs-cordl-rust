#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericPriorityQueue_2<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _numNodes: i32,
    pub _nodes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TItem>>,
    pub _numNodesEverEnqueued: i64,
    pub _comparer: quest_hook::libil2cpp::Gc<crate::System::Comparison_1<TPriority>>,
    __cordl_phantom_TItem: std::marker::PhantomData<TItem>,
    __cordl_phantom_TPriority: std::marker::PhantomData<TPriority>,
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Priority_Queue::GenericPriorityQueue_2 < TItem,
    TPriority > => "Priority_Queue"."GenericPriorityQueue`2" < TItem, TPriority >
);
#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::Priority_Queue::GenericPriorityQueue_2<TItem, TPriority> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Priority_Queue::GenericPriorityQueue_2<TItem, TPriority> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> crate::Priority_Queue::GenericPriorityQueue_2<TItem, TPriority> {
    pub fn CascadeDown(
        &mut self,
        node: TItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CascadeDown", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn CascadeUp(
        &mut self,
        node: TItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CascadeUp", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains(&mut self, node: TItem) -> quest_hook::libil2cpp::Result<bool>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dequeue(&mut self) -> quest_hook::libil2cpp::Result<TItem>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TItem = __cordl_object.invoke("Dequeue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Enqueue(
        &mut self,
        node: TItem,
        priority: TPriority,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enqueue", (node, priority))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TItem>,
        >,
    >
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TItem>,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasHigherPriority(
        &mut self,
        higher: TItem,
        lower: TItem,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasHigherPriority", (higher, lower))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidQueue(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValidQueue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Comparison_1_2(
        maxNodes: i32,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Comparison_1<TPriority>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxNodes, comparer))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IComparer_1_1(
        maxNodes: i32,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TPriority>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxNodes, comparer))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_0(
        maxNodes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxNodes))?;
        Ok(__cordl_object.into())
    }
    pub fn OnNodeUpdated(
        &mut self,
        node: TItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNodeUpdated", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        &mut self,
        node: TItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetNode(
        &mut self,
        node: TItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn Resize(
        &mut self,
        maxNodes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resize", (maxNodes))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
    pub fn UpdatePriority(
        &mut self,
        node: TItem,
        priority: TPriority,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePriority", (node, priority))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Comparison_1_2(
        &mut self,
        maxNodes: i32,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Comparison_1<TPriority>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (maxNodes, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IComparer_1_1(
        &mut self,
        maxNodes: i32,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TPriority>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (maxNodes, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        maxNodes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (maxNodes))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_First(&mut self) -> quest_hook::libil2cpp::Result<TItem>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TItem = __cordl_object.invoke("get_First", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxSize(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxSize", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Priority_Queue::GenericPriorityQueue_2<TItem, TPriority> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsRef<crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority>>
for crate::Priority_Queue::GenericPriorityQueue_2<TItem, TPriority> {
    fn as_ref(
        &self,
    ) -> &crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsMut<crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority>>
for crate::Priority_Queue::GenericPriorityQueue_2<TItem, TPriority> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsRef<crate::Priority_Queue::IPriorityQueue_2<TItem, TPriority>>
for crate::Priority_Queue::GenericPriorityQueue_2<TItem, TPriority> {
    fn as_ref(&self) -> &crate::Priority_Queue::IPriorityQueue_2<TItem, TPriority> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsMut<crate::Priority_Queue::IPriorityQueue_2<TItem, TPriority>>
for crate::Priority_Queue::GenericPriorityQueue_2<TItem, TPriority> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Priority_Queue::IPriorityQueue_2<TItem, TPriority> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<TItem>>
for crate::Priority_Queue::GenericPriorityQueue_2<TItem, TPriority> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<TItem> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<TItem>>
for crate::Priority_Queue::GenericPriorityQueue_2<TItem, TPriority> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<TItem> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerable>
for crate::Priority_Queue::GenericPriorityQueue_2<TItem, TPriority> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerable>
for crate::Priority_Queue::GenericPriorityQueue_2<TItem, TPriority> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
