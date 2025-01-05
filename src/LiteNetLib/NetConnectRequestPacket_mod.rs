#[cfg(feature = "LiteNetLib+NetConnectRequestPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct NetConnectRequestPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ConnectionTime: i64,
    pub ConnectionNumber: u8,
    pub TargetAddress: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub Data: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
}
#[cfg(feature = "LiteNetLib+NetConnectRequestPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetConnectRequestPacket =>
    "LiteNetLib"."NetConnectRequestPacket"
);
#[cfg(feature = "LiteNetLib+NetConnectRequestPacket")]
impl std::ops::Deref for crate::LiteNetLib::NetConnectRequestPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetConnectRequestPacket")]
impl std::ops::DerefMut for crate::LiteNetLib::NetConnectRequestPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetConnectRequestPacket")]
impl crate::LiteNetLib::NetConnectRequestPacket {
    pub const HeaderSize: i32 = 14i32;
    pub fn FromData(
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetConnectRequestPacket>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::NetConnectRequestPacket,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("FromData", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProtocolId(
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProtocolId", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn Make(
        connectData: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        addressBytes: quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress>,
        connectId: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Make", (connectData, addressBytes, connectId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        connectionTime: i64,
        connectionNumber: u8,
        targetAddress: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        data: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (connectionTime, connectionNumber, targetAddress, data),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        connectionTime: i64,
        connectionNumber: u8,
        targetAddress: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        data: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (connectionTime, connectionNumber, targetAddress, data))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+NetConnectRequestPacket")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetConnectRequestPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
