#[cfg(feature = "NetDataWriterExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct NetDataWriterExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "NetDataWriterExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NetDataWriterExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NetDataWriterExtensions";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                    u8,
                    u8,
                    crate::GlobalNamespace::PacketOption,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetUpPacket")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetUpPacket", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (netDataWriter, senderId, receiverId, packetOptions),
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                    u8,
                    u8,
                    u8,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetUpPacket")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetUpPacket", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (netDataWriter, senderId, receiverId, packetOptions),
                )
        };
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
