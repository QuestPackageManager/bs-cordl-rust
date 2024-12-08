#[cfg(feature = "System+Threading+ExecutionContext+CaptureOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionContext_CaptureOptions {
    IgnoreSyncCtx = 1i32,
    None = 0i32,
    OptimizeDefaultCase = 2i32,
}
#[cfg(feature = "System+Threading+ExecutionContext+CaptureOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::ExecutionContext_CaptureOptions => "System.Threading"
    ."ExecutionContext/CaptureOptions"
);
#[cfg(feature = "System+Threading+ExecutionContext")]
#[repr(C)]
#[derive(Debug)]
pub struct ExecutionContext {
    __cordl_parent: crate::System::Object,
    pub _syncContext: *mut crate::System::Threading::SynchronizationContext,
    pub _syncContextNoFlow: *mut crate::System::Threading::SynchronizationContext,
    pub _logicalCallContext: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
    pub _illogicalCallContext: *mut crate::System::Runtime::Remoting::Messaging::IllogicalCallContext,
    pub _flags: crate::System::Threading::ExecutionContext_Flags,
    pub _localValues: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Threading::IAsyncLocal,
        *mut crate::System::Object,
    >,
    pub _localChangeNotifications: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Threading::IAsyncLocal,
    >,
}
#[cfg(feature = "System+Threading+ExecutionContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::ExecutionContext =>
    "System.Threading"."ExecutionContext"
);
#[cfg(feature = "System+Threading+ExecutionContext")]
impl std::ops::Deref for crate::System::Threading::ExecutionContext {
    type Target = crate::System::Object;
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
    pub fn CreateCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::ExecutionContext> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::ExecutionContext = __cordl_object
            .invoke("CreateCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateMutableCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::ExecutionContext> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::ExecutionContext = __cordl_object
            .invoke("CreateMutableCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext2(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        isPreAllocatedDefault: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isPreAllocatedDefault))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_IllogicalCallContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::IllogicalCallContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::IllogicalCallContext = __cordl_object
            .invoke("get_IllogicalCallContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsPreAllocatedDefault(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsPreAllocatedDefault", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LogicalCallContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext = __cordl_object
            .invoke("get_LogicalCallContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SynchronizationContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::SynchronizationContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::SynchronizationContext = __cordl_object
            .invoke("get_SynchronizationContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SynchronizationContextNoFlow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::SynchronizationContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::SynchronizationContext = __cordl_object
            .invoke("get_SynchronizationContextNoFlow", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isFlowSuppressed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFlowSuppressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isNewCapture(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isNewCapture", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IllogicalCallContext(
        &mut self,
        value: *mut crate::System::Runtime::Remoting::Messaging::IllogicalCallContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IllogicalCallContext", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_LogicalCallContext(
        &mut self,
        value: *mut crate::System::Runtime::Remoting::Messaging::LogicalCallContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LogicalCallContext", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SynchronizationContext(
        &mut self,
        value: *mut crate::System::Threading::SynchronizationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SynchronizationContext", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SynchronizationContextNoFlow(
        &mut self,
        value: *mut crate::System::Threading::SynchronizationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SynchronizationContextNoFlow", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
#[cfg(feature = "System+Threading+ExecutionContext+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionContext_Flags {
    IsFlowSuppressed = 2i32,
    IsNewCapture = 1i32,
    IsPreAllocatedDefault = 4i32,
    None = 0i32,
}
#[cfg(feature = "System+Threading+ExecutionContext+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::ExecutionContext_Flags =>
    "System.Threading"."ExecutionContext/Flags"
);
#[cfg(feature = "System+Threading+ExecutionContext+Reader")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExecutionContext_Reader {
    pub m_ec: *mut crate::System::Threading::ExecutionContext,
}
#[cfg(feature = "System+Threading+ExecutionContext+Reader")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::ExecutionContext_Reader =>
    "System.Threading"."ExecutionContext/Reader"
);
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
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::ExecutionContext> {
        let __cordl_ret: *mut crate::System::Threading::ExecutionContext = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "DangerousGetRawExecutionContext",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn HasSameLocalValues(
        &mut self,
        other: *mut crate::System::Threading::ExecutionContext,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "HasSameLocalValues",
            (other),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        ec: *mut crate::System::Threading::ExecutionContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ec),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsFlowSuppressed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsFlowSuppressed",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNull",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_SynchronizationContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::SynchronizationContext,
    > {
        let __cordl_ret: *mut crate::System::Threading::SynchronizationContext = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_SynchronizationContext",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_SynchronizationContextNoFlow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::SynchronizationContext,
    > {
        let __cordl_ret: *mut crate::System::Threading::SynchronizationContext = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_SynchronizationContextNoFlow",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
