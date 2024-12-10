#[cfg(feature = "LiteNetLib+SimpleChannel")]
#[repr(C)]
#[derive(Debug)]
pub struct SimpleChannel {
    __cordl_parent: crate::LiteNetLib::BaseChannel,
}
#[cfg(feature = "LiteNetLib+SimpleChannel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::SimpleChannel => "LiteNetLib"
    ."SimpleChannel"
);
#[cfg(feature = "LiteNetLib+SimpleChannel")]
impl std::ops::Deref for crate::LiteNetLib::SimpleChannel {
    type Target = crate::LiteNetLib::BaseChannel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+SimpleChannel")]
impl std::ops::DerefMut for crate::LiteNetLib::SimpleChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+SimpleChannel")]
impl crate::LiteNetLib::SimpleChannel {
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
}
#[cfg(feature = "LiteNetLib+SimpleChannel")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::SimpleChannel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
