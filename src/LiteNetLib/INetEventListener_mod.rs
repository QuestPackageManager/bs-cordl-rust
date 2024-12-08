#[cfg(feature = "LiteNetLib+INetEventListener")]
#[repr(C)]
#[derive(Debug)]
pub struct INetEventListener {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LiteNetLib+INetEventListener")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::INetEventListener => "LiteNetLib"
    ."INetEventListener"
);
#[cfg(feature = "LiteNetLib+INetEventListener")]
impl std::ops::Deref for crate::LiteNetLib::INetEventListener {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+INetEventListener")]
impl std::ops::DerefMut for crate::LiteNetLib::INetEventListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+INetEventListener")]
impl crate::LiteNetLib::INetEventListener {
    pub fn OnNetworkReceiveUnconnected(
        &mut self,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        reader: *mut crate::LiteNetLib::NetPacketReader,
        messageType: crate::LiteNetLib::UnconnectedMessageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "OnNetworkReceiveUnconnected",
                (remoteEndPoint, reader, messageType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn OnNetworkReceive(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        reader: *mut crate::LiteNetLib::NetPacketReader,
        deliveryMethod: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNetworkReceive", (peer, reader, deliveryMethod))?;
        Ok(__cordl_ret)
    }
    pub fn OnNetworkError(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
        socketError: crate::System::Net::Sockets::SocketError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNetworkError", (endPoint, socketError))?;
        Ok(__cordl_ret)
    }
    pub fn OnPeerConnected(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPeerConnected", (peer))?;
        Ok(__cordl_ret)
    }
    pub fn OnNetworkLatencyUpdate(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        latency: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNetworkLatencyUpdate", (peer, latency))?;
        Ok(__cordl_ret)
    }
    pub fn OnPeerDisconnected(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        disconnectInfo: crate::LiteNetLib::DisconnectInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPeerDisconnected", (peer, disconnectInfo))?;
        Ok(__cordl_ret)
    }
    pub fn OnConnectionRequest(
        &mut self,
        request: *mut crate::LiteNetLib::ConnectionRequest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnConnectionRequest", (request))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "LiteNetLib+INetEventListener")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::INetEventListener {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
