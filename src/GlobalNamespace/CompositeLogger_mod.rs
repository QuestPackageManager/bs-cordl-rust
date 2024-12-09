#[cfg(feature = "CompositeLogger")]
#[repr(C)]
#[derive(Debug)]
pub struct CompositeLogger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _loggers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::IBeatSaberLogger,
    >,
}
#[cfg(feature = "CompositeLogger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CompositeLogger => ""
    ."CompositeLogger"
);
#[cfg(feature = "CompositeLogger")]
impl std::ops::Deref for crate::GlobalNamespace::CompositeLogger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CompositeLogger")]
impl std::ops::DerefMut for crate::GlobalNamespace::CompositeLogger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CompositeLogger")]
impl crate::GlobalNamespace::CompositeLogger {
    pub fn AddLogger(
        &mut self,
        logger: *mut crate::GlobalNamespace::IBeatSaberLogger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLogger", (logger))?;
        Ok(__cordl_ret)
    }
    pub fn LogError_Il2CppObject1(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppString,
        context: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogError", (message, context))?;
        Ok(__cordl_ret)
    }
    pub fn LogError_Il2CppString0(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogError", (message))?;
        Ok(__cordl_ret)
    }
    pub fn LogException_Exception0(
        &mut self,
        exception: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogException", (exception))?;
        Ok(__cordl_ret)
    }
    pub fn LogException_Il2CppObject1(
        &mut self,
        exception: *mut crate::System::Exception,
        context: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogException", (exception, context))?;
        Ok(__cordl_ret)
    }
    pub fn LogWarning_Il2CppObject1(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppString,
        context: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogWarning", (message, context))?;
        Ok(__cordl_ret)
    }
    pub fn LogWarning_Il2CppString0(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogWarning", (message))?;
        Ok(__cordl_ret)
    }
    pub fn Log_Il2CppObject1(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppString,
        context: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (message, context))?;
        Ok(__cordl_ret)
    }
    pub fn Log_Il2CppString0(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (message))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        loggers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::IBeatSaberLogger,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (loggers))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        loggers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::IBeatSaberLogger,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (loggers))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "CompositeLogger")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CompositeLogger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
