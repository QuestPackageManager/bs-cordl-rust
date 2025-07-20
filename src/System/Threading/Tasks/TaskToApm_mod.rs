#[cfg(feature = "System+Threading+Tasks+TaskToApm")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskToApm {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Threading::Tasks::TaskToApm {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "TaskToApm";
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
#[cfg(feature = "System+Threading+Tasks+TaskToApm")]
impl std::ops::Deref for crate::System::Threading::Tasks::TaskToApm {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm")]
impl std::ops::DerefMut for crate::System::Threading::Tasks::TaskToApm {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm")]
impl crate::System::Threading::Tasks::TaskToApm {
    #[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
    pub type TaskWrapperAsyncResult = crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult;
    pub fn Begin(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        3usize,
                    >("Begin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Begin",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info.invoke_unchecked((), (task, callback, state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn End_IAsyncResult0(
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("End")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "End",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (asyncResult))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn End_IAsyncResult1<TResult>(
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<TResult>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        TResult,
                        1usize,
                    >("End")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "End",
                            1usize
                        )
                    })
            });
        let __cordl_ret: TResult = unsafe {
            cordl_method_info.invoke_unchecked((), (asyncResult))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallbackWhenTaskCompletes(
        antecedent: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InvokeCallbackWhenTaskCompletes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeCallbackWhenTaskCompletes", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (antecedent, callback, asyncResult))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::Tasks::TaskToApm {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskToApm_TaskWrapperAsyncResult {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    pub _state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _completedSynchronously: bool,
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "TaskToApm/TaskWrapperAsyncResult";
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
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
impl std::ops::Deref
for crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
impl std::ops::DerefMut
for crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
impl crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult {
    pub fn New(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        completedSynchronously: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (task, state, completedSynchronously))?;
        Ok(__cordl_object.into())
    }
    pub fn System_IAsyncResult_get_AsyncState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("System.IAsyncResult.get_AsyncState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.IAsyncResult.get_AsyncState", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_IAsyncResult_get_AsyncWaitHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::WaitHandle>,
                        0usize,
                    >("System.IAsyncResult.get_AsyncWaitHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.IAsyncResult.get_AsyncWaitHandle", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::WaitHandle,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_IAsyncResult_get_CompletedSynchronously(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >("System.IAsyncResult.get_CompletedSynchronously")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.IAsyncResult.get_CompletedSynchronously", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_IAsyncResult_get_IsCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >("System.IAsyncResult.get_IsCompleted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.IAsyncResult.get_IsCompleted", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        completedSynchronously: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (task, state, completedSynchronously))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
impl AsRef<crate::System::IAsyncResult>
for crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult {
    fn as_ref(&self) -> &crate::System::IAsyncResult {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+TaskToApm+TaskWrapperAsyncResult")]
impl AsMut<crate::System::IAsyncResult>
for crate::System::Threading::Tasks::TaskToApm_TaskWrapperAsyncResult {
    fn as_mut(&mut self) -> &mut crate::System::IAsyncResult {
        unsafe { std::mem::transmute(self) }
    }
}
