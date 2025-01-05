#[cfg(feature = "System+Runtime+Remoting+Channels+ChannelServices")]
#[repr(C)]
#[derive(Debug)]
pub struct ChannelServices {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+ChannelServices")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Channels::ChannelServices =>
    "System.Runtime.Remoting.Channels"."ChannelServices"
);
#[cfg(feature = "System+Runtime+Remoting+Channels+ChannelServices")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Channels::ChannelServices {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+ChannelServices")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Channels::ChannelServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+ChannelServices")]
impl crate::System::Runtime::Remoting::Channels::ChannelServices {
    pub fn CheckIncomingMessage(
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::ReturnMessage,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::ReturnMessage,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckIncomingMessage", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckReturnMessage(
        callMsg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
        retMsg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckReturnMessage", (callMsg, retMsg))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClientChannelSinkChain_ByRefMut0(
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        remoteChannelData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        objectUri: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateClientChannelSinkChain",
                (url, remoteChannelData, objectUri),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClientChannelSinkChain_Gc_ByRefMut1(
        sender: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Channels::IChannelSender,
        >,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        channelDataArray: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        objectUri: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateClientChannelSinkChain",
                (sender, url, channelDataArray, objectUri),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateProvider(
        prov: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ProviderData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateProvider", (prov))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentChannelInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentChannelInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLocalCall(
        callMsg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLocalCall", (callMsg))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterChannelConfig(
        channel: quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::ChannelData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterChannelConfig", (channel))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterChannel_Gc0(
        chnl: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Channels::IChannel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterChannel", (chnl))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterChannel__cordl_bool1(
        chnl: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Channels::IChannel,
        >,
        ensureSecurity: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterChannel", (chnl, ensureSecurity))?;
        Ok(__cordl_ret.into())
    }
    pub fn SyncDispatchMessage(
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SyncDispatchMessage", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CrossContextChannel() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::CrossContextChannel,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::CrossContextChannel,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CrossContextChannel", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+ChannelServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Channels::ChannelServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
