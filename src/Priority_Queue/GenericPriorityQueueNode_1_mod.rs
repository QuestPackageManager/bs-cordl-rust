#[cfg(feature = "Priority_Queue+GenericPriorityQueueNode_1")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericPriorityQueueNode_1<TPriority: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Priority_k__BackingField: TPriority,
    pub _QueueIndex_k__BackingField: i32,
    pub _InsertionIndex_k__BackingField: i64,
    __cordl_phantom_TPriority: std::marker::PhantomData<TPriority>,
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueueNode_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Priority_Queue::GenericPriorityQueueNode_1 <
    TPriority > => "Priority_Queue"."GenericPriorityQueueNode`1" < TPriority >
);
#[cfg(feature = "Priority_Queue+GenericPriorityQueueNode_1")]
impl<TPriority: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Priority_Queue::GenericPriorityQueueNode_1<TPriority> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueueNode_1")]
impl<TPriority: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Priority_Queue::GenericPriorityQueueNode_1<TPriority> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueueNode_1")]
impl<
    TPriority: quest_hook::libil2cpp::Type,
> crate::Priority_Queue::GenericPriorityQueueNode_1<TPriority> {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InsertionIndex(&mut self) -> quest_hook::libil2cpp::Result<i64>
    where
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_InsertionIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Priority(&mut self) -> quest_hook::libil2cpp::Result<TPriority>
    where
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TPriority = __cordl_object.invoke("get_Priority", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_QueueIndex(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_QueueIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_InsertionIndex(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InsertionIndex", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Priority(
        &mut self,
        value: TPriority,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Priority", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_QueueIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_QueueIndex", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Priority_Queue+GenericPriorityQueueNode_1")]
impl<TPriority: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Priority_Queue::GenericPriorityQueueNode_1<TPriority> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
