#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelReceiver")]
#[repr(C)]
#[derive(Debug)]
pub struct IChannelReceiver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelReceiver")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Channels::IChannelReceiver {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Channels";
    const CLASS_NAME: &'static str = "IChannelReceiver";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartListening", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_ChannelData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_ChannelData", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelReceiver")]
impl AsRef<crate::System::Runtime::Remoting::Channels::IChannel>
for crate::System::Runtime::Remoting::Channels::IChannelReceiver {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Channels::IChannel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+IChannelReceiver")]
impl AsMut<crate::System::Runtime::Remoting::Channels::IChannel>
for crate::System::Runtime::Remoting::Channels::IChannelReceiver {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Remoting::Channels::IChannel {
        unsafe { std::mem::transmute(self) }
    }
}
