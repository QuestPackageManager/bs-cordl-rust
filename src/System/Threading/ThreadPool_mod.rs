#[cfg(feature = "System+Threading+ThreadPool")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadPool {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+ThreadPool")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::ThreadPool {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "ThreadPool";
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
#[cfg(feature = "System+Threading+ThreadPool")]
impl std::ops::Deref for crate::System::Threading::ThreadPool {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPool")]
impl std::ops::DerefMut for crate::System::Threading::ThreadPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPool")]
impl crate::System::Threading::ThreadPool {
    pub fn EnsureVMInitialized() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureVMInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeVMTp(
        enableWorkerTracking: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeVMTp", (enableWorkerTracking))?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyWorkItemComplete() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyWorkItemComplete", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyWorkItemProgress() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyWorkItemProgress", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyWorkItemProgressNative() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyWorkItemProgressNative", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NotifyWorkItemQueued() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NotifyWorkItemQueued", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueUserWorkItemHelper(
        callBack: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
        compressStack: bool,
        forceGlobal: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "QueueUserWorkItemHelper",
                (callBack, state, stackMark, compressStack, forceGlobal),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueUserWorkItem_Action_1_TState__cordl_bool2<TState>(
        callBack: quest_hook::libil2cpp::Gc<crate::System::Action_1<TState>>,
        state: TState,
        preferLocal: bool,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueUserWorkItem", (callBack, state, preferLocal))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueUserWorkItem_WaitCallback1(
        callBack: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueUserWorkItem", (callBack))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueUserWorkItem_WaitCallback_Il2CppObject0(
        callBack: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueUserWorkItem", (callBack, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterWaitForSingleObject_TimeSpan2(
        waitObject: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
        callBack: quest_hook::libil2cpp::Gc<
            crate::System::Threading::WaitOrTimerCallback,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        timeout: crate::System::TimeSpan,
        executeOnlyOnce: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::RegisteredWaitHandle>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::RegisteredWaitHandle,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RegisterWaitForSingleObject",
                (waitObject, callBack, state, timeout, executeOnlyOnce),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterWaitForSingleObject_i32_1(
        waitObject: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
        callBack: quest_hook::libil2cpp::Gc<
            crate::System::Threading::WaitOrTimerCallback,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        millisecondsTimeOutInterval: i32,
        executeOnlyOnce: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::RegisteredWaitHandle>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::RegisteredWaitHandle,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RegisterWaitForSingleObject",
                (
                    waitObject,
                    callBack,
                    state,
                    millisecondsTimeOutInterval,
                    executeOnlyOnce,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterWaitForSingleObject_u32_ByRefMut__cordl_bool0(
        waitObject: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
        callBack: quest_hook::libil2cpp::Gc<
            crate::System::Threading::WaitOrTimerCallback,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        millisecondsTimeOutInterval: u32,
        executeOnlyOnce: bool,
        stackMark: quest_hook::libil2cpp::ByRefMut<
            crate::System::Threading::StackCrawlMark,
        >,
        compressStack: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::RegisteredWaitHandle>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::RegisteredWaitHandle,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RegisterWaitForSingleObject",
                (
                    waitObject,
                    callBack,
                    state,
                    millisecondsTimeOutInterval,
                    executeOnlyOnce,
                    stackMark,
                    compressStack,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReportThreadStatus(
        isWorking: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReportThreadStatus", (isWorking))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestWorkerThread() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RequestWorkerThread", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryPopCustomWorkItem(
        workItem: quest_hook::libil2cpp::Gc<
            crate::System::Threading::IThreadPoolWorkItem,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryPopCustomWorkItem", (workItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeQueueCustomWorkItem(
        workItem: quest_hook::libil2cpp::Gc<
            crate::System::Threading::IThreadPoolWorkItem,
        >,
        forceGlobal: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeQueueCustomWorkItem", (workItem, forceGlobal))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeQueueUserWorkItem(
        callBack: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeQueueUserWorkItem", (callBack, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsThreadPoolThread() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsThreadPoolThread", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+ThreadPool")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::ThreadPool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
