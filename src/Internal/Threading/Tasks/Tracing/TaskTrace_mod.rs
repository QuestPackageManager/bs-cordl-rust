#[cfg(feature = "Internal+Threading+Tasks+Tracing+TaskTrace")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskTrace {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Internal+Threading+Tasks+Tracing+TaskTrace")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Internal::Threading::Tasks::Tracing::TaskTrace {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Internal.Threading.Tasks.Tracing";
    const CLASS_NAME: &'static str = "TaskTrace";
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
#[cfg(feature = "Internal+Threading+Tasks+Tracing+TaskTrace")]
impl std::ops::Deref for crate::Internal::Threading::Tasks::Tracing::TaskTrace {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Threading+Tasks+Tracing+TaskTrace")]
impl std::ops::DerefMut for crate::Internal::Threading::Tasks::Tracing::TaskTrace {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Threading+Tasks+Tracing+TaskTrace")]
impl crate::Internal::Threading::Tasks::Tracing::TaskTrace {
    pub fn TaskScheduled(
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
        CreatingTaskID: i32,
        TaskCreationOptions: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32, i32, i32, i32),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("TaskScheduled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TaskScheduled", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        OriginatingTaskSchedulerID,
                        OriginatingTaskID,
                        TaskID,
                        CreatingTaskID,
                        TaskCreationOptions,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TaskWaitBegin_Asynchronous(
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("TaskWaitBegin_Asynchronous")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TaskWaitBegin_Asynchronous", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (OriginatingTaskSchedulerID, OriginatingTaskID, TaskID),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TaskWaitBegin_Synchronous(
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("TaskWaitBegin_Synchronous")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TaskWaitBegin_Synchronous", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (OriginatingTaskSchedulerID, OriginatingTaskID, TaskID),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TaskWaitEnd(
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("TaskWaitEnd")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TaskWaitEnd", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (OriginatingTaskSchedulerID, OriginatingTaskID, TaskID),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Enabled() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_Enabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Enabled", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Internal+Threading+Tasks+Tracing+TaskTrace")]
impl quest_hook::libil2cpp::ObjectType
for crate::Internal::Threading::Tasks::Tracing::TaskTrace {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
