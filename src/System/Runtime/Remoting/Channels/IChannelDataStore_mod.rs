#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelDataStore")]
#[repr(C)]
#[derive(Debug)]
pub struct IChannelDataStore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelDataStore")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Channels::IChannelDataStore =>
    "System.Runtime.Remoting.Channels"."IChannelDataStore"
);
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelDataStore")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Channels::IChannelDataStore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelDataStore")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Channels::IChannelDataStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelDataStore")]
impl crate::System::Runtime::Remoting::Channels::IChannelDataStore {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelDataStore")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Channels::IChannelDataStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
