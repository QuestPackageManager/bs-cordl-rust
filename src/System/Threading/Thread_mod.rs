#[cfg(feature = "System+Threading+Thread")]
#[repr(C)]
#[derive(Debug)]
pub struct Thread {
    __cordl_parent: crate::System::Runtime::ConstrainedExecution::CriticalFinalizerObject,
    pub internal_thread: quest_hook::libil2cpp::Gc<
        crate::System::Threading::InternalThread,
    >,
    pub m_ThreadStartArg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub pending_exception: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub m_Delegate: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
    pub m_ExecutionContext: quest_hook::libil2cpp::Gc<
        crate::System::Threading::ExecutionContext,
    >,
    pub m_ExecutionContextBelongsToOuterScope: bool,
    pub principal: quest_hook::libil2cpp::Gc<
        crate::System::Security::Principal::IPrincipal,
    >,
    pub principal_version: i32,
}
#[cfg(feature = "System+Threading+Thread")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::Thread {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "Thread";
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
#[cfg(feature = "System+Threading+Thread")]
impl std::ops::Deref for crate::System::Threading::Thread {
    type Target = crate::System::Runtime::ConstrainedExecution::CriticalFinalizerObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Thread")]
impl std::ops::DerefMut for crate::System::Threading::Thread {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Thread")]
impl crate::System::Threading::Thread {
    pub fn Abort(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Abort", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Abort_internal(
        thread: quest_hook::libil2cpp::Gc<crate::System::Threading::InternalThread>,
        stateInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Abort_internal", (thread, stateInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsyncLocalSetCurrentCulture(
        args: crate::System::Threading::AsyncLocalValueChangedArgs_1<
            quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsyncLocalSetCurrentCulture", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsyncLocalSetCurrentUICulture(
        args: crate::System::Threading::AsyncLocalValueChangedArgs_1<
            quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsyncLocalSetCurrentUICulture", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginCriticalRegion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginCriticalRegion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClrState(
        thread: quest_hook::libil2cpp::Gc<crate::System::Threading::InternalThread>,
        clr: crate::System::Threading::ThreadState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClrState", (thread, clr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConstructInternalThread(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConstructInternalThread", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EndCriticalRegion() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndCriticalRegion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentCultureNoAppX(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = __cordl_object.invoke("GetCurrentCultureNoAppX", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentThread() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Threading::Thread> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentThread", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentThread_icall(
        thread: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentThread_icall", (thread))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentUICultureNoAppX(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = __cordl_object.invoke("GetCurrentUICultureNoAppX", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDomainID() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDomainID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExecutionContextReader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::ExecutionContext_Reader,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::ExecutionContext_Reader = __cordl_object
            .invoke("GetExecutionContextReader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMutableExecutionContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        > = __cordl_object.invoke("GetMutableExecutionContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPriorityNative(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPriorityNative", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProcessDefaultStackSize(
        maxStackSize: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProcessDefaultStackSize", (maxStackSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetState(
        thread: quest_hook::libil2cpp::Gc<crate::System::Threading::InternalThread>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::ThreadState> {
        let __cordl_ret: crate::System::Threading::ThreadState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetState", (thread))?;
        Ok(__cordl_ret.into())
    }
    pub fn Join(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Join", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn JoinInternal(
        &mut self,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("JoinInternal", (millisecondsTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn MemoryBarrier() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MemoryBarrier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_ParameterizedThreadStart1(
        start: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ParameterizedThreadStart,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (start))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ParameterizedThreadStart_i32_2(
        start: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ParameterizedThreadStart,
        >,
        maxStackSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (start, maxStackSize))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ThreadStart0(
        start: quest_hook::libil2cpp::Gc<crate::System::Threading::ThreadStart>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (start))?;
        Ok(__cordl_object.into())
    }
    pub fn SetExecutionContext_ExecutionContext0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
        belongsToCurrentScope: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExecutionContext", (value, belongsToCurrentScope))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetExecutionContext_ExecutionContext_Reader1(
        &mut self,
        value: crate::System::Threading::ExecutionContext_Reader,
        belongsToCurrentScope: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExecutionContext", (value, belongsToCurrentScope))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetName_icall(
        thread: quest_hook::libil2cpp::Gc<crate::System::Threading::InternalThread>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetName_icall", (thread, name, nameLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetName_internal(
        thread: quest_hook::libil2cpp::Gc<crate::System::Threading::InternalThread>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetName_internal", (thread, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPriorityNative(
        &mut self,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPriorityNative", (priority))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStart(
        &mut self,
        start: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
        maxStackSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStart", (start, maxStackSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStartHelper(
        &mut self,
        start: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        maxStackSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStartHelper", (start, maxStackSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetState(
        thread: quest_hook::libil2cpp::Gc<crate::System::Threading::InternalThread>,
        set: crate::System::Threading::ThreadState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetState", (thread, set))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sleep(
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sleep", (millisecondsTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn SleepInternal(
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SleepInternal", (millisecondsTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpinWait(
        iterations: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SpinWait", (iterations))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpinWait_nop() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SpinWait_nop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartInternal(
        &mut self,
        principal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartInternal", (principal, stackMark))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start_ByRefMut2(
        &mut self,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", (stackMark))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start_Il2CppObject1(
        &mut self,
        parameter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", (parameter))?;
        Ok(__cordl_ret.into())
    }
    pub fn SystemMaxStackStize() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SystemMaxStackStize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Thread_internal(
        &mut self,
        start: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Thread_internal", (start))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateThreadState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::ThreadState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::ThreadState = __cordl_object
            .invoke("ValidateThreadState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Yield() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Yield", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn YieldInternal() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("YieldInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ParameterizedThreadStart1(
        &mut self,
        start: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ParameterizedThreadStart,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (start))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ParameterizedThreadStart_i32_2(
        &mut self,
        start: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ParameterizedThreadStart,
        >,
        maxStackSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (start, maxStackSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ThreadStart0(
        &mut self,
        start: quest_hook::libil2cpp::Gc<crate::System::Threading::ThreadStart>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (start))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentContext() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Contexts::Context>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentCulture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = __cordl_object.invoke("get_CurrentCulture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentThread() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Threading::Thread> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentThread", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentThreadId() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CurrentThreadId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurrentUICulture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = __cordl_object.invoke("get_CurrentUICulture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExecutionContextBelongsToCurrentScope(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_ExecutionContextBelongsToCurrentScope", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Internal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::InternalThread>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::InternalThread,
        > = __cordl_object.invoke("get_Internal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAlive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAlive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsThreadPoolThread(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsThreadPoolThread", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsThreadPoolThreadInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsThreadPoolThreadInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ManagedThreadId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ManagedThreadId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Priority(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::ThreadPriority> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::ThreadPriority = __cordl_object
            .invoke("get_Priority", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn nativeInitCultureAccessors() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("nativeInitCultureAccessors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CurrentCulture(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CurrentCulture", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CurrentUICulture(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CurrentUICulture", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ExecutionContextBelongsToCurrentScope(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ExecutionContextBelongsToCurrentScope", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsBackground(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsBackground", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Name", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Priority(
        &mut self,
        value: crate::System::Threading::ThreadPriority,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Priority", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Thread")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::Thread {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
