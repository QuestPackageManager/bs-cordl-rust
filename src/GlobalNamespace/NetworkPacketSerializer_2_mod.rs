#[cfg(feature = "NetworkPacketSerializer_2")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkPacketSerializer_2<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _messsageHandlers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            u8,
            quest_hook::libil2cpp::Gc<
                crate::System::Action_3<
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                    i32,
                    TData,
                >,
            >,
        >,
    >,
    pub _typeRegistry: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            u8,
        >,
    >,
    pub _subSerializerRegistry: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::INetworkPacketSubSerializer_1<TData>,
            >,
            u8,
        >,
    >,
    pub _internalWriter: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::Utils::NetDataWriter,
    >,
    __cordl_phantom_TType: std::marker::PhantomData<TType>,
    __cordl_phantom_TData: std::marker::PhantomData<TData>,
}
#[cfg(feature = "NetworkPacketSerializer_2")]
unsafe impl<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NetworkPacketSerializer_2<TType, TData> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NetworkPacketSerializer`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find("", "NetworkPacketSerializer`2")
                    .unwrap()
                    .make_generic::<(TType, TData)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "NetworkPacketSerializer_2")]
impl<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::GlobalNamespace::NetworkPacketSerializer_2<TType, TData> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkPacketSerializer_2")]
impl<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::GlobalNamespace::NetworkPacketSerializer_2<TType, TData> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkPacketSerializer_2")]
impl<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::NetworkPacketSerializer_2<TType, TData> {
    pub fn CopyFrom(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NetworkPacketSerializer_2<TType, TData>,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandlesType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
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
        Ok(__cordl_ret.into())
    }
    pub fn INetworkPacketSubSerializer_TData__Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
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
        Ok(__cordl_ret.into())
    }
    pub fn INetworkPacketSubSerializer_TData__Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>,
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
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessAllPackets(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
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
        Ok(__cordl_ret.into())
    }
    pub fn ProcessPacket(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
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
        Ok(__cordl_ret.into())
    }
    pub fn ProcessPacketInternal(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
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
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCallback_Action_1_0<TPacket>(
        &mut self,
        packetType: TType,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TPacket>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCallback_Action_1_Func_1_1<TPacket>(
        &mut self,
        packetType: TType,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TPacket>>,
        constructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<TPacket>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCallback_Action_2_2<TPacket>(
        &mut self,
        packetType: TType,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_2<TPacket, TData>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCallback_Action_2_Func_1_3<TPacket>(
        &mut self,
        packetType: TType,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_2<TPacket, TData>>,
        constructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<TPacket>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCallback_Action_2_Func_2_4<TPacket>(
        &mut self,
        packetType: TType,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_2<TPacket, TData>>,
        constructor: quest_hook::libil2cpp::Gc<crate::System::Func_2<TData, TPacket>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn RegisterSubSerializer(
        &mut self,
        packetType: TType,
        subSubSerializer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::INetworkPacketSubSerializer_1<TData>,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn SerializePacket(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>,
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
        Ok(__cordl_ret.into())
    }
    pub fn SerializePacketInternal(
        &mut self,
        externalWriter: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetDataWriter,
        >,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::INetSerializable>,
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
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPacketType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        packetType: quest_hook::libil2cpp::ByRefMut<u8>,
        subSerializer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::INetworkPacketSubSerializer_1<TData>,
            >,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterSubSerializer(
        &mut self,
        packetType: TType,
        subSubSerializer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::INetworkPacketSubSerializer_1<TData>,
        >,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NetworkPacketSerializer_2")]
impl<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NetworkPacketSerializer_2<TType, TData> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NetworkPacketSerializer_2")]
impl<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> AsRef<crate::GlobalNamespace::INetworkPacketSerializer_1<TData>>
for crate::GlobalNamespace::NetworkPacketSerializer_2<TType, TData> {
    fn as_ref(&self) -> &crate::GlobalNamespace::INetworkPacketSerializer_1<TData> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NetworkPacketSerializer_2")]
impl<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> AsMut<crate::GlobalNamespace::INetworkPacketSerializer_1<TData>>
for crate::GlobalNamespace::NetworkPacketSerializer_2<TType, TData> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INetworkPacketSerializer_1<TData> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NetworkPacketSerializer_2")]
impl<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> AsRef<crate::GlobalNamespace::INetworkPacketSubSerializer_1<TData>>
for crate::GlobalNamespace::NetworkPacketSerializer_2<TType, TData> {
    fn as_ref(&self) -> &crate::GlobalNamespace::INetworkPacketSubSerializer_1<TData> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NetworkPacketSerializer_2")]
impl<
    TType: quest_hook::libil2cpp::Type,
    TData: quest_hook::libil2cpp::Type,
> AsMut<crate::GlobalNamespace::INetworkPacketSubSerializer_1<TData>>
for crate::GlobalNamespace::NetworkPacketSerializer_2<TType, TData> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INetworkPacketSubSerializer_1<TData> {
        unsafe { std::mem::transmute(self) }
    }
}
