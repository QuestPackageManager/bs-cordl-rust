#[cfg(feature = "LiteNetLib+SequencedChannel")]
#[repr(C)]
#[derive(Debug)]
pub struct SequencedChannel {
    __cordl_parent: crate::LiteNetLib::BaseChannel,
    pub _localSequence: i32,
    pub _remoteSequence: u16,
    pub _reliable: bool,
    pub _lastPacket: *mut crate::LiteNetLib::NetPacket,
    pub _ackPacket: *mut crate::LiteNetLib::NetPacket,
    pub _mustSendAck: bool,
    pub _id: u8,
    pub _lastPacketSendTime: i64,
}
#[cfg(feature = "LiteNetLib+SequencedChannel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::SequencedChannel => "LiteNetLib"
    ."SequencedChannel"
);
#[cfg(feature = "LiteNetLib+SequencedChannel")]
impl std::ops::Deref for crate::LiteNetLib::SequencedChannel {
    type Target = crate::LiteNetLib::BaseChannel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+SequencedChannel")]
impl std::ops::DerefMut for crate::LiteNetLib::SequencedChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+SequencedChannel")]
impl crate::LiteNetLib::SequencedChannel {
    pub fn SendNextPackets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendNextPackets", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPacket(
        &mut self,
        packet: *mut crate::LiteNetLib::NetPacket,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessPacket", (packet))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        reliable: bool,
        id: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (peer, reliable, id))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        peer: *mut crate::LiteNetLib::NetPeer,
        reliable: bool,
        id: u8,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (peer, reliable, id))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LiteNetLib+SequencedChannel")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::SequencedChannel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
