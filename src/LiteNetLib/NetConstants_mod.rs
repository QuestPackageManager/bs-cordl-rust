#[cfg(feature = "LiteNetLib+NetConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct NetConstants {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "LiteNetLib+NetConstants")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetConstants => "LiteNetLib"
    ."NetConstants"
);
#[cfg(feature = "LiteNetLib+NetConstants")]
impl std::ops::Deref for crate::LiteNetLib::NetConstants {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetConstants")]
impl std::ops::DerefMut for crate::LiteNetLib::NetConstants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetConstants")]
impl crate::LiteNetLib::NetConstants {
    pub const ChanneledHeaderSize: i32 = 4i32;
    pub const DefaultWindowSize: i32 = 256i32;
    pub const FragmentHeaderSize: i32 = 6i32;
    pub const FragmentedHeaderTotalSize: i32 = 10i32;
    pub const HalfMaxSequence: u16 = 16384u16;
    pub const HeaderSize: i32 = 1i32;
    pub const MaxConnectionNumber: u8 = 4u8;
    pub const MaxSequence: u16 = 32768u16;
    pub const MaxUdpHeaderSize: i32 = 68i32;
    pub const PacketPoolSize: i32 = 1000i32;
    pub const ProtocolId: i32 = 11i32;
    pub const SocketBufferSize: i32 = 1048576i32;
    pub const SocketTTL: i32 = 255i32;
}
#[cfg(feature = "LiteNetLib+NetConstants")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
