#[cfg(feature = "Priority_Queue+FastPriorityQueueNode")]
#[repr(C)]
#[derive(Debug)]
pub struct FastPriorityQueueNode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Priority_k__BackingField: f32,
    pub _QueueIndex_k__BackingField: i32,
}
#[cfg(feature = "Priority_Queue+FastPriorityQueueNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Priority_Queue::FastPriorityQueueNode =>
    "Priority_Queue"."FastPriorityQueueNode"
);
#[cfg(feature = "Priority_Queue+FastPriorityQueueNode")]
impl std::ops::Deref for crate::Priority_Queue::FastPriorityQueueNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Priority_Queue+FastPriorityQueueNode")]
impl std::ops::DerefMut for crate::Priority_Queue::FastPriorityQueueNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Priority_Queue+FastPriorityQueueNode")]
impl crate::Priority_Queue::FastPriorityQueueNode {
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
    pub fn get_Priority(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_Priority", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_QueueIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_QueueIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Priority(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Priority", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_QueueIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_QueueIndex", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Priority_Queue+FastPriorityQueueNode")]
impl quest_hook::libil2cpp::ObjectType for crate::Priority_Queue::FastPriorityQueueNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
