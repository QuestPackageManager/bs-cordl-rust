#[cfg(feature = "LiteNetLib+NetPacketPool")]
#[repr(C)]
#[derive(Debug)]
pub struct NetPacketPool {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _pool: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::LiteNetLib::NetPacket>,
    >,
    pub _lock: quest_hook::libil2cpp::Gc<crate::System::Threading::ReaderWriterLockSlim>,
    pub _count: i32,
}
#[cfg(feature = "LiteNetLib+NetPacketPool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetPacketPool => "LiteNetLib"
    ."NetPacketPool"
);
#[cfg(feature = "LiteNetLib+NetPacketPool")]
impl std::ops::Deref for crate::LiteNetLib::NetPacketPool {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetPacketPool")]
impl std::ops::DerefMut for crate::LiteNetLib::NetPacketPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket> = __cordl_object
            .invoke("GetPacket", (_cordl_size))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket> = __cordl_object
            .invoke("GetWithData", (property, data, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWithProperty_PacketProperty1(
        &mut self,
        property: crate::LiteNetLib::PacketProperty,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket> = __cordl_object
            .invoke("GetWithProperty", (property))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWithProperty_i32_0(
        &mut self,
        property: crate::LiteNetLib::PacketProperty,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket> = __cordl_object
            .invoke("GetWithProperty", (property, _cordl_size))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Recycle", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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
