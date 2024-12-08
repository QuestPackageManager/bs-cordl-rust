#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceRequestPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct NatPunchModule_NatIntroduceRequestPacket {
    __cordl_parent: crate::System::Object,
    pub _Internal_k__BackingField: *mut crate::System::Net::IPEndPoint,
    pub _Token_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceRequestPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket => "LiteNetLib"
    ."NatPunchModule/NatIntroduceRequestPacket"
);
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceRequestPacket")]
impl std::ops::Deref for crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceRequestPacket")]
impl std::ops::DerefMut for crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceRequestPacket")]
impl crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Internal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::IPEndPoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::IPEndPoint = __cordl_object
            .invoke("get_Internal", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Token(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Token", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Internal(
        &mut self,
        value: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Internal", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Token(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Token", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceRequestPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceResponsePacket")]
#[repr(C)]
#[derive(Debug)]
pub struct NatPunchModule_NatIntroduceResponsePacket {
    __cordl_parent: crate::System::Object,
    pub _Internal_k__BackingField: *mut crate::System::Net::IPEndPoint,
    pub _External_k__BackingField: *mut crate::System::Net::IPEndPoint,
    pub _Token_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceResponsePacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket => "LiteNetLib"
    ."NatPunchModule/NatIntroduceResponsePacket"
);
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceResponsePacket")]
impl std::ops::Deref for crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceResponsePacket")]
impl std::ops::DerefMut
for crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceResponsePacket")]
impl crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_External(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::IPEndPoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::IPEndPoint = __cordl_object
            .invoke("get_External", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Internal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::IPEndPoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::IPEndPoint = __cordl_object
            .invoke("get_Internal", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Token(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Token", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_External(
        &mut self,
        value: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_External", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Internal(
        &mut self,
        value: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Internal", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Token(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Token", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceResponsePacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule")]
#[repr(C)]
#[derive(Debug)]
pub struct NatPunchModule {
    __cordl_parent: crate::System::Object,
    pub _socket: *mut crate::LiteNetLib::NetSocket,
    pub _requestEvents: *mut crate::System::Collections::Generic::Queue_1<
        crate::LiteNetLib::NatPunchModule_RequestEventData,
    >,
    pub _successEvents: *mut crate::System::Collections::Generic::Queue_1<
        crate::LiteNetLib::NatPunchModule_SuccessEventData,
    >,
    pub _cacheReader: *mut crate::LiteNetLib::Utils::NetDataReader,
    pub _cacheWriter: *mut crate::LiteNetLib::Utils::NetDataWriter,
    pub _netPacketProcessor: *mut crate::LiteNetLib::Utils::NetPacketProcessor,
    pub _natPunchListener: *mut crate::LiteNetLib::INatPunchListener,
}
#[cfg(feature = "LiteNetLib+NatPunchModule")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NatPunchModule => "LiteNetLib"
    ."NatPunchModule"
);
#[cfg(feature = "LiteNetLib+NatPunchModule")]
impl std::ops::Deref for crate::LiteNetLib::NatPunchModule {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule")]
impl std::ops::DerefMut for crate::LiteNetLib::NatPunchModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule")]
impl crate::LiteNetLib::NatPunchModule {
    pub const MaxTokenLength: i32 = 256i32;
    #[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceResponsePacket")]
    pub type NatIntroduceResponsePacket = crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket;
    #[cfg(feature = "LiteNetLib+NatPunchModule+SuccessEventData")]
    pub type SuccessEventData = crate::LiteNetLib::NatPunchModule_SuccessEventData;
    #[cfg(feature = "LiteNetLib+NatPunchModule+NatPunchPacket")]
    pub type NatPunchPacket = crate::LiteNetLib::NatPunchModule_NatPunchPacket;
    #[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceRequestPacket")]
    pub type NatIntroduceRequestPacket = crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket;
    #[cfg(feature = "LiteNetLib+NatPunchModule+RequestEventData")]
    pub type RequestEventData = crate::LiteNetLib::NatPunchModule_RequestEventData;
    pub fn Init(
        &mut self,
        listener: *mut crate::LiteNetLib::INatPunchListener,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (listener))?;
        Ok(__cordl_ret)
    }
    pub fn NatIntroduce(
        &mut self,
        hostInternal: *mut crate::System::Net::IPEndPoint,
        hostExternal: *mut crate::System::Net::IPEndPoint,
        clientInternal: *mut crate::System::Net::IPEndPoint,
        clientExternal: *mut crate::System::Net::IPEndPoint,
        additionalInfo: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "NatIntroduce",
                (
                    hostInternal,
                    hostExternal,
                    clientInternal,
                    clientExternal,
                    additionalInfo,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        socket: *mut crate::LiteNetLib::NetSocket,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (socket))?;
        Ok(__cordl_object)
    }
    pub fn OnNatIntroductionRequest(
        &mut self,
        req: *mut crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket,
        senderEndPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNatIntroductionRequest", (req, senderEndPoint))?;
        Ok(__cordl_ret)
    }
    pub fn OnNatIntroductionResponse(
        &mut self,
        req: *mut crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNatIntroductionResponse", (req))?;
        Ok(__cordl_ret)
    }
    pub fn OnNatPunch(
        &mut self,
        req: *mut crate::LiteNetLib::NatPunchModule_NatPunchPacket,
        senderEndPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNatPunch", (req, senderEndPoint))?;
        Ok(__cordl_ret)
    }
    pub fn PollEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PollEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMessage(
        &mut self,
        senderEndPoint: *mut crate::System::Net::IPEndPoint,
        packet: *mut crate::LiteNetLib::NetPacket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMessage", (senderEndPoint, packet))?;
        Ok(__cordl_ret)
    }
    pub fn Send<T>(
        &mut self,
        packet: T,
        target: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (packet, target))?;
        Ok(__cordl_ret)
    }
    pub fn SendNatIntroduceRequest_IPEndPoint_String1(
        &mut self,
        masterServerEndPoint: *mut crate::System::Net::IPEndPoint,
        additionalInfo: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendNatIntroduceRequest", (masterServerEndPoint, additionalInfo))?;
        Ok(__cordl_ret)
    }
    pub fn SendNatIntroduceRequest_String_i32_String0(
        &mut self,
        host: *mut crate::System::String,
        port: i32,
        additionalInfo: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendNatIntroduceRequest", (host, port, additionalInfo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        socket: *mut crate::LiteNetLib::NetSocket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (socket))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NatPunchModule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatPunchPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct NatPunchModule_NatPunchPacket {
    __cordl_parent: crate::System::Object,
    pub _Token_k__BackingField: *mut crate::System::String,
    pub _IsExternal_k__BackingField: bool,
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatPunchPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NatPunchModule_NatPunchPacket =>
    "LiteNetLib"."NatPunchModule/NatPunchPacket"
);
#[cfg(feature = "LiteNetLib+NatPunchModule+NatPunchPacket")]
impl std::ops::Deref for crate::LiteNetLib::NatPunchModule_NatPunchPacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatPunchPacket")]
impl std::ops::DerefMut for crate::LiteNetLib::NatPunchModule_NatPunchPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatPunchPacket")]
impl crate::LiteNetLib::NatPunchModule_NatPunchPacket {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsExternal(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsExternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Token(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Token", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IsExternal(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsExternal", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Token(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Token", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatPunchPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::NatPunchModule_NatPunchPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+RequestEventData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NatPunchModule_RequestEventData {
    pub LocalEndPoint: *mut crate::System::Net::IPEndPoint,
    pub RemoteEndPoint: *mut crate::System::Net::IPEndPoint,
    pub Token: *mut crate::System::String,
}
#[cfg(feature = "LiteNetLib+NatPunchModule+RequestEventData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NatPunchModule_RequestEventData =>
    "LiteNetLib"."NatPunchModule/RequestEventData"
);
#[cfg(feature = "LiteNetLib+NatPunchModule+RequestEventData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LiteNetLib::NatPunchModule_RequestEventData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+RequestEventData")]
impl crate::LiteNetLib::NatPunchModule_RequestEventData {}
#[cfg(feature = "LiteNetLib+NatPunchModule+SuccessEventData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NatPunchModule_SuccessEventData {
    pub TargetEndPoint: *mut crate::System::Net::IPEndPoint,
    pub Type: crate::LiteNetLib::NatAddressType,
    pub Token: *mut crate::System::String,
}
#[cfg(feature = "LiteNetLib+NatPunchModule+SuccessEventData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NatPunchModule_SuccessEventData =>
    "LiteNetLib"."NatPunchModule/SuccessEventData"
);
#[cfg(feature = "LiteNetLib+NatPunchModule+SuccessEventData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LiteNetLib::NatPunchModule_SuccessEventData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+SuccessEventData")]
impl crate::LiteNetLib::NatPunchModule_SuccessEventData {}