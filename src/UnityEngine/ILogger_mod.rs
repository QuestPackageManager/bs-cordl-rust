#[cfg(feature = "UnityEngine+ILogger")]
#[repr(C)]
#[derive(Debug)]
pub struct ILogger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ILogger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ILogger => "UnityEngine"."ILogger"
);
#[cfg(feature = "UnityEngine+ILogger")]
impl std::ops::Deref for crate::UnityEngine::ILogger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ILogger")]
impl std::ops::DerefMut for crate::UnityEngine::ILogger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ILogger")]
impl crate::UnityEngine::ILogger {
    pub fn LogError(
        &mut self,
        tag: *mut crate::System::String,
        message: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogError", (tag, message))?;
        Ok(__cordl_ret)
    }
    pub fn LogFormat(
        &mut self,
        logType: crate::UnityEngine::LogType,
        format: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogFormat", (logType, format, args))?;
        Ok(__cordl_ret)
    }
    pub fn Log_LogType_Object0(
        &mut self,
        logType: crate::UnityEngine::LogType,
        message: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (logType, message))?;
        Ok(__cordl_ret)
    }
    pub fn Log_Object1(
        &mut self,
        logType: crate::UnityEngine::LogType,
        message: *mut crate::System::Object,
        context: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (logType, message, context))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_logEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_logEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_logHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::ILogHandler> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ILogHandler = __cordl_object
            .invoke("get_logHandler", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ILogger")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ILogger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}