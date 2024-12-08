#[cfg(feature = "LiteNetLib+BaseChannel")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseChannel {
    __cordl_parent: crate::System::Object,
    pub Next: *mut crate::LiteNetLib::BaseChannel,
    pub Peer: *mut crate::LiteNetLib::NetPeer,
    pub OutgoingQueue: *mut crate::System::Collections::Generic::Queue_1<
        *mut crate::LiteNetLib::NetPacket,
    >,
}
#[cfg(feature = "LiteNetLib+BaseChannel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::BaseChannel => "LiteNetLib"
    ."BaseChannel"
);
#[cfg(feature = "LiteNetLib+BaseChannel")]
impl std::ops::Deref for crate::LiteNetLib::BaseChannel {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (peer))?;
        Ok(__cordl_ret)
    }
    pub fn AddToQueue(
        &mut self,
        packet: *mut crate::LiteNetLib::NetPacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToQueue", (packet))?;
        Ok(__cordl_ret)
    }
    pub fn get_PacketsInQueue(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PacketsInQueue", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPacket(
        &mut self,
        packet: *mut crate::LiteNetLib::NetPacket,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessPacket", (packet))?;
        Ok(__cordl_ret)
    }
    pub fn SendNextPackets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendNextPackets", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        peer: *mut crate::LiteNetLib::NetPeer,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (peer))?;
        Ok(__cordl_object)
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
