#[cfg(feature = "HoudiniEngineUnity+HEU_TaskManager")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_TaskManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TaskManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::HoudiniEngineUnity::HEU_TaskManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HEU_TaskManager";
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
#[cfg(feature = "HoudiniEngineUnity+HEU_TaskManager")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_TaskManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TaskManager")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_TaskManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TaskManager")]
impl crate::HoudiniEngineUnity::HEU_TaskManager {
    pub fn AddTask(
        task: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddTask", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompleteTask(
        task: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Task>,
        result: crate::HoudiniEngineUnity::HEU_Task_TaskResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompleteTask", (task, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteTask(
        task: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExecuteTask", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTask(
        taskGuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTask", (taskGuid))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalCompleteTask(
        task: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalCompleteTask", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn KillTask_Guid1(
        taskGuid: crate::System::Guid,
        bRemove: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("KillTask", (taskGuid, bRemove))?;
        Ok(__cordl_ret.into())
    }
    pub fn KillTask_HEU_Task0(
        task: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Task>,
        bRemove: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("KillTask", (task, bRemove))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveTask(
        task: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveTask", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Update", ())?;
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
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TaskManager")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_TaskManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
