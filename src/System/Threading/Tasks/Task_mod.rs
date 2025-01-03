#[cfg(feature = "System+Threading+Tasks+Task")]
#[repr(C)]
#[derive(Debug)]
pub struct Task {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_taskId: i32,
    pub m_action: *mut crate::System::Delegate,
    pub m_stateObject: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_taskScheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
    pub m_parent: *mut crate::System::Threading::Tasks::Task,
    pub m_stateFlags: i32,
    pub m_continuationObject: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_contingentProperties: *mut crate::System::Threading::Tasks::Task_ContingentProperties,
}
#[cfg(feature = "System+Threading+Tasks+Task")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::Task =>
    "System.Threading.Tasks"."Task"
);
#[cfg(feature = "System+Threading+Tasks+Task")]
impl std::ops::Deref for crate::System::Threading::Tasks::Task {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::Task {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task")]
impl crate::System::Threading::Tasks::Task {
    pub const CANCELLATION_REQUESTED: i32 = 1i32;
    pub const OptionsMask: i32 = 65535i32;
    pub const TASK_STATE_CANCELED: i32 = 4194304i32;
    pub const TASK_STATE_CANCELLATIONACKNOWLEDGED: i32 = 1048576i32;
    pub const TASK_STATE_COMPLETED_MASK: i32 = 23068672i32;
    pub const TASK_STATE_COMPLETION_RESERVED: i32 = 67108864i32;
    pub const TASK_STATE_DELEGATE_INVOKED: i32 = 131072i32;
    pub const TASK_STATE_DISPOSED: i32 = 262144i32;
    pub const TASK_STATE_EXCEPTIONOBSERVEDBYPARENT: i32 = 524288i32;
    pub const TASK_STATE_FAULTED: i32 = 2097152i32;
    pub const TASK_STATE_RAN_TO_COMPLETION: i32 = 16777216i32;
    pub const TASK_STATE_STARTED: i32 = 65536i32;
    pub const TASK_STATE_THREAD_WAS_ABORTED: i32 = 134217728i32;
    pub const TASK_STATE_WAITINGFORACTIVATION: i32 = 33554432i32;
    pub const TASK_STATE_WAITING_ON_CHILDREN: i32 = 8388608i32;
    pub const TASK_STATE_WAIT_COMPLETION_NOTIFICATION: i32 = 268435456i32;
    #[cfg(feature = "System+Threading+Tasks+Task+ContingentProperties")]
    pub type ContingentProperties = crate::System::Threading::Tasks::Task_ContingentProperties;
    #[cfg(feature = "System+Threading+Tasks+Task+DelayPromise")]
    pub type DelayPromise = crate::GlobalNamespace::Task_DelayPromise;
    #[cfg(feature = "System+Threading+Tasks+Task+SetOnInvokeMres")]
    pub type SetOnInvokeMres = crate::System::Threading::Tasks::Task_SetOnInvokeMres;
    #[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise")]
    pub type WhenAllPromise = crate::GlobalNamespace::Task_WhenAllPromise;
    #[cfg(feature = "System+Threading+Tasks+Task+WhenAllPromise_1")]
    pub type WhenAllPromise_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::Task_WhenAllPromise_1<
        T,
    >;
    pub fn AddCompletionAction_ITaskCompletionAction0(
        &mut self,
        action: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::ITaskCompletionAction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCompletionAction", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCompletionAction__cordl_bool1(
        &mut self,
        action: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::ITaskCompletionAction,
        >,
        addBeforeOthers: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCompletionAction", (action, addBeforeOthers))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddException_Il2CppObject0(
        &mut self,
        exceptionObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddException", (exceptionObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddException__cordl_bool1(
        &mut self,
        exceptionObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        representsCancellation: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddException", (exceptionObject, representsCancellation))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddExceptionsFromChildren(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddExceptionsFromChildren", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AddNewChild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNewChild", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTaskContinuation(
        &mut self,
        tc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        addBeforeOthers: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddTaskContinuation", (tc, addBeforeOthers))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddTaskContinuationComplex(
        &mut self,
        tc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        addBeforeOthers: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddTaskContinuationComplex", (tc, addBeforeOthers))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddToActiveTasks(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddToActiveTasks", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn AnyTaskRequiresNotifyDebuggerOfWaitCompletion(
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Threading::Tasks::Task,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AnyTaskRequiresNotifyDebuggerOfWaitCompletion", (tasks))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssignCancellationToken(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
        antecedent: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        continuation: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskContinuation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AssignCancellationToken",
                (cancellationToken, antecedent, continuation),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AtomicStateUpdate_ByRefMut1(
        &mut self,
        newBits: i32,
        illegalBits: i32,
        oldFlags: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AtomicStateUpdate", (newBits, illegalBits, oldFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn AtomicStateUpdate_i32_i32_0(
        &mut self,
        newBits: i32,
        illegalBits: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AtomicStateUpdate", (newBits, illegalBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn CancellationCleanupLogic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancellationCleanupLogic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureAwait(
        &mut self,
        continueOnCapturedContext: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable = __cordl_object
            .invoke("ConfigureAwait", (continueOnCapturedContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContinueWithCore(
        &mut self,
        continuationTask: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        >,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
        options: crate::System::Threading::Tasks::TaskContinuationOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ContinueWithCore",
                (continuationTask, scheduler, cancellationToken, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ContinueWith_Action_1_0(
        &mut self,
        continuationAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::System::Threading::Tasks::Task>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ContinueWith", (continuationAction))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContinueWith_Action_1_TaskScheduler_CancellationToken_TaskContinuationOptions1(
        &mut self,
        continuationAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::System::Threading::Tasks::Task>,
        >,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke(
                "ContinueWith",
                (continuationAction, scheduler, cancellationToken, continuationOptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ContinueWith_Action_2_Il2CppObject_CancellationToken_TaskContinuationOptions_TaskScheduler2(
        &mut self,
        continuationAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::System::Threading::Tasks::Task,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cancellationToken: crate::System::Threading::CancellationToken,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
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
        > = __cordl_object
            .invoke(
                "ContinueWith",
                (
                    continuationAction,
                    state,
                    cancellationToken,
                    continuationOptions,
                    scheduler,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ContinueWith_Action_2_Il2CppObject_TaskScheduler_CancellationToken_TaskContinuationOptions3(
        &mut self,
        continuationAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::System::Threading::Tasks::Task,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke(
                "ContinueWith",
                (
                    continuationAction,
                    state,
                    scheduler,
                    cancellationToken,
                    continuationOptions,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUnwrapPromise<TResult>(
        outerTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        lookForOce: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateUnwrapPromise", (outerTask, lookForOce))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreationOptionsFromContinuationOptions(
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
        creationOptions: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::Tasks::TaskCreationOptions,
        >,
        internalOptions: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::Tasks::InternalTaskOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreationOptionsFromContinuationOptions",
                (continuationOptions, creationOptions, internalOptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Delay_TimeSpan_CancellationToken0(
        delay: crate::System::TimeSpan,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Delay", (delay, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Delay_i32_1(
        millisecondsDelay: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Delay", (millisecondsDelay))?;
        Ok(__cordl_ret.into())
    }
    pub fn Delay_i32_CancellationToken2(
        millisecondsDelay: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Delay", (millisecondsDelay, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisregardChild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisregardChild", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureContingentPropertiesInitialized(
        &mut self,
        needsProtection: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_ContingentProperties,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_ContingentProperties,
        > = __cordl_object
            .invoke("EnsureContingentPropertiesInitialized", (needsProtection))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureContingentPropertiesInitializedCore(
        &mut self,
        needsProtection: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_ContingentProperties,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_ContingentProperties,
        > = __cordl_object
            .invoke("EnsureContingentPropertiesInitializedCore", (needsProtection))?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Execute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteEntry(
        &mut self,
        bPreventDoubleExecution: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ExecuteEntry", (bPreventDoubleExecution))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteWithThreadLocal(
        &mut self,
        currentTaskSlot: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Threading::Tasks::Task,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteWithThreadLocal", (currentTaskSlot))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecutionContextCallback(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExecutionContextCallback", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn Finish(
        &mut self,
        bUserDelegateExecuted: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", (bUserDelegateExecuted))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinishContinuations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishContinuations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FinishStageThree(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishStageThree", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FinishStageTwo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishStageTwo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromCanceled_CancellationToken0(
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromCanceled", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromCanceled_CancellationToken1<TResult>(
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromCanceled", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromCancellation_CancellationToken0(
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromCancellation", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromCancellation_CancellationToken1<TResult>(
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromCancellation", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromCancellation_OperationCanceledException2<TResult>(
        exception: quest_hook::libil2cpp::Gc<crate::System::OperationCanceledException>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromCancellation", (exception))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromException_Exception0(
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromException", (exception))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromException_Exception1<TResult>(
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromException", (exception))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromResult<TResult>(
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromResult", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAwaiter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::TaskAwaiter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter = __cordl_object
            .invoke("GetAwaiter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCancellationExceptionDispatchInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        > = __cordl_object.invoke("GetCancellationExceptionDispatchInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExceptionDispatchInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
            >,
        > = __cordl_object.invoke("GetExceptionDispatchInfos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExceptions(
        &mut self,
        includeTaskCanceledExceptions: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::AggregateException>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::AggregateException> = __cordl_object
            .invoke("GetExceptions", (includeTaskCanceledExceptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleException(
        &mut self,
        unhandledException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleException", (unhandledException))?;
        Ok(__cordl_ret.into())
    }
    pub fn InnerInvoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InnerInvoke", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalCancel(
        &mut self,
        bCancelNonExecutingOnly: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalCancel", (bCancelNonExecutingOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalCurrentIfAttached(
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalCurrentIfAttached", (creationOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalStartNew(
        creatingTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        action: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cancellationToken: crate::System::Threading::CancellationToken,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
        options: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InternalStartNew",
                (
                    creatingTask,
                    action,
                    state,
                    cancellationToken,
                    scheduler,
                    options,
                    internalOptions,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWait(
        &mut self,
        millisecondsTimeout: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalWait", (millisecondsTimeout, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWhenAll_Il2CppArray0(
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Threading::Tasks::Task,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalWhenAll", (tasks))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWhenAll_Il2CppArray1<TResult>(
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Threading::Tasks::Task_1<TResult>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppArray<TResult>,
            >,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppArray<TResult>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalWhenAll", (tasks))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCompletedMethod(flags: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCompletedMethod", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogFinishCompletionNotification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogFinishCompletionNotification", ())?;
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
    pub fn MarkStarted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MarkStarted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Action_CancellationToken3(
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, cancellationToken))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Delegate_Il2CppObject_Task_CancellationToken_TaskCreationOptions_InternalTaskOptions_TaskScheduler4(
        action: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        parent: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    action,
                    state,
                    parent,
                    cancellationToken,
                    creationOptions,
                    internalOptions,
                    scheduler,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppObject_TaskCreationOptions__cordl_bool2(
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        promiseStyle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (state, creationOptions, promiseStyle))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_TaskCreationOptions_CancellationToken0(
        canceled: bool,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        ct: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (canceled, creationOptions, ct))?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyDebuggerOfWaitCompletion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyDebuggerOfWaitCompletion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyDebuggerOfWaitCompletionIfNecessary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("NotifyDebuggerOfWaitCompletionIfNecessary", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OptionsMethod(
        flags: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::Tasks::TaskCreationOptions,
    > {
        let __cordl_ret: crate::System::Threading::Tasks::TaskCreationOptions = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OptionsMethod", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessChildCompletion(
        &mut self,
        childTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessChildCompletion", (childTask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordInternalCancellationRequest_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecordInternalCancellationRequest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordInternalCancellationRequest_CancellationToken1(
        &mut self,
        tokenToRecord: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecordInternalCancellationRequest", (tokenToRecord))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecordInternalCancellationRequest_CancellationToken_Il2CppObject2(
        &mut self,
        tokenToRecord: crate::System::Threading::CancellationToken,
        cancellationException: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RecordInternalCancellationRequest",
                (tokenToRecord, cancellationException),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveContinuation(
        &mut self,
        continuationObject: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveContinuation", (continuationObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFromActiveTasks(
        taskId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveFromActiveTasks", (taskId))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run_Action0(
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Run", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run_Action_CancellationToken1(
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Run", (action, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run_Func_1_2<TResult>(
        function: quest_hook::libil2cpp::Gc<crate::System::Func_1<TResult>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Run", (function))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run_Func_1_4(
        function: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::System::Threading::Tasks::Task>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Run", (function))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run_Func_1_6<TResult>(
        function: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::System::Threading::Tasks::Task_1<TResult>>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Run", (function))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run_Func_1_CancellationToken3<TResult>(
        function: quest_hook::libil2cpp::Gc<crate::System::Func_1<TResult>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Run", (function, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run_Func_1_CancellationToken5(
        function: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::System::Threading::Tasks::Task>,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Run", (function, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run_Func_1_CancellationToken7<TResult>(
        function: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::System::Threading::Tasks::Task_1<TResult>>,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Run", (function, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleAndStart(
        &mut self,
        needsProtection: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScheduleAndStart", (needsProtection))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCancellationAcknowledged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCancellationAcknowledged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetContinuationForAwait(
        &mut self,
        continuationAction: quest_hook::libil2cpp::Gc<crate::System::Action>,
        continueOnCapturedContext: bool,
        flowExecutionContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetContinuationForAwait",
                (continuationAction, continueOnCapturedContext, flowExecutionContext),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNotificationForWaitCompletion(
        &mut self,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNotificationForWaitCompletion", (enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpinThenBlockingWait(
        &mut self,
        millisecondsTimeout: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SpinThenBlockingWait", (millisecondsTimeout, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpinWait(
        &mut self,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SpinWait", (millisecondsTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IAsyncResult_get_AsyncWaitHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::WaitHandle,
        > = __cordl_object.invoke("System.IAsyncResult.get_AsyncWaitHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IAsyncResult_get_CompletedSynchronously(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.IAsyncResult.get_CompletedSynchronously", ())?;
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
    pub fn TaskCancelCallback(
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TaskCancelCallback", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn TaskConstructorCore(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TaskConstructorCore",
                (
                    action,
                    state,
                    cancellationToken,
                    creationOptions,
                    internalOptions,
                    scheduler,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowIfExceptional(
        &mut self,
        includeTaskCanceledExceptions: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfExceptional", (includeTaskCanceledExceptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySetCanceled_CancellationToken0(
        &mut self,
        tokenToRecord: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TrySetCanceled", (tokenToRecord))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySetCanceled_Il2CppObject1(
        &mut self,
        tokenToRecord: crate::System::Threading::CancellationToken,
        cancellationException: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TrySetCanceled", (tokenToRecord, cancellationException))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySetException(
        &mut self,
        exceptionObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TrySetException", (exceptionObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateExceptionObservedStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateExceptionObservedStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Wait_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Wait", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Wait_i32_CancellationToken1(
        &mut self,
        millisecondsTimeout: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Wait", (millisecondsTimeout, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WhenAll_IEnumerable_1_0(
        tasks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Threading::Tasks::Task,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("WhenAll", (tasks))?;
        Ok(__cordl_ret.into())
    }
    pub fn WhenAll_IEnumerable_1_2<TResult>(
        tasks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Threading::Tasks::Task_1<TResult>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppArray<TResult>,
            >,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppArray<TResult>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("WhenAll", (tasks))?;
        Ok(__cordl_ret.into())
    }
    pub fn WhenAll_Il2CppArray1(
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Threading::Tasks::Task,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("WhenAll", (tasks))?;
        Ok(__cordl_ret.into())
    }
    pub fn WhenAll_Il2CppArray3<TResult>(
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Threading::Tasks::Task_1<TResult>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppArray<TResult>,
            >,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppArray<TResult>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("WhenAll", (tasks))?;
        Ok(__cordl_ret.into())
    }
    pub fn WhenAny_IEnumerable_1_1(
        tasks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Threading::Tasks::Task,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Threading::Tasks::Task,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Threading::Tasks::Task,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("WhenAny", (tasks))?;
        Ok(__cordl_ret.into())
    }
    pub fn WhenAny_IEnumerable_1_3<TResult>(
        tasks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Threading::Tasks::Task_1<TResult>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Threading::Tasks::Task_1<TResult>,
            >,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Threading::Tasks::Task_1<TResult>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("WhenAny", (tasks))?;
        Ok(__cordl_ret.into())
    }
    pub fn WhenAny_Il2CppArray0(
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Threading::Tasks::Task,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Threading::Tasks::Task,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Threading::Tasks::Task,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("WhenAny", (tasks))?;
        Ok(__cordl_ret.into())
    }
    pub fn WhenAny_Il2CppArray2<TResult>(
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Threading::Tasks::Task_1<TResult>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Threading::Tasks::Task_1<TResult>,
            >,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Threading::Tasks::Task_1<TResult>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("WhenAny", (tasks))?;
        Ok(__cordl_ret.into())
    }
    pub fn WrappedTryRunInline(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WrappedTryRunInline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Action_CancellationToken3(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Delegate_Il2CppObject_Task_CancellationToken_TaskCreationOptions_InternalTaskOptions_TaskScheduler4(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        parent: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    action,
                    state,
                    parent,
                    cancellationToken,
                    creationOptions,
                    internalOptions,
                    scheduler,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_TaskCreationOptions__cordl_bool2(
        &mut self,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        promiseStyle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (state, creationOptions, promiseStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_TaskCreationOptions_CancellationToken0(
        &mut self,
        canceled: bool,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        ct: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (canceled, creationOptions, ct))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AsyncState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_AsyncState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CancellationToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::CancellationToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::CancellationToken = __cordl_object
            .invoke("get_CancellationToken", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CapturedContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        > = __cordl_object.invoke("get_CapturedContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompletedEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::ManualResetEventSlim>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ManualResetEventSlim,
        > = __cordl_object.invoke("get_CompletedEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CompletedTask() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CompletedTask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CreationOptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::Tasks::TaskCreationOptions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::TaskCreationOptions = __cordl_object
            .invoke("get_CreationOptions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentStackGuard() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::StackGuard>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::StackGuard,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentStackGuard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Exception(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::AggregateException>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::AggregateException> = __cordl_object
            .invoke("get_Exception", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExceptionRecorded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ExceptionRecorded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExecutingTaskScheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::TaskScheduler>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        > = __cordl_object.invoke("get_ExecutingTaskScheduler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Factory() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::TaskFactory>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskFactory,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Factory", ())?;
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
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InternalCurrent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCanceled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCanceled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCancellationAcknowledged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsCancellationAcknowledged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCancellationRequested(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsCancellationRequested", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCompleted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCompletedSuccessfully(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsCompletedSuccessfully", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDelegateInvoked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDelegateInvoked", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsExceptionObservedByParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsExceptionObservedByParent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFaulted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsFaulted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsWaitNotificationEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsWaitNotificationEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsWaitNotificationEnabledOrNotRanToCompletion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsWaitNotificationEnabledOrNotRanToCompletion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Options(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::Tasks::TaskCreationOptions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::TaskCreationOptions = __cordl_object
            .invoke("get_Options", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShouldNotifyDebuggerOfWaitCompletion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ShouldNotifyDebuggerOfWaitCompletion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::TaskStatus> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::TaskStatus = __cordl_object
            .invoke("get_Status", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CapturedContext(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CapturedContext", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+Task")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::Tasks::Task {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+Task")]
impl AsRef<crate::System::IAsyncResult> for crate::System::Threading::Tasks::Task {
    fn as_ref(&self) -> &crate::System::IAsyncResult {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task")]
impl AsMut<crate::System::IAsyncResult> for crate::System::Threading::Tasks::Task {
    fn as_mut(&mut self) -> &mut crate::System::IAsyncResult {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task")]
impl AsRef<crate::System::IDisposable> for crate::System::Threading::Tasks::Task {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task")]
impl AsMut<crate::System::IDisposable> for crate::System::Threading::Tasks::Task {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task")]
impl AsRef<crate::System::Threading::IThreadPoolWorkItem>
for crate::System::Threading::Tasks::Task {
    fn as_ref(&self) -> &crate::System::Threading::IThreadPoolWorkItem {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task")]
impl AsMut<crate::System::Threading::IThreadPoolWorkItem>
for crate::System::Threading::Tasks::Task {
    fn as_mut(&mut self) -> &mut crate::System::Threading::IThreadPoolWorkItem {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+ContingentProperties")]
#[repr(C)]
#[derive(Debug)]
pub struct Task_ContingentProperties {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_capturedContext: *mut crate::System::Threading::ExecutionContext,
    pub m_completionEvent: *mut crate::System::Threading::ManualResetEventSlim,
    pub m_exceptionsHolder: *mut crate::System::Threading::Tasks::TaskExceptionHolder,
    pub m_cancellationToken: crate::System::Threading::CancellationToken,
    pub m_cancellationRegistration: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_internalCancellationRequested: i32,
    pub m_completionCountdown: i32,
    pub m_exceptionalChildren: *mut crate::System::Collections::Generic::LowLevelListWithIList_1<
        *mut crate::System::Threading::Tasks::Task,
    >,
}
#[cfg(feature = "System+Threading+Tasks+Task+ContingentProperties")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::Task_ContingentProperties => "System.Threading.Tasks"
    ."Task/ContingentProperties"
);
#[cfg(feature = "System+Threading+Tasks+Task+ContingentProperties")]
impl std::ops::Deref for crate::System::Threading::Tasks::Task_ContingentProperties {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+ContingentProperties")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::Task_ContingentProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+ContingentProperties")]
impl crate::System::Threading::Tasks::Task_ContingentProperties {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCompleted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterCancellationCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterCancellationCallback", ())?;
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
}
#[cfg(feature = "System+Threading+Tasks+Task+ContingentProperties")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::Task_ContingentProperties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+SetOnInvokeMres")]
#[repr(C)]
#[derive(Debug)]
pub struct Task_SetOnInvokeMres {
    __cordl_parent: crate::System::Threading::ManualResetEventSlim,
}
#[cfg(feature = "System+Threading+Tasks+Task+SetOnInvokeMres")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::Task_SetOnInvokeMres
    => "System.Threading.Tasks"."Task/SetOnInvokeMres"
);
#[cfg(feature = "System+Threading+Tasks+Task+SetOnInvokeMres")]
impl std::ops::Deref for crate::System::Threading::Tasks::Task_SetOnInvokeMres {
    type Target = crate::System::Threading::ManualResetEventSlim;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+SetOnInvokeMres")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::Task_SetOnInvokeMres {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+SetOnInvokeMres")]
impl crate::System::Threading::Tasks::Task_SetOnInvokeMres {
    pub fn Invoke(
        &mut self,
        completingTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (completingTask))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_InvokeMayRunArbitraryCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_InvokeMayRunArbitraryCode", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+SetOnInvokeMres")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::Task_SetOnInvokeMres {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+SetOnInvokeMres")]
impl AsRef<crate::System::Threading::Tasks::ITaskCompletionAction>
for crate::System::Threading::Tasks::Task_SetOnInvokeMres {
    fn as_ref(&self) -> &crate::System::Threading::Tasks::ITaskCompletionAction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task+SetOnInvokeMres")]
impl AsMut<crate::System::Threading::Tasks::ITaskCompletionAction>
for crate::System::Threading::Tasks::Task_SetOnInvokeMres {
    fn as_mut(&mut self) -> &mut crate::System::Threading::Tasks::ITaskCompletionAction {
        unsafe { std::mem::transmute(self) }
    }
}
