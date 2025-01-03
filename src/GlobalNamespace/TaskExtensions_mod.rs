#[cfg(feature = "TaskExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TaskExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TaskExtensions => ""
    ."TaskExtensions"
);
#[cfg(feature = "TaskExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::TaskExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TaskExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::TaskExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TaskExtensions")]
impl crate::GlobalNamespace::TaskExtensions {
    #[cfg(feature = "TaskExtensions+_WaitAsyncInternal_d__3")]
    pub type _WaitAsyncInternal_d__3 = crate::GlobalNamespace::TaskExtensions__WaitAsyncInternal_d__3;
    #[cfg(feature = "TaskExtensions+_WaitAsyncInternal_d__4_1")]
    pub type _WaitAsyncInternal_d__4_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::TaskExtensions__WaitAsyncInternal_d__4_1<
        T,
    >;
    #[cfg(feature = "TaskExtensions+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::GlobalNamespace::TaskExtensions___c__DisplayClass3_0;
    #[cfg(feature = "TaskExtensions+__c__DisplayClass4_0_1")]
    pub type __c__DisplayClass4_0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::TaskExtensions___c__DisplayClass4_0_1<
        T,
    >;
    pub fn WaitAsyncInternal_Task0(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WaitAsyncInternal", (task, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitAsyncInternal_Task_1_1<T>(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WaitAsyncInternal", (task, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitAsync_Task0(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WaitAsync", (task, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitAsync_Task_1_1<T>(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WaitAsync", (task, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithCancellation<T>(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WithCancellation", (task, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TaskExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TaskExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
