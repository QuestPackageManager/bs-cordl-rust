#[cfg(feature = "System+Threading+Tasks+Task_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Task_1<TResult: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Threading::Tasks::Task,
    pub m_result: TResult,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Threading+Tasks+Task_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::Task_1 < TResult > =>
    "System.Threading.Tasks"."Task`1" < TResult >
);
#[cfg(feature = "System+Threading+Tasks+Task_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::Tasks::Task_1<TResult> {
    type Target = crate::System::Threading::Tasks::Task;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::Tasks::Task_1<TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_1<
            TResult,
        > = __cordl_object.invoke("ConfigureAwait", (continueOnCapturedContext))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ContinueWith", (continuationAction, scheduler))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke(
                "ContinueWith",
                (continuationAction, scheduler, cancellationToken, continuationOptions),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TNewResult>,
        > = __cordl_object.invoke("ContinueWith", (continuationFunction))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TNewResult>,
        > = __cordl_object
            .invoke(
                "ContinueWith",
                (continuationFunction, cancellationToken, continuationOptions, scheduler),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TNewResult>,
        > = __cordl_object
            .invoke("ContinueWith", (continuationFunction, continuationOptions))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TNewResult>,
        > = __cordl_object
            .invoke(
                "ContinueWith",
                (continuationFunction, scheduler, cancellationToken, continuationOptions),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DangerousSetResult", (result))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            TResult,
        > = __cordl_object.invoke("GetAwaiter", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TResult = __cordl_object
            .invoke("GetResultCore", (waitCompletionNotification))?;
        Ok(__cordl_ret.into())
    }
    pub fn InnerInvoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InnerInvoke", ())?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "StartNew",
                (
                    parent,
                    function,
                    cancellationToken,
                    creationOptions,
                    internalOptions,
                    scheduler,
                ),
            )?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "StartNew",
                (
                    parent,
                    function,
                    state,
                    cancellationToken,
                    creationOptions,
                    internalOptions,
                    scheduler,
                ),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TrySetResult", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (function, cancellationToken))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (function, state, cancellationToken, creationOptions))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (state, options))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (result))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (canceled, result, creationOptions, ct))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::TaskFactory_1<TResult>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Factory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Result(&mut self) -> quest_hook::libil2cpp::Result<TResult>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TResult = __cordl_object.invoke("get_Result", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ResultOnSuccess(&mut self) -> quest_hook::libil2cpp::Result<TResult>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TResult = __cordl_object.invoke("get_ResultOnSuccess", ())?;
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::Task_1_TaskWhenAnyCast
    < TResult > => "System.Threading.Tasks"."Task`1/TaskWhenAnyCast" < TResult >
);
#[cfg(feature = "System+Threading+Tasks+Task_1+TaskWhenAnyCast")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::Tasks::Task_1_TaskWhenAnyCast<TResult> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Task_1+TaskWhenAnyCast")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::Tasks::Task_1_TaskWhenAnyCast<TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
