#[cfg(feature = "System+Threading+Tasks+Task+ContingentProperties")]
#[repr(C)]
#[derive(Debug)]
pub struct Task_ContingentProperties {
    __cordl_parent: crate::System::Object,
    pub m_capturedContext: *mut crate::System::Threading::ExecutionContext,
    pub m_completionEvent: *mut crate::System::Threading::ManualResetEventSlim,
    pub m_exceptionsHolder: *mut crate::System::Threading::Tasks::TaskExceptionHolder,
    pub m_cancellationToken: crate::System::Threading::CancellationToken,
    pub m_cancellationRegistration: *mut crate::System::Object,
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
    type Target = crate::System::Object;
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCompleted", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterCancellationCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterCancellationCallback", ())?;
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
        completingTask: *mut crate::System::Threading::Tasks::Task,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (completingTask))?;
        Ok(__cordl_ret)
    }
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
    pub fn get_InvokeMayRunArbitraryCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_InvokeMayRunArbitraryCode", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "System+Threading+Tasks+Task")]
#[repr(C)]
#[derive(Debug)]
pub struct Task {
    __cordl_parent: crate::System::Object,
    pub m_taskId: i32,
    pub m_action: *mut crate::System::Delegate,
    pub m_stateObject: *mut crate::System::Object,
    pub m_taskScheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
    pub m_parent: *mut crate::System::Threading::Tasks::Task,
    pub m_stateFlags: i32,
    pub m_continuationObject: *mut crate::System::Object,
    pub m_contingentProperties: *mut crate::System::Threading::Tasks::Task_ContingentProperties,
}
#[cfg(feature = "System+Threading+Tasks+Task")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::Task =>
    "System.Threading.Tasks"."Task"
);
#[cfg(feature = "System+Threading+Tasks+Task")]
impl std::ops::Deref for crate::System::Threading::Tasks::Task {
    type Target = crate::System::Object;
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
    #[cfg(feature = "System+Threading+Tasks+Task+__c")]
    pub type __c = crate::System::Threading::Tasks::Task___c;
    pub fn AddCompletionAction_ITaskCompletionAction0(
        &mut self,
        action: *mut crate::System::Threading::Tasks::ITaskCompletionAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCompletionAction", (action))?;
        Ok(__cordl_ret)
    }
    pub fn AddCompletionAction__cordl_bool1(
        &mut self,
        action: *mut crate::System::Threading::Tasks::ITaskCompletionAction,
        addBeforeOthers: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCompletionAction", (action, addBeforeOthers))?;
        Ok(__cordl_ret)
    }
    pub fn AddException_Object0(
        &mut self,
        exceptionObject: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddException", (exceptionObject))?;
        Ok(__cordl_ret)
    }
    pub fn AddException__cordl_bool1(
        &mut self,
        exceptionObject: *mut crate::System::Object,
        representsCancellation: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddException", (exceptionObject, representsCancellation))?;
        Ok(__cordl_ret)
    }
    pub fn AddExceptionsFromChildren(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddExceptionsFromChildren", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddNewChild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNewChild", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddTaskContinuation(
        &mut self,
        tc: *mut crate::System::Object,
        addBeforeOthers: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddTaskContinuation", (tc, addBeforeOthers))?;
        Ok(__cordl_ret)
    }
    pub fn AddTaskContinuationComplex(
        &mut self,
        tc: *mut crate::System::Object,
        addBeforeOthers: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddTaskContinuationComplex", (tc, addBeforeOthers))?;
        Ok(__cordl_ret)
    }
    pub fn AssignCancellationToken(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
        antecedent: *mut crate::System::Threading::Tasks::Task,
        continuation: *mut crate::System::Threading::Tasks::TaskContinuation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AssignCancellationToken",
                (cancellationToken, antecedent, continuation),
            )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn CancellationCleanupLogic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancellationCleanupLogic", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ContinueWithCore(
        &mut self,
        continuationTask: *mut crate::System::Threading::Tasks::Task,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
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
        Ok(__cordl_ret)
    }
    pub fn ContinueWith_Action_1_0(
        &mut self,
        continuationAction: *mut crate::System::Action_1<
            *mut crate::System::Threading::Tasks::Task,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("ContinueWith", (continuationAction))?;
        Ok(__cordl_ret)
    }
    pub fn ContinueWith_Action_1_TaskScheduler_CancellationToken_TaskContinuationOptions1(
        &mut self,
        continuationAction: *mut crate::System::Action_1<
            *mut crate::System::Threading::Tasks::Task,
        >,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
        cancellationToken: crate::System::Threading::CancellationToken,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke(
                "ContinueWith",
                (continuationAction, scheduler, cancellationToken, continuationOptions),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ContinueWith_Action_2_Object_CancellationToken_TaskContinuationOptions_TaskScheduler2(
        &mut self,
        continuationAction: *mut crate::System::Action_2<
            *mut crate::System::Threading::Tasks::Task,
            *mut crate::System::Object,
        >,
        state: *mut crate::System::Object,
        cancellationToken: crate::System::Threading::CancellationToken,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
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
        Ok(__cordl_ret)
    }
    pub fn ContinueWith_Action_2_Object_TaskScheduler_CancellationToken_TaskContinuationOptions3(
        &mut self,
        continuationAction: *mut crate::System::Action_2<
            *mut crate::System::Threading::Tasks::Task,
            *mut crate::System::Object,
        >,
        state: *mut crate::System::Object,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
        cancellationToken: crate::System::Threading::CancellationToken,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
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
        Ok(__cordl_ret)
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn DisregardChild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisregardChild", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnsureContingentPropertiesInitialized(
        &mut self,
        needsProtection: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_ContingentProperties,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_ContingentProperties = __cordl_object
            .invoke("EnsureContingentPropertiesInitialized", (needsProtection))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureContingentPropertiesInitializedCore(
        &mut self,
        needsProtection: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_ContingentProperties,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_ContingentProperties = __cordl_object
            .invoke("EnsureContingentPropertiesInitializedCore", (needsProtection))?;
        Ok(__cordl_ret)
    }
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Execute", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn FinishContinuations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishContinuations", ())?;
        Ok(__cordl_ret)
    }
    pub fn FinishStageThree(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishStageThree", ())?;
        Ok(__cordl_ret)
    }
    pub fn FinishStageTwo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishStageTwo", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetCancellationExceptionDispatchInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo = __cordl_object
            .invoke("GetCancellationExceptionDispatchInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetExceptionDispatchInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
        > = __cordl_object.invoke("GetExceptionDispatchInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetExceptions(
        &mut self,
        includeTaskCanceledExceptions: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::AggregateException> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::AggregateException = __cordl_object
            .invoke("GetExceptions", (includeTaskCanceledExceptions))?;
        Ok(__cordl_ret)
    }
    pub fn HandleException(
        &mut self,
        unhandledException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleException", (unhandledException))?;
        Ok(__cordl_ret)
    }
    pub fn InnerInvoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InnerInvoke", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn LogFinishCompletionNotification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogFinishCompletionNotification", ())?;
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
    pub fn MarkStarted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MarkStarted", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Action_CancellationToken3(
        action: *mut crate::System::Action,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, cancellationToken))?;
        Ok(__cordl_object)
    }
    pub fn New_Delegate_Object_Task_CancellationToken_TaskCreationOptions_InternalTaskOptions_TaskScheduler4(
        action: *mut crate::System::Delegate,
        state: *mut crate::System::Object,
        parent: *mut crate::System::Threading::Tasks::Task,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
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
        Ok(__cordl_object)
    }
    pub fn New_Object_TaskCreationOptions__cordl_bool2(
        state: *mut crate::System::Object,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        promiseStyle: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (state, creationOptions, promiseStyle))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool_TaskCreationOptions_CancellationToken0(
        canceled: bool,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        ct: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (canceled, creationOptions, ct))?;
        Ok(__cordl_object)
    }
    pub fn NotifyDebuggerOfWaitCompletion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyDebuggerOfWaitCompletion", ())?;
        Ok(__cordl_ret)
    }
    pub fn NotifyDebuggerOfWaitCompletionIfNecessary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("NotifyDebuggerOfWaitCompletionIfNecessary", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessChildCompletion(
        &mut self,
        childTask: *mut crate::System::Threading::Tasks::Task,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessChildCompletion", (childTask))?;
        Ok(__cordl_ret)
    }
    pub fn RecordInternalCancellationRequest_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecordInternalCancellationRequest", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn RecordInternalCancellationRequest_CancellationToken_Object2(
        &mut self,
        tokenToRecord: crate::System::Threading::CancellationToken,
        cancellationException: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RecordInternalCancellationRequest",
                (tokenToRecord, cancellationException),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RemoveContinuation(
        &mut self,
        continuationObject: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveContinuation", (continuationObject))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn SetCancellationAcknowledged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCancellationAcknowledged", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetContinuationForAwait(
        &mut self,
        continuationAction: *mut crate::System::Action,
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn System_IAsyncResult_get_AsyncWaitHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::WaitHandle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::WaitHandle = __cordl_object
            .invoke("System.IAsyncResult.get_AsyncWaitHandle", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_IAsyncResult_get_CompletedSynchronously(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.IAsyncResult.get_CompletedSynchronously", ())?;
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
    pub fn TaskConstructorCore(
        &mut self,
        action: *mut crate::System::Delegate,
        state: *mut crate::System::Object,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn TrySetCanceled_Object1(
        &mut self,
        tokenToRecord: crate::System::Threading::CancellationToken,
        cancellationException: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TrySetCanceled", (tokenToRecord, cancellationException))?;
        Ok(__cordl_ret)
    }
    pub fn TrySetException(
        &mut self,
        exceptionObject: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TrySetException", (exceptionObject))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateExceptionObservedStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateExceptionObservedStatus", ())?;
        Ok(__cordl_ret)
    }
    pub fn Wait_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Wait", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn WrappedTryRunInline(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WrappedTryRunInline", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Action_CancellationToken3(
        &mut self,
        action: *mut crate::System::Action,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Delegate_Object_Task_CancellationToken_TaskCreationOptions_InternalTaskOptions_TaskScheduler4(
        &mut self,
        action: *mut crate::System::Delegate,
        state: *mut crate::System::Object,
        parent: *mut crate::System::Threading::Tasks::Task,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object_TaskCreationOptions__cordl_bool2(
        &mut self,
        state: *mut crate::System::Object,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        promiseStyle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (state, creationOptions, promiseStyle))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_AsyncState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_AsyncState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CancellationToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::CancellationToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::CancellationToken = __cordl_object
            .invoke("get_CancellationToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CapturedContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::ExecutionContext> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::ExecutionContext = __cordl_object
            .invoke("get_CapturedContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CompletedEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::ManualResetEventSlim,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::ManualResetEventSlim = __cordl_object
            .invoke("get_CompletedEvent", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_Exception(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::AggregateException> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::AggregateException = __cordl_object
            .invoke("get_Exception", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ExceptionRecorded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ExceptionRecorded", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ExecutingTaskScheduler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::TaskScheduler,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::TaskScheduler = __cordl_object
            .invoke("get_ExecutingTaskScheduler", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Id", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCanceled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCanceled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCancellationAcknowledged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsCancellationAcknowledged", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCancellationRequested(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsCancellationRequested", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCompleted", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCompletedSuccessfully(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsCompletedSuccessfully", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDelegateInvoked(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDelegateInvoked", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsExceptionObservedByParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsExceptionObservedByParent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsFaulted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsFaulted", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsWaitNotificationEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsWaitNotificationEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsWaitNotificationEnabledOrNotRanToCompletion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsWaitNotificationEnabledOrNotRanToCompletion", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_ShouldNotifyDebuggerOfWaitCompletion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ShouldNotifyDebuggerOfWaitCompletion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::TaskStatus> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::TaskStatus = __cordl_object
            .invoke("get_Status", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CapturedContext(
        &mut self,
        value: *mut crate::System::Threading::ExecutionContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CapturedContext", (value))?;
        Ok(__cordl_ret)
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
