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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("EnsureVMInitialized")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "EnsureVMInitialized", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeVMTp(
        enableWorkerTracking: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<bool>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InitializeVMTp")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "InitializeVMTp", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (enableWorkerTracking))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyWorkItemComplete() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("NotifyWorkItemComplete")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "NotifyWorkItemComplete", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyWorkItemProgress() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("NotifyWorkItemProgress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "NotifyWorkItemProgress", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyWorkItemProgressNative() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("NotifyWorkItemProgressNative")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "NotifyWorkItemProgressNative", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NotifyWorkItemQueued() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("NotifyWorkItemQueued")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "NotifyWorkItemQueued", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Threading::StackCrawlMark,
                    >,
                    bool,
                    bool,
                ),
                bool,
                5usize,
            >("QueueUserWorkItemHelper")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "QueueUserWorkItemHelper", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (callBack, state, stackMark, compressStack, forceGlobal),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Action_1<TState>>,
                    TState,
                    bool,
                ),
                bool,
                3usize,
            >("QueueUserWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "QueueUserWorkItem", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (callBack, state, preferLocal))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn QueueUserWorkItem_WaitCallback1(
        callBack: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>),
                bool,
                1usize,
            >("QueueUserWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "QueueUserWorkItem", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (callBack))? };
        Ok(__cordl_ret.into())
    }
    pub fn QueueUserWorkItem_WaitCallback_Il2CppObject0(
        callBack: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                bool,
                2usize,
            >("QueueUserWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "QueueUserWorkItem", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (callBack, state))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::WaitOrTimerCallback,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::System::TimeSpan,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::RegisteredWaitHandle,
                >,
                5usize,
            >("RegisterWaitForSingleObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "RegisterWaitForSingleObject", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::RegisteredWaitHandle,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (waitObject, callBack, state, timeout, executeOnlyOnce),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::WaitOrTimerCallback,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::RegisteredWaitHandle,
                >,
                5usize,
            >("RegisterWaitForSingleObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "RegisterWaitForSingleObject", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::RegisteredWaitHandle,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        waitObject,
                        callBack,
                        state,
                        millisecondsTimeOutInterval,
                        executeOnlyOnce,
                    ),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::WaitOrTimerCallback,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    u32,
                    bool,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Threading::StackCrawlMark,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::RegisteredWaitHandle,
                >,
                7usize,
            >("RegisterWaitForSingleObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "RegisterWaitForSingleObject", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::RegisteredWaitHandle,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        waitObject,
                        callBack,
                        state,
                        millisecondsTimeOutInterval,
                        executeOnlyOnce,
                        stackMark,
                        compressStack,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReportThreadStatus(
        isWorking: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReportThreadStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "ReportThreadStatus", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (isWorking))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestWorkerThread() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("RequestWorkerThread")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "RequestWorkerThread", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TryPopCustomWorkItem(
        workItem: quest_hook::libil2cpp::Gc<
            crate::System::Threading::IThreadPoolWorkItem,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Threading::IThreadPoolWorkItem,
                >),
                bool,
                1usize,
            >("TryPopCustomWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "TryPopCustomWorkItem", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (workItem))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeQueueCustomWorkItem(
        workItem: quest_hook::libil2cpp::Gc<
            crate::System::Threading::IThreadPoolWorkItem,
        >,
        forceGlobal: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::IThreadPoolWorkItem,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UnsafeQueueCustomWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "UnsafeQueueCustomWorkItem", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (workItem, forceGlobal))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeQueueUserWorkItem(
        callBack: quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Threading::WaitCallback>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                bool,
                2usize,
            >("UnsafeQueueUserWorkItem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "UnsafeQueueUserWorkItem", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (callBack, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsThreadPoolThread() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Threading::ThreadPool as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsThreadPoolThread")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Threading::ThreadPool as quest_hook::libil2cpp::Type
                    > ::class(), "get_IsThreadPoolThread", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ())? };
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
