#[cfg(feature = "cordl_class_LiteNetLib+NetConnectAcceptPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct NetConnectAcceptPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ConnectionId: i64,
    pub ConnectionNumber: u8,
    pub IsReusedPeer: bool,
}
#[cfg(feature = "cordl_class_LiteNetLib+NetConnectAcceptPacket")]
unsafe impl quest_hook::libil2cpp::Type for crate::LiteNetLib::NetConnectAcceptPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "NetConnectAcceptPacket";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "LiteNetLib+NetConnectAcceptPacket")]
impl std::ops::Deref for crate::LiteNetLib::NetConnectAcceptPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetConnectAcceptPacket")]
impl std::ops::DerefMut for crate::LiteNetLib::NetConnectAcceptPacket {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>),
                        quest_hook::libil2cpp::Gc<
                            crate::LiteNetLib::NetConnectAcceptPacket,
                        >,
                        1usize,
                    >("FromData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetConnectAcceptPacket> =
            unsafe { cordl_method_info.invoke_unchecked((), (packet))? };
        Ok(__cordl_ret.into())
    }
    pub fn Make(
        connectId: i64,
        connectNum: u8,
        reusedPeer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i64, u8, bool),
                        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
                        3usize,
                    >("Make")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Make",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket> =
            unsafe { cordl_method_info.invoke_unchecked((), (connectId, connectNum, reusedPeer))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        connectionId: i64,
        connectionNumber: u8,
        isReusedPeer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i64, u8, bool), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (connectionId, connectionNumber, isReusedPeer))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_LiteNetLib+NetConnectAcceptPacket")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetConnectAcceptPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
