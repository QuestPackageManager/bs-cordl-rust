#[cfg(feature = "System+Threading+ExecutionContext")]
#[repr(C)]
#[derive(Debug)]
pub struct ExecutionContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _syncContext: quest_hook::libil2cpp::Gc<
        crate::System::Threading::SynchronizationContext,
    >,
    pub _syncContextNoFlow: quest_hook::libil2cpp::Gc<
        crate::System::Threading::SynchronizationContext,
    >,
    pub _logicalCallContext: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
    >,
    pub _illogicalCallContext: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IllogicalCallContext,
    >,
    pub _flags: crate::System::Threading::ExecutionContext_Flags,
    pub _localValues: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Threading::IAsyncLocal>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
    pub _localChangeNotifications: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::Threading::IAsyncLocal>,
        >,
    >,
}
#[cfg(feature = "System+Threading+ExecutionContext")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::ExecutionContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "ExecutionContext";
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
#[cfg(feature = "System+Threading+ExecutionContext")]
impl std::ops::Deref for crate::System::Threading::ExecutionContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ExecutionContext")]
impl std::ops::DerefMut for crate::System::Threading::ExecutionContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ExecutionContext")]
impl crate::System::Threading::ExecutionContext {
    #[cfg(feature = "System+Threading+ExecutionContext+CaptureOptions")]
    pub type CaptureOptions = crate::System::Threading::ExecutionContext_CaptureOptions;
    #[cfg(feature = "System+Threading+ExecutionContext+Flags")]
    pub type Flags = crate::System::Threading::ExecutionContext_Flags;
    #[cfg(feature = "System+Threading+ExecutionContext+Reader")]
    pub type Reader = crate::System::Threading::ExecutionContext_Reader;
    pub fn Capture_0() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Capture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Capture_ByRefMut_ExecutionContext_CaptureOptions1(
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
        options: crate::System::Threading::ExecutionContext_CaptureOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Capture", (stackMark, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        > = __cordl_object.invoke("CreateCopy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMutableCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        > = __cordl_object.invoke("CreateMutableCopy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EstablishCopyOnWriteScope_ByRefMut0(
        ecsw: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::ExecutionContextSwitcher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EstablishCopyOnWriteScope", (ecsw))?;
        Ok(__cordl_ret.into())
    }
    pub fn EstablishCopyOnWriteScope_Thread__cordl_bool_ByRefMut1(
        currentThread: quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
        knownNullWindowsIdentity: bool,
        ecsw: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::ExecutionContextSwitcher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EstablishCopyOnWriteScope",
                (currentThread, knownNullWindowsIdentity, ecsw),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FastCapture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("FastCapture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDefaultFTContext(
        &mut self,
        ignoreSyncCtx: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsDefaultFTContext", (ignoreSyncCtx))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFlowSuppressed() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsFlowSuppressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext2(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool1(
        isPreAllocatedDefault: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isPreAllocatedDefault))?;
        Ok(__cordl_object.into())
    }
    pub fn OnAsyncLocalContextChanged(
        previous: quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
        current: quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnAsyncLocalContextChanged", (previous, current))?;
        Ok(__cordl_ret.into())
    }
    pub fn RunInternal_ExecutionContext_ContextCallback_Il2CppObject0(
        executionContext: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::Threading::ContextCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RunInternal", (executionContext, callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn RunInternal__cordl_bool1(
        executionContext: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::Threading::ContextCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        preserveSyncCtx: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RunInternal",
                (executionContext, callback, state, preserveSyncCtx),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Run_ExecutionContext_ContextCallback_Il2CppObject0(
        executionContext: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::Threading::ContextCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Run", (executionContext, callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run__cordl_bool1(
        executionContext: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::Threading::ContextCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        preserveSyncCtx: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Run", (executionContext, callback, state, preserveSyncCtx))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetExecutionContext(
        executionContext: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        >,
        preserveSyncCtx: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::ExecutionContextSwitcher,
    > {
        let __cordl_ret: crate::System::Threading::ExecutionContextSwitcher = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetExecutionContext", (executionContext, preserveSyncCtx))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalValue(
        local: quest_hook::libil2cpp::Gc<crate::System::Threading::IAsyncLocal>,
        newValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        needChangeNotifications: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLocalValue", (local, newValue, needChangeNotifications))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        isPreAllocatedDefault: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isPreAllocatedDefault))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IllogicalCallContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IllogicalCallContext,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IllogicalCallContext,
        > = __cordl_object.invoke("get_IllogicalCallContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsPreAllocatedDefault(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsPreAllocatedDefault", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LogicalCallContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        > = __cordl_object.invoke("get_LogicalCallContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SynchronizationContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::SynchronizationContext>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SynchronizationContext,
        > = __cordl_object.invoke("get_SynchronizationContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SynchronizationContextNoFlow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::SynchronizationContext>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SynchronizationContext,
        > = __cordl_object.invoke("get_SynchronizationContextNoFlow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isFlowSuppressed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFlowSuppressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isNewCapture(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isNewCapture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IllogicalCallContext(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IllogicalCallContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IllogicalCallContext", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LogicalCallContext(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LogicalCallContext", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SynchronizationContext(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SynchronizationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SynchronizationContext", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SynchronizationContextNoFlow(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SynchronizationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SynchronizationContextNoFlow", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isFlowSuppressed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isFlowSuppressed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isNewCapture(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isNewCapture", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+ExecutionContext")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::ExecutionContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+ExecutionContext")]
impl AsRef<crate::System::IDisposable> for crate::System::Threading::ExecutionContext {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+ExecutionContext")]
impl AsMut<crate::System::IDisposable> for crate::System::Threading::ExecutionContext {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+ExecutionContext")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Threading::ExecutionContext {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+ExecutionContext")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Threading::ExecutionContext {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+CaptureOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExecutionContext_CaptureOptions {
    #[default]
    IgnoreSyncCtx = 1i32,
    None = 0i32,
    OptimizeDefaultCase = 2i32,
}
#[cfg(feature = "System+Threading+ExecutionContext+CaptureOptions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::ExecutionContext_CaptureOptions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "ExecutionContext/CaptureOptions";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+CaptureOptions")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Threading::ExecutionContext_CaptureOptions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+CaptureOptions")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Threading::ExecutionContext_CaptureOptions {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+CaptureOptions")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Threading::ExecutionContext_CaptureOptions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+CaptureOptions")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Threading::ExecutionContext_CaptureOptions {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExecutionContext_Flags {
    #[default]
    IsFlowSuppressed = 2i32,
    IsNewCapture = 1i32,
    IsPreAllocatedDefault = 4i32,
    None = 0i32,
}
#[cfg(feature = "System+Threading+ExecutionContext+Flags")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::ExecutionContext_Flags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "ExecutionContext/Flags";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+Flags")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Threading::ExecutionContext_Flags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+Flags")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Threading::ExecutionContext_Flags {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+Flags")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Threading::ExecutionContext_Flags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+Flags")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Threading::ExecutionContext_Flags {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+Reader")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExecutionContext_Reader {
    pub m_ec: quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
}
#[cfg(feature = "System+Threading+ExecutionContext+Reader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::ExecutionContext_Reader {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "ExecutionContext/Reader";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+Reader")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Threading::ExecutionContext_Reader {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+Reader")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Threading::ExecutionContext_Reader {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+Reader")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Threading::ExecutionContext_Reader {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+Reader")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Threading::ExecutionContext_Reader {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+Reader")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::ExecutionContext_Reader {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+ExecutionContext+Reader")]
impl crate::System::Threading::ExecutionContext_Reader {
    pub fn DangerousGetRawExecutionContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DangerousGetRawExecutionContext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn HasSameLocalValues(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "HasSameLocalValues",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDefaultFTContext(
        &mut self,
        ignoreSyncCtx: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsDefaultFTContext",
            (ignoreSyncCtx),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        ec: quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ec),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFlowSuppressed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsFlowSuppressed",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNull",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LogicalCallContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Remoting::Messaging::LogicalCallContext_Reader,
    > {
        let __cordl_ret: crate::System::Runtime::Remoting::Messaging::LogicalCallContext_Reader = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_LogicalCallContext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SynchronizationContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::SynchronizationContext>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SynchronizationContext,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_SynchronizationContext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SynchronizationContextNoFlow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::SynchronizationContext>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SynchronizationContext,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_SynchronizationContextNoFlow",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
