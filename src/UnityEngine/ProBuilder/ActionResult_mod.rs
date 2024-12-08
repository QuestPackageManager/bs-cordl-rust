#[cfg(feature = "UnityEngine+ProBuilder+ActionResult")]
#[repr(C)]
#[derive(Debug)]
pub struct ActionResult {
    __cordl_parent: crate::System::Object,
    pub _status_k__BackingField: crate::UnityEngine::ProBuilder::ActionResult_Status,
    pub _notification_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+ProBuilder+ActionResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ActionResult =>
    "UnityEngine.ProBuilder"."ActionResult"
);
#[cfg(feature = "UnityEngine+ProBuilder+ActionResult")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ActionResult {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ActionResult")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ActionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ActionResult")]
impl crate::UnityEngine::ProBuilder::ActionResult {
    #[cfg(feature = "UnityEngine+ProBuilder+ActionResult+Status")]
    pub type Status = crate::UnityEngine::ProBuilder::ActionResult_Status;
    pub fn _ctor(
        &mut self,
        status: crate::UnityEngine::ProBuilder::ActionResult_Status,
        notification: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (status, notification))?;
        Ok(__cordl_ret)
    }
    pub fn get_notification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_notification", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToBool(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ToBool", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::ActionResult_Status,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ProBuilder::ActionResult_Status = __cordl_object
            .invoke("get_status", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_notification(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_notification", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_status(
        &mut self,
        value: crate::UnityEngine::ProBuilder::ActionResult_Status,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_status", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        status: crate::UnityEngine::ProBuilder::ActionResult_Status,
        notification: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (status, notification))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ActionResult")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::ActionResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ActionResult+Status")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionResult_Status {
    Canceled = 2i32,
    Failure = 1i32,
    NoChange = 3i32,
    Success = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+ActionResult+Status")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ActionResult_Status =>
    "UnityEngine.ProBuilder"."ActionResult/Status"
);
