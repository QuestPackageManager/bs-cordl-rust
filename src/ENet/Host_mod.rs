#[cfg(feature = "ENet+Host")]
#[repr(C)]
#[derive(Debug)]
pub struct Host {
    __cordl_parent: crate::System::Object,
    pub nativeHost: crate::System::IntPtr,
}
#[cfg(feature = "ENet+Host")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ENet::Host => "ENet"."Host"
);
#[cfg(feature = "ENet+Host")]
impl std::ops::Deref for crate::ENet::Host {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+Host")]
impl std::ops::DerefMut for crate::ENet::Host {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+Host")]
impl crate::ENet::Host {
    pub fn Service(
        &mut self,
        timeout: i32,
        event: quest_hook::libil2cpp::ByRefMut<crate::ENet::Event>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Service", (timeout, event))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsSet(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSet", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PeersCount(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_PeersCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NativeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("get_NativeData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PacketsSent(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_PacketsSent", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn PreventConnections(
        &mut self,
        state: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreventConnections", (state))?;
        Ok(__cordl_ret)
    }
    pub fn SetInterceptCallback_IntPtr0(
        &mut self,
        callback: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInterceptCallback", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn SetInterceptCallback_InterceptCallback1(
        &mut self,
        callback: *mut crate::ENet::InterceptCallback,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInterceptCallback", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn Broadcast_u8_ByRefMut0(
        &mut self,
        channelID: u8,
        packet: quest_hook::libil2cpp::ByRefMut<crate::ENet::Packet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Broadcast", (channelID, packet))?;
        Ok(__cordl_ret)
    }
    pub fn Broadcast_Peer1(
        &mut self,
        channelID: u8,
        packet: quest_hook::libil2cpp::ByRefMut<crate::ENet::Packet>,
        excludedPeer: crate::ENet::Peer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Broadcast", (channelID, packet, excludedPeer))?;
        Ok(__cordl_ret)
    }
    pub fn Broadcast_Il2CppArray2(
        &mut self,
        channelID: u8,
        packet: quest_hook::libil2cpp::ByRefMut<crate::ENet::Packet>,
        peers: *mut quest_hook::libil2cpp::Il2CppArray<crate::ENet::Peer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Broadcast", (channelID, packet, peers))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PacketsReceived(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_PacketsReceived", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckEvents(
        &mut self,
        event: quest_hook::libil2cpp::ByRefMut<crate::ENet::Event>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CheckEvents", (event))?;
        Ok(__cordl_ret)
    }
    pub fn get_BytesReceived(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_BytesReceived", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BytesSent(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_BytesSent", ())?;
        Ok(__cordl_ret)
    }
    pub fn Create(
        &mut self,
        address: crate::System::Nullable_1<crate::ENet::Address>,
        peerLimit: i32,
        channelLimit: i32,
        incomingBandwidth: u32,
        outgoingBandwidth: u32,
        bufferSize: i32,
        sslConfiguration: crate::ENet::SslConfiguration,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Create",
                (
                    address,
                    peerLimit,
                    channelLimit,
                    incomingBandwidth,
                    outgoingBandwidth,
                    bufferSize,
                    sslConfiguration,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetChecksumCallback_IntPtr0(
        &mut self,
        callback: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetChecksumCallback", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn SetChecksumCallback_ChecksumCallback1(
        &mut self,
        callback: *mut crate::ENet::ChecksumCallback,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetChecksumCallback", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowIfNotCreated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfNotCreated", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_NativeData(
        &mut self,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NativeData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetBandwidthLimit(
        &mut self,
        incomingBandwidth: u32,
        outgoingBandwidth: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBandwidthLimit", (incomingBandwidth, outgoingBandwidth))?;
        Ok(__cordl_ret)
    }
    pub fn SetChannelLimit(
        &mut self,
        channelLimit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetChannelLimit", (channelLimit))?;
        Ok(__cordl_ret)
    }
    pub fn SetMaxDuplicatePeers(
        &mut self,
        number: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMaxDuplicatePeers", (number))?;
        Ok(__cordl_ret)
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret)
    }
    pub fn Connect_Address0(
        &mut self,
        address: crate::ENet::Address,
    ) -> quest_hook::libil2cpp::Result<crate::ENet::Peer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::ENet::Peer = __cordl_object
            .invoke("Connect", (address))?;
        Ok(__cordl_ret)
    }
    pub fn Connect_i32_1(
        &mut self,
        address: crate::ENet::Address,
        channelLimit: i32,
    ) -> quest_hook::libil2cpp::Result<crate::ENet::Peer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::ENet::Peer = __cordl_object
            .invoke("Connect", (address, channelLimit))?;
        Ok(__cordl_ret)
    }
    pub fn Connect_i32_u32_2(
        &mut self,
        address: crate::ENet::Address,
        channelLimit: i32,
        data: u32,
    ) -> quest_hook::libil2cpp::Result<crate::ENet::Peer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::ENet::Peer = __cordl_object
            .invoke("Connect", (address, channelLimit, data))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "ENet+Host")]
impl quest_hook::libil2cpp::ObjectType for crate::ENet::Host {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
