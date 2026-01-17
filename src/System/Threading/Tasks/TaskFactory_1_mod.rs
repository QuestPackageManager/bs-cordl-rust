#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskFactory_1")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct TaskFactory_1<TResult: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_defaultCancellationToken: crate::System::Threading::CancellationToken,
    pub m_defaultScheduler:
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::TaskScheduler>,
    pub m_defaultCreationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    pub m_defaultContinuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskFactory_1")]
unsafe impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::System::Threading::Tasks::TaskFactory_1<TResult>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "TaskFactory`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("System.Threading.Tasks", "TaskFactory`1")
                .unwrap()
                .make_generic::<(TResult)>()
                .unwrap()
                .unwrap()
        })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::System::Threading::Tasks::TaskFactory_1<TResult>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::System::Threading::Tasks::TaskFactory_1<TResult>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1")]
impl<TResult: quest_hook::libil2cpp::Type> crate::System::Threading::Tasks::TaskFactory_1<TResult> {
    #[cfg(feature = "System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
    pub type FromAsyncTrimPromise_1<TInstance: quest_hook::libil2cpp::Type> =
        crate::System::Threading::Tasks::TaskFactory_1_FromAsyncTrimPromise_1<TResult, TInstance>;
    pub fn FromAsyncCoreLogic(
        iar: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        endFunction: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, TResult>,
        >,
        endAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>>,
        >,
        promise: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
        requiresSynchronization: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Func_2<
                                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                TResult,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>("FromAsyncCoreLogic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FromAsyncCoreLogic",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    iar,
                    endFunction,
                    endAction,
                    promise,
                    requiresSynchronization,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromAsyncImpl_Func_3_Il2CppObject_TaskCreationOptions1(
        beginMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
            >,
        >,
        endFunction: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, TResult>,
        >,
        endAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>>,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_3<
                                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                    TResult,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResult>,
                        >,
                        5usize,
                    >("FromAsyncImpl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromAsyncImpl", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (beginMethod, endFunction, endAction, state, creationOptions),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromAsyncImpl_Func_4_TArg1_Il2CppObject_TaskCreationOptions2<TArg1>(
        beginMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_4<
                TArg1,
                quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
            >,
        >,
        endFunction: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, TResult>,
        >,
        endAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>>,
        >,
        arg1: TArg1,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_4<
                                    TArg1,
                                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                    TResult,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                >,
                            >,
                            TArg1,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResult>,
                        >,
                        6usize,
                    >("FromAsyncImpl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromAsyncImpl", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    beginMethod,
                    endFunction,
                    endAction,
                    arg1,
                    state,
                    creationOptions,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromAsyncImpl_Func_5_TArg1_TArg2_Il2CppObject_TaskCreationOptions3<TArg1, TArg2>(
        beginMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_5<
                TArg1,
                TArg2,
                quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
            >,
        >,
        endFunction: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, TResult>,
        >,
        endAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>>,
        >,
        arg1: TArg1,
        arg2: TArg2,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_5<
                                    TArg1,
                                    TArg2,
                                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                    TResult,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                >,
                            >,
                            TArg1,
                            TArg2,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResult>,
                        >,
                        7usize,
                    >("FromAsyncImpl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromAsyncImpl", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    beginMethod,
                    endFunction,
                    endAction,
                    arg1,
                    arg2,
                    state,
                    creationOptions,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromAsyncImpl_Func_6_TArg1_TArg2_TArg3_Il2CppObject_TaskCreationOptions4<
        TArg1,
        TArg2,
        TArg3,
    >(
        beginMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_6<
                TArg1,
                TArg2,
                TArg3,
                quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
            >,
        >,
        endFunction: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, TResult>,
        >,
        endAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>>,
        >,
        arg1: TArg1,
        arg2: TArg2,
        arg3: TArg3,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_6<
                                    TArg1,
                                    TArg2,
                                    TArg3,
                                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                    TResult,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                >,
                            >,
                            TArg1,
                            TArg2,
                            TArg3,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResult>,
                        >,
                        8usize,
                    >("FromAsyncImpl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromAsyncImpl", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    beginMethod,
                    endFunction,
                    endAction,
                    arg1,
                    arg2,
                    arg3,
                    state,
                    creationOptions,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromAsyncImpl_IAsyncResult_TaskCreationOptions_TaskScheduler0(
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        endFunction: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, TResult>,
        >,
        endAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>>,
        >,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        scheduler: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::TaskScheduler>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                    TResult,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                >,
                            >,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResult>,
                        >,
                        5usize,
                    >("FromAsyncImpl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromAsyncImpl", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    asyncResult,
                    endFunction,
                    endAction,
                    creationOptions,
                    scheduler,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromAsyncTrim<TInstance, TArgs>(
        thisRef: TInstance,
        args: TArgs,
        beginMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_5<
                TInstance,
                TArgs,
                quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
            >,
        >,
        endMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                TInstance,
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                TResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TInstance: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArgs: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            TInstance,
                            TArgs,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_5<
                                    TInstance,
                                    TArgs,
                                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_3<
                                    TInstance,
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                    TResult,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResult>,
                        >,
                        4usize,
                    >("FromAsyncTrim")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromAsyncTrim", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (thisRef, args, beginMethod, endMethod))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromAsync_Func_3_Il2CppObject0(
        &mut self,
        beginMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
            >,
        >,
        endMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, TResult>,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_3<
                                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                    TResult,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResult>,
                        >,
                        3usize,
                    >("FromAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromAsync", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (beginMethod, endMethod, state))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromAsync_Func_4_TArg1_Il2CppObject1<TArg1>(
        &mut self,
        beginMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_4<
                TArg1,
                quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
            >,
        >,
        endMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, TResult>,
        >,
        arg1: TArg1,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_4<
                                    TArg1,
                                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                    TResult,
                                >,
                            >,
                            TArg1,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResult>,
                        >,
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
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (beginMethod, endMethod, arg1, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromAsync_Func_6_TArg1_TArg2_TArg3_Il2CppObject2<TArg1, TArg2, TArg3>(
        &mut self,
        beginMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_6<
                TArg1,
                TArg2,
                TArg3,
                quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
            >,
        >,
        endMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>, TResult>,
        >,
        arg1: TArg1,
        arg2: TArg2,
        arg3: TArg3,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_6<
                                    TArg1,
                                    TArg2,
                                    TArg3,
                                    quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                    TResult,
                                >,
                            >,
                            TArg1,
                            TArg2,
                            TArg3,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResult>,
                        >,
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
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (beginMethod, endMethod, arg1, arg2, arg3, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_CancellationToken_TaskCreationOptions_TaskContinuationOptions_TaskScheduler1(
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
        scheduler: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::TaskScheduler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                cancellationToken,
                creationOptions,
                continuationOptions,
                scheduler,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn StartNew(
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
        scheduler: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::TaskScheduler>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResult>>,
    >
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::TaskScheduler,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResult>,
                        >,
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
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    function,
                    state,
                    cancellationToken,
                    creationOptions,
                    scheduler,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CancellationToken_TaskCreationOptions_TaskContinuationOptions_TaskScheduler1(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
        scheduler: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::TaskScheduler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::System::Threading::CancellationToken,
                        crate::System::Threading::Tasks::TaskCreationOptions,
                        crate::System::Threading::Tasks::TaskContinuationOptions,
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::TaskScheduler>,
                    ), quest_hook::libil2cpp::Void, 4usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cancellationToken,
                    creationOptions,
                    continuationOptions,
                    scheduler,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskFactory_1")]
impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
    for crate::System::Threading::Tasks::TaskFactory_1<TResult>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct TaskFactory_1_FromAsyncTrimPromise_1<
    TResult: quest_hook::libil2cpp::Type,
    TInstance: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Threading::Tasks::Task_1<TResult>,
    pub m_thisRef: TInstance,
    pub m_endMethod: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            TInstance,
            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
            TResult,
        >,
    >,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
    __cordl_phantom_TInstance: std::marker::PhantomData<TInstance>,
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
unsafe impl<TResult: quest_hook::libil2cpp::Type, TInstance: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type
    for crate::System::Threading::Tasks::TaskFactory_1_FromAsyncTrimPromise_1<TResult, TInstance>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "TaskFactory`1/FromAsyncTrimPromise`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "System.Threading.Tasks",
                "TaskFactory`1/FromAsyncTrimPromise`1",
            )
            .unwrap()
            .make_generic::<(TResult, TInstance)>()
            .unwrap()
            .unwrap()
        })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
impl<TResult: quest_hook::libil2cpp::Type, TInstance: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::System::Threading::Tasks::TaskFactory_1_FromAsyncTrimPromise_1<TResult, TInstance>
{
    type Target = crate::System::Threading::Tasks::Task_1<TResult>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
impl<TResult: quest_hook::libil2cpp::Type, TInstance: quest_hook::libil2cpp::Type>
    std::ops::DerefMut
    for crate::System::Threading::Tasks::TaskFactory_1_FromAsyncTrimPromise_1<TResult, TInstance>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
impl<TResult: quest_hook::libil2cpp::Type, TInstance: quest_hook::libil2cpp::Type>
    crate::System::Threading::Tasks::TaskFactory_1_FromAsyncTrimPromise_1<TResult, TInstance>
{
    pub fn Complete(
        &mut self,
        thisRef: TInstance,
        endMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                TInstance,
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                TResult,
            >,
        >,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
        requiresSynchronization: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TInstance: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        TInstance,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Func_3<
                                TInstance,
                                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                TResult,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>("Complete")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Complete",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (thisRef, endMethod, asyncResult, requiresSynchronization),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompleteFromAsyncResult(
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TInstance: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CompleteFromAsyncResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompleteFromAsyncResult", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (asyncResult))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        thisRef: TInstance,
        endMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                TInstance,
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                TResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TInstance: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (thisRef, endMethod))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        thisRef: TInstance,
        endMethod: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                TInstance,
                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                TResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TInstance: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        TInstance,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Func_3<
                                TInstance,
                                quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                                TResult,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (thisRef, endMethod))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
impl<TResult: quest_hook::libil2cpp::Type, TInstance: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ObjectType
    for crate::System::Threading::Tasks::TaskFactory_1_FromAsyncTrimPromise_1<TResult, TInstance>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
