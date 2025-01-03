#[cfg(feature = "NetDataWriterExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct NetDataWriterExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "NetDataWriterExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NetDataWriterExtensions => ""
    ."NetDataWriterExtensions"
);
#[cfg(feature = "NetDataWriterExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::NetDataWriterExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NetDataWriterExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::NetDataWriterExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NetDataWriterExtensions")]
impl crate::GlobalNamespace::NetDataWriterExtensions {
    pub fn SetUpPacket_PacketOption0(
        netDataWriter: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetDataWriter,
        >,
        senderId: u8,
        receiverId: u8,
        packetOptions: crate::GlobalNamespace::PacketOption,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetUpPacket",
                (netDataWriter, senderId, receiverId, packetOptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUpPacket_u8_1(
        netDataWriter: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetDataWriter,
        >,
        senderId: u8,
        receiverId: u8,
        packetOptions: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetUpPacket",
                (netDataWriter, senderId, receiverId, packetOptions),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NetDataWriterExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NetDataWriterExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
