#[cfg(feature = "System+Threading+Tasks+ThreadPoolTaskScheduler")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadPoolTaskScheduler {
    __cordl_parent: crate::System::Threading::Tasks::TaskScheduler,
}
#[cfg(feature = "System+Threading+Tasks+ThreadPoolTaskScheduler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::ThreadPoolTaskScheduler => "System.Threading.Tasks"
    ."ThreadPoolTaskScheduler"
);
#[cfg(feature = "System+Threading+Tasks+ThreadPoolTaskScheduler")]
impl std::ops::Deref for crate::System::Threading::Tasks::ThreadPoolTaskScheduler {
    type Target = crate::System::Threading::Tasks::TaskScheduler;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ThreadPoolTaskScheduler")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::ThreadPoolTaskScheduler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ThreadPoolTaskScheduler")]
impl crate::System::Threading::Tasks::ThreadPoolTaskScheduler {
    #[cfg(feature = "System+Threading+Tasks+ThreadPoolTaskScheduler+__c")]
    pub type __c = crate::System::Threading::Tasks::ThreadPoolTaskScheduler___c;
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
    pub fn NotifyWorkItemProgress(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyWorkItemProgress", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryDequeue(
        &mut self,
        task: *mut crate::System::Threading::Tasks::Task,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryDequeue", (task))?;
        Ok(__cordl_ret)
    }
    pub fn get_RequiresAtomicStartTransition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_RequiresAtomicStartTransition", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryExecuteTaskInline(
        &mut self,
        task: *mut crate::System::Threading::Tasks::Task,
        taskWasPreviouslyQueued: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryExecuteTaskInline", (task, taskWasPreviouslyQueued))?;
        Ok(__cordl_ret)
    }
    pub fn QueueTask(
        &mut self,
        task: *mut crate::System::Threading::Tasks::Task,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueTask", (task))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Threading+Tasks+ThreadPoolTaskScheduler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::ThreadPoolTaskScheduler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
