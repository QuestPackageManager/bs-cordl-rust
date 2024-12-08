#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+HashCache_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetPacketProcessor_HashCache_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+HashCache_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetPacketProcessor_HashCache_1 < T > => "LiteNetLib.Utils"
    ."NetPacketProcessor/HashCache`1" < T >
);
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+HashCache_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetPacketProcessor_HashCache_1<T> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+HashCache_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetPacketProcessor_HashCache_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+HashCache_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetPacketProcessor_HashCache_1<T> {}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+HashCache_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetPacketProcessor_HashCache_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct NetPacketProcessor {
    __cordl_parent: crate::System::Object,
    pub _netSerializer: *mut crate::LiteNetLib::Utils::NetSerializer,
    pub _callbacks: *mut crate::System::Collections::Generic::Dictionary_2<
        u64,
        *mut crate::LiteNetLib::Utils::NetPacketProcessor_SubscribeDelegate,
    >,
    pub _netDataWriter: *mut crate::LiteNetLib::Utils::NetDataWriter,
}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::NetPacketProcessor =>
    "LiteNetLib.Utils"."NetPacketProcessor"
);
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor")]
impl std::ops::Deref for crate::LiteNetLib::Utils::NetPacketProcessor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor")]
impl std::ops::DerefMut for crate::LiteNetLib::Utils::NetPacketProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor")]
impl crate::LiteNetLib::Utils::NetPacketProcessor {
    #[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+__c__DisplayClass27_0_1")]
    pub type __c__DisplayClass27_0_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetPacketProcessor___c__DisplayClass27_0_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+__c__DisplayClass31_0_2")]
    pub type __c__DisplayClass31_0_2<
        T: quest_hook::libil2cpp::Type,
        TUserData: quest_hook::libil2cpp::Type,
    > = crate::LiteNetLib::Utils::NetPacketProcessor___c__DisplayClass31_0_2<
        T,
        TUserData,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+SubscribeDelegate")]
    pub type SubscribeDelegate = crate::LiteNetLib::Utils::NetPacketProcessor_SubscribeDelegate;
    #[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+__c__DisplayClass28_0_2")]
    pub type __c__DisplayClass28_0_2<
        T: quest_hook::libil2cpp::Type,
        TUserData: quest_hook::libil2cpp::Type,
    > = crate::LiteNetLib::Utils::NetPacketProcessor___c__DisplayClass28_0_2<
        T,
        TUserData,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+__c__DisplayClass29_0_2")]
    pub type __c__DisplayClass29_0_2<
        T: quest_hook::libil2cpp::Type,
        TUserData: quest_hook::libil2cpp::Type,
    > = crate::LiteNetLib::Utils::NetPacketProcessor___c__DisplayClass29_0_2<
        T,
        TUserData,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+HashCache_1")]
    pub type HashCache_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetPacketProcessor_HashCache_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+__c__DisplayClass25_0_1")]
    pub type __c__DisplayClass25_0_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetPacketProcessor___c__DisplayClass25_0_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+__c__DisplayClass26_0_2")]
    pub type __c__DisplayClass26_0_2<
        T: quest_hook::libil2cpp::Type,
        TUserData: quest_hook::libil2cpp::Type,
    > = crate::LiteNetLib::Utils::NetPacketProcessor___c__DisplayClass26_0_2<
        T,
        TUserData,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+__c__DisplayClass30_0_1")]
    pub type __c__DisplayClass30_0_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetPacketProcessor___c__DisplayClass30_0_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+__c__DisplayClass32_0_1")]
    pub type __c__DisplayClass32_0_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetPacketProcessor___c__DisplayClass32_0_1<
        T,
    >;
    pub fn ReadPacket_NetDataReader0(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadPacket", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn ReadPacket_Object1(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        userData: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadPacket", (reader, userData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_1(
        &mut self,
        maxStringLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (maxStringLength))?;
        Ok(__cordl_ret)
    }
    pub fn WriteHash<T>(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteHash", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterNestedType_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterNestedType", ())?;
        Ok(__cordl_ret)
    }
    pub fn RegisterNestedType_Action_2_Func_2_1<T>(
        &mut self,
        writeDelegate: *mut crate::System::Action_2<
            *mut crate::LiteNetLib::Utils::NetDataWriter,
            T,
        >,
        readDelegate: *mut crate::System::Func_2<
            *mut crate::LiteNetLib::Utils::NetDataReader,
            T,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterNestedType", (writeDelegate, readDelegate))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterNestedType_Func_1_2<T>(
        &mut self,
        constructor: *mut crate::System::Func_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterNestedType", (constructor))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAllPackets_NetDataReader0(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAllPackets", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAllPackets_Object1(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        userData: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadAllPackets", (reader, userData))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveSubscription<T>(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RemoveSubscription", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendNetSerializable_NetPeer0<T>(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        packet: T,
        options: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendNetSerializable", (peer, packet, options))?;
        Ok(__cordl_ret)
    }
    pub fn SendNetSerializable_NetManager1<T>(
        &mut self,
        manager: *mut crate::LiteNetLib::NetManager,
        packet: T,
        options: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendNetSerializable", (manager, packet, options))?;
        Ok(__cordl_ret)
    }
    pub fn GetHash<T>(&mut self) -> quest_hook::libil2cpp::Result<u64>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("GetHash", ())?;
        Ok(__cordl_ret)
    }
    pub fn Send_NetPeer0<T>(
        &mut self,
        peer: *mut crate::LiteNetLib::NetPeer,
        packet: T,
        options: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (peer, packet, options))?;
        Ok(__cordl_ret)
    }
    pub fn Send_NetManager1<T>(
        &mut self,
        manager: *mut crate::LiteNetLib::NetManager,
        packet: T,
        options: crate::LiteNetLib::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (manager, packet, options))?;
        Ok(__cordl_ret)
    }
    pub fn Subscribe_Action_1_0<T>(
        &mut self,
        onReceive: *mut crate::System::Action_1<T>,
        packetConstructor: *mut crate::System::Func_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Subscribe", (onReceive, packetConstructor))?;
        Ok(__cordl_ret)
    }
    pub fn Subscribe_Action_2_1<T, TUserData>(
        &mut self,
        onReceive: *mut crate::System::Action_2<T, TUserData>,
        packetConstructor: *mut crate::System::Func_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TUserData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Subscribe", (onReceive, packetConstructor))?;
        Ok(__cordl_ret)
    }
    pub fn SubscribeReusable_Action_1_0<T>(
        &mut self,
        onReceive: *mut crate::System::Action_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeReusable", (onReceive))?;
        Ok(__cordl_ret)
    }
    pub fn SubscribeReusable_Action_2_1<T, TUserData>(
        &mut self,
        onReceive: *mut crate::System::Action_2<T, TUserData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TUserData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeReusable", (onReceive))?;
        Ok(__cordl_ret)
    }
    pub fn SubscribeNetSerializable_Action_2_Func_1_0<T, TUserData>(
        &mut self,
        onReceive: *mut crate::System::Action_2<T, TUserData>,
        packetConstructor: *mut crate::System::Func_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TUserData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeNetSerializable", (onReceive, packetConstructor))?;
        Ok(__cordl_ret)
    }
    pub fn SubscribeNetSerializable_Action_1_Func_1_1<T>(
        &mut self,
        onReceive: *mut crate::System::Action_1<T>,
        packetConstructor: *mut crate::System::Func_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeNetSerializable", (onReceive, packetConstructor))?;
        Ok(__cordl_ret)
    }
    pub fn SubscribeNetSerializable_Action_2_2<T, TUserData>(
        &mut self,
        onReceive: *mut crate::System::Action_2<T, TUserData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TUserData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeNetSerializable", (onReceive))?;
        Ok(__cordl_ret)
    }
    pub fn SubscribeNetSerializable_Action_1_3<T>(
        &mut self,
        onReceive: *mut crate::System::Action_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeNetSerializable", (onReceive))?;
        Ok(__cordl_ret)
    }
    pub fn Write_NetDataWriter_T0<T>(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        packet: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (writer, packet))?;
        Ok(__cordl_ret)
    }
    pub fn Write_T1<T>(
        &mut self,
        packet: T,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Write", (packet))?;
        Ok(__cordl_ret)
    }
    pub fn WriteNetSerializable_NetDataWriter_T0<T>(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        packet: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteNetSerializable", (writer, packet))?;
        Ok(__cordl_ret)
    }
    pub fn WriteNetSerializable_T1<T>(
        &mut self,
        packet: T,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("WriteNetSerializable", (packet))?;
        Ok(__cordl_ret)
    }
    pub fn GetCallbackFromData(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::LiteNetLib::Utils::NetPacketProcessor_SubscribeDelegate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::Utils::NetPacketProcessor_SubscribeDelegate = __cordl_object
            .invoke("GetCallbackFromData", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_i32_1(
        maxStringLength: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxStringLength))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::Utils::NetPacketProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+SubscribeDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct NetPacketProcessor_SubscribeDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+SubscribeDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetPacketProcessor_SubscribeDelegate => "LiteNetLib.Utils"
    ."NetPacketProcessor/SubscribeDelegate"
);
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+SubscribeDelegate")]
impl std::ops::Deref for crate::LiteNetLib::Utils::NetPacketProcessor_SubscribeDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+SubscribeDelegate")]
impl std::ops::DerefMut
for crate::LiteNetLib::Utils::NetPacketProcessor_SubscribeDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+SubscribeDelegate")]
impl crate::LiteNetLib::Utils::NetPacketProcessor_SubscribeDelegate {
    pub fn Invoke(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        userData: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (reader, userData))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn BeginInvoke(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        userData: *mut crate::System::Object,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (reader, userData, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetPacketProcessor+SubscribeDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetPacketProcessor_SubscribeDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
