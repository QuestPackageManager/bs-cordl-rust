#[cfg(feature = "LiteNetLib+NetPeer")]
#[repr(C)]
#[derive(Debug)]
pub struct NetPeer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _rtt: i32,
    pub _avgRtt: i32,
    pub _rttCount: i32,
    pub _resendDelay: f64,
    pub _pingSendTimer: i32,
    pub _rttResetTimer: i32,
    pub _pingTimer: *mut crate::System::Diagnostics::Stopwatch,
    pub _timeSinceLastPacket: i32,
    pub _remoteDelta: i64,
    pub _packetPool: *mut crate::LiteNetLib::NetPacketPool,
    pub _flushLock: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _sendLock: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _shutdownLock: *mut quest_hook::libil2cpp::Il2CppObject,
    pub NextPeer: *mut crate::LiteNetLib::NetPeer,
    pub PrevPeer: *mut crate::LiteNetLib::NetPeer,
    pub _unreliableChannel: *mut crate::System::Collections::Generic::Queue_1<
        *mut crate::LiteNetLib::NetPacket,
    >,
    pub _channels: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::LiteNetLib::BaseChannel,
    >,
    pub _headChannel: *mut crate::LiteNetLib::BaseChannel,
    pub _mtu: i32,
    pub _mtuIdx: i32,
    pub _finishMtu: bool,
    pub _mtuCheckTimer: i32,
    pub _mtuCheckAttempts: i32,
    pub _mtuMutex: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _fragmentId: u16,
    pub _holdedFragments: *mut crate::System::Collections::Generic::Dictionary_2<
        u16,
        *mut crate::LiteNetLib::NetPeer_IncomingFragments,
    >,
    pub _deliveredFramgnets: *mut crate::System::Collections::Generic::Dictionary_2<
        u16,
        u16,
    >,
    pub _mergeData: *mut crate::LiteNetLib::NetPacket,
    pub _mergePos: i32,
    pub _mergeCount: i32,
    pub _connectAttempts: i32,
    pub _connectTimer: i32,
    pub _connectTime: i64,
    pub _connectNum: u8,
    pub _connectionState: crate::LiteNetLib::ConnectionState,
    pub _shutdownPacket: *mut crate::LiteNetLib::NetPacket,
    pub _shutdownTimer: i32,
    pub _pingPacket: *mut crate::LiteNetLib::NetPacket,
    pub _pongPacket: *mut crate::LiteNetLib::NetPacket,
    pub _connectRequestPacket: *mut crate::LiteNetLib::NetPacket,
    pub _connectAcceptPacket: *mut crate::LiteNetLib::NetPacket,
    pub EndPoint: *mut crate::System::Net::IPEndPoint,
    pub NetManager: *mut crate::LiteNetLib::NetManager,
    pub Id: i32,
    pub Tag: *mut quest_hook::libil2cpp::Il2CppObject,
    pub Statistics: *mut crate::LiteNetLib::NetStatistics,
}
#[cfg(feature = "LiteNetLib+NetPeer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetPeer => "LiteNetLib"."NetPeer"
);
#[cfg(feature = "LiteNetLib+NetPeer")]
impl std::ops::Deref for crate::LiteNetLib::NetPeer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetPeer")]
impl std::ops::DerefMut for crate::LiteNetLib::NetPeer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetPeer")]
impl crate::LiteNetLib::NetPeer {
    pub const MaxMtuCheckAttempts: i32 = 4i32;
    pub const MtuCheckDelay: i32 = 1000i32;
    pub const ShutdownDelay: i32 = 300i32;
    #[cfg(feature = "LiteNetLib+NetPeer+IncomingFragments")]
    pub type IncomingFragments = crate::LiteNetLib::NetPeer_IncomingFragments;
    pub fn AddReliablePacket(
        &mut self,
        method: crate::LiteNetLib::DeliveryMethod,
        p: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddReliablePacket", (method, p))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateChannel(
        &mut self,
        idx: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::BaseChannel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::BaseChannel> = __cordl_object
            .invoke("CreateChannel", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn Disconnect_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disconnect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Disconnect_Il2CppArray0(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disconnect", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn Disconnect_Il2CppArray_i32_i32_2(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disconnect", (data, start, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Disconnect_NetDataWriter1(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disconnect", (writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxSinglePacketSize(
        &mut self,
        options: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetMaxSinglePacketSize", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPacketsCountInReliableQueue(
        &mut self,
        channelNumber: u8,
        ordered: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPacketsCountInReliableQueue", (channelNumber, ordered))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_NetManager_IPEndPoint_i32_0(
        netManager: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetManager>,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (netManager, remoteEndPoint, id))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i64_u8_2(
        netManager: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetManager>,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        id: i32,
        connectId: i64,
        connectNum: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (netManager, remoteEndPoint, id, connectId, connectNum),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_u8_NetDataWriter1(
        netManager: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetManager>,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        id: i32,
        connectNum: u8,
        connectData: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (netManager, remoteEndPoint, id, connectNum, connectData),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessConnectAccept(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetConnectAcceptPacket>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessConnectAccept", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessConnectRequest(
        &mut self,
        connRequest: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::NetConnectRequestPacket,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::LiteNetLib::ConnectRequestResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::LiteNetLib::ConnectRequestResult = __cordl_object
            .invoke("ProcessConnectRequest", (connRequest))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessDisconnect(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<crate::LiteNetLib::DisconnectResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::LiteNetLib::DisconnectResult = __cordl_object
            .invoke("ProcessDisconnect", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMtuPacket(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMtuPacket", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessPacket(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPacket", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecycleAndDeliver(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecycleAndDeliver", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reject(
        &mut self,
        connectionId: i64,
        connectionNumber: u8,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reject", (connectionId, connectionNumber, data, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendInternal(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        length: i32,
        channelNumber: u8,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendInternal",
                (data, start, length, channelNumber, deliveryMethod, userData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SendMerged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendMerged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendUserData(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendUserData", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendWithDeliveryEvent_Il2CppArray_i32_i32_u8_DeliveryMethod_Il2CppObject1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        length: i32,
        channelNumber: u8,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendWithDeliveryEvent",
                (data, start, length, channelNumber, deliveryMethod, userData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SendWithDeliveryEvent_Il2CppArray_u8_DeliveryMethod_Il2CppObject0(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        channelNumber: u8,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendWithDeliveryEvent",
                (data, channelNumber, deliveryMethod, userData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SendWithDeliveryEvent_NetDataWriter_u8_DeliveryMethod_Il2CppObject2(
        &mut self,
        dataWriter: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        channelNumber: u8,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendWithDeliveryEvent",
                (dataWriter, channelNumber, deliveryMethod, userData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_Il2CppArray_DeliveryMethod0(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (data, deliveryMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_Il2CppArray_i32_i32_DeliveryMethod2(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        length: i32,
        options: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (data, start, length, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_Il2CppArray_i32_i32_u8_DeliveryMethod5(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        length: i32,
        channelNumber: u8,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (data, start, length, channelNumber, deliveryMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_Il2CppArray_u8_DeliveryMethod3(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        channelNumber: u8,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (data, channelNumber, deliveryMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_NetDataWriter_DeliveryMethod1(
        &mut self,
        dataWriter: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (dataWriter, deliveryMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send_NetDataWriter_u8_DeliveryMethod4(
        &mut self,
        dataWriter: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        channelNumber: u8,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (dataWriter, channelNumber, deliveryMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMtu(
        &mut self,
        mtuIdx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMtu", (mtuIdx))?;
        Ok(__cordl_ret.into())
    }
    pub fn Shutdown(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        length: i32,
        force: bool,
    ) -> quest_hook::libil2cpp::Result<crate::LiteNetLib::ShutdownResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::LiteNetLib::ShutdownResult = __cordl_object
            .invoke("Shutdown", (data, start, length, force))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
        deltaTime: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (deltaTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateMtuLogic(
        &mut self,
        deltaTime: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMtuLogic", (deltaTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRoundTripTime(
        &mut self,
        roundTripTime: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRoundTripTime", (roundTripTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_NetManager_IPEndPoint_i32_0(
        &mut self,
        netManager: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetManager>,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (netManager, remoteEndPoint, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_u8_2(
        &mut self,
        netManager: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetManager>,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        id: i32,
        connectId: i64,
        connectNum: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (netManager, remoteEndPoint, id, connectId, connectNum))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u8_NetDataWriter1(
        &mut self,
        netManager: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetManager>,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        id: i32,
        connectNum: u8,
        connectData: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (netManager, remoteEndPoint, id, connectNum, connectData))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConnectTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_ConnectTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConnectionNum(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_ConnectionNum", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConnectionState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::LiteNetLib::ConnectionState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::LiteNetLib::ConnectionState = __cordl_object
            .invoke("get_ConnectionState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Mtu(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Mtu", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Ping(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Ping", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RemoteTimeDelta(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_RemoteTimeDelta", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RemoteUtcTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_RemoteUtcTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ResendDelay(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_ResendDelay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TimeSinceLastPacket(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TimeSinceLastPacket", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ConnectionNum(
        &mut self,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ConnectionNum", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+NetPeer")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetPeer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+NetPeer+IncomingFragments")]
#[repr(C)]
#[derive(Debug)]
pub struct NetPeer_IncomingFragments {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Fragments: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::LiteNetLib::NetPacket,
    >,
    pub ReceivedCount: i32,
    pub TotalSize: i32,
    pub ChannelId: u8,
}
#[cfg(feature = "LiteNetLib+NetPeer+IncomingFragments")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetPeer_IncomingFragments =>
    "LiteNetLib"."NetPeer/IncomingFragments"
);
#[cfg(feature = "LiteNetLib+NetPeer+IncomingFragments")]
impl std::ops::Deref for crate::LiteNetLib::NetPeer_IncomingFragments {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetPeer+IncomingFragments")]
impl std::ops::DerefMut for crate::LiteNetLib::NetPeer_IncomingFragments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetPeer+IncomingFragments")]
impl crate::LiteNetLib::NetPeer_IncomingFragments {
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
#[cfg(feature = "LiteNetLib+NetPeer+IncomingFragments")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetPeer_IncomingFragments {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
