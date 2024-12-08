#[cfg(feature = "System+Runtime+Remoting+Channels+IServerChannelSinkProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IServerChannelSinkProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IServerChannelSinkProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Channels::IServerChannelSinkProvider =>
    "System.Runtime.Remoting.Channels"."IServerChannelSinkProvider"
);
#[cfg(feature = "System+Runtime+Remoting+Channels+IServerChannelSinkProvider")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Channels::IServerChannelSinkProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IServerChannelSinkProvider")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Channels::IServerChannelSinkProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IServerChannelSinkProvider")]
impl crate::System::Runtime::Remoting::Channels::IServerChannelSinkProvider {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn set_Next(
        &mut self,
        value: *mut crate::System::Runtime::Remoting::Channels::IServerChannelSinkProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Next", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IServerChannelSinkProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Channels::IServerChannelSinkProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
