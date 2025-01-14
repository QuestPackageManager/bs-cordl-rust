#[cfg(feature = "BGNet+Core+ITaskUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct ITaskUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGNet+Core+ITaskUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::BGNet::Core::ITaskUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGNet.Core";
    const CLASS_NAME: &'static str = "ITaskUtility";
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
#[cfg(feature = "BGNet+Core+ITaskUtility")]
impl std::ops::Deref for crate::BGNet::Core::ITaskUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+ITaskUtility")]
impl std::ops::DerefMut for crate::BGNet::Core::ITaskUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+ITaskUtility")]
impl crate::BGNet::Core::ITaskUtility {
    pub fn CancellationTokenWithDelay(
        &mut self,
        timeSpan: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::CancellationToken> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::TimeSpan),
                crate::System::Threading::CancellationToken,
                1usize,
            >("CancellationTokenWithDelay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CancellationTokenWithDelay", 1usize
                )
            });
        let __cordl_ret: crate::System::Threading::CancellationToken = unsafe {
            method.invoke_unchecked(self, (timeSpan))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContinueWith<T1, T2>(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T1>>,
        continuation: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T1>>,
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T2>>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T2>>,
    >
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Threading::Tasks::Task_1<T1>,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task_1<T1>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task_1<T2>,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T2>>,
                2usize,
            >("ContinueWith")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ContinueWith", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T2>,
        > = unsafe { method.invoke_unchecked(self, (task, continuation)) };
        Ok(__cordl_ret.into())
    }
    pub fn Delay(
        &mut self,
        timeSpan: crate::System::TimeSpan,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::TimeSpan, crate::System::Threading::CancellationToken),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                2usize,
            >("Delay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Delay", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked(self, (timeSpan, cancellationToken)) };
        Ok(__cordl_ret.into())
    }
    pub fn Run_Action0(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                2usize,
            >("Run")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Run", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked(self, (action, cancellationToken)) };
        Ok(__cordl_ret.into())
    }
    pub fn Run_Func_1_1(
        &mut self,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
            >,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_1<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                        >,
                    >,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                2usize,
            >("Run")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Run", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked(self, (func, cancellationToken)) };
        Ok(__cordl_ret.into())
    }
    pub fn Run_Func_1_2<T>(
        &mut self,
        func: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
                2usize,
            >("Run")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Run", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = unsafe { method.invoke_unchecked(self, (func, cancellationToken)) };
        Ok(__cordl_ret.into())
    }
    pub fn Wait_Task0(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Wait")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Wait", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (task))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Wait_Task_1_1<T>(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>),
                T,
                1usize,
            >("Wait")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Wait", 1usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked(self, (task)) };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "BGNet+Core+ITaskUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::BGNet::Core::ITaskUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
