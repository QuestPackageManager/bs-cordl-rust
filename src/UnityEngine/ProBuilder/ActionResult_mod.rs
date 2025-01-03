#[cfg(feature = "UnityEngine+ProBuilder+ActionResult")]
#[repr(C)]
#[derive(Debug)]
pub struct ActionResult {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _status_k__BackingField: crate::UnityEngine::ProBuilder::ActionResult_Status,
    pub _notification_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+ProBuilder+ActionResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ActionResult =>
    "UnityEngine.ProBuilder"."ActionResult"
);
#[cfg(feature = "UnityEngine+ProBuilder+ActionResult")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ActionResult {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn FromBool(success: bool) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBool", (success))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        status: crate::UnityEngine::ProBuilder::ActionResult_Status,
        notification: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (status, notification))?;
        Ok(__cordl_object.into())
    }
    pub fn ToBool(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ToBool", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        status: crate::UnityEngine::ProBuilder::ActionResult_Status,
        notification: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (status, notification))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NoSelection() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_NoSelection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Success() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Success", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserCanceled() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UserCanceled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_notification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_notification", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        res: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (res))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_notification(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_notification", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
