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
        action: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("CreateTask", (action, state, scheduler))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInvokeActionCallback() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::ContextCallback>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ContextCallback,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInvokeActionCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeAction(
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeAction", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkAborted(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::System::Threading::ThreadAbortException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkAborted", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
        flowExecutionContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, flowExecutionContext))?;
        Ok(__cordl_object.into())
    }
    pub fn Run(
        &mut self,
        ignored: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        canInlineContinuationTask: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Run", (ignored, canInlineContinuationTask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RunCallback(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Threading::ContextCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        currentTask: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Threading::Tasks::Task,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RunCallback", (callback, state, currentTask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RunOrScheduleAction(
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
        allowInlining: bool,
        currentTask: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Threading::Tasks::Task,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RunOrScheduleAction", (action, allowInlining, currentTask))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Threading_IThreadPoolWorkItem_ExecuteWorkItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Threading.IThreadPoolWorkItem.ExecuteWorkItem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowAsyncIfNecessary(
        exc: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowAsyncIfNecessary", (exc))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeScheduleAction(
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeScheduleAction", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
        flowExecutionContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action, flowExecutionContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsValidLocationForInlining() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsValidLocationForInlining", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Threading+Tasks+AwaitTaskContinuation")]
impl AsRef<crate::System::Threading::IThreadPoolWorkItem>
for crate::System::Threading::Tasks::AwaitTaskContinuation {
    fn as_ref(&self) -> &crate::System::Threading::IThreadPoolWorkItem {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+AwaitTaskContinuation")]
impl AsMut<crate::System::Threading::IThreadPoolWorkItem>
for crate::System::Threading::Tasks::AwaitTaskContinuation {
    fn as_mut(&mut self) -> &mut crate::System::Threading::IThreadPoolWorkItem {
        unsafe { std::mem::transmute(self) }
    }
}
