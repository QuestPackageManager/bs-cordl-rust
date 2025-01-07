#[cfg(feature = "LiteNetLib+NetSocket")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSocket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _udpSocketv4: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    pub _udpSocketv6: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    pub _threadv4: quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
    pub _threadv6: quest_hook::libil2cpp::Gc<crate::System::Threading::Thread>,
    pub _listener: quest_hook::libil2cpp::Gc<crate::LiteNetLib::INetSocketListener>,
    pub _LocalPort_k__BackingField: i32,
    pub IsRunning: bool,
}
#[cfg(feature = "LiteNetLib+NetSocket")]
unsafe impl quest_hook::libil2cpp::Type for crate::LiteNetLib::NetSocket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib";
    const CLASS_NAME: &'static str = "NetSocket";
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
#[cfg(feature = "LiteNetLib+NetSocket")]
impl std::ops::Deref for crate::LiteNetLib::NetSocket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetSocket")]
impl std::ops::DerefMut for crate::LiteNetLib::NetSocket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetSocket")]
impl crate::LiteNetLib::NetSocket {
    pub const ReceivePollingTime: i32 = 500000i32;
    pub const SioUdpConnreset: i32 = -1744830452i32;
    pub fn Bind(
        &mut self,
        addressIPv4: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
        addressIPv6: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
        port: i32,
        reuseAddress: bool,
        ipv6: bool,
        priority: crate::System::Threading::ThreadPriority,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "Bind",
                (addressIPv4, addressIPv6, port, reuseAddress, ipv6, priority),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BindSocket(
        &mut self,
        socket: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
        ep: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        reuseAddress: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("BindSocket", (socket, ep, reuseAddress))?;
        Ok(__cordl_ret.into())
    }
    pub fn Close(
        &mut self,
        suspend: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", (suspend))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsActive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        listener: quest_hook::libil2cpp::Gc<crate::LiteNetLib::INetSocketListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listener))?;
        Ok(__cordl_object.into())
    }
    pub fn ReceiveLogic(
        &mut self,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReceiveLogic", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendBroadcast(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SendBroadcast", (data, offset, _cordl_size, port))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendTo(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        _cordl_size: i32,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("SendTo", (data, offset, _cordl_size, remoteEndPoint, errorCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        listener: quest_hook::libil2cpp::Gc<crate::LiteNetLib::INetSocketListener>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (listener))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LocalPort", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Ttl(&mut self) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i16 = __cordl_object.invoke("get_Ttl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LocalPort(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LocalPort", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Ttl(
        &mut self,
        value: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Ttl", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+NetSocket")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetSocket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
