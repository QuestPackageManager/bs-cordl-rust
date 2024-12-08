#[cfg(feature = "Internal+Runtime+Augments+TaskTraceCallbacks")]
#[repr(C)]
#[derive(Debug)]
pub struct TaskTraceCallbacks {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Internal+Runtime+Augments+TaskTraceCallbacks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Internal::Runtime::Augments::TaskTraceCallbacks
    => "Internal.Runtime.Augments"."TaskTraceCallbacks"
);
#[cfg(feature = "Internal+Runtime+Augments+TaskTraceCallbacks")]
impl std::ops::Deref for crate::Internal::Runtime::Augments::TaskTraceCallbacks {
    type Target = crate::System::Object;
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
    pub fn TaskWaitBegin_Synchronous(
        &mut self,
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TaskWaitBegin_Synchronous",
                (OriginatingTaskSchedulerID, OriginatingTaskID, TaskID),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TaskWaitBegin_Asynchronous(
        &mut self,
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TaskWaitBegin_Asynchronous",
                (OriginatingTaskSchedulerID, OriginatingTaskID, TaskID),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_Enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Enabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn TaskScheduled(
        &mut self,
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
        CreatingTaskID: i32,
        TaskCreationOptions: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
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
        Ok(__cordl_ret)
    }
    pub fn TaskWaitEnd(
        &mut self,
        OriginatingTaskSchedulerID: i32,
        OriginatingTaskID: i32,
        TaskID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TaskWaitEnd",
                (OriginatingTaskSchedulerID, OriginatingTaskID, TaskID),
            )?;
        Ok(__cordl_ret)
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
