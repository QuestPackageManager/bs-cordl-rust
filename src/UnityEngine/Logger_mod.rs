#[cfg(feature = "UnityEngine+Logger")]
#[repr(C)]
#[derive(Debug)]
pub struct Logger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _logHandler_k__BackingField: *mut crate::UnityEngine::ILogHandler,
    pub _logEnabled_k__BackingField: bool,
    pub _filterLogType_k__BackingField: crate::UnityEngine::LogType,
}
#[cfg(feature = "UnityEngine+Logger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Logger => "UnityEngine"."Logger"
);
#[cfg(feature = "UnityEngine+Logger")]
impl std::ops::Deref for crate::UnityEngine::Logger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Logger")]
impl std::ops::DerefMut for crate::UnityEngine::Logger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Logger")]
impl crate::UnityEngine::Logger {
    pub fn GetString(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetString", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLogTypeAllowed(
        &mut self,
        logType: crate::UnityEngine::LogType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsLogTypeAllowed", (logType))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogError(
        &mut self,
        tag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogError", (tag, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogException(
        &mut self,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        context: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogException", (exception, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogFormat_Il2CppString_Il2CppArray0(
        &mut self,
        logType: crate::UnityEngine::LogType,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogFormat", (logType, format, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogFormat_Object_Il2CppString_Il2CppArray1(
        &mut self,
        logType: crate::UnityEngine::LogType,
        context: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogFormat", (logType, context, format, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log_LogType_Il2CppObject0(
        &mut self,
        logType: crate::UnityEngine::LogType,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (logType, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log_Object1(
        &mut self,
        logType: crate::UnityEngine::LogType,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (logType, message, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        logHandler: quest_hook::libil2cpp::Gc<crate::UnityEngine::ILogHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (logHandler))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        logHandler: quest_hook::libil2cpp::Gc<crate::UnityEngine::ILogHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (logHandler))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_filterLogType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LogType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LogType = __cordl_object
            .invoke("get_filterLogType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_logEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_logEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_logHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ILogHandler>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ILogHandler> = __cordl_object
            .invoke("get_logHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_filterLogType(
        &mut self,
        value: crate::UnityEngine::LogType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_filterLogType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_logEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_logEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_logHandler(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ILogHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_logHandler", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Logger")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Logger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Logger")]
impl AsRef<crate::UnityEngine::ILogHandler> for crate::UnityEngine::Logger {
    fn as_ref(&self) -> &crate::UnityEngine::ILogHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Logger")]
impl AsMut<crate::UnityEngine::ILogHandler> for crate::UnityEngine::Logger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ILogHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Logger")]
impl AsRef<crate::UnityEngine::ILogger> for crate::UnityEngine::Logger {
    fn as_ref(&self) -> &crate::UnityEngine::ILogger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Logger")]
impl AsMut<crate::UnityEngine::ILogger> for crate::UnityEngine::Logger {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ILogger {
        unsafe { std::mem::transmute(self) }
    }
}
