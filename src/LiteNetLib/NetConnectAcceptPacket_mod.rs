#[cfg(feature = "LiteNetLib+NetConnectAcceptPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct NetConnectAcceptPacket {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub ConnectionId: i64,
    pub ConnectionNumber: u8,
    pub IsReusedPeer: bool,
}
#[cfg(feature = "LiteNetLib+NetConnectAcceptPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetConnectAcceptPacket =>
    "LiteNetLib"."NetConnectAcceptPacket"
);
#[cfg(feature = "LiteNetLib+NetConnectAcceptPacket")]
impl std::ops::Deref for crate::LiteNetLib::NetConnectAcceptPacket {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetConnectAcceptPacket")]
impl std::ops::DerefMut for crate::LiteNetLib::NetConnectAcceptPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetConnectAcceptPacket")]
impl crate::LiteNetLib::NetConnectAcceptPacket {
    pub const Size: i32 = 11i32;
    pub fn FromData(
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetConnectAcceptPacket>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::NetConnectAcceptPacket,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("FromData", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn Make(
        connectId: i64,
        connectNum: u8,
        reusedPeer: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Make", (connectId, connectNum, reusedPeer))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        connectionId: i64,
        connectionNumber: u8,
        isReusedPeer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connectionId, connectionNumber, isReusedPeer))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        connectionId: i64,
        connectionNumber: u8,
        isReusedPeer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (connectionId, connectionNumber, isReusedPeer))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+NetConnectAcceptPacket")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetConnectAcceptPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
