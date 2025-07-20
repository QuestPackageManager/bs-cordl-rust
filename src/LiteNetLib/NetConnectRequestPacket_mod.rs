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
unsafe impl quest_hook::libil2cpp::Type for crate::LiteNetLib::NetConnectRequestPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "NetConnectRequestPacket";
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
#[cfg(feature = "LiteNetLib+NetConnectRequestPacket")]
impl std::ops::Deref for crate::LiteNetLib::NetConnectRequestPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetConnectRequestPacket")]
impl std::ops::DerefMut for crate::LiteNetLib::NetConnectRequestPacket {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>),
                        quest_hook::libil2cpp::Gc<
                            crate::LiteNetLib::NetConnectRequestPacket,
                        >,
                        1usize,
                    >("FromData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FromData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::NetConnectRequestPacket,
        > = unsafe { method.invoke_unchecked((), (packet))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetProtocolId(
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>),
                        i32,
                        1usize,
                    >("GetProtocolId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetProtocolId", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (packet))? };
        Ok(__cordl_ret.into())
    }
    pub fn Make(
        connectData: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        addressBytes: quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress>,
        connectId: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::LiteNetLib::Utils::NetDataWriter,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Net::SocketAddress>,
                            i64,
                        ),
                        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
                        3usize,
                    >("Make")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Make", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket> = unsafe {
            method.invoke_unchecked((), (connectData, addressBytes, connectId))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i64,
                            u8,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::LiteNetLib::Utils::NetDataReader,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (connectionTime, connectionNumber, targetAddress, data),
                )?
        };
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
