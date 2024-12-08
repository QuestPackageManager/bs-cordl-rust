#[cfg(feature = "System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskFactory_CompleteOnInvokePromise {
    __cordl_parent: crate::System::Threading::Tasks::Task_1<
        *mut crate::System::Threading::Tasks::Task,
    >,
    pub _tasks: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::System::Threading::Tasks::Task,
    >,
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise =>
    "System.Threading.Tasks"."TaskFactory/CompleteOnInvokePromise"
);
#[cfg(feature = "System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise")]
impl std::ops::Deref
for crate::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise {
    type Target = crate::System::Threading::Tasks::Task_1<
        *mut crate::System::Threading::Tasks::Task,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise")]
impl std::ops::DerefMut
for crate::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise")]
impl crate::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise {
    pub fn Invoke(
        &mut self,
        completingTask: *mut crate::System::Threading::Tasks::Task,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (completingTask))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        tasks: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::System::Threading::Tasks::Task,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tasks))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        tasks: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::System::Threading::Tasks::Task,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tasks))?;
        Ok(__cordl_ret)
    }
    pub fn get_InvokeMayRunArbitraryCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_InvokeMayRunArbitraryCode", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskFactory {
    __cordl_parent: crate::System::Object,
    pub m_defaultCancellationToken: crate::System::Threading::CancellationToken,
    pub m_defaultScheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
    pub m_defaultCreationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    pub m_defaultContinuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::TaskFactory =>
    "System.Threading.Tasks"."TaskFactory"
);
#[cfg(feature = "System+Threading+Tasks+TaskFactory")]
impl std::ops::Deref for crate::System::Threading::Tasks::TaskFactory {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::TaskFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory")]
impl crate::System::Threading::Tasks::TaskFactory {
    #[cfg(feature = "System+Threading+Tasks+TaskFactory+CompleteOnInvokePromise")]
    pub type CompleteOnInvokePromise = crate::System::Threading::Tasks::TaskFactory_CompleteOnInvokePromise;
    pub fn FromAsync_Func_4_Object0<TArg1>(
        &mut self,
        beginMethod: *mut crate::System::Func_4<
            TArg1,
            *mut crate::System::AsyncCallback,
            *mut crate::System::Object,
            *mut crate::System::IAsyncResult,
        >,
        endMethod: *mut crate::System::Action_1<*mut crate::System::IAsyncResult>,
        arg1: TArg1,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task>
    where
        TArg1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("FromAsync", (beginMethod, endMethod, arg1, state))?;
        Ok(__cordl_ret)
    }
    pub fn FromAsync_Func_4_Object_TaskCreationOptions1<TArg1>(
        &mut self,
        beginMethod: *mut crate::System::Func_4<
            TArg1,
            *mut crate::System::AsyncCallback,
            *mut crate::System::Object,
            *mut crate::System::IAsyncResult,
        >,
        endMethod: *mut crate::System::Action_1<*mut crate::System::IAsyncResult>,
        arg1: TArg1,
        state: *mut crate::System::Object,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task>
    where
        TArg1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke(
                "FromAsync",
                (beginMethod, endMethod, arg1, state, creationOptions),
            )?;
        Ok(__cordl_ret)
    }
    pub fn FromAsync_Func_5_TArg2_Object2<TArg1, TArg2>(
        &mut self,
        beginMethod: *mut crate::System::Func_5<
            TArg1,
            TArg2,
            *mut crate::System::AsyncCallback,
            *mut crate::System::Object,
            *mut crate::System::IAsyncResult,
        >,
        endMethod: *mut crate::System::Action_1<*mut crate::System::IAsyncResult>,
        arg1: TArg1,
        arg2: TArg2,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task>
    where
        TArg1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("FromAsync", (beginMethod, endMethod, arg1, arg2, state))?;
        Ok(__cordl_ret)
    }
    pub fn FromAsync_Func_5_TArg2_Object_TaskCreationOptions3<TArg1, TArg2>(
        &mut self,
        beginMethod: *mut crate::System::Func_5<
            TArg1,
            TArg2,
            *mut crate::System::AsyncCallback,
            *mut crate::System::Object,
            *mut crate::System::IAsyncResult,
        >,
        endMethod: *mut crate::System::Action_1<*mut crate::System::IAsyncResult>,
        arg1: TArg1,
        arg2: TArg2,
        state: *mut crate::System::Object,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task>
    where
        TArg1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TArg2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke(
                "FromAsync",
                (beginMethod, endMethod, arg1, arg2, state, creationOptions),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_CancellationToken_TaskCreationOptions_TaskContinuationOptions_TaskScheduler1(
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (cancellationToken, creationOptions, continuationOptions, scheduler),
            )?;
        Ok(__cordl_object)
    }
    pub fn StartNew_Action_1_Object_CancellationToken_TaskCreationOptions_TaskScheduler0(
        &mut self,
        action: *mut crate::System::Action_1<*mut crate::System::Object>,
        state: *mut crate::System::Object,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke(
                "StartNew",
                (action, state, cancellationToken, creationOptions, scheduler),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartNew_Func_1_CancellationToken_TaskCreationOptions_TaskScheduler1<TResult>(
        &mut self,
        function: *mut crate::System::Func_1<TResult>,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<TResult> = __cordl_object
            .invoke(
                "StartNew",
                (function, cancellationToken, creationOptions, scheduler),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartNew_Func_2_Object_CancellationToken_TaskCreationOptions_TaskScheduler2<
        TResult,
    >(
        &mut self,
        function: *mut crate::System::Func_2<*mut crate::System::Object, TResult>,
        state: *mut crate::System::Object,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<TResult> = __cordl_object
            .invoke(
                "StartNew",
                (function, state, cancellationToken, creationOptions, scheduler),
            )?;
        Ok(__cordl_ret)
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
    pub fn _ctor_CancellationToken_TaskCreationOptions_TaskContinuationOptions_TaskScheduler1(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (cancellationToken, creationOptions, continuationOptions, scheduler),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::Tasks::TaskFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}