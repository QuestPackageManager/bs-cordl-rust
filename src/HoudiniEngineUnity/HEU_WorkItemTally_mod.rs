#[cfg(feature = "HoudiniEngineUnity+HEU_WorkItemTally")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_WorkItemTally {
    __cordl_parent: crate::System::Object,
    pub _totalWorkItems: i32,
    pub _waitingWorkItems: i32,
    pub _scheduledWorkItems: i32,
    pub _cookingWorkItems: i32,
    pub _cookedWorkItems: i32,
    pub _erroredWorkItems: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_WorkItemTally")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_WorkItemTally =>
    "HoudiniEngineUnity"."HEU_WorkItemTally"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_WorkItemTally")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_WorkItemTally {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_WorkItemTally")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_WorkItemTally {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_WorkItemTally")]
impl crate::HoudiniEngineUnity::HEU_WorkItemTally {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn AnyWorkItemsFailed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AnyWorkItemsFailed", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProgressRatio(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ProgressRatio", ())?;
        Ok(__cordl_ret)
    }
    pub fn ZeroAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ZeroAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn AreAllWorkItemsComplete(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AreAllWorkItemsComplete", ())?;
        Ok(__cordl_ret)
    }
    pub fn AnyWorkItemsPending(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AnyWorkItemsPending", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_WorkItemTally")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_WorkItemTally {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
