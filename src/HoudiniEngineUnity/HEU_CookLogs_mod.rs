#[cfg(feature = "HoudiniEngineUnity+HEU_CookLogs")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_CookLogs {
    __cordl_parent: crate::System::Object,
    pub _cookLogs: *mut crate::System::Text::StringBuilder,
    pub _currentCookLogCount: i32,
    pub _lastLogStr: *mut crate::System::String,
    pub _uniqueStrOnly: bool,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_CookLogs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_CookLogs =>
    "HoudiniEngineUnity"."HEU_CookLogs"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_CookLogs")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_CookLogs {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_CookLogs")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_CookLogs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_CookLogs")]
impl crate::HoudiniEngineUnity::HEU_CookLogs {
    pub const MAX_COOK_LOG_COUNT: i32 = 9001i32;
    pub const MaxLogSize: i64 = 50000000i64;
    pub fn AppendCookLog(
        &mut self,
        logStr: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendCookLog", (logStr))?;
        Ok(__cordl_ret)
    }
    pub fn ClearCookLog(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearCookLog", ())?;
        Ok(__cordl_ret)
    }
    pub fn DeleteCookingFile(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteCookingFile", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCookLogFilePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetCookLogFilePath", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCookLogString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetCookLogString", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetFileSizeOfLogFile(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetFileSizeOfLogFile", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn WriteToLogFile(
        &mut self,
        logStr: *mut crate::System::String,
        checkLastLogStr: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteToLogFile", (logStr, checkLastLogStr))?;
        Ok(__cordl_ret)
    }
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
}
#[cfg(feature = "HoudiniEngineUnity+HEU_CookLogs")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_CookLogs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
