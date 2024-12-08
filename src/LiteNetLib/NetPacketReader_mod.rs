#[cfg(feature = "LiteNetLib+NetPacketReader")]
#[repr(C)]
#[derive(Debug)]
pub struct NetPacketReader {
    __cordl_parent: crate::LiteNetLib::Utils::NetDataReader,
    pub _packet: *mut crate::LiteNetLib::NetPacket,
    pub _manager: *mut crate::LiteNetLib::NetManager,
    pub _evt: *mut crate::LiteNetLib::NetEvent,
}
#[cfg(feature = "LiteNetLib+NetPacketReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetPacketReader => "LiteNetLib"
    ."NetPacketReader"
);
#[cfg(feature = "LiteNetLib+NetPacketReader")]
impl std::ops::Deref for crate::LiteNetLib::NetPacketReader {
    type Target = crate::LiteNetLib::Utils::NetDataReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetPacketReader")]
impl std::ops::DerefMut for crate::LiteNetLib::NetPacketReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetPacketReader")]
impl crate::LiteNetLib::NetPacketReader {
    pub fn New(
        manager: *mut crate::LiteNetLib::NetManager,
        evt: *mut crate::LiteNetLib::NetEvent,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (manager, evt))?;
        Ok(__cordl_object)
    }
    pub fn Recycle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Recycle", ())?;
        Ok(__cordl_ret)
    }
    pub fn RecycleInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecycleInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetSource(
        &mut self,
        packet: *mut crate::LiteNetLib::NetPacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSource", (packet))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        manager: *mut crate::LiteNetLib::NetManager,
        evt: *mut crate::LiteNetLib::NetEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (manager, evt))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LiteNetLib+NetPacketReader")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetPacketReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
