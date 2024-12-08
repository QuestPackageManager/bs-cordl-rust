#[cfg(feature = "System+Threading+Tasks+TaskSchedulerAwaitTaskContinuation")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskSchedulerAwaitTaskContinuation {
    __cordl_parent: crate::System::Threading::Tasks::AwaitTaskContinuation,
    pub m_scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
}
#[cfg(feature = "System+Threading+Tasks+TaskSchedulerAwaitTaskContinuation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::TaskSchedulerAwaitTaskContinuation =>
    "System.Threading.Tasks"."TaskSchedulerAwaitTaskContinuation"
);
#[cfg(feature = "System+Threading+Tasks+TaskSchedulerAwaitTaskContinuation")]
impl std::ops::Deref
for crate::System::Threading::Tasks::TaskSchedulerAwaitTaskContinuation {
    type Target = crate::System::Threading::Tasks::AwaitTaskContinuation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskSchedulerAwaitTaskContinuation")]
impl std::ops::DerefMut
for crate::System::Threading::Tasks::TaskSchedulerAwaitTaskContinuation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskSchedulerAwaitTaskContinuation")]
impl crate::System::Threading::Tasks::TaskSchedulerAwaitTaskContinuation {
    #[cfg(feature = "System+Threading+Tasks+TaskSchedulerAwaitTaskContinuation+__c")]
    pub type __c = crate::System::Threading::Tasks::TaskSchedulerAwaitTaskContinuation___c;
    pub fn Run(
        &mut self,
        ignored: *mut crate::System::Threading::Tasks::Task,
        canInlineContinuationTask: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Run", (ignored, canInlineContinuationTask))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
        action: *mut crate::System::Action,
        flowExecutionContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (scheduler, action, flowExecutionContext))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
        action: *mut crate::System::Action,
        flowExecutionContext: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scheduler, action, flowExecutionContext))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskSchedulerAwaitTaskContinuation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::TaskSchedulerAwaitTaskContinuation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
