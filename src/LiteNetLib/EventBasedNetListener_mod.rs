#[cfg(feature = "LiteNetLib+EventBasedNetListener")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNetListener {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub PeerConnectedEvent: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::EventBasedNetListener_OnPeerConnected,
    >,
    pub PeerDisconnectedEvent: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::EventBasedNetListener_OnPeerDisconnected,
    >,
    pub NetworkErrorEvent: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::EventBasedNetListener_OnNetworkError,
    >,
    pub NetworkReceiveEvent: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::EventBasedNetListener_OnNetworkReceive,
    >,
    pub NetworkReceiveUnconnectedEvent: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::EventBasedNetListener_OnNetworkReceiveUnconnected,
    >,
    pub NetworkLatencyUpdateEvent: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::EventBasedNetListener_OnNetworkLatencyUpdate,
    >,
    pub ConnectionRequestEvent: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::EventBasedNetListener_OnConnectionRequest,
    >,
    pub DeliveryEvent: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::EventBasedNetListener_OnDeliveryEvent,
    >,
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::EventBasedNetListener =>
    "LiteNetLib"."EventBasedNetListener"
);
#[cfg(feature = "LiteNetLib+EventBasedNetListener")]
impl std::ops::Deref for crate::LiteNetLib::EventBasedNetListener {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener")]
impl std::ops::DerefMut for crate::LiteNetLib::EventBasedNetListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener")]
impl crate::LiteNetLib::EventBasedNetListener {
    #[cfg(feature = "LiteNetLib+EventBasedNetListener+OnConnectionRequest")]
    pub type OnConnectionRequest = crate::LiteNetLib::EventBasedNetListener_OnConnectionRequest;
    #[cfg(feature = "LiteNetLib+EventBasedNetListener+OnDeliveryEvent")]
    pub type OnDeliveryEvent = crate::LiteNetLib::EventBasedNetListener_OnDeliveryEvent;
    #[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkError")]
    pub type OnNetworkError = crate::LiteNetLib::EventBasedNetListener_OnNetworkError;
    #[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkLatencyUpdate")]
    pub type OnNetworkLatencyUpdate = crate::LiteNetLib::EventBasedNetListener_OnNetworkLatencyUpdate;
    #[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceive")]
    pub type OnNetworkReceive = crate::LiteNetLib::EventBasedNetListener_OnNetworkReceive;
    #[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceiveUnconnected")]
    pub type OnNetworkReceiveUnconnected = crate::LiteNetLib::EventBasedNetListener_OnNetworkReceiveUnconnected;
    #[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerConnected")]
    pub type OnPeerConnected = crate::LiteNetLib::EventBasedNetListener_OnPeerConnected;
    #[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerDisconnected")]
    pub type OnPeerDisconnected = crate::LiteNetLib::EventBasedNetListener_OnPeerDisconnected;
    pub fn ClearConnectionRequestEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearConnectionRequestEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDeliveryEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearDeliveryEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearNetworkErrorEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearNetworkErrorEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearNetworkLatencyUpdateEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearNetworkLatencyUpdateEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearNetworkReceiveEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearNetworkReceiveEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearNetworkReceiveUnconnectedEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearNetworkReceiveUnconnectedEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearPeerConnectedEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearPeerConnectedEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearPeerDisconnectedEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearPeerDisconnectedEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_IDeliveryEventListener_OnMessageDelivered(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.IDeliveryEventListener.OnMessageDelivered",
                (peer, userData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_INetEventListener_OnConnectionRequest(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::LiteNetLib::ConnectionRequest>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LiteNetLib.INetEventListener.OnConnectionRequest", (request))?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_INetEventListener_OnNetworkError(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        socketErrorCode: crate::System::Net::Sockets::SocketError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INetEventListener.OnNetworkError",
                (endPoint, socketErrorCode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_INetEventListener_OnNetworkLatencyUpdate(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        latency: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INetEventListener.OnNetworkLatencyUpdate",
                (peer, latency),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_INetEventListener_OnNetworkReceive(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacketReader>,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INetEventListener.OnNetworkReceive",
                (peer, reader, deliveryMethod),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_INetEventListener_OnNetworkReceiveUnconnected(
        &mut self,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacketReader>,
        messageType: crate::LiteNetLib::UnconnectedMessageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INetEventListener.OnNetworkReceiveUnconnected",
                (remoteEndPoint, reader, messageType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_INetEventListener_OnPeerConnected(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LiteNetLib.INetEventListener.OnPeerConnected", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_INetEventListener_OnPeerDisconnected(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        disconnectInfo: crate::LiteNetLib::DisconnectInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INetEventListener.OnPeerDisconnected",
                (peer, disconnectInfo),
            )?;
        Ok(__cordl_ret.into())
    }
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
    pub fn add_ConnectionRequestEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnConnectionRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_ConnectionRequestEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_DeliveryEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnDeliveryEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_DeliveryEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_NetworkErrorEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnNetworkError,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_NetworkErrorEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_NetworkLatencyUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnNetworkLatencyUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_NetworkLatencyUpdateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_NetworkReceiveEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnNetworkReceive,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_NetworkReceiveEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_NetworkReceiveUnconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnNetworkReceiveUnconnected,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_NetworkReceiveUnconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_PeerConnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnPeerConnected,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_PeerConnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_PeerDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnPeerDisconnected,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_PeerDisconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_ConnectionRequestEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnConnectionRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_ConnectionRequestEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_DeliveryEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnDeliveryEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_DeliveryEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_NetworkErrorEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnNetworkError,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_NetworkErrorEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_NetworkLatencyUpdateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnNetworkLatencyUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_NetworkLatencyUpdateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_NetworkReceiveEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnNetworkReceive,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_NetworkReceiveEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_NetworkReceiveUnconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnNetworkReceiveUnconnected,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_NetworkReceiveUnconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_PeerConnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnPeerConnected,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_PeerConnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_PeerDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::EventBasedNetListener_OnPeerDisconnected,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_PeerDisconnectedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::EventBasedNetListener {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::LiteNetLib::IDeliveryEventListener>>
for crate::LiteNetLib::EventBasedNetListener {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::LiteNetLib::IDeliveryEventListener> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::LiteNetLib::IDeliveryEventListener>>
for crate::LiteNetLib::EventBasedNetListener {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::LiteNetLib::IDeliveryEventListener> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::LiteNetLib::INetEventListener>>
for crate::LiteNetLib::EventBasedNetListener {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::LiteNetLib::INetEventListener> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::LiteNetLib::INetEventListener>>
for crate::LiteNetLib::EventBasedNetListener {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::LiteNetLib::INetEventListener> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnConnectionRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNetListener_OnConnectionRequest {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnConnectionRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::EventBasedNetListener_OnConnectionRequest => "LiteNetLib"
    ."EventBasedNetListener/OnConnectionRequest"
);
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnConnectionRequest")]
impl std::ops::Deref for crate::LiteNetLib::EventBasedNetListener_OnConnectionRequest {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnConnectionRequest")]
impl std::ops::DerefMut
for crate::LiteNetLib::EventBasedNetListener_OnConnectionRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnConnectionRequest")]
impl crate::LiteNetLib::EventBasedNetListener_OnConnectionRequest {
    pub fn BeginInvoke(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::LiteNetLib::ConnectionRequest>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (request, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::LiteNetLib::ConnectionRequest>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (request))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnConnectionRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::EventBasedNetListener_OnConnectionRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnDeliveryEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNetListener_OnDeliveryEvent {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnDeliveryEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::EventBasedNetListener_OnDeliveryEvent => "LiteNetLib"
    ."EventBasedNetListener/OnDeliveryEvent"
);
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnDeliveryEvent")]
impl std::ops::Deref for crate::LiteNetLib::EventBasedNetListener_OnDeliveryEvent {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnDeliveryEvent")]
impl std::ops::DerefMut for crate::LiteNetLib::EventBasedNetListener_OnDeliveryEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnDeliveryEvent")]
impl crate::LiteNetLib::EventBasedNetListener_OnDeliveryEvent {
    pub fn BeginInvoke(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (peer, userData, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (peer, userData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnDeliveryEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::EventBasedNetListener_OnDeliveryEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkError")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNetListener_OnNetworkError {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkError")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::EventBasedNetListener_OnNetworkError
    => "LiteNetLib"."EventBasedNetListener/OnNetworkError"
);
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkError")]
impl std::ops::Deref for crate::LiteNetLib::EventBasedNetListener_OnNetworkError {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkError")]
impl std::ops::DerefMut for crate::LiteNetLib::EventBasedNetListener_OnNetworkError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkError")]
impl crate::LiteNetLib::EventBasedNetListener_OnNetworkError {
    pub fn BeginInvoke(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        socketError: crate::System::Net::Sockets::SocketError,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (endPoint, socketError, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        socketError: crate::System::Net::Sockets::SocketError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (endPoint, socketError))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkError")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::EventBasedNetListener_OnNetworkError {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkLatencyUpdate")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNetListener_OnNetworkLatencyUpdate {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkLatencyUpdate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::EventBasedNetListener_OnNetworkLatencyUpdate => "LiteNetLib"
    ."EventBasedNetListener/OnNetworkLatencyUpdate"
);
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkLatencyUpdate")]
impl std::ops::Deref
for crate::LiteNetLib::EventBasedNetListener_OnNetworkLatencyUpdate {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkLatencyUpdate")]
impl std::ops::DerefMut
for crate::LiteNetLib::EventBasedNetListener_OnNetworkLatencyUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkLatencyUpdate")]
impl crate::LiteNetLib::EventBasedNetListener_OnNetworkLatencyUpdate {
    pub fn BeginInvoke(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        latency: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (peer, latency, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        latency: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (peer, latency))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkLatencyUpdate")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::EventBasedNetListener_OnNetworkLatencyUpdate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceive")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNetListener_OnNetworkReceive {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceive")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::EventBasedNetListener_OnNetworkReceive => "LiteNetLib"
    ."EventBasedNetListener/OnNetworkReceive"
);
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceive")]
impl std::ops::Deref for crate::LiteNetLib::EventBasedNetListener_OnNetworkReceive {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceive")]
impl std::ops::DerefMut for crate::LiteNetLib::EventBasedNetListener_OnNetworkReceive {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceive")]
impl crate::LiteNetLib::EventBasedNetListener_OnNetworkReceive {
    pub fn BeginInvoke(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacketReader>,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (peer, reader, deliveryMethod, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacketReader>,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (peer, reader, deliveryMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceive")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::EventBasedNetListener_OnNetworkReceive {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceiveUnconnected")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNetListener_OnNetworkReceiveUnconnected {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceiveUnconnected")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::EventBasedNetListener_OnNetworkReceiveUnconnected => "LiteNetLib"
    ."EventBasedNetListener/OnNetworkReceiveUnconnected"
);
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceiveUnconnected")]
impl std::ops::Deref
for crate::LiteNetLib::EventBasedNetListener_OnNetworkReceiveUnconnected {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceiveUnconnected")]
impl std::ops::DerefMut
for crate::LiteNetLib::EventBasedNetListener_OnNetworkReceiveUnconnected {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceiveUnconnected")]
impl crate::LiteNetLib::EventBasedNetListener_OnNetworkReceiveUnconnected {
    pub fn BeginInvoke(
        &mut self,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacketReader>,
        messageType: crate::LiteNetLib::UnconnectedMessageType,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (remoteEndPoint, reader, messageType, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacketReader>,
        messageType: crate::LiteNetLib::UnconnectedMessageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (remoteEndPoint, reader, messageType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnNetworkReceiveUnconnected")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::EventBasedNetListener_OnNetworkReceiveUnconnected {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerConnected")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNetListener_OnPeerConnected {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerConnected")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::EventBasedNetListener_OnPeerConnected => "LiteNetLib"
    ."EventBasedNetListener/OnPeerConnected"
);
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerConnected")]
impl std::ops::Deref for crate::LiteNetLib::EventBasedNetListener_OnPeerConnected {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerConnected")]
impl std::ops::DerefMut for crate::LiteNetLib::EventBasedNetListener_OnPeerConnected {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerConnected")]
impl crate::LiteNetLib::EventBasedNetListener_OnPeerConnected {
    pub fn BeginInvoke(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (peer, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerConnected")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::EventBasedNetListener_OnPeerConnected {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerDisconnected")]
#[repr(C)]
#[derive(Debug)]
pub struct EventBasedNetListener_OnPeerDisconnected {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerDisconnected")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::EventBasedNetListener_OnPeerDisconnected => "LiteNetLib"
    ."EventBasedNetListener/OnPeerDisconnected"
);
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerDisconnected")]
impl std::ops::Deref for crate::LiteNetLib::EventBasedNetListener_OnPeerDisconnected {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerDisconnected")]
impl std::ops::DerefMut for crate::LiteNetLib::EventBasedNetListener_OnPeerDisconnected {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerDisconnected")]
impl crate::LiteNetLib::EventBasedNetListener_OnPeerDisconnected {
    pub fn BeginInvoke(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        disconnectInfo: crate::LiteNetLib::DisconnectInfo,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (peer, disconnectInfo, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        disconnectInfo: crate::LiteNetLib::DisconnectInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (peer, disconnectInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+EventBasedNetListener+OnPeerDisconnected")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::EventBasedNetListener_OnPeerDisconnected {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
