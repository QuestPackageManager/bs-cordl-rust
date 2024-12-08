#[cfg(feature = "System+Threading+Tasks+AwaitTaskContinuation")]
#[repr(C)]
#[derive(Debug)]
pub struct AwaitTaskContinuation {
    __cordl_parent: crate::System::Threading::Tasks::TaskContinuation,
    pub m_capturedContext: *mut crate::System::Threading::ExecutionContext,
    pub m_action: *mut crate::System::Action,
}
#[cfg(feature = "System+Threading+Tasks+AwaitTaskContinuation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::AwaitTaskContinuation
    => "System.Threading.Tasks"."AwaitTaskContinuation"
);
#[cfg(feature = "System+Threading+Tasks+AwaitTaskContinuation")]
impl std::ops::Deref for crate::System::Threading::Tasks::AwaitTaskContinuation {
    type Target = crate::System::Threading::Tasks::TaskContinuation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+AwaitTaskContinuation")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::AwaitTaskContinuation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+AwaitTaskContinuation")]
impl crate::System::Threading::Tasks::AwaitTaskContinuation {
    pub fn CreateTask(
        &mut self,
        action: *mut crate::System::Action_1<*mut crate::System::Object>,
        state: *mut crate::System::Object,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("CreateTask", (action, state, scheduler))?;
        Ok(__cordl_ret)
    }
    pub fn MarkAborted(
        &mut self,
        e: *mut crate::System::Threading::ThreadAbortException,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkAborted", (e))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        action: *mut crate::System::Action,
        flowExecutionContext: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, flowExecutionContext))?;
        Ok(__cordl_object)
    }
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
    pub fn RunCallback(
        &mut self,
        callback: *mut crate::System::Threading::ContextCallback,
        state: *mut crate::System::Object,
        currentTask: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Threading::Tasks::Task,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RunCallback", (callback, state, currentTask))?;
        Ok(__cordl_ret)
    }
    pub fn System_Threading_IThreadPoolWorkItem_ExecuteWorkItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Threading.IThreadPoolWorkItem.ExecuteWorkItem", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        action: *mut crate::System::Action,
        flowExecutionContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action, flowExecutionContext))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+Tasks+AwaitTaskContinuation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::AwaitTaskContinuation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
