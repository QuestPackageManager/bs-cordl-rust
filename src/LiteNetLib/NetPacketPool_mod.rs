#[cfg(feature = "LiteNetLib+NetPacketPool")]
#[repr(C)]
#[derive(Debug)]
pub struct NetPacketPool {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _pool: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
        >,
    >,
    pub _lock: quest_hook::libil2cpp::Gc<crate::System::Threading::ReaderWriterLockSlim>,
    pub _count: i32,
}
#[cfg(feature = "LiteNetLib+NetPacketPool")]
unsafe impl quest_hook::libil2cpp::Type for crate::LiteNetLib::NetPacketPool {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "NetPacketPool";
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
#[cfg(feature = "LiteNetLib+NetPacketPool")]
impl std::ops::Deref for crate::LiteNetLib::NetPacketPool {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetPacketPool")]
impl std::ops::DerefMut for crate::LiteNetLib::NetPacketPool {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetPacketPool")]
impl crate::LiteNetLib::NetPacketPool {
    pub fn GetPacket(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
                        1usize,
                    >("GetPacket")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPacket", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket> = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetWithData(
        &mut self,
        property: crate::LiteNetLib::PacketProperty,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::LiteNetLib::PacketProperty,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
                        4usize,
                    >("GetWithData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetWithData", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket> = unsafe {
            cordl_method_info.invoke_unchecked(self, (property, data, start, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetWithProperty_PacketProperty1(
        &mut self,
        property: crate::LiteNetLib::PacketProperty,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::LiteNetLib::PacketProperty),
                        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
                        1usize,
                    >("GetWithProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetWithProperty", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket> = unsafe {
            cordl_method_info.invoke_unchecked(self, (property))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetWithProperty_i32_0(
        &mut self,
        property: crate::LiteNetLib::PacketProperty,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::LiteNetLib::PacketProperty, i32),
                        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
                        2usize,
                    >("GetWithProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetWithProperty", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket> = unsafe {
            cordl_method_info.invoke_unchecked(self, (property, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Recycle(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Recycle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Recycle",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (packet))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+NetPacketPool")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetPacketPool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
