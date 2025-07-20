#[cfg(feature = "Internal+Runtime+Augments+TaskTraceCallbacks")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskTraceCallbacks {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Internal+Runtime+Augments+TaskTraceCallbacks")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Internal::Runtime::Augments::TaskTraceCallbacks {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Internal.Runtime.Augments";
    const CLASS_NAME: &'static str = "TaskTraceCallbacks";
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
#[cfg(feature = "Internal+Runtime+Augments+TaskTraceCallbacks")]
impl std::ops::Deref for crate::Internal::Runtime::Augments::TaskTraceCallbacks {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Runtime+Augments+TaskTraceCallbacks")]
impl std::ops::DerefMut for crate::Internal::Runtime::Augments::TaskTraceCallbacks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Runtime+Augments+TaskTraceCallbacks")]
impl crate::Internal::Runtime::Augments::TaskTraceCallbacks {
    pub fn TaskScheduled(
        &mut self,
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
        CreatingTaskID: i32,
        TaskCreationOptions: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Internal::Runtime::Augments::TaskTraceCallbacks as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, i32, i32),
                quest_hook::libil2cpp::Void,
                5usize,
            >("TaskScheduled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Internal::Runtime::Augments::TaskTraceCallbacks as
                    quest_hook::libil2cpp::Type > ::class(), "TaskScheduled", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
        &mut self,
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Internal::Runtime::Augments::TaskTraceCallbacks as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("TaskWaitBegin_Asynchronous")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Internal::Runtime::Augments::TaskTraceCallbacks as
                    quest_hook::libil2cpp::Type > ::class(),
                    "TaskWaitBegin_Asynchronous", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (OriginatingTaskSchedulerID, OriginatingTaskID, TaskID),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TaskWaitBegin_Synchronous(
        &mut self,
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Internal::Runtime::Augments::TaskTraceCallbacks as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("TaskWaitBegin_Synchronous")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Internal::Runtime::Augments::TaskTraceCallbacks as
                    quest_hook::libil2cpp::Type > ::class(), "TaskWaitBegin_Synchronous",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (OriginatingTaskSchedulerID, OriginatingTaskID, TaskID),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TaskWaitEnd(
        &mut self,
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Internal::Runtime::Augments::TaskTraceCallbacks as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("TaskWaitEnd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Internal::Runtime::Augments::TaskTraceCallbacks as
                    quest_hook::libil2cpp::Type > ::class(), "TaskWaitEnd", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (OriginatingTaskSchedulerID, OriginatingTaskID, TaskID),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Internal::Runtime::Augments::TaskTraceCallbacks as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_Enabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Internal::Runtime::Augments::TaskTraceCallbacks as
                    quest_hook::libil2cpp::Type > ::class(), "get_Enabled", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Internal+Runtime+Augments+TaskTraceCallbacks")]
impl quest_hook::libil2cpp::ObjectType
for crate::Internal::Runtime::Augments::TaskTraceCallbacks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
