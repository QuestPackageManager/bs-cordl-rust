#[cfg(feature = "BGNet+Logging+Debug")]
#[repr(C)]
#[derive(Debug)]
pub struct Debug {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGNet+Logging+Debug")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGNet::Logging::Debug => "BGNet.Logging"."Debug"
);
#[cfg(feature = "BGNet+Logging+Debug")]
impl std::ops::Deref for crate::BGNet::Logging::Debug {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Logging+Debug")]
impl std::ops::DerefMut for crate::BGNet::Logging::Debug {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Logging+Debug")]
impl crate::BGNet::Logging::Debug {
    #[cfg(feature = "BGNet+Logging+Debug+ILogger")]
    type ILogger = crate::BGNet::Logging::Debug_ILogger;
    #[cfg(feature = "BGNet+Logging+Debug+LoggerLinkedList")]
    pub type LoggerLinkedList = crate::BGNet::Logging::Debug_LoggerLinkedList;
    pub fn AddLogger(
        logger: quest_hook::libil2cpp::Gc<crate::BGNet::Logging::Debug_ILogger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddLogger", (logger))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogError(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogError", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogException(
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogException", (exception, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogWarning(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogWarning", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn NoDomainReloadInit() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NoDomainReloadInit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveLogger(
        logger: quest_hook::libil2cpp::Gc<crate::BGNet::Logging::Debug_ILogger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveLogger", (logger))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGNet+Logging+Debug")]
impl quest_hook::libil2cpp::ObjectType for crate::BGNet::Logging::Debug {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGNet+Logging+Debug+ILogger")]
#[repr(C)]
#[derive(Debug)]
pub struct Debug_ILogger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGNet+Logging+Debug+ILogger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGNet::Logging::Debug_ILogger => "BGNet.Logging"
    ."Debug/ILogger"
);
#[cfg(feature = "BGNet+Logging+Debug+ILogger")]
impl std::ops::Deref for crate::BGNet::Logging::Debug_ILogger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Logging+Debug+ILogger")]
impl std::ops::DerefMut for crate::BGNet::Logging::Debug_ILogger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Logging+Debug+ILogger")]
impl crate::BGNet::Logging::Debug_ILogger {
    pub fn LogError(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogError", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogException(
        &mut self,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogException", (exception, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogInfo(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogInfo", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogWarning(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogWarning", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "BGNet+Logging+Debug+ILogger")]
impl quest_hook::libil2cpp::ObjectType for crate::BGNet::Logging::Debug_ILogger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGNet+Logging+Debug+LoggerLinkedList")]
#[repr(C)]
#[derive(Debug)]
pub struct Debug_LoggerLinkedList {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub logger: *mut crate::BGNet::Logging::Debug_ILogger,
    pub next: *mut crate::BGNet::Logging::Debug_LoggerLinkedList,
}
#[cfg(feature = "BGNet+Logging+Debug+LoggerLinkedList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGNet::Logging::Debug_LoggerLinkedList =>
    "BGNet.Logging"."Debug/LoggerLinkedList"
);
#[cfg(feature = "BGNet+Logging+Debug+LoggerLinkedList")]
impl std::ops::Deref for crate::BGNet::Logging::Debug_LoggerLinkedList {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Logging+Debug+LoggerLinkedList")]
impl std::ops::DerefMut for crate::BGNet::Logging::Debug_LoggerLinkedList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Logging+Debug+LoggerLinkedList")]
impl crate::BGNet::Logging::Debug_LoggerLinkedList {
    pub fn New(
        logger: quest_hook::libil2cpp::Gc<crate::BGNet::Logging::Debug_ILogger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (logger))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        logger: quest_hook::libil2cpp::Gc<crate::BGNet::Logging::Debug_ILogger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (logger))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGNet+Logging+Debug+LoggerLinkedList")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGNet::Logging::Debug_LoggerLinkedList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
