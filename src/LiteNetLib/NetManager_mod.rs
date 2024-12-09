#[cfg(feature = "LiteNetLib+NetManager")]
#[repr(C)]
#[derive(Debug)]
pub struct NetManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _socket: *mut crate::LiteNetLib::NetSocket,
    pub _logicThread: *mut crate::System::Threading::Thread,
    pub _netEventsQueue: *mut crate::System::Collections::Generic::Queue_1<
        *mut crate::LiteNetLib::NetEvent,
    >,
    pub _netEventsPool: *mut crate::System::Collections::Generic::Stack_1<
        *mut crate::LiteNetLib::NetEvent,
    >,
    pub _netEventListener: *mut crate::LiteNetLib::INetEventListener,
    pub _deliveryEventListener: *mut crate::LiteNetLib::IDeliveryEventListener,
    pub _peersDict: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Net::IPEndPoint,
        *mut crate::LiteNetLib::NetPeer,
    >,
    pub _requestsDict: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Net::IPEndPoint,
        *mut crate::LiteNetLib::ConnectionRequest,
    >,
    pub _peersLock: *mut crate::System::Threading::ReaderWriterLockSlim,
    pub _headPeer: *mut crate::LiteNetLib::NetPeer,
    pub _connectedPeersCount: i32,
    pub _connectedPeerListCache: *mut crate::System::Collections::Generic::List_1<
        *mut crate::LiteNetLib::NetPeer,
    >,
    pub _peersArray: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::LiteNetLib::NetPeer,
    >,
    pub _extraPacketLayer: *mut crate::LiteNetLib::Layers::PacketLayerBase,
    pub _lastPeerId: i32,
    pub _peerIds: *mut crate::System::Collections::Generic::Queue_1<i32>,
    pub _channelsCount: u8,
    pub NetPacketPool: *mut crate::LiteNetLib::NetPacketPool,
    pub UnconnectedMessagesEnabled: bool,
    pub NatPunchEnabled: bool,
    pub UpdateTime: i32,
    pub PingInterval: i32,
    pub DisconnectTimeout: i32,
    pub SimulatePacketLoss: bool,
    pub SimulateLatency: bool,
    pub SimulationPacketLossChance: i32,
    pub SimulationMinLatency: i32,
    pub SimulationMaxLatency: i32,
    pub UnsyncedEvents: bool,
    pub UnsyncedDeliveryEvent: bool,
    pub BroadcastReceiveEnabled: bool,
    pub ReconnectDelay: i32,
    pub MaxConnectAttempts: i32,
    pub ReuseAddress: bool,
    pub Statistics: *mut crate::LiteNetLib::NetStatistics,
    pub EnableStatistics: bool,
    pub NatPunchModule: *mut crate::LiteNetLib::NatPunchModule,
    pub AutoRecycle: bool,
    pub IPv6Enabled: bool,
    pub ThreadPriority: crate::System::Threading::ThreadPriority,
}
#[cfg(feature = "LiteNetLib+NetManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetManager => "LiteNetLib"
    ."NetManager"
);
#[cfg(feature = "LiteNetLib+NetManager")]
impl std::ops::Deref for crate::LiteNetLib::NetManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetManager")]
impl std::ops::DerefMut for crate::LiteNetLib::NetManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetManager")]
impl crate::LiteNetLib::NetManager {
    #[cfg(feature = "LiteNetLib+NetManager+IPEndPointComparer")]
    pub type IPEndPointComparer = crate::LiteNetLib::NetManager_IPEndPointComparer;
    #[cfg(feature = "LiteNetLib+NetManager+NetPeerEnumerator")]
    pub type NetPeerEnumerator = crate::LiteNetLib::NetManager_NetPeerEnumerator;
    pub fn AddPeer(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPeer", (peer))?;
        Ok(__cordl_ret)
    }
    pub fn Connect_IPEndPoint_Il2CppString2(
        &mut self,
        target: *mut crate::System::Net::IPEndPoint,
        key: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::LiteNetLib::NetPeer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::NetPeer = __cordl_object
            .invoke("Connect", (target, key))?;
        Ok(__cordl_ret)
    }
    pub fn Connect_IPEndPoint_NetDataWriter3(
        &mut self,
        target: *mut crate::System::Net::IPEndPoint,
        connectionData: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<*mut crate::LiteNetLib::NetPeer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::NetPeer = __cordl_object
            .invoke("Connect", (target, connectionData))?;
        Ok(__cordl_ret)
    }
    pub fn Connect_Il2CppString_i32_Il2CppString0(
        &mut self,
        address: *mut quest_hook::libil2cpp::Il2CppString,
        port: i32,
        key: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::LiteNetLib::NetPeer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::NetPeer = __cordl_object
            .invoke("Connect", (address, port, key))?;
        Ok(__cordl_ret)
    }
    pub fn Connect_Il2CppString_i32_NetDataWriter1(
        &mut self,
        address: *mut quest_hook::libil2cpp::Il2CppString,
        port: i32,
        connectionData: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<*mut crate::LiteNetLib::NetPeer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::NetPeer = __cordl_object
            .invoke("Connect", (address, port, connectionData))?;
        Ok(__cordl_ret)
    }
    pub fn ConnectionLatencyUpdated(
        &mut self,
        fromPeer: *mut crate::LiteNetLib::NetPeer,
        latency: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConnectionLatencyUpdated", (fromPeer, latency))?;
        Ok(__cordl_ret)
    }
    pub fn CreateEvent(
        &mut self,
        _cordl_type: crate::LiteNetLib::NetEvent_EType,
        peer: *mut crate::LiteNetLib::NetPeer,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        errorCode: crate::System::Net::Sockets::SocketError,
        latency: i32,
        disconnectReason: crate::LiteNetLib::DisconnectReason,
        connectionRequest: *mut crate::LiteNetLib::ConnectionRequest,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
        readerSource: *mut crate::LiteNetLib::NetPacket,
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CreateEvent",
                (
                    _cordl_type,
                    peer,
                    remoteEndPoint,
                    errorCode,
                    latency,
                    disconnectReason,
                    connectionRequest,
                    deliveryMethod,
                    readerSource,
                    userData,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateReceiveEvent(
        &mut self,
        packet: *mut crate::LiteNetLib::NetPacket,
        method: crate::LiteNetLib::DeliveryMethod,
        fromPeer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateReceiveEvent", (packet, method, fromPeer))?;
        Ok(__cordl_ret)
    }
    pub fn DataReceived(
        &mut self,
        reusableBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        count: i32,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DataReceived", (reusableBuffer, count, remoteEndPoint))?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectAll_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectAll_Il2CppArray_i32_i32_1(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectAll", (data, start, count))?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectPeerForce_DisconnectReason_SocketError_NetPacket0(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        reason: crate::LiteNetLib::DisconnectReason,
        socketErrorCode: crate::System::Net::Sockets::SocketError,
        eventData: *mut crate::LiteNetLib::NetPacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectPeerForce", (peer, reason, socketErrorCode, eventData))?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectPeerForce_NetPeer1(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectPeerForce", (peer))?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectPeer_DisconnectReason_SocketError__cordl_bool_Il2CppArray_i32_i32_NetPacket0(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        reason: crate::LiteNetLib::DisconnectReason,
        socketErrorCode: crate::System::Net::Sockets::SocketError,
        force: bool,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        count: i32,
        eventData: *mut crate::LiteNetLib::NetPacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DisconnectPeer",
                (peer, reason, socketErrorCode, force, data, start, count, eventData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectPeer_Il2CppArray2(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectPeer", (peer, data))?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectPeer_Il2CppArray_i32_i32_4(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectPeer", (peer, data, start, count))?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectPeer_NetDataWriter3(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectPeer", (peer, writer))?;
        Ok(__cordl_ret)
    }
    pub fn DisconnectPeer_NetPeer1(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectPeer", (peer))?;
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
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::LiteNetLib::NetManager_NetPeerEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::LiteNetLib::NetManager_NetPeerEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNextPeerId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetNextPeerId", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPeerById(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::LiteNetLib::NetPeer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::NetPeer = __cordl_object
            .invoke("GetPeerById", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetPeersCount(
        &mut self,
        peerState: crate::LiteNetLib::ConnectionState,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPeersCount", (peerState))?;
        Ok(__cordl_ret)
    }
    pub fn GetPeersNonAlloc(
        &mut self,
        peers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::LiteNetLib::NetPeer,
        >,
        peerState: crate::LiteNetLib::ConnectionState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetPeersNonAlloc", (peers, peerState))?;
        Ok(__cordl_ret)
    }
    pub fn LiteNetLib_INetSocketListener_OnMessageReceived(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        length: i32,
        errorCode: crate::System::Net::Sockets::SocketError,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INetSocketListener.OnMessageReceived",
                (data, length, errorCode, remoteEndPoint),
            )?;
        Ok(__cordl_ret)
    }
    pub fn MessageDelivered(
        &mut self,
        fromPeer: *mut crate::LiteNetLib::NetPeer,
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MessageDelivered", (fromPeer, userData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        listener: *mut crate::LiteNetLib::INetEventListener,
        extraPacketLayer: *mut crate::LiteNetLib::Layers::PacketLayerBase,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listener, extraPacketLayer))?;
        Ok(__cordl_object)
    }
    pub fn OnConnectionSolved(
        &mut self,
        request: *mut crate::LiteNetLib::ConnectionRequest,
        rejectData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::LiteNetLib::NetPeer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::NetPeer = __cordl_object
            .invoke("OnConnectionSolved", (request, rejectData, start, length))?;
        Ok(__cordl_ret)
    }
    pub fn PollEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PollEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessConnectRequest(
        &mut self,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        netPeer: *mut crate::LiteNetLib::NetPeer,
        connRequest: *mut crate::LiteNetLib::NetConnectRequestPacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessConnectRequest", (remoteEndPoint, netPeer, connRequest))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessEvent(
        &mut self,
        evt: *mut crate::LiteNetLib::NetEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn RecycleEvent(
        &mut self,
        evt: *mut crate::LiteNetLib::NetEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecycleEvent", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn RemovePeer(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemovePeer", (peer))?;
        Ok(__cordl_ret)
    }
    pub fn RemovePeerInternal(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemovePeerInternal", (peer))?;
        Ok(__cordl_ret)
    }
    pub fn SendBroadcast_Il2CppArray1(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SendBroadcast", (data, port))?;
        Ok(__cordl_ret)
    }
    pub fn SendBroadcast_Il2CppArray_i32_i32_2(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        length: i32,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SendBroadcast", (data, start, length, port))?;
        Ok(__cordl_ret)
    }
    pub fn SendBroadcast_NetDataWriter0(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("SendBroadcast", (writer, port))?;
        Ok(__cordl_ret)
    }
    pub fn SendRawAndRecycle(
        &mut self,
        packet: *mut crate::LiteNetLib::NetPacket,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("SendRawAndRecycle", (packet, remoteEndPoint))?;
        Ok(__cordl_ret)
    }
    pub fn SendRaw_Il2CppArray_i32_i32_IPEndPoint1(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        length: i32,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("SendRaw", (message, start, length, remoteEndPoint))?;
        Ok(__cordl_ret)
    }
    pub fn SendRaw_NetPacket_IPEndPoint0(
        &mut self,
        packet: *mut crate::LiteNetLib::NetPacket,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("SendRaw", (packet, remoteEndPoint))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_Il2CppArray_DeliveryMethod1(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        options: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (data, options))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_Il2CppArray_DeliveryMethod_NetPeer7(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        options: crate::LiteNetLib::DeliveryMethod,
        excludePeer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (data, options, excludePeer))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_Il2CppArray_i32_i32_DeliveryMethod2(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        length: i32,
        options: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (data, start, length, options))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_Il2CppArray_i32_i32_DeliveryMethod_NetPeer8(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        length: i32,
        options: crate::LiteNetLib::DeliveryMethod,
        excludePeer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (data, start, length, options, excludePeer))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_Il2CppArray_i32_i32_u8_DeliveryMethod5(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        length: i32,
        channelNumber: u8,
        options: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (data, start, length, channelNumber, options))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_Il2CppArray_i32_i32_u8_DeliveryMethod_NetPeer11(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        length: i32,
        channelNumber: u8,
        options: crate::LiteNetLib::DeliveryMethod,
        excludePeer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendToAll",
                (data, start, length, channelNumber, options, excludePeer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_Il2CppArray_u8_DeliveryMethod4(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        channelNumber: u8,
        options: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (data, channelNumber, options))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_Il2CppArray_u8_DeliveryMethod_NetPeer10(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        channelNumber: u8,
        options: crate::LiteNetLib::DeliveryMethod,
        excludePeer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (data, channelNumber, options, excludePeer))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_NetDataWriter_DeliveryMethod0(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        options: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (writer, options))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_NetDataWriter_DeliveryMethod_NetPeer6(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        options: crate::LiteNetLib::DeliveryMethod,
        excludePeer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (writer, options, excludePeer))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_NetDataWriter_u8_DeliveryMethod3(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        channelNumber: u8,
        options: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (writer, channelNumber, options))?;
        Ok(__cordl_ret)
    }
    pub fn SendToAll_NetDataWriter_u8_DeliveryMethod_NetPeer9(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        channelNumber: u8,
        options: crate::LiteNetLib::DeliveryMethod,
        excludePeer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToAll", (writer, channelNumber, options, excludePeer))?;
        Ok(__cordl_ret)
    }
    pub fn SendUnconnectedMessage_Il2CppArray_IPEndPoint0(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SendUnconnectedMessage", (message, remoteEndPoint))?;
        Ok(__cordl_ret)
    }
    pub fn SendUnconnectedMessage_Il2CppArray_i32_i32_IPEndPoint2(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        start: i32,
        length: i32,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SendUnconnectedMessage", (message, start, length, remoteEndPoint))?;
        Ok(__cordl_ret)
    }
    pub fn SendUnconnectedMessage_NetDataWriter_IPEndPoint1(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SendUnconnectedMessage", (writer, remoteEndPoint))?;
        Ok(__cordl_ret)
    }
    pub fn Start_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start_IPAddress_IPAddress_i32_1(
        &mut self,
        addressIPv4: *mut crate::System::Net::IPAddress,
        addressIPv6: *mut crate::System::Net::IPAddress,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (addressIPv4, addressIPv6, port))?;
        Ok(__cordl_ret)
    }
    pub fn Start_Il2CppString_Il2CppString_i32_2(
        &mut self,
        addressIPv4: *mut quest_hook::libil2cpp::Il2CppString,
        addressIPv6: *mut quest_hook::libil2cpp::Il2CppString,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (addressIPv4, addressIPv6, port))?;
        Ok(__cordl_ret)
    }
    pub fn Start_i32_3(&mut self, port: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Start", (port))?;
        Ok(__cordl_ret)
    }
    pub fn Stop_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", ())?;
        Ok(__cordl_ret)
    }
    pub fn Stop__cordl_bool1(
        &mut self,
        sendDisconnectMessages: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", (sendDisconnectMessages))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IEnumerable_LiteNetLib_NetPeer__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::LiteNetLib::NetPeer,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            *mut crate::LiteNetLib::NetPeer,
        > = __cordl_object
            .invoke(
                "System.Collections.Generic.IEnumerable<LiteNetLib.NetPeer>.GetEnumerator",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryGetPeer(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
        peer: quest_hook::libil2cpp::ByRefMut<*mut crate::LiteNetLib::NetPeer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetPeer", (endPoint, peer))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateLogic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLogic", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        listener: *mut crate::LiteNetLib::INetEventListener,
        extraPacketLayer: *mut crate::LiteNetLib::Layers::PacketLayerBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (listener, extraPacketLayer))?;
        Ok(__cordl_ret)
    }
    pub fn get_ChannelsCount(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_ChannelsCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ConnectedPeerList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::LiteNetLib::NetPeer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::LiteNetLib::NetPeer,
        > = __cordl_object.invoke("get_ConnectedPeerList", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ConnectedPeersCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ConnectedPeersCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FirstPeer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::LiteNetLib::NetPeer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::NetPeer = __cordl_object
            .invoke("get_FirstPeer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsRunning(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsRunning", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LocalPort", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ChannelsCount(
        &mut self,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ChannelsCount", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LiteNetLib+NetManager")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+NetManager+IPEndPointComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct NetManager_IPEndPointComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LiteNetLib+NetManager+IPEndPointComparer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetManager_IPEndPointComparer =>
    "LiteNetLib"."NetManager/IPEndPointComparer"
);
#[cfg(feature = "LiteNetLib+NetManager+IPEndPointComparer")]
impl std::ops::Deref for crate::LiteNetLib::NetManager_IPEndPointComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetManager+IPEndPointComparer")]
impl std::ops::DerefMut for crate::LiteNetLib::NetManager_IPEndPointComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetManager+IPEndPointComparer")]
impl crate::LiteNetLib::NetManager_IPEndPointComparer {
    pub fn Equals(
        &mut self,
        x: *mut crate::System::Net::IPEndPoint,
        y: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (x, y))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(
        &mut self,
        obj: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "LiteNetLib+NetManager+IPEndPointComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::NetManager_IPEndPointComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+NetManager+NetPeerEnumerator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NetManager_NetPeerEnumerator {
    pub _initialPeer: *mut crate::LiteNetLib::NetPeer,
    pub _p: *mut crate::LiteNetLib::NetPeer,
}
#[cfg(feature = "LiteNetLib+NetManager+NetPeerEnumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetManager_NetPeerEnumerator =>
    "LiteNetLib"."NetManager/NetPeerEnumerator"
);
#[cfg(feature = "LiteNetLib+NetManager+NetPeerEnumerator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LiteNetLib::NetManager_NetPeerEnumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LiteNetLib+NetManager+NetPeerEnumerator")]
impl crate::LiteNetLib::NetManager_NetPeerEnumerator {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        p: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (p),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::LiteNetLib::NetPeer> {
        let __cordl_ret: *mut crate::LiteNetLib::NetPeer = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
