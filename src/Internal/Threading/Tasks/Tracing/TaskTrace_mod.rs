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
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Threading+Tasks+Tracing+TaskTrace")]
impl std::ops::DerefMut for crate::Internal::Threading::Tasks::Tracing::TaskTrace {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TaskScheduled",
                (
                    OriginatingTaskSchedulerID,
                    OriginatingTaskID,
                    TaskID,
                    CreatingTaskID,
                    TaskCreationOptions,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TaskWaitBegin_Asynchronous(
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TaskWaitBegin_Asynchronous",
                (OriginatingTaskSchedulerID, OriginatingTaskID, TaskID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TaskWaitBegin_Synchronous(
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TaskWaitBegin_Synchronous",
                (OriginatingTaskSchedulerID, OriginatingTaskID, TaskID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TaskWaitEnd(
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TaskWaitEnd",
                (OriginatingTaskSchedulerID, OriginatingTaskID, TaskID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Enabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Enabled", ())?;
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
