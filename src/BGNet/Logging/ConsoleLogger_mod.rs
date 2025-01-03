#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
#[repr(C)]
#[derive(Debug)]
pub struct ConsoleLogger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGNet::Logging::ConsoleLogger => "BGNet.Logging"
    ."ConsoleLogger"
);
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
impl std::ops::Deref for crate::BGNet::Logging::ConsoleLogger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
impl std::ops::DerefMut for crate::BGNet::Logging::ConsoleLogger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
impl crate::BGNet::Logging::ConsoleLogger {
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
impl quest_hook::libil2cpp::ObjectType for crate::BGNet::Logging::ConsoleLogger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
impl AsRef<crate::BGNet::Logging::Debug_ILogger>
for crate::BGNet::Logging::ConsoleLogger {
    fn as_ref(&self) -> &crate::BGNet::Logging::Debug_ILogger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BGNet+Logging+ConsoleLogger")]
impl AsMut<crate::BGNet::Logging::Debug_ILogger>
for crate::BGNet::Logging::ConsoleLogger {
    fn as_mut(&mut self) -> &mut crate::BGNet::Logging::Debug_ILogger {
        unsafe { std::mem::transmute(self) }
    }
}
