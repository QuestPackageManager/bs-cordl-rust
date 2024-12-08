#[cfg(feature = "Priority_Queue+StablePriorityQueueNode")]
#[repr(C)]
#[derive(Debug)]
pub struct StablePriorityQueueNode {
    __cordl_parent: crate::Priority_Queue::FastPriorityQueueNode,
    pub _InsertionIndex_k__BackingField: i64,
}
#[cfg(feature = "Priority_Queue+StablePriorityQueueNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Priority_Queue::StablePriorityQueueNode =>
    "Priority_Queue"."StablePriorityQueueNode"
);
#[cfg(feature = "Priority_Queue+StablePriorityQueueNode")]
impl std::ops::Deref for crate::Priority_Queue::StablePriorityQueueNode {
    type Target = crate::Priority_Queue::FastPriorityQueueNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Priority_Queue+StablePriorityQueueNode")]
impl std::ops::DerefMut for crate::Priority_Queue::StablePriorityQueueNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Priority_Queue+StablePriorityQueueNode")]
impl crate::Priority_Queue::StablePriorityQueueNode {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InsertionIndex(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_InsertionIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_InsertionIndex(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InsertionIndex", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Priority_Queue+StablePriorityQueueNode")]
impl quest_hook::libil2cpp::ObjectType
for crate::Priority_Queue::StablePriorityQueueNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
