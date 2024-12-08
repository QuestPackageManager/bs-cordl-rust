#[cfg(feature = "NetworkPacketSerializer_2")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkPacketSerializer_2<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Object,
    pub _messsageHandlers: *mut crate::System::Collections::Generic::Dictionary_2<
        u8,
        *mut crate::System::Action_3<
            *mut crate::LiteNetLib::Utils::NetDataReader,
            i32,
            TData,
        >,
    >,
    pub _typeRegistry: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Type,
        u8,
    >,
    pub _subSerializerRegistry: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut INetworkPacketSubSerializer_1<TData>,
        u8,
    >,
    pub _internalWriter: *mut crate::LiteNetLib::Utils::NetDataWriter,
    __cordl_phantom_TType: std::marker::PhantomData<TType>,
    __cordl_phantom_TData: std::marker::PhantomData<TData>,
}
#[cfg(feature = "NetworkPacketSerializer_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NetworkPacketSerializer_2 < TType, TData > => ""
    ."NetworkPacketSerializer`2" < TType, TData >
);
#[cfg(feature = "NetworkPacketSerializer_2")]
impl<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> std::ops::Deref for NetworkPacketSerializer_2<TType, TData> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkPacketSerializer_2")]
impl<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for NetworkPacketSerializer_2<TType, TData> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkPacketSerializer_2")]
impl<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> NetworkPacketSerializer_2<TType, TData> {
    #[cfg(feature = "NetworkPacketSerializer_2+__c__DisplayClass5_0_1")]
    pub type __c__DisplayClass5_0_1<TPacket: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::NetworkPacketSerializer_2___c__DisplayClass5_0_1<
        TType,
        TData,
        TPacket,
    >;
    #[cfg(feature = "NetworkPacketSerializer_2+__c__DisplayClass8_0_1")]
    pub type __c__DisplayClass8_0_1<TPacket: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::NetworkPacketSerializer_2___c__DisplayClass8_0_1<
        TType,
        TData,
        TPacket,
    >;
    #[cfg(feature = "NetworkPacketSerializer_2+__c__6_1")]
    pub type __c__6_1<TPacket: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::NetworkPacketSerializer_2___c__6_1<
        TType,
        TData,
        TPacket,
    >;
    #[cfg(feature = "NetworkPacketSerializer_2+__c__DisplayClass7_0_1")]
    pub type __c__DisplayClass7_0_1<TPacket: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::NetworkPacketSerializer_2___c__DisplayClass7_0_1<
        TType,
        TData,
        TPacket,
    >;
    #[cfg(feature = "NetworkPacketSerializer_2+__c__DisplayClass4_0_1")]
    pub type __c__DisplayClass4_0_1<TPacket: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::NetworkPacketSerializer_2___c__DisplayClass4_0_1<
        TType,
        TData,
        TPacket,
    >;
    #[cfg(feature = "NetworkPacketSerializer_2+__c__DisplayClass10_0")]
    pub type __c__DisplayClass10_0 = crate::GlobalNamespace::NetworkPacketSerializer_2___c__DisplayClass10_0<
        TType,
        TData,
    >;
    pub fn SerializePacketInternal(
        &mut self,
        externalWriter: *mut crate::LiteNetLib::Utils::NetDataWriter,
        packet: *mut crate::LiteNetLib::Utils::INetSerializable,
        prependLength: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SerializePacketInternal", (externalWriter, packet, prependLength))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPacketInternal(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        length: i32,
        data: TData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPacketInternal", (reader, length, data))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterSubSerializer(
        &mut self,
        packetType: TType,
        subSubSerializer: *mut INetworkPacketSubSerializer_1<TData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterSubSerializer", (packetType, subSubSerializer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterSubSerializer(
        &mut self,
        packetType: TType,
        subSubSerializer: *mut INetworkPacketSubSerializer_1<TData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterSubSerializer", (packetType, subSubSerializer))?;
        Ok(__cordl_ret)
    }
    pub fn CopyFrom(
        &mut self,
        other: *mut NetworkPacketSerializer_2<TType, TData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFrom", (other))?;
        Ok(__cordl_ret)
    }
    pub fn SerializePacket(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        packet: *mut crate::LiteNetLib::Utils::INetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SerializePacket", (writer, packet))?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterCallback<TPacket>(
        &mut self,
        packetType: TType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPacket: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterCallback", (packetType))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallback_Action_1_0<TPacket>(
        &mut self,
        packetType: TType,
        callback: *mut crate::System::Action_1<TPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPacket: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (packetType, callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallback_Action_1_Func_1_1<TPacket>(
        &mut self,
        packetType: TType,
        callback: *mut crate::System::Action_1<TPacket>,
        constructor: *mut crate::System::Func_1<TPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPacket: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (packetType, callback, constructor))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallback_Action_2_2<TPacket>(
        &mut self,
        packetType: TType,
        callback: *mut crate::System::Action_2<TPacket, TData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPacket: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (packetType, callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallback_Action_2_Func_1_3<TPacket>(
        &mut self,
        packetType: TType,
        callback: *mut crate::System::Action_2<TPacket, TData>,
        constructor: *mut crate::System::Func_1<TPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPacket: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (packetType, callback, constructor))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallback_Action_2_Func_2_4<TPacket>(
        &mut self,
        packetType: TType,
        callback: *mut crate::System::Action_2<TPacket, TData>,
        constructor: *mut crate::System::Func_2<TData, TPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPacket: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (packetType, callback, constructor))?;
        Ok(__cordl_ret)
    }
    pub fn INetworkPacketSubSerializer_TData__Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        length: i32,
        data: TData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "INetworkPacketSubSerializer<TData>.Deserialize",
                (reader, length, data),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Log(
        &mut self,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (message))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessAllPackets(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        data: TData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAllPackets", (reader, data))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPacket(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        data: TData,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessPacket", (reader, data))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetPacketType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        packetType: quest_hook::libil2cpp::ByRefMut<u8>,
        subSerializer: quest_hook::libil2cpp::ByRefMut<
            *mut INetworkPacketSubSerializer_1<TData>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetPacketType", (_cordl_type, packetType, subSerializer))?;
        Ok(__cordl_ret)
    }
    pub fn INetworkPacketSubSerializer_TData__Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        packet: *mut crate::LiteNetLib::Utils::INetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("INetworkPacketSubSerializer<TData>.Serialize", (writer, packet))?;
        Ok(__cordl_ret)
    }
    pub fn HandlesType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HandlesType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "NetworkPacketSerializer_2")]
impl<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType for NetworkPacketSerializer_2<TType, TData> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
