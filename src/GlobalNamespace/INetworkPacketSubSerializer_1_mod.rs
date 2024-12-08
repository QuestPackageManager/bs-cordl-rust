#[cfg(feature = "INetworkPacketSubSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct INetworkPacketSubSerializer_1<TData: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TData: std::marker::PhantomData<TData>,
}
#[cfg(feature = "INetworkPacketSubSerializer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for INetworkPacketSubSerializer_1 < TData > => ""
    ."INetworkPacketSubSerializer`1" < TData >
);
#[cfg(feature = "INetworkPacketSubSerializer_1")]
impl<TData: quest_hook::libil2cpp::Type> std::ops::Deref
for INetworkPacketSubSerializer_1<TData> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "INetworkPacketSubSerializer_1")]
impl<TData: quest_hook::libil2cpp::Type> std::ops::DerefMut
for INetworkPacketSubSerializer_1<TData> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "INetworkPacketSubSerializer_1")]
impl<TData: quest_hook::libil2cpp::Type> INetworkPacketSubSerializer_1<TData> {
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        length: i32,
        data: TData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader, length, data))?;
        Ok(__cordl_ret)
    }
    pub fn HandlesType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HandlesType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        packet: *mut crate::LiteNetLib::Utils::INetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer, packet))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "INetworkPacketSubSerializer_1")]
impl<TData: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for INetworkPacketSubSerializer_1<TData> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}