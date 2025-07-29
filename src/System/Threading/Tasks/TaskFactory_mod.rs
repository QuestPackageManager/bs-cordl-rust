#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_defaultCancellationToken: crate::System::Threading::CancellationToken,
    pub m_defaultScheduler: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::TaskScheduler,
    >,
    pub m_defaultCreationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    pub m_defaultContinuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskFactory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::TaskFactory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "TaskFactory";
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
#[cfg(feature = "System+Threading+Tasks+TaskFactory")]
impl std::ops::Deref for crate::System::Threading::Tasks::TaskFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::TaskFactory {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory")]
impl crate::System::Threading::Tasks::TaskFactory {
    #[cfg(feature = "System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise")]
    pub type CompleteOnInvokePromise = crate::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise;
    pub fn CheckCreationOptions(
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Threading::Tasks::TaskCreationOptions),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckCreationOptions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckCreationOptions", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (creationOptions))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckFromAsyncOptions(
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        hasBeginMethod: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Threading::Tasks::TaskCreationOptions, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CheckFromAsyncOptions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckFromAsyncOptions", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (creationOptions, hasBeginMethod))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckMultiTaskContinuationOptions(
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Threading::Tasks::TaskContinuationOptions),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckMultiTaskContinuationOptions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckMultiTaskContinuationOptions", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (continuationOptions))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CommonCWAnyLogic(
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                        >,
                        1usize,
                    >("CommonCWAnyLogic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CommonCWAnyLogic", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (tasks))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromAsync_Il2CppObject0<TArg1>(
        &mut self,
        beginMethod: quest_hook::libil2cpp::Gc<
            TArg1,
            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        >,
        endMethod: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        >,
        arg1: TArg1,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    >
    where
        TArg1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                TArg1,
                                quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                            >,
                            TArg1,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        4usize,
                    >("FromAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromAsync", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (beginMethod, endMethod, arg1, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromAsync_Il2CppObject_TaskCreationOptions1<TArg1>(
        &mut self,
        beginMethod: quest_hook::libil2cpp::Gc<
            TArg1,
            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        >,
        endMethod: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        >,
        arg1: TArg1,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    >
    where
        TArg1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                TArg1,
                                quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                            >,
                            TArg1,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        5usize,
                    >("FromAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromAsync", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (beginMethod, endMethod, arg1, state, creationOptions),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromAsync_TArg2_Il2CppObject2<TArg1, TArg2>(
        &mut self,
        beginMethod: quest_hook::libil2cpp::Gc<
            TArg1,
            TArg2,
            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        >,
        endMethod: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        >,
        arg1: TArg1,
        arg2: TArg2,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    >
    where
        TArg1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                TArg1,
                                TArg2,
                                quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                            >,
                            TArg1,
                            TArg2,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        5usize,
                    >("FromAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromAsync", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (beginMethod, endMethod, arg1, arg2, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromAsync_TArg2_Il2CppObject_TaskCreationOptions3<TArg1, TArg2>(
        &mut self,
        beginMethod: quest_hook::libil2cpp::Gc<
            TArg1,
            TArg2,
            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        >,
        endMethod: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        >,
        arg1: TArg1,
        arg2: TArg2,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    >
    where
        TArg1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                TArg1,
                                TArg2,
                                quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                            >,
                            TArg1,
                            TArg2,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        6usize,
                    >("FromAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromAsync", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (beginMethod, endMethod, arg1, arg2, state, creationOptions),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_CancellationToken_TaskCreationOptions_TaskContinuationOptions_TaskScheduler1(
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (cancellationToken, creationOptions, continuationOptions, scheduler),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn StartNew_CancellationToken_TaskCreationOptions_TaskScheduler1<TResult>(
        &mut self,
        function: quest_hook::libil2cpp::Gc<TResult>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<TResult>,
                            crate::System::Threading::CancellationToken,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<TResult>,
                        4usize,
                    >("StartNew")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartNew", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (function, cancellationToken, creationOptions, scheduler),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartNew_Il2CppObject_CancellationToken_TaskCreationOptions_TaskScheduler0(
        &mut self,
        action: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::Threading::CancellationToken,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        5usize,
                    >("StartNew")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartNew", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (action, state, cancellationToken, creationOptions, scheduler),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartNew_Il2CppObject_CancellationToken_TaskCreationOptions_TaskScheduler2<
        TResult,
    >(
        &mut self,
        function: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            TResult,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                                TResult,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::Threading::CancellationToken,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<TResult>,
                        5usize,
                    >("StartNew")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartNew", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResult> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (function, state, cancellationToken, creationOptions, scheduler),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CancellationToken_TaskCreationOptions_TaskContinuationOptions_TaskScheduler1(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Threading::CancellationToken,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                            crate::System::Threading::Tasks::TaskContinuationOptions,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (cancellationToken, creationOptions, continuationOptions, scheduler),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskFactory")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::Tasks::TaskFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TaskFactory_CompleteOnInvokePromise {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    >,
    pub _tasks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    >,
}
#[cfg(
    feature = "cordl_class_System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "TaskFactory/CompleteOnInvokePromise";
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
#[cfg(feature = "System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise")]
impl std::ops::Deref
for crate::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise {
    type Target = quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise")]
impl std::ops::DerefMut
for crate::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise")]
impl crate::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise {
    pub fn Invoke(
        &mut self,
        completingTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (completingTask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tasks))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        tasks: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (tasks))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_InvokeMayRunArbitraryCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_InvokeMayRunArbitraryCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_InvokeMayRunArbitraryCode", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise")]
impl AsRef<crate::System::Threading::Tasks::ITaskCompletionAction>
for crate::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise {
    fn as_ref(&self) -> &crate::System::Threading::Tasks::ITaskCompletionAction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise")]
impl AsMut<crate::System::Threading::Tasks::ITaskCompletionAction>
for crate::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise {
    fn as_mut(&mut self) -> &mut crate::System::Threading::Tasks::ITaskCompletionAction {
        unsafe { std::mem::transmute(self) }
    }
}
