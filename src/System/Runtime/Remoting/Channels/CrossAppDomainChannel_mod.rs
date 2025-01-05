#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainChannel")]
#[repr(C)]
#[derive(Debug)]
pub struct CrossAppDomainChannel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainChannel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Channels::CrossAppDomainChannel =>
    "System.Runtime.Remoting.Channels"."CrossAppDomainChannel"
);
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainChannel")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Channels::CrossAppDomainChannel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainChannel")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Channels::CrossAppDomainChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainChannel")]
impl crate::System::Runtime::Remoting::Channels::CrossAppDomainChannel {
    pub fn CreateMessageSink(
        &mut self,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        uri: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        > = __cordl_object.invoke("CreateMessageSink", (url, data, uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterCrossAppDomainChannel() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterCrossAppDomainChannel", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_ChannelName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ChannelName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChannelPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ChannelPriority", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainChannel")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Channels::CrossAppDomainChannel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainChannel")]
impl AsRef<crate::System::Runtime::Remoting::Channels::IChannel>
for crate::System::Runtime::Remoting::Channels::CrossAppDomainChannel {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Channels::IChannel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainChannel")]
impl AsMut<crate::System::Runtime::Remoting::Channels::IChannel>
for crate::System::Runtime::Remoting::Channels::CrossAppDomainChannel {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Remoting::Channels::IChannel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainChannel")]
impl AsRef<crate::System::Runtime::Remoting::Channels::IChannelReceiver>
for crate::System::Runtime::Remoting::Channels::CrossAppDomainChannel {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Channels::IChannelReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainChannel")]
impl AsMut<crate::System::Runtime::Remoting::Channels::IChannelReceiver>
for crate::System::Runtime::Remoting::Channels::CrossAppDomainChannel {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Channels::IChannelReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainChannel")]
impl AsRef<crate::System::Runtime::Remoting::Channels::IChannelSender>
for crate::System::Runtime::Remoting::Channels::CrossAppDomainChannel {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Channels::IChannelSender {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+CrossAppDomainChannel")]
impl AsMut<crate::System::Runtime::Remoting::Channels::IChannelSender>
for crate::System::Runtime::Remoting::Channels::CrossAppDomainChannel {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Channels::IChannelSender {
        unsafe { std::mem::transmute(self) }
    }
}
