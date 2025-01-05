#[cfg(feature = "HoudiniEngineUnity+HEU_Task")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_Task {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _status: crate::HoudiniEngineUnity::HEU_Task_TaskStatus,
    pub _result: crate::HoudiniEngineUnity::HEU_Task_TaskResult,
    pub _guid: crate::System::Guid,
    pub _taskCompletedDelegate: quest_hook::libil2cpp::Gc<
        crate::HoudiniEngineUnity::HEU_Task_TaskCallback,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Task")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_Task =>
    "HoudiniEngineUnity"."HEU_Task"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_Task")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_Task {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Task")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_Task {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Task")]
impl crate::HoudiniEngineUnity::HEU_Task {
    #[cfg(feature = "HoudiniEngineUnity+HEU_Task+TaskCallback")]
    pub type TaskCallback = crate::HoudiniEngineUnity::HEU_Task_TaskCallback;
    #[cfg(feature = "HoudiniEngineUnity+HEU_Task+TaskResult")]
    pub type TaskResult = crate::HoudiniEngineUnity::HEU_Task_TaskResult;
    #[cfg(feature = "HoudiniEngineUnity+HEU_Task+TaskStatus")]
    pub type TaskStatus = crate::HoudiniEngineUnity::HEU_Task_TaskStatus;
    pub fn CompleteTask(
        &mut self,
        result: crate::HoudiniEngineUnity::HEU_Task_TaskResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteTask", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoTask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoTask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn KillTask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KillTask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateTask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TaskGuid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object
            .invoke("get_TaskGuid", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Task")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_Task {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Task+TaskCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_Task_TaskCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Task+TaskCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_Task_TaskCallback =>
    "HoudiniEngineUnity"."HEU_Task/TaskCallback"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_Task+TaskCallback")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_Task_TaskCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Task+TaskCallback")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_Task_TaskCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Task+TaskCallback")]
impl crate::HoudiniEngineUnity::HEU_Task_TaskCallback {
    pub fn BeginInvoke(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Task>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (task, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Task+TaskCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_Task_TaskCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Task+TaskResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_Task_TaskResult {
    #[default]
    FAILED = 2i32,
    KILLED = 3i32,
    NONE = 0i32,
    SUCCESS = 1i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Task+TaskResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_Task_TaskResult =>
    "HoudiniEngineUnity"."HEU_Task/TaskResult"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_Task+TaskStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HEU_Task_TaskStatus {
    #[default]
    COMPLETED = 5i32,
    NONE = 0i32,
    PENDING_COMPLETE = 4i32,
    PENDING_START = 1i32,
    REQUIRE_UPDATE = 3i32,
    STARTED = 2i32,
    UNUSED = 6i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Task+TaskStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_Task_TaskStatus =>
    "HoudiniEngineUnity"."HEU_Task/TaskStatus"
);
