#[cfg(feature = "LiteNetLib+ReliableChannel")]
#[repr(C)]
#[derive(Debug)]
pub struct ReliableChannel {
    __cordl_parent: crate::LiteNetLib::BaseChannel,
    pub _outgoingAcks: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    pub _pendingPackets: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::LiteNetLib::ReliableChannel_PendingPacket,
        >,
    >,
    pub _receivedPackets: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
        >,
    >,
    pub _earlyReceived: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<bool>,
    >,
    pub _localSeqence: i32,
    pub _remoteSequence: i32,
    pub _localWindowStart: i32,
    pub _remoteWindowStart: i32,
    pub _mustSendAcks: bool,
    pub _deliveryMethod: crate::LiteNetLib::DeliveryMethod,
    pub _ordered: bool,
    pub _windowSize: i32,
    pub _id: u8,
}
#[cfg(feature = "LiteNetLib+ReliableChannel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::ReliableChannel => "LiteNetLib"
    ."ReliableChannel"
);
#[cfg(feature = "LiteNetLib+ReliableChannel")]
impl std::ops::Deref for crate::LiteNetLib::ReliableChannel {
    type Target = crate::LiteNetLib::BaseChannel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+ReliableChannel")]
impl std::ops::DerefMut for crate::LiteNetLib::ReliableChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+ReliableChannel")]
impl crate::LiteNetLib::ReliableChannel {
    pub const BitsInByte: i32 = 8i32;
    #[cfg(feature = "LiteNetLib+ReliableChannel+PendingPacket")]
    pub type PendingPacket = crate::LiteNetLib::ReliableChannel_PendingPacket;
    pub fn New(
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        ordered: bool,
        id: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (peer, ordered, id))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessAck(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAck", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessPacket(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessPacket", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendNextPackets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendNextPackets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
        ordered: bool,
        id: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (peer, ordered, id))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+ReliableChannel")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::ReliableChannel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+ReliableChannel+PendingPacket")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReliableChannel_PendingPacket {
    pub _packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    pub _timeStamp: i64,
    pub _isSent: bool,
}
#[cfg(feature = "LiteNetLib+ReliableChannel+PendingPacket")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::ReliableChannel_PendingPacket =>
    "LiteNetLib"."ReliableChannel/PendingPacket"
);
#[cfg(feature = "LiteNetLib+ReliableChannel+PendingPacket")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LiteNetLib::ReliableChannel_PendingPacket {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LiteNetLib+ReliableChannel+PendingPacket")]
impl crate::LiteNetLib::ReliableChannel_PendingPacket {
    pub fn Clear(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (peer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (packet),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySend(
        &mut self,
        currentTime: i64,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TrySend",
            (currentTime, peer),
        )?;
        Ok(__cordl_ret.into())
    }
}
