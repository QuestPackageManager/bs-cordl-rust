#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvoyInfo {
    __cordl_parent: crate::System::Object,
    pub envoySinks: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
}
#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::EnvoyInfo =>
    "System.Runtime.Remoting"."EnvoyInfo"
);
#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
impl std::ops::Deref for crate::System::Runtime::Remoting::EnvoyInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::EnvoyInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
impl crate::System::Runtime::Remoting::EnvoyInfo {
    pub fn _ctor(
        &mut self,
        sinks: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sinks))?;
        Ok(__cordl_ret)
    }
    pub fn get_EnvoySinks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink = __cordl_object
            .invoke("get_EnvoySinks", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        sinks: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sinks))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Runtime::Remoting::EnvoyInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
