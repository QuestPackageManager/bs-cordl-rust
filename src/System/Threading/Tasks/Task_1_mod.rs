#[cfg(feature = "System+Threading+Tasks+Task_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Task_1<TResult: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Threading::Tasks::Task,
    pub m_result: TResult,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Threading+Tasks+Task_1")]
unsafe impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::Task_1<TResult> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "Task`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Threading.Tasks",
                        "Task`1",
                    )
                    .unwrap()
                    .make_generic::<(TResult)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "System+Threading+Tasks+Task_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::Tasks::Task_1<TResult> {
    type Target = crate::System::Threading::Tasks::Task;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::Tasks::Task_1<TResult> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Threading::Tasks::Task_1<TResult> {
    #[cfg(feature = "System+Threading+Tasks+Task_1+TaskWhenAnyCast")]
    pub type TaskWhenAnyCast = crate::System::Threading::Tasks::Task_1_TaskWhenAnyCast<
        TResult,
    >;
    pub fn ConfigureAwait(
        &mut self,
        continueOnCapturedContext: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_1<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_1<
                            TResult,
                        >,
                        1usize,
                    >("ConfigureAwait")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConfigureAwait", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_1<
            TResult,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (continueOnCapturedContext))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContinueWith_Action_1_TaskScheduler0(
        &mut self,
        continuationAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<TResult>,
                >,
            >,
        >,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Threading::Tasks::Task_1<TResult>,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        2usize,
                    >("ContinueWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContinueWith", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (continuationAction, scheduler))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContinueWith_Action_1_TaskScheduler_CancellationToken_TaskContinuationOptions1(
        &mut self,
        continuationAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<TResult>,
                >,
            >,
        >,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Threading::Tasks::Task_1<TResult>,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                            crate::System::Threading::CancellationToken,
                            crate::System::Threading::Tasks::TaskContinuationOptions,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        4usize,
                    >("ContinueWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContinueWith", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        continuationAction,
                        scheduler,
                        cancellationToken,
                        continuationOptions,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContinueWith_Func_2_2<TNewResult>(
        &mut self,
        continuationFunction: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<TResult>,
                >,
                TNewResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TNewResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TNewResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Func_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Threading::Tasks::Task_1<TResult>,
                                >,
                                TNewResult,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TNewResult>,
                        >,
                        1usize,
                    >("ContinueWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContinueWith", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TNewResult>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (continuationFunction))? };
        Ok(__cordl_ret.into())
    }
    pub fn ContinueWith_Func_2_CancellationToken_TaskContinuationOptions_TaskScheduler4<
        TNewResult,
    >(
        &mut self,
        continuationFunction: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<TResult>,
                >,
                TNewResult,
            >,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TNewResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TNewResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Threading::Tasks::Task_1<TResult>,
                                    >,
                                    TNewResult,
                                >,
                            >,
                            crate::System::Threading::CancellationToken,
                            crate::System::Threading::Tasks::TaskContinuationOptions,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TNewResult>,
                        >,
                        4usize,
                    >("ContinueWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContinueWith", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TNewResult>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        continuationFunction,
                        cancellationToken,
                        continuationOptions,
                        scheduler,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContinueWith_Func_2_TaskContinuationOptions3<TNewResult>(
        &mut self,
        continuationFunction: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<TResult>,
                >,
                TNewResult,
            >,
        >,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TNewResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TNewResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Threading::Tasks::Task_1<TResult>,
                                    >,
                                    TNewResult,
                                >,
                            >,
                            crate::System::Threading::Tasks::TaskContinuationOptions,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TNewResult>,
                        >,
                        2usize,
                    >("ContinueWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContinueWith", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TNewResult>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (continuationFunction, continuationOptions))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContinueWith_Func_2_TaskScheduler_CancellationToken_TaskContinuationOptions5<
        TNewResult,
    >(
        &mut self,
        continuationFunction: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<TResult>,
                >,
                TNewResult,
            >,
        >,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TNewResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TNewResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Threading::Tasks::Task_1<TResult>,
                                    >,
                                    TNewResult,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                            crate::System::Threading::CancellationToken,
                            crate::System::Threading::Tasks::TaskContinuationOptions,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TNewResult>,
                        >,
                        4usize,
                    >("ContinueWith")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContinueWith", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TNewResult>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        continuationFunction,
                        scheduler,
                        cancellationToken,
                        continuationOptions,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DangerousSetResult(
        &mut self,
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (TResult),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DangerousSetResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DangerousSetResult", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAwaiter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::TaskAwaiter_1<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::System::Runtime::CompilerServices::TaskAwaiter_1<TResult>,
                        0usize,
                    >("GetAwaiter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAwaiter", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            TResult,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetResultCore(
        &mut self,
        waitCompletionNotification: bool,
    ) -> quest_hook::libil2cpp::Result<TResult>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), TResult, 1usize>("GetResultCore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetResultCore", 1usize
                        )
                    })
            });
        let __cordl_ret: TResult = unsafe {
            cordl_method_info.invoke_unchecked(self, (waitCompletionNotification))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InnerInvoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("InnerInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InnerInvoke", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Delegate_Il2CppObject_Task_CancellationToken_TaskCreationOptions_InternalTaskOptions_TaskScheduler7(
        valueSelector: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        parent: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    valueSelector,
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
    pub fn New_Func_1_CancellationToken4(
        function: quest_hook::libil2cpp::Gc<crate::System::Func_1<TResult>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (function, cancellationToken))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Func_1_Task_CancellationToken_TaskCreationOptions_InternalTaskOptions_TaskScheduler6(
        valueSelector: quest_hook::libil2cpp::Gc<crate::System::Func_1<TResult>>,
        parent: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    valueSelector,
                    parent,
                    cancellationToken,
                    creationOptions,
                    internalOptions,
                    scheduler,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Func_2_Il2CppObject_CancellationToken_TaskCreationOptions5(
        function: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                TResult,
            >,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (function, state, cancellationToken, creationOptions),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppObject_TaskCreationOptions1(
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        options: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (state, options))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TResult2(
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (result))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool_TResult_TaskCreationOptions_CancellationToken3(
        canceled: bool,
        result: TResult,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        ct: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (canceled, result, creationOptions, ct))?;
        Ok(__cordl_object.into())
    }
    pub fn StartNew_Func_1_CancellationToken_TaskCreationOptions_InternalTaskOptions_TaskScheduler0(
        parent: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        function: quest_hook::libil2cpp::Gc<crate::System::Func_1<TResult>>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Func_1<TResult>>,
                            crate::System::Threading::CancellationToken,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                            crate::System::Threading::Tasks::InternalTaskOptions,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResult>,
                        >,
                        6usize,
                    >("StartNew")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartNew", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        parent,
                        function,
                        cancellationToken,
                        creationOptions,
                        internalOptions,
                        scheduler,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartNew_Func_2_Il2CppObject_CancellationToken_TaskCreationOptions_InternalTaskOptions_TaskScheduler1(
        parent: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        function: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                TResult,
            >,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                    TResult,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::Threading::CancellationToken,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                            crate::System::Threading::Tasks::InternalTaskOptions,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResult>,
                        >,
                        7usize,
                    >("StartNew")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartNew", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        parent,
                        function,
                        state,
                        cancellationToken,
                        creationOptions,
                        internalOptions,
                        scheduler,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrySetResult(
        &mut self,
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(TResult), bool, 1usize>("TrySetResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrySetResult", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
    pub fn _ctor_Delegate_Il2CppObject_Task_CancellationToken_TaskCreationOptions_InternalTaskOptions_TaskScheduler7(
        &mut self,
        valueSelector: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        parent: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                            crate::System::Threading::CancellationToken,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                            crate::System::Threading::Tasks::InternalTaskOptions,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        valueSelector,
                        state,
                        parent,
                        cancellationToken,
                        creationOptions,
                        internalOptions,
                        scheduler,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Func_1_CancellationToken4(
        &mut self,
        function: quest_hook::libil2cpp::Gc<crate::System::Func_1<TResult>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Func_1<TResult>>,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (function, cancellationToken))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Func_1_Task_CancellationToken_TaskCreationOptions_InternalTaskOptions_TaskScheduler6(
        &mut self,
        valueSelector: quest_hook::libil2cpp::Gc<crate::System::Func_1<TResult>>,
        parent: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
        scheduler: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskScheduler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Func_1<TResult>>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                            crate::System::Threading::CancellationToken,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                            crate::System::Threading::Tasks::InternalTaskOptions,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        valueSelector,
                        parent,
                        cancellationToken,
                        creationOptions,
                        internalOptions,
                        scheduler,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Func_2_Il2CppObject_CancellationToken_TaskCreationOptions5(
        &mut self,
        function: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                TResult,
            >,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                    TResult,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::Threading::CancellationToken,
                            crate::System::Threading::Tasks::TaskCreationOptions,
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
                    (function, state, cancellationToken, creationOptions),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_TaskCreationOptions1(
        &mut self,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        options: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, options))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TResult2(
        &mut self,
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (TResult),
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
            cordl_method_info.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool_TResult_TaskCreationOptions_CancellationToken3(
        &mut self,
        canceled: bool,
        result: TResult,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        ct: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            bool,
                            TResult,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                            crate::System::Threading::CancellationToken,
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
                .invoke_unchecked(self, (canceled, result, creationOptions, ct))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Factory() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskFactory_1<TResult>,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::TaskFactory_1<TResult>,
                        >,
                        0usize,
                    >("get_Factory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Factory", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskFactory_1<TResult>,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Result(&mut self) -> quest_hook::libil2cpp::Result<TResult>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), TResult, 0usize>("get_Result")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Result", 0usize
                        )
                    })
            });
        let __cordl_ret: TResult = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ResultOnSuccess(&mut self) -> quest_hook::libil2cpp::Result<TResult>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), TResult, 0usize>("get_ResultOnSuccess")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ResultOnSuccess", 0usize
                        )
                    })
            });
        let __cordl_ret: TResult = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+Task_1")]
impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::Task_1<TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+Task_1+TaskWhenAnyCast")]
#[repr(C)]
#[derive(Debug)]
pub struct Task_1_TaskWhenAnyCast<TResult: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Threading+Tasks+Task_1+TaskWhenAnyCast")]
unsafe impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::Task_1_TaskWhenAnyCast<TResult> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "Task`1/TaskWhenAnyCast";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Threading.Tasks",
                        "Task`1/TaskWhenAnyCast",
                    )
                    .unwrap()
                    .make_generic::<(TResult)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "System+Threading+Tasks+Task_1+TaskWhenAnyCast")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::Tasks::Task_1_TaskWhenAnyCast<TResult> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task_1+TaskWhenAnyCast")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::Tasks::Task_1_TaskWhenAnyCast<TResult> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task_1+TaskWhenAnyCast")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Threading::Tasks::Task_1_TaskWhenAnyCast<TResult> {}
#[cfg(feature = "System+Threading+Tasks+Task_1+TaskWhenAnyCast")]
impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::Task_1_TaskWhenAnyCast<TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
