#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelReceiver")]
#[repr(C)]
#[derive(Debug)]
pub struct IChannelReceiver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelReceiver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Channels::IChannelReceiver =>
    "System.Runtime.Remoting.Channels"."IChannelReceiver"
);
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelReceiver")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Channels::IChannelReceiver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelReceiver")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Channels::IChannelReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelReceiver")]
impl crate::System::Runtime::Remoting::Channels::IChannelReceiver {
    pub fn StartListening(
        &mut self,
        data: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartListening", (data))?;
        Ok(__cordl_ret)
    }
    pub fn get_ChannelData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_ChannelData", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelReceiver")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Channels::IChannelReceiver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
