#[cfg(feature = "LiteNetLib+NatPunchModule")]
#[repr(C)]
#[derive(Debug)]
pub struct NatPunchModule {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _socket: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetSocket>,
    pub _requestEvents: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<
            crate::LiteNetLib::NatPunchModule_RequestEventData,
        >,
    >,
    pub _successEvents: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<
            crate::LiteNetLib::NatPunchModule_SuccessEventData,
        >,
    >,
    pub _cacheReader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    pub _cacheWriter: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    pub _netPacketProcessor: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::Utils::NetPacketProcessor,
    >,
    pub _natPunchListener: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::INatPunchListener,
    >,
}
#[cfg(feature = "LiteNetLib+NatPunchModule")]
unsafe impl quest_hook::libil2cpp::Type for crate::LiteNetLib::NatPunchModule {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "NatPunchModule";
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
#[cfg(feature = "LiteNetLib+NatPunchModule")]
impl std::ops::Deref for crate::LiteNetLib::NatPunchModule {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    #[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceRequestPacket")]
    pub type NatIntroduceRequestPacket = crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket;
    #[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceResponsePacket")]
    pub type NatIntroduceResponsePacket = crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket;
    #[cfg(feature = "LiteNetLib+NatPunchModule+NatPunchPacket")]
    pub type NatPunchPacket = crate::LiteNetLib::NatPunchModule_NatPunchPacket;
    #[cfg(feature = "LiteNetLib+NatPunchModule+RequestEventData")]
    pub type RequestEventData = crate::LiteNetLib::NatPunchModule_RequestEventData;
    #[cfg(feature = "LiteNetLib+NatPunchModule+SuccessEventData")]
    pub type SuccessEventData = crate::LiteNetLib::NatPunchModule_SuccessEventData;
    pub fn Init(
        &mut self,
        listener: quest_hook::libil2cpp::Gc<crate::LiteNetLib::INatPunchListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::INatPunchListener>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type >
                    ::class(), "Init", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (listener))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NatIntroduce(
        &mut self,
        hostInternal: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        hostExternal: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        clientInternal: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        clientExternal: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        additionalInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                    quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                    quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                    quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("NatIntroduce")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type >
                    ::class(), "NatIntroduce", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        hostInternal,
                        hostExternal,
                        clientInternal,
                        clientExternal,
                        additionalInfo,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        socket: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetSocket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (socket))?;
        Ok(__cordl_object.into())
    }
    pub fn OnNatIntroductionRequest(
        &mut self,
        req: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket,
        >,
        senderEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnNatIntroductionRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type >
                    ::class(), "OnNatIntroductionRequest", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (req, senderEndPoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnNatIntroductionResponse(
        &mut self,
        req: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnNatIntroductionResponse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type >
                    ::class(), "OnNatIntroductionResponse", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (req))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnNatPunch(
        &mut self,
        req: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NatPunchModule_NatPunchPacket>,
        senderEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::LiteNetLib::NatPunchModule_NatPunchPacket,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("OnNatPunch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type >
                    ::class(), "OnNatPunch", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (req, senderEndPoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PollEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("PollEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type >
                    ::class(), "PollEvents", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMessage(
        &mut self,
        senderEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        packet: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPacket>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ProcessMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type >
                    ::class(), "ProcessMessage", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (senderEndPoint, packet))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Send<T>(
        &mut self,
        packet: T,
        target: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Send")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type >
                    ::class(), "Send", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (packet, target))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendNatIntroduceRequest_IPEndPoint_Il2CppString1(
        &mut self,
        masterServerEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        additionalInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SendNatIntroduceRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type >
                    ::class(), "SendNatIntroduceRequest", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (masterServerEndPoint, additionalInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendNatIntroduceRequest_Il2CppString_i32_Il2CppString0(
        &mut self,
        host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        port: i32,
        additionalInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SendNatIntroduceRequest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type >
                    ::class(), "SendNatIntroduceRequest", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (host, port, additionalInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        socket: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetSocket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetSocket>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (socket))?
        };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceRequestPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct NatPunchModule_NatIntroduceRequestPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Internal_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Net::IPEndPoint,
    >,
    pub _Token_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceRequestPacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "NatPunchModule/NatIntroduceRequestPacket";
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
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceRequestPacket")]
impl std::ops::Deref for crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Internal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                0usize,
            >("get_Internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket as
                    quest_hook::libil2cpp::Type > ::class(), "get_Internal", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Token(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Token")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket as
                    quest_hook::libil2cpp::Type > ::class(), "get_Token", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Internal(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket as
                    quest_hook::libil2cpp::Type > ::class(), "set_Internal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Token(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Token")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatIntroduceRequestPacket as
                    quest_hook::libil2cpp::Type > ::class(), "set_Token", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Internal_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Net::IPEndPoint,
    >,
    pub _External_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Net::IPEndPoint,
    >,
    pub _Token_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceResponsePacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "NatPunchModule/NatIntroduceResponsePacket";
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
#[cfg(feature = "LiteNetLib+NatPunchModule+NatIntroduceResponsePacket")]
impl std::ops::Deref for crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_External(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                0usize,
            >("get_External")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as
                    quest_hook::libil2cpp::Type > ::class(), "get_External", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Internal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                0usize,
            >("get_Internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as
                    quest_hook::libil2cpp::Type > ::class(), "get_Internal", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Token(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Token")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as
                    quest_hook::libil2cpp::Type > ::class(), "get_Token", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_External(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_External")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as
                    quest_hook::libil2cpp::Type > ::class(), "set_External", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Internal(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as
                    quest_hook::libil2cpp::Type > ::class(), "set_Internal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Token(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Token")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatIntroduceResponsePacket as
                    quest_hook::libil2cpp::Type > ::class(), "set_Token", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "LiteNetLib+NatPunchModule+NatPunchPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct NatPunchModule_NatPunchPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Token_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _IsExternal_k__BackingField: bool,
}
#[cfg(feature = "LiteNetLib+NatPunchModule+NatPunchPacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::LiteNetLib::NatPunchModule_NatPunchPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "NatPunchModule/NatPunchPacket";
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
#[cfg(feature = "LiteNetLib+NatPunchModule+NatPunchPacket")]
impl std::ops::Deref for crate::LiteNetLib::NatPunchModule_NatPunchPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatPunchPacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatPunchPacket as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsExternal(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatPunchPacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsExternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatPunchPacket as
                    quest_hook::libil2cpp::Type > ::class(), "get_IsExternal", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Token(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatPunchPacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Token")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatPunchPacket as
                    quest_hook::libil2cpp::Type > ::class(), "get_Token", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_IsExternal(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatPunchPacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_IsExternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatPunchPacket as
                    quest_hook::libil2cpp::Type > ::class(), "set_IsExternal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Token(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::NatPunchModule_NatPunchPacket as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Token")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::NatPunchModule_NatPunchPacket as
                    quest_hook::libil2cpp::Type > ::class(), "set_Token", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NatPunchModule_RequestEventData {
    pub LocalEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    pub RemoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    pub Token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "LiteNetLib+NatPunchModule+RequestEventData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::LiteNetLib::NatPunchModule_RequestEventData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "NatPunchModule/RequestEventData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+RequestEventData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::LiteNetLib::NatPunchModule_RequestEventData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+RequestEventData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::LiteNetLib::NatPunchModule_RequestEventData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+RequestEventData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::LiteNetLib::NatPunchModule_RequestEventData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+RequestEventData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::LiteNetLib::NatPunchModule_RequestEventData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NatPunchModule_SuccessEventData {
    pub TargetEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    pub Type: crate::LiteNetLib::NatAddressType,
    pub Token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "LiteNetLib+NatPunchModule+SuccessEventData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::LiteNetLib::NatPunchModule_SuccessEventData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "NatPunchModule/SuccessEventData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+SuccessEventData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::LiteNetLib::NatPunchModule_SuccessEventData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+SuccessEventData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::LiteNetLib::NatPunchModule_SuccessEventData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+SuccessEventData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::LiteNetLib::NatPunchModule_SuccessEventData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "LiteNetLib+NatPunchModule+SuccessEventData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::LiteNetLib::NatPunchModule_SuccessEventData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
