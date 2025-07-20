#[cfg(feature = "LiteNetLib+NetConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct NetConstants {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LiteNetLib+NetConstants")]
unsafe impl quest_hook::libil2cpp::Type for crate::LiteNetLib::NetConstants {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "NetConstants";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "LiteNetLib+NetConstants")]
impl std::ops::Deref for crate::LiteNetLib::NetConstants {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetConstants")]
impl std::ops::DerefMut for crate::LiteNetLib::NetConstants {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
