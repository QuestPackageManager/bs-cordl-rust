#[cfg(feature = "System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskFactory_1_FromAsyncTrimPromise_1<
    TResult: quest_hook::libil2cpp::Type,
    TInstance: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Threading::Tasks::Task_1<TResult>,
    pub m_thisRef: TInstance,
    pub m_endMethod: *mut crate::System::Func_3<
        TInstance,
        *mut crate::System::IAsyncResult,
        TResult,
    >,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
    __cordl_phantom_TInstance: std::marker::PhantomData<TInstance>,
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::TaskFactory_1_FromAsyncTrimPromise_1 < TResult, TInstance
    > => "System.Threading.Tasks"."TaskFactory`1/FromAsyncTrimPromise`1" < TResult,
    TInstance >
);
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
    TInstance: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::System::Threading::Tasks::TaskFactory_1_FromAsyncTrimPromise_1<
    TResult,
    TInstance,
> {
    type Target = crate::System::Threading::Tasks::Task_1<TResult>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
    TInstance: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::System::Threading::Tasks::TaskFactory_1_FromAsyncTrimPromise_1<
    TResult,
    TInstance,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
    TInstance: quest_hook::libil2cpp::Type,
> crate::System::Threading::Tasks::TaskFactory_1_FromAsyncTrimPromise_1<
    TResult,
    TInstance,
> {
    pub fn Complete(
        &mut self,
        thisRef: TInstance,
        endMethod: *mut crate::System::Func_3<
            TInstance,
            *mut crate::System::IAsyncResult,
            TResult,
        >,
        asyncResult: *mut crate::System::IAsyncResult,
        requiresSynchronization: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TInstance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Complete",
                (thisRef, endMethod, asyncResult, requiresSynchronization),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        thisRef: TInstance,
        endMethod: *mut crate::System::Func_3<
            TInstance,
            *mut crate::System::IAsyncResult,
            TResult,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (thisRef, endMethod))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        thisRef: TInstance,
        endMethod: *mut crate::System::Func_3<
            TInstance,
            *mut crate::System::IAsyncResult,
            TResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TInstance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (thisRef, endMethod))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
    TInstance: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::TaskFactory_1_FromAsyncTrimPromise_1<
    TResult,
    TInstance,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskFactory_1<TResult: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub m_defaultCancellationToken: crate::System::Threading::CancellationToken,
    pub m_defaultScheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
    pub m_defaultCreationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
    pub m_defaultContinuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::TaskFactory_1 <
    TResult > => "System.Threading.Tasks"."TaskFactory`1" < TResult >
);
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::Tasks::TaskFactory_1<TResult> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::Tasks::TaskFactory_1<TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Threading::Tasks::TaskFactory_1<TResult> {
    #[cfg(feature = "System+Threading+Tasks+TaskFactory_1+FromAsyncTrimPromise_1")]
    pub type FromAsyncTrimPromise_1<TInstance: quest_hook::libil2cpp::Type> = crate::System::Threading::Tasks::TaskFactory_1_FromAsyncTrimPromise_1<
        TResult,
        TInstance,
    >;
    #[cfg(feature = "System+Threading+Tasks+TaskFactory_1+__c__DisplayClass41_0_2")]
    pub type __c__DisplayClass41_0_2<
        TArg1: quest_hook::libil2cpp::Type,
        TArg2: quest_hook::libil2cpp::Type,
    > = crate::System::Threading::Tasks::TaskFactory_1___c__DisplayClass41_0_2<
        TResult,
        TArg1,
        TArg2,
    >;
    #[cfg(feature = "System+Threading+Tasks+TaskFactory_1+__c__DisplayClass35_0")]
    pub type __c__DisplayClass35_0 = crate::System::Threading::Tasks::TaskFactory_1___c__DisplayClass35_0<
        TResult,
    >;
    #[cfg(feature = "System+Threading+Tasks+TaskFactory_1+__c__DisplayClass38_0_1")]
    pub type __c__DisplayClass38_0_1<TArg1: quest_hook::libil2cpp::Type> = crate::System::Threading::Tasks::TaskFactory_1___c__DisplayClass38_0_1<
        TResult,
        TArg1,
    >;
    pub fn FromAsync_Func_3_Object0(
        &mut self,
        beginMethod: *mut crate::System::Func_3<
            *mut crate::System::AsyncCallback,
            *mut crate::System::Object,
            *mut crate::System::IAsyncResult,
        >,
        endMethod: *mut crate::System::Func_2<*mut crate::System::IAsyncResult, TResult>,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<TResult> = __cordl_object
            .invoke("FromAsync", (beginMethod, endMethod, state))?;
        Ok(__cordl_ret)
    }
    pub fn FromAsync_Func_4_TArg1_Object1<TArg1>(
        &mut self,
        beginMethod: *mut crate::System::Func_4<
            TArg1,
            *mut crate::System::AsyncCallback,
            *mut crate::System::Object,
            *mut crate::System::IAsyncResult,
        >,
        endMethod: *mut crate::System::Func_2<*mut crate::System::IAsyncResult, TResult>,
        arg1: TArg1,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TArg1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<TResult> = __cordl_object
            .invoke("FromAsync", (beginMethod, endMethod, arg1, state))?;
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
    pub fn StartNew(
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
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_CancellationToken_TaskCreationOptions_TaskContinuationOptions_TaskScheduler1(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        continuationOptions: crate::System::Threading::Tasks::TaskContinuationOptions,
        scheduler: *mut crate::System::Threading::Tasks::TaskScheduler,
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
                (cancellationToken, creationOptions, continuationOptions, scheduler),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskFactory_1")]
impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::TaskFactory_1<TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
