#[cfg(feature = "LiteNetLib+NetEvent+EType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetEvent_EType {
    Broadcast = 6i32,
    Connect = 0i32,
    ConnectionLatencyUpdated = 5i32,
    ConnectionRequest = 7i32,
    Disconnect = 1i32,
    Error = 4i32,
    MessageDelivered = 8i32,
    Receive = 2i32,
    ReceiveUnconnected = 3i32,
}
#[cfg(feature = "LiteNetLib+NetEvent+EType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetEvent_EType => "LiteNetLib"
    ."NetEvent/EType"
);
#[cfg(feature = "LiteNetLib+NetEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct NetEvent {
    __cordl_parent: crate::System::Object,
    pub Type: crate::LiteNetLib::NetEvent_EType,
    pub Peer: *mut crate::LiteNetLib::NetPeer,
    pub RemoteEndPoint: *mut crate::System::Net::IPEndPoint,
    pub UserData: *mut crate::System::Object,
    pub Latency: i32,
    pub ErrorCode: crate::System::Net::Sockets::SocketError,
    pub DisconnectReason: crate::LiteNetLib::DisconnectReason,
    pub ConnectionRequest: *mut crate::LiteNetLib::ConnectionRequest,
    pub DeliveryMethod: crate::LiteNetLib::DeliveryMethod,
    pub DataReader: *mut crate::LiteNetLib::NetPacketReader,
}
#[cfg(feature = "LiteNetLib+NetEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetEvent => "LiteNetLib"."NetEvent"
);
#[cfg(feature = "LiteNetLib+NetEvent")]
impl std::ops::Deref for crate::LiteNetLib::NetEvent {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetEvent")]
impl std::ops::DerefMut for crate::LiteNetLib::NetEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetEvent")]
impl crate::LiteNetLib::NetEvent {
    #[cfg(feature = "LiteNetLib+NetEvent+EType")]
    pub type EType = crate::LiteNetLib::NetEvent_EType;
    pub fn New(
        manager: *mut crate::LiteNetLib::NetManager,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (manager))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        manager: *mut crate::LiteNetLib::NetManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (manager))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LiteNetLib+NetEvent")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}