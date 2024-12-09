#[cfg(feature = "LiteNetLib+Layers+PacketLayerBase")]
#[repr(C)]
#[derive(Debug)]
pub struct PacketLayerBase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ExtraPacketSizeForLayer: i32,
}
#[cfg(feature = "LiteNetLib+Layers+PacketLayerBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Layers::PacketLayerBase =>
    "LiteNetLib.Layers"."PacketLayerBase"
);
#[cfg(feature = "LiteNetLib+Layers+PacketLayerBase")]
impl std::ops::Deref for crate::LiteNetLib::Layers::PacketLayerBase {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Layers+PacketLayerBase")]
impl std::ops::DerefMut for crate::LiteNetLib::Layers::PacketLayerBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Layers+PacketLayerBase")]
impl crate::LiteNetLib::Layers::PacketLayerBase {
    pub fn New(
        extraPacketSizeForLayer: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (extraPacketSizeForLayer))?;
        Ok(__cordl_object)
    }
    pub fn ProcessInboundPacket(
        &mut self,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessInboundPacket", (remoteEndPoint, data, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessOutBoundPacket(
        &mut self,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        length: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessOutBoundPacket", (remoteEndPoint, data, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        extraPacketSizeForLayer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (extraPacketSizeForLayer))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LiteNetLib+Layers+PacketLayerBase")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::Layers::PacketLayerBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
