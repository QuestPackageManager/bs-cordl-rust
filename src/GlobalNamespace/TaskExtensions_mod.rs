#[cfg(feature = "TaskExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TaskExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::TaskExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TaskExtensions";
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
    pub fn WaitAsyncInternal_Task0(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                2usize,
            >("WaitAsyncInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WaitAsyncInternal", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked((), (task, cancellationToken)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::Tasks::Task_1<T>,
                    >,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
                2usize,
            >("WaitAsyncInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WaitAsyncInternal", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = unsafe { method.invoke_unchecked((), (task, cancellationToken)) };
        Ok(__cordl_ret.into())
    }
    pub fn WaitAsync_Task0(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                2usize,
            >("WaitAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WaitAsync", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked((), (task, cancellationToken)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::Tasks::Task_1<T>,
                    >,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
                2usize,
            >("WaitAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WaitAsync", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = unsafe { method.invoke_unchecked((), (task, cancellationToken)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::Tasks::Task_1<T>,
                    >,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
                2usize,
            >("WithCancellation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WithCancellation", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = unsafe { method.invoke_unchecked((), (task, cancellationToken)) };
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
