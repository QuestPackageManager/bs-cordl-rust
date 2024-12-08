#[cfg(feature = "LiteNetLib+NetSocket")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSocket {
    __cordl_parent: crate::System::Object,
    pub _udpSocketv4: *mut crate::System::Net::Sockets::Socket,
    pub _udpSocketv6: *mut crate::System::Net::Sockets::Socket,
    pub _threadv4: *mut crate::System::Threading::Thread,
    pub _threadv6: *mut crate::System::Threading::Thread,
    pub _listener: *mut crate::LiteNetLib::INetSocketListener,
    pub _LocalPort_k__BackingField: i32,
    pub IsRunning: bool,
}
#[cfg(feature = "LiteNetLib+NetSocket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetSocket => "LiteNetLib"
    ."NetSocket"
);
#[cfg(feature = "LiteNetLib+NetSocket")]
impl std::ops::Deref for crate::LiteNetLib::NetSocket {
    type Target = crate::System::Object;
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
        addressIPv4: *mut crate::System::Net::IPAddress,
        addressIPv6: *mut crate::System::Net::IPAddress,
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
        Ok(__cordl_ret)
    }
    pub fn BindSocket(
        &mut self,
        socket: *mut crate::System::Net::Sockets::Socket,
        ep: *mut crate::System::Net::IPEndPoint,
        reuseAddress: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("BindSocket", (socket, ep, reuseAddress))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn IsActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsActive", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        listener: *mut crate::LiteNetLib::INetSocketListener,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listener))?;
        Ok(__cordl_object)
    }
    pub fn ReceiveLogic(
        &mut self,
        state: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReceiveLogic", (state))?;
        Ok(__cordl_ret)
    }
    pub fn SendBroadcast(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SendBroadcast", (data, offset, _cordl_size, port))?;
        Ok(__cordl_ret)
    }
    pub fn SendTo(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
        errorCode: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Sockets::SocketError,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("SendTo", (data, offset, _cordl_size, remoteEndPoint, errorCode))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        listener: *mut crate::LiteNetLib::INetSocketListener,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (listener))?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LocalPort", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Ttl(&mut self) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i16 = __cordl_object.invoke("get_Ttl", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
