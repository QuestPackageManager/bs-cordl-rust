#[cfg(feature = "UnityEngine+UnitySynchronizationContext")]
#[repr(C)]
#[derive(Debug)]
pub struct UnitySynchronizationContext {
    __cordl_parent: crate::System::Threading::SynchronizationContext,
    pub m_AsyncWorkQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UnitySynchronizationContext_WorkRequest,
        >,
    >,
    pub m_CurrentFrameWork: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UnitySynchronizationContext_WorkRequest,
        >,
    >,
    pub m_MainThreadID: i32,
    pub m_TrackedCount: i32,
}
#[cfg(feature = "UnityEngine+UnitySynchronizationContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UnitySynchronizationContext =>
    "UnityEngine"."UnitySynchronizationContext"
);
#[cfg(feature = "UnityEngine+UnitySynchronizationContext")]
impl std::ops::Deref for crate::UnityEngine::UnitySynchronizationContext {
    type Target = crate::System::Threading::SynchronizationContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UnitySynchronizationContext")]
impl std::ops::DerefMut for crate::UnityEngine::UnitySynchronizationContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UnitySynchronizationContext")]
impl crate::UnityEngine::UnitySynchronizationContext {
    #[cfg(feature = "UnityEngine+UnitySynchronizationContext+WorkRequest")]
    pub type WorkRequest = crate::UnityEngine::UnitySynchronizationContext_WorkRequest;
    pub fn CreateCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::SynchronizationContext>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SynchronizationContext,
        > = __cordl_object.invoke("CreateCopy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Exec(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Exec", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecutePendingTasks(
        millisecondsTimeout: i64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExecutePendingTasks", (millisecondsTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteTasks() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExecuteTasks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasPendingTasks(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasPendingTasks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeSynchronizationContext() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeSynchronizationContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_List_1_i32_1(
        queue: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UnitySynchronizationContext_WorkRequest,
            >,
        >,
        mainThreadID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (queue, mainThreadID))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_0(
        mainThreadID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mainThreadID))?;
        Ok(__cordl_object.into())
    }
    pub fn OperationCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OperationCompleted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OperationStarted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OperationStarted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Post(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SendOrPostCallback,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Post", (callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SendOrPostCallback,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (callback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_List_1_i32_1(
        &mut self,
        queue: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UnitySynchronizationContext_WorkRequest,
            >,
        >,
        mainThreadID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (queue, mainThreadID))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        mainThreadID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mainThreadID))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UnitySynchronizationContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UnitySynchronizationContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UnitySynchronizationContext+WorkRequest")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnitySynchronizationContext_WorkRequest {
    pub m_DelagateCallback: quest_hook::libil2cpp::Gc<
        crate::System::Threading::SendOrPostCallback,
    >,
    pub m_DelagateState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_WaitHandle: quest_hook::libil2cpp::Gc<
        crate::System::Threading::ManualResetEvent,
    >,
}
#[cfg(feature = "UnityEngine+UnitySynchronizationContext+WorkRequest")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UnitySynchronizationContext_WorkRequest => "UnityEngine"
    ."UnitySynchronizationContext/WorkRequest"
);
#[cfg(feature = "UnityEngine+UnitySynchronizationContext+WorkRequest")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UnitySynchronizationContext_WorkRequest {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UnitySynchronizationContext+WorkRequest")]
impl crate::UnityEngine::UnitySynchronizationContext_WorkRequest {
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Invoke",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SendOrPostCallback,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        waitHandle: quest_hook::libil2cpp::Gc<crate::System::Threading::ManualResetEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (callback, state, waitHandle),
        )?;
        Ok(__cordl_ret.into())
    }
}
