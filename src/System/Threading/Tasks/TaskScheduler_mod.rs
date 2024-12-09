#[cfg(feature = "System+Threading+Tasks+TaskScheduler")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskScheduler {
    __cordl_parent: crate::System::Object,
    pub m_taskSchedulerId: i32,
}
#[cfg(feature = "System+Threading+Tasks+TaskScheduler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::TaskScheduler =>
    "System.Threading.Tasks"."TaskScheduler"
);
#[cfg(feature = "System+Threading+Tasks+TaskScheduler")]
impl std::ops::Deref for crate::System::Threading::Tasks::TaskScheduler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskScheduler")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::TaskScheduler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskScheduler")]
impl crate::System::Threading::Tasks::TaskScheduler {
    #[cfg(
        feature = "System+Threading+Tasks+TaskScheduler+SystemThreadingTasks_TaskSchedulerDebugView"
    )]
    pub type SystemThreadingTasks_TaskSchedulerDebugView = crate::System::Threading::Tasks::TaskScheduler_SystemThreadingTasks_TaskSchedulerDebugView;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn TryRunInline(
        &mut self,
        task: *mut crate::System::Threading::Tasks::Task,
        taskWasPreviouslyQueued: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryRunInline", (task, taskWasPreviouslyQueued))?;
        Ok(__cordl_ret)
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
    pub fn get_Id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Id", ())?;
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
}
#[cfg(feature = "System+Threading+Tasks+TaskScheduler")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::TaskScheduler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+Threading+Tasks+TaskScheduler+SystemThreadingTasks_TaskSchedulerDebugView"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TaskScheduler_SystemThreadingTasks_TaskSchedulerDebugView {
    __cordl_parent: crate::System::Object,
}
#[cfg(
    feature = "System+Threading+Tasks+TaskScheduler+SystemThreadingTasks_TaskSchedulerDebugView"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::TaskScheduler_SystemThreadingTasks_TaskSchedulerDebugView
    => "System.Threading.Tasks"
    ."TaskScheduler/SystemThreadingTasks_TaskSchedulerDebugView"
);
#[cfg(
    feature = "System+Threading+Tasks+TaskScheduler+SystemThreadingTasks_TaskSchedulerDebugView"
)]
impl std::ops::Deref
for crate::System::Threading::Tasks::TaskScheduler_SystemThreadingTasks_TaskSchedulerDebugView {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Threading+Tasks+TaskScheduler+SystemThreadingTasks_TaskSchedulerDebugView"
)]
impl std::ops::DerefMut
for crate::System::Threading::Tasks::TaskScheduler_SystemThreadingTasks_TaskSchedulerDebugView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Threading+Tasks+TaskScheduler+SystemThreadingTasks_TaskSchedulerDebugView"
)]
impl crate::System::Threading::Tasks::TaskScheduler_SystemThreadingTasks_TaskSchedulerDebugView {}
#[cfg(
    feature = "System+Threading+Tasks+TaskScheduler+SystemThreadingTasks_TaskSchedulerDebugView"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::TaskScheduler_SystemThreadingTasks_TaskSchedulerDebugView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
