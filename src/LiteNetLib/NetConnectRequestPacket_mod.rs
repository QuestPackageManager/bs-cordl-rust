#[cfg(feature = "LiteNetLib+NetConnectRequestPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct NetConnectRequestPacket {
    __cordl_parent: crate::System::Object,
    pub ConnectionTime: i64,
    pub ConnectionNumber: u8,
    pub TargetAddress: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub Data: *mut crate::LiteNetLib::Utils::NetDataReader,
}
#[cfg(feature = "LiteNetLib+NetConnectRequestPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetConnectRequestPacket =>
    "LiteNetLib"."NetConnectRequestPacket"
);
#[cfg(feature = "LiteNetLib+NetConnectRequestPacket")]
impl std::ops::Deref for crate::LiteNetLib::NetConnectRequestPacket {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        connectionTime: i64,
        connectionNumber: u8,
        targetAddress: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        data: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (connectionTime, connectionNumber, targetAddress, data))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        connectionTime: i64,
        connectionNumber: u8,
        targetAddress: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        data: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (connectionTime, connectionNumber, targetAddress, data),
            )?;
        Ok(__cordl_object)
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
