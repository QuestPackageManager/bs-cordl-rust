#[cfg(feature = "LiteNetLib+BaseChannel")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseChannel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Next: quest_hook::libil2cpp::Gc<crate::LiteNetLib::BaseChannel>,
    pub Peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
    pub OutgoingQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<
            quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
        >,
    >,
}
#[cfg(feature = "LiteNetLib+BaseChannel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::BaseChannel => "LiteNetLib"
    ."BaseChannel"
);
#[cfg(feature = "LiteNetLib+BaseChannel")]
impl std::ops::Deref for crate::LiteNetLib::BaseChannel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+BaseChannel")]
impl std::ops::DerefMut for crate::LiteNetLib::BaseChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+BaseChannel")]
impl crate::LiteNetLib::BaseChannel {
    pub fn AddToQueue(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToQueue", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (peer))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessPacket(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessPacket", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendNextPackets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendNextPackets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        peer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PacketsInQueue(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PacketsInQueue", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+BaseChannel")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::BaseChannel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
