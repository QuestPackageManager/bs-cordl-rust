#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelSender")]
#[repr(C)]
#[derive(Debug)]
pub struct IChannelSender {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelSender")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Channels::IChannelSender =>
    "System.Runtime.Remoting.Channels"."IChannelSender"
);
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelSender")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Channels::IChannelSender {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelSender")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Channels::IChannelSender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelSender")]
impl crate::System::Runtime::Remoting::Channels::IChannelSender {
    pub fn CreateMessageSink(
        &mut self,
        url: *mut quest_hook::libil2cpp::Il2CppString,
        remoteChannelData: *mut quest_hook::libil2cpp::Il2CppObject,
        objectURI: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Remoting::Messaging::IMessageSink,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Messaging::IMessageSink = __cordl_object
            .invoke("CreateMessageSink", (url, remoteChannelData, objectURI))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelSender")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Channels::IChannelSender {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
