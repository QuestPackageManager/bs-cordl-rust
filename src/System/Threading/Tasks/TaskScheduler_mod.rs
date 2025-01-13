#[cfg(feature = "System+Threading+Tasks+TaskScheduler")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskScheduler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_taskSchedulerId: i32,
}
#[cfg(feature = "System+Threading+Tasks+TaskScheduler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::TaskScheduler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "TaskScheduler";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskScheduler")]
impl std::ops::Deref for crate::System::Threading::Tasks::TaskScheduler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyWorkItemProgress(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyWorkItemProgress", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PublishUnobservedTaskException(
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ueea: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PublishUnobservedTaskException", (sender, ueea))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueTask(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueTask", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryDequeue(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryDequeue", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryExecuteTaskInline(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        taskWasPreviouslyQueued: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryExecuteTaskInline", (task, taskWasPreviouslyQueued))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryRunInline(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        taskWasPreviouslyQueued: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryRunInline", (task, taskWasPreviouslyQueued))?;
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
    pub fn get_Current() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::TaskScheduler>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Default() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::TaskScheduler>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Default", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Id", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalCurrent() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::TaskScheduler>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InternalCurrent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RequiresAtomicStartTransition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_RequiresAtomicStartTransition", ())?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "System+Threading+Tasks+TaskScheduler+SystemThreadingTasks_TaskSchedulerDebugView"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::TaskScheduler_SystemThreadingTasks_TaskSchedulerDebugView {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "TaskScheduler/SystemThreadingTasks_TaskSchedulerDebugView";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(
    feature = "System+Threading+Tasks+TaskScheduler+SystemThreadingTasks_TaskSchedulerDebugView"
)]
impl std::ops::Deref
for crate::System::Threading::Tasks::TaskScheduler_SystemThreadingTasks_TaskSchedulerDebugView {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
